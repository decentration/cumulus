// Copyright 2023 Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

// TODO [now]: docs + rename file here

use codec::{Decode, Encode};
use cumulus_client_collator::service::ServiceInterface as CollatorServiceInterface;
use cumulus_client_consensus_common::{
	ParachainBlockImportMarker, ParachainCandidate, ParentSearchParams,
};
use cumulus_client_consensus_proposer::ProposerInterface;
use cumulus_primitives_core::{
	relay_chain::Hash as PHash, CollectCollationInfo, PersistedValidationData,
};
use cumulus_primitives_parachain_inherent::ParachainInherentData;
use cumulus_relay_chain_interface::RelayChainInterface;

use polkadot_node_primitives::{CollationResult, MaybeCompressedPoV};
use polkadot_overseer::Handle as OverseerHandle;
use polkadot_primitives::{Block as PBlock, CollatorPair, Header as PHeader, Id as ParaId};

use futures::prelude::*;
use sc_client_api::{backend::AuxStore, BlockBackend, BlockOf};
use sc_consensus::{
	import_queue::{BasicQueue, Verifier as VerifierT},
	BlockImport, BlockImportParams, ForkChoiceStrategy, StateAction,
};
use sc_consensus_aura::standalone as aura_internal;
use sc_telemetry::{telemetry, TelemetryHandle, CONSENSUS_DEBUG, CONSENSUS_TRACE};
use sp_api::ProvideRuntimeApi;
use sp_application_crypto::AppPublic;
use sp_block_builder::BlockBuilder as BlockBuilderApi;
use sp_blockchain::HeaderBackend;
use sp_consensus::{error::Error as ConsensusError, BlockOrigin, SyncOracle};
use sp_consensus_aura::{AuraApi, Slot, SlotDuration};
use sp_core::crypto::Pair;
use sp_inherents::{CreateInherentDataProviders, InherentData, InherentDataProvider};
use sp_keystore::KeystorePtr;
use sp_runtime::{
	generic::Digest,
	traits::{Block as BlockT, HashFor, Header as HeaderT, Member},
};
use sp_state_machine::StorageChanges;
use sp_timestamp::Timestamp;
use std::{convert::TryFrom, error::Error, fmt::Debug, hash::Hash, sync::Arc, time::Duration};

struct Verifier<P, Client, Block, CIDP> {
	client: Arc<Client>,
	create_inherent_data_providers: CIDP,
	slot_duration: SlotDuration,
	telemetry: Option<TelemetryHandle>,
	_marker: std::marker::PhantomData<(Block, P)>,
}

#[async_trait::async_trait]
impl<P, Client, Block, CIDP> VerifierT<Block> for Verifier<P, Client, Block, CIDP>
where
	P: Pair,
	P::Signature: Encode + Decode,
	P::Public: Encode + Decode + PartialEq + Clone + Debug,
	Block: BlockT,
	Client: ProvideRuntimeApi<Block> + Send + Sync,
	<Client as ProvideRuntimeApi<Block>>::Api: BlockBuilderApi<Block> + AuraApi<Block, P::Public>,

	CIDP: CreateInherentDataProviders<Block, ()>,
{
	async fn verify(
		&mut self,
		mut block_params: BlockImportParams<Block, ()>,
	) -> Result<BlockImportParams<Block, ()>, String> {
		// Skip checks that include execution, if being told so, or when importing only state.
		//
		// This is done for example when gap syncing and it is expected that the block after the gap
		// was checked/chosen properly, e.g. by warp syncing to this block using a finality proof.
		if block_params.state_action.skip_execution_checks() || block_params.with_state() {
			return Ok(block_params)
		}

		let post_hash = block_params.header.hash();
		let parent_hash = *block_params.header.parent_hash();

		// check seal and update pre-hash/post-hash
		{
			let authorities = aura_internal::fetch_authorities(self.client.as_ref(), parent_hash)
				.map_err(|e| {
				format!("Could not fetch authorities at {:?}: {}", parent_hash, e)
			})?;

			let slot_now = slot_now(self.slot_duration);
			let res = aura_internal::check_header_slot_and_seal::<Block, P>(
				slot_now,
				block_params.header,
				&authorities,
			);

			match res {
				Ok((pre_header, _slot, seal_digest)) => {
					telemetry!(
						self.telemetry;
						CONSENSUS_TRACE;
						"aura.checked_and_importing";
						"pre_header" => ?pre_header,
					);

					block_params.header = pre_header;
					block_params.post_digests.push(seal_digest);
					block_params.fork_choice = Some(ForkChoiceStrategy::LongestChain);
					block_params.post_hash = Some(post_hash);
				},
				Err(aura_internal::SealVerificationError::Deferred(hdr, slot)) => {
					telemetry!(
						self.telemetry;
						CONSENSUS_DEBUG;
						"aura.header_too_far_in_future";
						"hash" => ?post_hash,
						"a" => ?hdr,
						"b" => ?slot,
					);

					return Err(format!(
						"Rejecting block ({:?}) from future slot {:?}",
						post_hash, slot
					))
				},
				Err(e) =>
					return Err(format!(
						"Rejecting block ({:?}) with invalid seal ({:?})",
						post_hash, e
					)),
			}
		}

		// check inherents.
		if let Some(body) = block_params.body.clone() {
			let block = Block::new(block_params.header.clone(), body);
			let create_inherent_data_providers = self
				.create_inherent_data_providers
				.create_inherent_data_providers(parent_hash, ())
				.await
				.map_err(|e| format!("Could not create inherent data {:?}", e))?;

			let inherent_data = create_inherent_data_providers
				.create_inherent_data()
				.await
				.map_err(|e| format!("Could not create inherent data {:?}", e))?;

			let inherent_res = self
				.client
				.runtime_api()
				.check_inherents_with_context(
					parent_hash,
					block_params.origin.into(),
					block,
					inherent_data,
				)
				.map_err(|e| format!("Unable to check block inherents {:?}", e))?;

			if !inherent_res.ok() {
				for (i, e) in inherent_res.into_errors() {
					match create_inherent_data_providers.try_handle_error(&i, &e).await {
						Some(res) => res.map_err(|e| format!("Inherent Error {:?}", e))?,
						None =>
							return Err(format!(
								"Unknown inherent error, source {:?}",
								String::from_utf8_lossy(&i[..])
							)),
					}
				}
			}
		}

		Ok(block_params)
	}
}

fn slot_now(slot_duration: SlotDuration) -> Slot {
	let timestamp = sp_timestamp::InherentDataProvider::from_system_time().timestamp();
	Slot::from_timestamp(timestamp, slot_duration)
}

/// Start an import queue for a Cumulus node which checks blocks' seals and inherent data.
///
/// Pass in only inherent data providers which don't include aura or parachain consensus inherents,
/// e.g. things like timestamp and custom inherents for the runtime.
///
/// The others are generated explicitly internally.
///
/// This should only be used for runtimes where the runtime does not check all inherents and
/// seals in `execute_block` (see <https://github.com/paritytech/cumulus/issues/2436>)
pub fn fully_verifying_import_queue<P, Client, Block: BlockT, I, CIDP>(
	client: Arc<Client>,
	block_import: I,
	create_inherent_data_providers: CIDP,
	slot_duration: SlotDuration,
	spawner: &impl sp_core::traits::SpawnEssentialNamed,
	registry: Option<&substrate_prometheus_endpoint::Registry>,
	telemetry: Option<TelemetryHandle>,
) -> BasicQueue<Block, I::Transaction>
where
	P: Pair,
	P::Signature: Encode + Decode,
	P::Public: Encode + Decode + PartialEq + Clone + Debug,
	I: BlockImport<Block, Error = ConsensusError>
		+ ParachainBlockImportMarker
		+ Send
		+ Sync
		+ 'static,
	I::Transaction: Send,
	Client: ProvideRuntimeApi<Block> + Send + Sync + 'static,
	<Client as ProvideRuntimeApi<Block>>::Api: BlockBuilderApi<Block> + AuraApi<Block, P::Public>,
	CIDP: CreateInherentDataProviders<Block, ()> + 'static,
{
	let verifier = Verifier::<P, _, _, _> {
		client,
		create_inherent_data_providers,
		slot_duration,
		telemetry,
		_marker: std::marker::PhantomData,
	};

	BasicQueue::new(verifier, Box::new(block_import), None, spawner, registry)
}
