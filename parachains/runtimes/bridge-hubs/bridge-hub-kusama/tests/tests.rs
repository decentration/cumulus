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

use bp_polkadot_core::Signature;
pub use bridge_hub_kusama_runtime::{
	bridge_hub_config,
	constants::fee::WeightToFee,
	xcm_config::{RelayNetwork, XcmConfig},
	Balances, BridgeGrandpaPolkadotInstance, BridgeRejectObsoleteHeadersAndMessages,
	ExistentialDeposit, ParachainSystem, PolkadotXcm, Runtime, RuntimeCall, RuntimeEvent,
	SessionKeys, WithBridgeHubPolkadotMessagesInstance,
};
use bridge_hub_kusama_runtime::{
	bridge_hub_config::WithBridgeHubPolkadotMessageBridge, BridgeParachainPolkadotInstance,
	DeliveryRewardInBalance, Executive, RequiredStakeForStakeAndSlash, SignedExtra,
	UncheckedExtrinsic,
};
use codec::{Decode, Encode};
use frame_support::parameter_types;
use parachains_common::{AccountId, AuraId, Balance};
use sp_keyring::AccountKeyring::Alice;
use sp_runtime::{
	generic::{Era, SignedPayload},
	AccountId32,
};
use xcm::latest::prelude::*;

// Para id of sibling chain (e.g. Statemine) used in tests.
pub const SIBLING_PARACHAIN_ID: u32 = 1000;

parameter_types! {
	pub CheckingAccount: AccountId = PolkadotXcm::check_account();
	pub RuntimeNetwork: NetworkId = RelayNetwork::get().unwrap();
}

fn construct_extrinsic(
	sender: sp_keyring::AccountKeyring,
	call: RuntimeCall,
) -> UncheckedExtrinsic {
	let extra: SignedExtra = (
		frame_system::CheckNonZeroSender::<Runtime>::new(),
		frame_system::CheckSpecVersion::<Runtime>::new(),
		frame_system::CheckTxVersion::<Runtime>::new(),
		frame_system::CheckGenesis::<Runtime>::new(),
		frame_system::CheckEra::<Runtime>::from(Era::immortal()),
		frame_system::CheckNonce::<Runtime>::from(0),
		frame_system::CheckWeight::<Runtime>::new(),
		pallet_transaction_payment::ChargeTransactionPayment::<Runtime>::from(0),
		BridgeRejectObsoleteHeadersAndMessages {},
		bridge_hub_config::BridgeRefundBridgeHubPolkadotMessages::default(),
	);
	let payload = SignedPayload::new(call.clone(), extra.clone()).unwrap();
	let signature = payload.using_encoded(|e| sender.sign(e));
	UncheckedExtrinsic::new_signed(
		call,
		AccountId32::from(sender.public()).into(),
		Signature::Sr25519(signature.clone()),
		extra,
	)
}

fn construct_and_apply_extrinsic(
	relayer_at_target: sp_keyring::AccountKeyring,
	batch: pallet_utility::Call<Runtime>,
) -> sp_runtime::DispatchOutcome {
	let batch_call = RuntimeCall::Utility(batch);
	let xt = construct_extrinsic(relayer_at_target, batch_call);
	let r = Executive::apply_extrinsic(xt);
	r.unwrap()
}

fn executive_init_block(header: &<Runtime as frame_system::Config>::Header) {
	Executive::initialize_block(header)
}

fn collator_session_keys() -> bridge_hub_test_utils::CollatorSessionKeys<Runtime> {
	bridge_hub_test_utils::CollatorSessionKeys::new(
		AccountId::from(Alice),
		AccountId::from(Alice),
		SessionKeys { aura: AuraId::from(Alice.public()) },
	)
}

bridge_hub_test_utils::test_cases::include_teleports_for_native_asset_works!(
	Runtime,
	XcmConfig,
	CheckingAccount,
	WeightToFee,
	ParachainSystem,
	collator_session_keys(),
	ExistentialDeposit::get(),
	Box::new(|runtime_event_encoded: Vec<u8>| {
		match RuntimeEvent::decode(&mut &runtime_event_encoded[..]) {
			Ok(RuntimeEvent::PolkadotXcm(event)) => Some(event),
			_ => None,
		}
	}),
	Box::new(|runtime_event_encoded: Vec<u8>| {
		match RuntimeEvent::decode(&mut &runtime_event_encoded[..]) {
			Ok(RuntimeEvent::XcmpQueue(event)) => Some(event),
			_ => None,
		}
	}),
	1002
);

#[test]
fn initialize_bridge_by_governance_works() {
	bridge_hub_test_utils::test_cases::initialize_bridge_by_governance_works::<
		Runtime,
		BridgeGrandpaPolkadotInstance,
	>(
		collator_session_keys(),
		bp_bridge_hub_kusama::BRIDGE_HUB_KUSAMA_PARACHAIN_ID,
		Box::new(|call| RuntimeCall::BridgePolkadotGrandpa(call).encode()),
	)
}

#[test]
fn change_delivery_reward_by_governance_works() {
	bridge_hub_test_utils::test_cases::change_storage_constant_by_governance_works::<
		Runtime,
		DeliveryRewardInBalance,
		u64,
	>(
		collator_session_keys(),
		bp_bridge_hub_kusama::BRIDGE_HUB_KUSAMA_PARACHAIN_ID,
		Box::new(|call| RuntimeCall::System(call).encode()),
		|| (DeliveryRewardInBalance::key().to_vec(), DeliveryRewardInBalance::get()),
		|old_value| old_value.checked_mul(2).unwrap(),
	)
}

#[test]
fn change_required_stake_by_governance_works() {
	bridge_hub_test_utils::test_cases::change_storage_constant_by_governance_works::<
		Runtime,
		RequiredStakeForStakeAndSlash,
		Balance,
	>(
		collator_session_keys(),
		bp_bridge_hub_kusama::BRIDGE_HUB_KUSAMA_PARACHAIN_ID,
		Box::new(|call| RuntimeCall::System(call).encode()),
		|| (RequiredStakeForStakeAndSlash::key().to_vec(), RequiredStakeForStakeAndSlash::get()),
		|old_value| old_value.checked_mul(2).unwrap(),
	)
}

#[test]
fn handle_export_message_from_system_parachain_add_to_outbound_queue_works() {
	bridge_hub_test_utils::test_cases::handle_export_message_from_system_parachain_to_outbound_queue_works::<
		Runtime,
		XcmConfig,
		WithBridgeHubPolkadotMessagesInstance,
	>(
		collator_session_keys(),
		bp_bridge_hub_kusama::BRIDGE_HUB_KUSAMA_PARACHAIN_ID,
		SIBLING_PARACHAIN_ID,
		Box::new(|runtime_event_encoded: Vec<u8>| {
			match RuntimeEvent::decode(&mut &runtime_event_encoded[..]) {
				Ok(RuntimeEvent::BridgePolkadotMessages(event)) => Some(event),
				_ => None,
			}
		}),
		|| ExportMessage { network: Polkadot, destination: X1(Parachain(1234)), xcm: Xcm(vec![]) },
		bridge_hub_config::STATEMINE_TO_STATEMINT_LANE_ID
	)
}

#[test]
fn message_dispatch_routing_works() {
	bridge_hub_test_utils::test_cases::message_dispatch_routing_works::<
		Runtime,
		XcmConfig,
		ParachainSystem,
		WithBridgeHubPolkadotMessagesInstance,
		RuntimeNetwork,
		bridge_hub_config::PolkadotGlobalConsensusNetwork,
	>(
		collator_session_keys(),
		bp_bridge_hub_kusama::BRIDGE_HUB_KUSAMA_PARACHAIN_ID,
		SIBLING_PARACHAIN_ID,
		Box::new(|runtime_event_encoded: Vec<u8>| {
			match RuntimeEvent::decode(&mut &runtime_event_encoded[..]) {
				Ok(RuntimeEvent::ParachainSystem(event)) => Some(event),
				_ => None,
			}
		}),
		Box::new(|runtime_event_encoded: Vec<u8>| {
			match RuntimeEvent::decode(&mut &runtime_event_encoded[..]) {
				Ok(RuntimeEvent::XcmpQueue(event)) => Some(event),
				_ => None,
			}
		}),
		bridge_hub_config::STATEMINE_TO_STATEMINT_LANE_ID,
	)
}

#[test]
fn relayed_incoming_message_works() {
	bridge_hub_test_utils::test_cases::relayed_incoming_message_works::<
		Runtime,
		XcmConfig,
		ParachainSystem,
		BridgeGrandpaPolkadotInstance,
		BridgeParachainPolkadotInstance,
		WithBridgeHubPolkadotMessagesInstance,
		WithBridgeHubPolkadotMessageBridge,
	>(
		collator_session_keys(),
		bp_bridge_hub_kusama::BRIDGE_HUB_KUSAMA_PARACHAIN_ID,
		bp_bridge_hub_polkadot::BRIDGE_HUB_POLKADOT_PARACHAIN_ID,
		SIBLING_PARACHAIN_ID,
		RuntimeNetwork::get(),
		bridge_hub_config::STATEMINE_TO_STATEMINT_LANE_ID,
	)
}

#[test]
pub fn complex_relay_extrinsic_works() {
	bridge_hub_test_utils::test_cases::complex_relay_extrinsic_works::<
		Runtime,
		XcmConfig,
		ParachainSystem,
		BridgeGrandpaPolkadotInstance,
		BridgeParachainPolkadotInstance,
		WithBridgeHubPolkadotMessagesInstance,
		WithBridgeHubPolkadotMessageBridge,
	>(
		collator_session_keys(),
		bp_bridge_hub_kusama::BRIDGE_HUB_KUSAMA_PARACHAIN_ID,
		bp_bridge_hub_polkadot::BRIDGE_HUB_POLKADOT_PARACHAIN_ID,
		SIBLING_PARACHAIN_ID,
		bridge_hub_config::BridgeHubPolkadotChainId::get(),
		RuntimeNetwork::get(),
		bridge_hub_config::STATEMINE_TO_STATEMINT_LANE_ID,
		ExistentialDeposit::get(),
		executive_init_block,
		construct_and_apply_extrinsic,
	);
}
