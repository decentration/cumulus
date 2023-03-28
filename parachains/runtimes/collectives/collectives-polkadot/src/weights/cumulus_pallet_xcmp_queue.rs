
//! Autogenerated weights for `cumulus_pallet_xcmp_queue`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `i9`, CPU: `13th Gen Intel(R) Core(TM) i9-13900K`
//! EXECUTION: Some(Native), WASM-EXECUTION: Compiled, CHAIN: Some("collectives-polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/polkadot-parachain
// benchmark
// pallet
// --chain
// collectives-polkadot-dev
// --pallet
// cumulus_pallet_xcmp_queue
// --extrinsic
// 
// --execution
// native
// --output
// parachains/runtimes/collectives/collectives-polkadot/src/weights
// --steps
// 50
// --repeat
// 20

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `cumulus_pallet_xcmp_queue`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> cumulus_pallet_xcmp_queue::WeightInfo for WeightInfo<T> {
	/// Storage: XcmpQueue QueueConfig (r:1 w:1)
	/// Proof Skipped: XcmpQueue QueueConfig (max_values: Some(1), max_size: None, mode: Measured)
	fn set_config_with_u32() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `1594`
		// Minimum execution time: 1_849_000 picoseconds.
		Weight::from_parts(1_960_000, 0)
			.saturating_add(Weight::from_parts(0, 1594))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: XcmpQueue QueueConfig (r:1 w:0)
	/// Proof Skipped: XcmpQueue QueueConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: MessageQueue ServiceHead (r:1 w:1)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:0 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65585), added: 68060, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 1000]`.
	fn enqueue_xcmp_messages(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `167`
		//  Estimated: `6659`
		// Minimum execution time: 3_212_000 picoseconds.
		Weight::from_parts(3_321_000, 0)
			.saturating_add(Weight::from_parts(0, 6659))
			// Standard Error: 990
			.saturating_add(Weight::from_parts(976_738, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: XcmpQueue QueueSuspended (r:1 w:0)
	/// Proof Skipped: XcmpQueue QueueSuspended (max_values: Some(1), max_size: None, mode: Measured)
	fn process_message() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `1594`
		// Minimum execution time: 1_240_000 picoseconds.
		Weight::from_parts(1_287_000, 0)
			.saturating_add(Weight::from_parts(0, 1594))
			.saturating_add(T::DbWeight::get().reads(1))
	}
}
