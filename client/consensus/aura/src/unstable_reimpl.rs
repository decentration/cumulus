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

//! The AuRa consensus algorithm for parachains.
//!
//! This extends the Substrate provided AuRa consensus implementation to make it compatible for
//! parachains. This provides the option to run a "bare" relay-chain driven Aura implementation,
//! but also exposes the core functionalities separately to be composed into more complex implementations.
//!
//! For more information about AuRa, the Substrate crate should be checked.

use codec::{Decode, Encode};
use cumulus_client_collator::service::ServiceInterface as CollatorServiceInterface;
use cumulus_client_consensus_common::{ParachainBlockImportMarker, ParachainCandidate, ParentSearchParams};
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

/// Parameters for [`run_bare_relay_driven`].
pub struct Params<BI, CIDP, Client, RClient, SO, Proposer, CS> {
	pub create_inherent_data_providers: CIDP,
	pub block_import: BI,
	pub para_client: Arc<Client>,
	pub relay_client: Arc<RClient>,
	pub sync_oracle: SO,
	pub keystore: KeystorePtr,
	pub key: CollatorPair,
	pub para_id: ParaId,
	pub overseer_handle: OverseerHandle,
	pub slot_duration: SlotDuration,
	pub relay_chain_slot_duration: SlotDuration,
	pub proposer: Proposer,
	pub collator_service: CS,
}

/// Run async-backing-friendly Aura.
pub async fn run_async_backing_driven<Block, P, BI, CIDP, Client, RClient, SO, Proposer, CS>(
	params: Params<BI, CIDP, Client, RClient, SO, Proposer, CS>,
) where
	Block: BlockT,
	Client: ProvideRuntimeApi<Block>
		+ BlockOf
		+ AuxStore
		+ HeaderBackend<Block>
		+ BlockBackend<Block>
		+ Send
		+ Sync
		+ 'static,
	Client::Api: AuraApi<Block, P::Public> + CollectCollationInfo<Block>,
	RClient: RelayChainInterface,
	CIDP: CreateInherentDataProviders<Block, ()> + 'static,
	BI: BlockImport<Block> + ParachainBlockImportMarker + Send + Sync + 'static,
	SO: SyncOracle + Send + Sync + Clone + 'static,
	Proposer: ProposerInterface<Block, Transaction = BI::Transaction>,
	Proposer::Transaction: Sync,
	CS: CollatorServiceInterface<Block>,
	P: Pair + Send + Sync,
	P::Public: AppPublic + Hash + Member + Encode + Decode,
	P::Signature: TryFrom<Vec<u8>> + Hash + Member + Encode + Decode,
{
	let mut params = params;

	let mut import_notifications = match params.relay_client.import_notification_stream().await {
		Ok(s) => s,
		Err(err) => {
			tracing::error!(
				target: crate::LOG_TARGET,
				?err,
				"Failed to initialize consensus: no relay chain import notification stream"
			);

			return
		},
	};

	while let Some(relay_parent_header) = import_notifications.next().await {
		let relay_parent = relay_parent_header.hash();

		// TODO [now]: get asynchronous backing parameters from the relay-chain
		// runtime.

		let parent_search_params = ParentSearchParams {
			relay_parent,
			para_id: params.para_id,
			ancestry_lookback: unimplemented!(),
			max_depth: unimplemented!(), // max unincluded segment len
			ignore_alternative_branches: true,
		};

		// TODO [now]: remove this in favor of one passed in as a parameter.
		let fake_hack: sc_client_api::in_mem::Blockchain::<Block> = unimplemented!();

		let potential_parents = cumulus_client_consensus_common::find_potential_parents::<Block>(
			parent_search_params,
			&fake_hack, // sp_blockchain::Backend
			&params.relay_client,
		).await;

		let mut potential_parents = match potential_parents {
			Err(e) => {
				tracing::error!(
					target: crate::LOG_TARGET,
					?relay_parent,
					err = ?e,
					"Could not fetch potential parents to build upon"
				);

				continue;
			}
			Ok(x) => x,
		};

		// Sort by depth, descending, to choose the longest chain, and lazily filter
		// by those with space.
		potential_parents.sort_by(|a, b| b.depth.cmp(&a.depth));
		let potential_parents = potential_parents
			.into_iter()
			.filter(|p| can_build_upon(p.hash, &*params.para_client));

		if let Some(parent) = potential_parents.next() {
			// TODO [now]: build and announce collations recursively until
			// `can_build_upon` fails.
			unimplemented!()
		}
	}
}

fn can_build_upon<Block: BlockT, Client>(
	block_hash: Block::Hash,
	client: &Client,
) -> bool where
	Client: ProvideRuntimeApi<Block>
{
	// TODO [now]: claim slot, maybe with an authorities cache to avoid
	// all validators doing this every new relay-chain block.
	// Actually, as long as sessions are based on slot number then they should
	// be the same for all...
	//
	// TODO [now]: new runtime API,
	// AuraUnincludedSegmentApi::has_space(slot) or something like it.
	unimplemented!()
}

/// Run bare Aura consensus as a relay-chain-driven collator.
pub async fn run_bare_relay_driven<Block, P, BI, CIDP, Client, RClient, SO, Proposer, CS>(
	params: Params<BI, CIDP, Client, RClient, SO, Proposer, CS>,
) where
	Block: BlockT,
	Client: ProvideRuntimeApi<Block>
		+ BlockOf
		+ AuxStore
		+ HeaderBackend<Block>
		+ BlockBackend<Block>
		+ Send
		+ Sync
		+ 'static,
	Client::Api: AuraApi<Block, P::Public> + CollectCollationInfo<Block>,
	RClient: RelayChainInterface,
	CIDP: CreateInherentDataProviders<Block, ()> + 'static,
	BI: BlockImport<Block> + ParachainBlockImportMarker + Send + Sync + 'static,
	SO: SyncOracle + Send + Sync + Clone + 'static,
	Proposer: ProposerInterface<Block, Transaction = BI::Transaction>,
	Proposer::Transaction: Sync,
	CS: CollatorServiceInterface<Block>,
	P: Pair + Send + Sync,
	P::Public: AppPublic + Hash + Member + Encode + Decode,
	P::Signature: TryFrom<Vec<u8>> + Hash + Member + Encode + Decode,
{
	let mut params = params;

	let mut collation_requests = cumulus_client_collator::relay_chain_driven::init(
		params.key,
		params.para_id,
		params.overseer_handle,
	)
	.await;

	while let Some(request) = collation_requests.next().await {
		macro_rules! reject_with_error {
			($err:expr) => {{
				request.complete(None);
				tracing::error!(target: crate::LOG_TARGET, err = ?{ $err });
				continue;
			}};
		}

		macro_rules! try_request {
			($x:expr) => {{
				match $x {
					Ok(x) => x,
					Err(e) => reject_with_error!(e),
				}
			}};
		}

		let validation_data = request.persisted_validation_data();

		let parent_header =
			try_request!(Block::Header::decode(&mut &validation_data.parent_head.0[..]));

		let parent_hash = parent_header.hash();

		if !params.collator_service.check_block_status(parent_hash, &parent_header) {
			continue
		}

		let relay_parent_header = match params.relay_client.header(*request.relay_parent()).await {
			Err(e) => reject_with_error!(e),
			Ok(None) => continue, // sanity: would be inconsistent to get `None` here
			Ok(Some(h)) => h,
		};

		let claim = match claim_slot::<_, _, P>(
			&*params.para_client,
			parent_hash,
			&relay_parent_header,
			params.slot_duration,
			params.relay_chain_slot_duration,
			&params.keystore,
		)
		.await
		{
			Ok(None) => continue,
			Ok(Some(c)) => c,
			Err(e) => reject_with_error!(e),
		};

		let (parachain_inherent_data, other_inherent_data) = try_request!(
			create_inherent_data(
				*request.relay_parent(),
				&validation_data,
				parent_hash,
				params.para_id,
				claim.timestamp,
				&params.relay_client,
				&params.create_inherent_data_providers,
			)
			.await
		);

		let proposal = try_request!(
			params.proposer
				.propose(
					&parent_header,
					&parachain_inherent_data,
					other_inherent_data,
					Digest { logs: vec![claim.pre_digest] },
					// TODO [https://github.com/paritytech/cumulus/issues/2439]
					// We should call out to a pluggable interface that provides
					// the proposal duration.
					Duration::from_millis(500),
					// Set the block limit to 50% of the maximum PoV size.
					//
					// TODO: If we got benchmarking that includes the proof size,
					// we should be able to use the maximum pov size.
					Some((validation_data.max_pov_size / 2) as usize),
				)
				.await
		);

		let sealed_importable = try_request!(seal::<_, _, P>(
			proposal.block,
			proposal.storage_changes,
			&claim.author_pub,
			&params.keystore,
		));

		let post_hash = sealed_importable.post_hash();
		let block = Block::new(
			sealed_importable.post_header(),
			sealed_importable
				.body
				.as_ref()
				.expect("body always created with this `propose` fn; qed")
				.clone(),
		);

		try_request!(params.block_import.import_block(sealed_importable).await);

		let response = if let Some((collation, b)) = params.collator_service.build_collation(
			&parent_header,
			post_hash,
			ParachainCandidate { block, proof: proposal.proof },
		) {
			tracing::info!(
				target: crate::LOG_TARGET,
				"PoV size {{ header: {}kb, extrinsics: {}kb, storage_proof: {}kb }}",
				b.header().encode().len() as f64 / 1024f64,
				b.extrinsics().encode().len() as f64 / 1024f64,
				b.storage_proof().encode().len() as f64 / 1024f64,
			);

			if let MaybeCompressedPoV::Compressed(ref pov) = collation.proof_of_validity {
				tracing::info!(
					target: crate::LOG_TARGET,
					"Compressed PoV size: {}kb",
					pov.block_data.0.len() as f64 / 1024f64,
				);
			}

			let result_sender = params.collator_service.announce_with_barrier(post_hash);
			Some(CollationResult { collation, result_sender: Some(result_sender) })
		} else {
			None
		};

		request.complete(response);
	}
}

fn slot_now(slot_duration: SlotDuration) -> Slot {
	let timestamp = sp_timestamp::InherentDataProvider::from_system_time().timestamp();
	Slot::from_timestamp(timestamp, slot_duration)
}

/// A claim on an Aura slot.
struct SlotClaim<Pub> {
	author_pub: Pub,
	pre_digest: sp_runtime::DigestItem,
	timestamp: Timestamp,
}

async fn claim_slot<B, C, P>(
	client: &C,
	parent_hash: B::Hash,
	relay_parent_header: &PHeader,
	slot_duration: SlotDuration,
	relay_chain_slot_duration: SlotDuration,
	keystore: &KeystorePtr,
) -> Result<Option<SlotClaim<P::Public>>, Box<dyn Error>>
where
	B: BlockT,
	C: ProvideRuntimeApi<B> + Send + Sync + 'static,
	C::Api: AuraApi<B, P::Public>,
	P: Pair,
	P::Public: Encode + Decode,
	P::Signature: Encode + Decode,
{
	// load authorities
	let authorities = client.runtime_api().authorities(parent_hash).map_err(Box::new)?;

	// Determine the current slot and timestamp based on the relay-parent's.
	let (slot_now, timestamp) =
		match sc_consensus_babe::find_pre_digest::<PBlock>(relay_parent_header) {
			Ok(babe_pre_digest) => {
				let t =
					Timestamp::new(relay_chain_slot_duration.as_millis() * *babe_pre_digest.slot());
				let slot = Slot::from_timestamp(t, slot_duration);

				(slot, t)
			},
			Err(_) => return Ok(None),
		};

	// Try to claim the slot locally.
	let author_pub = {
		let res = aura_internal::claim_slot::<P>(slot_now, &authorities, keystore).await;
		match res {
			Some(p) => p,
			None => return Ok(None),
		}
	};

	// Produce the pre-digest.
	let pre_digest = aura_internal::pre_digest::<P>(slot_now);

	Ok(Some(SlotClaim { author_pub, pre_digest, timestamp }))
}

// This explicitly creates the inherent data for parachains, as well as overriding the
// timestamp based on the slot number.
async fn create_inherent_data<B: BlockT>(
	relay_parent: PHash,
	validation_data: &PersistedValidationData,
	parent_hash: B::Hash,
	para_id: ParaId,
	timestamp: Timestamp,
	relay_chain_interface: &impl RelayChainInterface,
	create_inherent_data_providers: &impl CreateInherentDataProviders<B, ()>,
) -> Result<(ParachainInherentData, InherentData), Box<dyn Error>> {
	let paras_inherent_data = ParachainInherentData::create_at(
		relay_parent,
		relay_chain_interface,
		validation_data,
		para_id,
	)
	.await;

	let paras_inherent_data = match paras_inherent_data {
		Some(p) => p,
		None =>
			return Err(format!("Could not create paras inherent data at {:?}", relay_parent).into()),
	};

	let mut other_inherent_data = create_inherent_data_providers
		.create_inherent_data_providers(parent_hash, ())
		.map_err(|e| e as Box<dyn Error>)
		.await?
		.create_inherent_data()
		.await
		.map_err(Box::new)?;

	other_inherent_data.replace_data(sp_timestamp::INHERENT_IDENTIFIER, &timestamp);

	Ok((paras_inherent_data, other_inherent_data))
}

fn seal<B: BlockT, T, P>(
	pre_sealed: B,
	storage_changes: StorageChanges<T, HashFor<B>>,
	author_pub: &P::Public,
	keystore: &KeystorePtr,
) -> Result<BlockImportParams<B, T>, Box<dyn Error>>
where
	P: Pair,
	P::Signature: Encode + Decode + TryFrom<Vec<u8>>,
	P::Public: AppPublic,
{
	let (pre_header, body) = pre_sealed.deconstruct();
	let pre_hash = pre_header.hash();
	let block_number = *pre_header.number();

	// seal the block.
	let block_import_params = {
		let seal_digest =
			aura_internal::seal::<_, P>(&pre_hash, &author_pub, keystore).map_err(Box::new)?;
		let mut block_import_params = BlockImportParams::new(BlockOrigin::Own, pre_header);
		block_import_params.post_digests.push(seal_digest);
		block_import_params.body = Some(body.clone());
		block_import_params.state_action =
			StateAction::ApplyChanges(sc_consensus::StorageChanges::Changes(storage_changes));
		block_import_params.fork_choice = Some(ForkChoiceStrategy::LongestChain);
		block_import_params
	};
	let post_hash = block_import_params.post_hash();

	tracing::info!(
		target: crate::LOG_TARGET,
		"🔖 Pre-sealed block for proposal at {}. Hash now {:?}, previously {:?}.",
		block_number,
		post_hash,
		pre_hash,
	);

	Ok(block_import_params)
}

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
