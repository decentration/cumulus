// Copyright 2021 Parity Technologies (UK) Ltd.
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

//! Autogenerated weights for `pallet_uniques`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-23, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm6`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("statemint-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot-parachain
// benchmark
// pallet
// --chain=statemint-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_uniques
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./file_header.txt
// --output=./parachains/runtimes/assets/statemint/src/weights/pallet_uniques.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_uniques`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_uniques::WeightInfo for WeightInfo<T> {
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	/// Storage: Uniques ClassAccount (r:0 w:1)
	/// Proof: Uniques ClassAccount (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `177`
		//  Estimated: `2653`
		// Minimum execution time: 24_452 nanoseconds.
		Weight::from_ref_time(24_907_000)
			.saturating_add(Weight::from_proof_size(2653))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	/// Storage: Uniques ClassAccount (r:0 w:1)
	/// Proof: Uniques ClassAccount (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn force_create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `2653`
		// Minimum execution time: 13_324 nanoseconds.
		Weight::from_ref_time(13_703_000)
			.saturating_add(Weight::from_proof_size(2653))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	/// Storage: Uniques Asset (r:1001 w:1000)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(122), added: 2597, mode: MaxEncodedLen)
	/// Storage: Uniques InstanceMetadataOf (r:1000 w:1000)
	/// Proof: Uniques InstanceMetadataOf (max_values: None, max_size: Some(187), added: 2662, mode: MaxEncodedLen)
	/// Storage: Uniques Attribute (r:1000 w:1000)
	/// Proof: Uniques Attribute (max_values: None, max_size: Some(172), added: 2647, mode: MaxEncodedLen)
	/// Storage: Uniques ClassAccount (r:0 w:1)
	/// Proof: Uniques ClassAccount (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	/// Storage: Uniques ClassMetadataOf (r:0 w:1)
	/// Proof: Uniques ClassMetadataOf (max_values: None, max_size: Some(167), added: 2642, mode: MaxEncodedLen)
	/// Storage: Uniques Account (r:0 w:1000)
	/// Proof: Uniques Account (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	/// Storage: Uniques CollectionMaxSupply (r:0 w:1)
	/// Proof: Uniques CollectionMaxSupply (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 1000]`.
	/// The range of component `m` is `[0, 1000]`.
	/// The range of component `a` is `[0, 1000]`.
	fn destroy(n: u32, m: u32, a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `289 + n * (108 ±0) + m * (56 ±0) + a * (107 ±0)`
		//  Estimated: `5250 + n * (2597 ±0) + m * (2662 ±0) + a * (2647 ±0)`
		// Minimum execution time: 2_440_561 nanoseconds.
		Weight::from_ref_time(2_451_205_000)
			.saturating_add(Weight::from_proof_size(5250))
			// Standard Error: 26_890
			.saturating_add(Weight::from_ref_time(8_337_868).saturating_mul(n.into()))
			// Standard Error: 26_890
			.saturating_add(Weight::from_ref_time(286_516).saturating_mul(m.into()))
			// Standard Error: 26_890
			.saturating_add(Weight::from_ref_time(283_516).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(m.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(a.into())))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(m.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(a.into())))
			.saturating_add(Weight::from_proof_size(2597).saturating_mul(n.into()))
			.saturating_add(Weight::from_proof_size(2662).saturating_mul(m.into()))
			.saturating_add(Weight::from_proof_size(2647).saturating_mul(a.into()))
	}
	/// Storage: Uniques Asset (r:1 w:1)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(122), added: 2597, mode: MaxEncodedLen)
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	/// Storage: Uniques CollectionMaxSupply (r:1 w:0)
	/// Proof: Uniques CollectionMaxSupply (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// Storage: Uniques Account (r:0 w:1)
	/// Proof: Uniques Account (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	fn mint() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `314`
		//  Estimated: `7749`
		// Minimum execution time: 29_171 nanoseconds.
		Weight::from_ref_time(29_816_000)
			.saturating_add(Weight::from_proof_size(7749))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	/// Storage: Uniques Asset (r:1 w:1)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(122), added: 2597, mode: MaxEncodedLen)
	/// Storage: Uniques Account (r:0 w:1)
	/// Proof: Uniques Account (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	/// Storage: Uniques ItemPriceOf (r:0 w:1)
	/// Proof: Uniques ItemPriceOf (max_values: None, max_size: Some(89), added: 2564, mode: MaxEncodedLen)
	fn burn() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `492`
		//  Estimated: `5250`
		// Minimum execution time: 30_187 nanoseconds.
		Weight::from_ref_time(30_574_000)
			.saturating_add(Weight::from_proof_size(5250))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Uniques Class (r:1 w:0)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	/// Storage: Uniques Asset (r:1 w:1)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(122), added: 2597, mode: MaxEncodedLen)
	/// Storage: Uniques Account (r:0 w:2)
	/// Proof: Uniques Account (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	/// Storage: Uniques ItemPriceOf (r:0 w:1)
	/// Proof: Uniques ItemPriceOf (max_values: None, max_size: Some(89), added: 2564, mode: MaxEncodedLen)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `492`
		//  Estimated: `5250`
		// Minimum execution time: 24_239 nanoseconds.
		Weight::from_ref_time(24_656_000)
			.saturating_add(Weight::from_proof_size(5250))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	/// Storage: Uniques Asset (r:5000 w:5000)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(122), added: 2597, mode: MaxEncodedLen)
	/// The range of component `i` is `[0, 5000]`.
	fn redeposit(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `770 + i * (108 ±0)`
		//  Estimated: `2653 + i * (2597 ±0)`
		// Minimum execution time: 14_075 nanoseconds.
		Weight::from_ref_time(14_301_000)
			.saturating_add(Weight::from_proof_size(2653))
			// Standard Error: 12_522
			.saturating_add(Weight::from_ref_time(12_138_612).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(i.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i.into())))
			.saturating_add(Weight::from_proof_size(2597).saturating_mul(i.into()))
	}
	/// Storage: Uniques Asset (r:1 w:1)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(122), added: 2597, mode: MaxEncodedLen)
	/// Storage: Uniques Class (r:1 w:0)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	fn freeze() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `492`
		//  Estimated: `5250`
		// Minimum execution time: 17_546 nanoseconds.
		Weight::from_ref_time(17_900_000)
			.saturating_add(Weight::from_proof_size(5250))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Uniques Asset (r:1 w:1)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(122), added: 2597, mode: MaxEncodedLen)
	/// Storage: Uniques Class (r:1 w:0)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	fn thaw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `492`
		//  Estimated: `5250`
		// Minimum execution time: 17_291 nanoseconds.
		Weight::from_ref_time(17_529_000)
			.saturating_add(Weight::from_proof_size(5250))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	fn freeze_collection() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `314`
		//  Estimated: `2653`
		// Minimum execution time: 13_105 nanoseconds.
		Weight::from_ref_time(13_408_000)
			.saturating_add(Weight::from_proof_size(2653))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	fn thaw_collection() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `314`
		//  Estimated: `2653`
		// Minimum execution time: 13_167 nanoseconds.
		Weight::from_ref_time(13_508_000)
			.saturating_add(Weight::from_proof_size(2653))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Uniques OwnershipAcceptance (r:1 w:1)
	/// Proof: Uniques OwnershipAcceptance (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	/// Storage: Uniques ClassAccount (r:0 w:2)
	/// Proof: Uniques ClassAccount (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn transfer_ownership() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `388`
		//  Estimated: `5180`
		// Minimum execution time: 20_855 nanoseconds.
		Weight::from_ref_time(21_269_000)
			.saturating_add(Weight::from_proof_size(5180))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	fn set_team() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `314`
		//  Estimated: `2653`
		// Minimum execution time: 13_782 nanoseconds.
		Weight::from_ref_time(14_207_000)
			.saturating_add(Weight::from_proof_size(2653))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	/// Storage: Uniques ClassAccount (r:0 w:1)
	/// Proof: Uniques ClassAccount (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn force_item_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `314`
		//  Estimated: `2653`
		// Minimum execution time: 16_569 nanoseconds.
		Weight::from_ref_time(17_040_000)
			.saturating_add(Weight::from_proof_size(2653))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	/// Storage: Uniques InstanceMetadataOf (r:1 w:0)
	/// Proof: Uniques InstanceMetadataOf (max_values: None, max_size: Some(187), added: 2662, mode: MaxEncodedLen)
	/// Storage: Uniques Attribute (r:1 w:1)
	/// Proof: Uniques Attribute (max_values: None, max_size: Some(172), added: 2647, mode: MaxEncodedLen)
	fn set_attribute() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `623`
		//  Estimated: `7962`
		// Minimum execution time: 34_492 nanoseconds.
		Weight::from_ref_time(34_838_000)
			.saturating_add(Weight::from_proof_size(7962))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	/// Storage: Uniques InstanceMetadataOf (r:1 w:0)
	/// Proof: Uniques InstanceMetadataOf (max_values: None, max_size: Some(187), added: 2662, mode: MaxEncodedLen)
	/// Storage: Uniques Attribute (r:1 w:1)
	/// Proof: Uniques Attribute (max_values: None, max_size: Some(172), added: 2647, mode: MaxEncodedLen)
	fn clear_attribute() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `851`
		//  Estimated: `7962`
		// Minimum execution time: 34_161 nanoseconds.
		Weight::from_ref_time(34_590_000)
			.saturating_add(Weight::from_proof_size(7962))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	/// Storage: Uniques InstanceMetadataOf (r:1 w:1)
	/// Proof: Uniques InstanceMetadataOf (max_values: None, max_size: Some(187), added: 2662, mode: MaxEncodedLen)
	fn set_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `380`
		//  Estimated: `5315`
		// Minimum execution time: 26_981 nanoseconds.
		Weight::from_ref_time(27_614_000)
			.saturating_add(Weight::from_proof_size(5315))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	/// Storage: Uniques InstanceMetadataOf (r:1 w:1)
	/// Proof: Uniques InstanceMetadataOf (max_values: None, max_size: Some(187), added: 2662, mode: MaxEncodedLen)
	fn clear_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `623`
		//  Estimated: `5315`
		// Minimum execution time: 27_067 nanoseconds.
		Weight::from_ref_time(27_589_000)
			.saturating_add(Weight::from_proof_size(5315))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	/// Storage: Uniques ClassMetadataOf (r:1 w:1)
	/// Proof: Uniques ClassMetadataOf (max_values: None, max_size: Some(167), added: 2642, mode: MaxEncodedLen)
	fn set_collection_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `314`
		//  Estimated: `5295`
		// Minimum execution time: 25_696 nanoseconds.
		Weight::from_ref_time(26_092_000)
			.saturating_add(Weight::from_proof_size(5295))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Uniques Class (r:1 w:0)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	/// Storage: Uniques ClassMetadataOf (r:1 w:1)
	/// Proof: Uniques ClassMetadataOf (max_values: None, max_size: Some(167), added: 2642, mode: MaxEncodedLen)
	fn clear_collection_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `537`
		//  Estimated: `5295`
		// Minimum execution time: 24_850 nanoseconds.
		Weight::from_ref_time(25_332_000)
			.saturating_add(Weight::from_proof_size(5295))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Uniques Class (r:1 w:0)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	/// Storage: Uniques Asset (r:1 w:1)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(122), added: 2597, mode: MaxEncodedLen)
	fn approve_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `492`
		//  Estimated: `5250`
		// Minimum execution time: 19_810 nanoseconds.
		Weight::from_ref_time(20_161_000)
			.saturating_add(Weight::from_proof_size(5250))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Uniques Class (r:1 w:0)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	/// Storage: Uniques Asset (r:1 w:1)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(122), added: 2597, mode: MaxEncodedLen)
	fn cancel_approval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `525`
		//  Estimated: `5250`
		// Minimum execution time: 19_653 nanoseconds.
		Weight::from_ref_time(19_929_000)
			.saturating_add(Weight::from_proof_size(5250))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Uniques OwnershipAcceptance (r:1 w:1)
	/// Proof: Uniques OwnershipAcceptance (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn set_accept_ownership() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `2527`
		// Minimum execution time: 14_736 nanoseconds.
		Weight::from_ref_time(15_338_000)
			.saturating_add(Weight::from_proof_size(2527))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Uniques CollectionMaxSupply (r:1 w:1)
	/// Proof: Uniques CollectionMaxSupply (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// Storage: Uniques Class (r:1 w:0)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	fn set_collection_max_supply() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `314`
		//  Estimated: `5152`
		// Minimum execution time: 16_237 nanoseconds.
		Weight::from_ref_time(16_449_000)
			.saturating_add(Weight::from_proof_size(5152))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Uniques Asset (r:1 w:0)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(122), added: 2597, mode: MaxEncodedLen)
	/// Storage: Uniques ItemPriceOf (r:0 w:1)
	/// Proof: Uniques ItemPriceOf (max_values: None, max_size: Some(89), added: 2564, mode: MaxEncodedLen)
	fn set_price() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `291`
		//  Estimated: `2597`
		// Minimum execution time: 15_919 nanoseconds.
		Weight::from_ref_time(16_179_000)
			.saturating_add(Weight::from_proof_size(2597))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Uniques Asset (r:1 w:1)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(122), added: 2597, mode: MaxEncodedLen)
	/// Storage: Uniques ItemPriceOf (r:1 w:1)
	/// Proof: Uniques ItemPriceOf (max_values: None, max_size: Some(89), added: 2564, mode: MaxEncodedLen)
	/// Storage: Uniques Class (r:1 w:0)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	/// Storage: Uniques Account (r:0 w:2)
	/// Proof: Uniques Account (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	fn buy_item() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `636`
		//  Estimated: `7814`
		// Minimum execution time: 34_918 nanoseconds.
		Weight::from_ref_time(35_301_000)
			.saturating_add(Weight::from_proof_size(7814))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(4))
	}
}
