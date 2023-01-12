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

//! Autogenerated weights for `pallet_alliance`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-21, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm6`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("collectives-polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./artifacts/polkadot-parachain
// benchmark
// pallet
// --chain=collectives-polkadot-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_alliance
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./file_header.txt
// --output=./parachains/runtimes/collectives/collectives-polkadot/src/weights/pallet_alliance.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_alliance`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_alliance::WeightInfo for WeightInfo<T> {
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion ProposalOf (r:1 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: AllianceMotion ProposalCount (r:1 w:1)
	// Storage: AllianceMotion Voting (r:0 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 35_162 nanoseconds.
		Weight::from_ref_time(36_412_712)
			// Standard Error: 71
			.saturating_add(Weight::from_ref_time(381).saturating_mul(b.into()))
			// Standard Error: 745
			.saturating_add(Weight::from_ref_time(22_365).saturating_mul(m.into()))
			// Standard Error: 735
			.saturating_add(Weight::from_ref_time(104_171).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion Voting (r:1 w:1)
	/// The range of component `m` is `[5, 100]`.
	fn vote(m: u32, ) -> Weight {
		// Minimum execution time: 31_249 nanoseconds.
		Weight::from_ref_time(32_188_065)
			// Standard Error: 832
			.saturating_add(Weight::from_ref_time(54_591).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion Voting (r:1 w:1)
	// Storage: AllianceMotion Members (r:1 w:0)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: AllianceMotion ProposalOf (r:0 w:1)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 40_517 nanoseconds.
		Weight::from_ref_time(37_804_735)
			// Standard Error: 724
			.saturating_add(Weight::from_ref_time(46_629).saturating_mul(m.into()))
			// Standard Error: 706
			.saturating_add(Weight::from_ref_time(96_011).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion Voting (r:1 w:1)
	// Storage: AllianceMotion Members (r:1 w:0)
	// Storage: AllianceMotion ProposalOf (r:1 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 50_587 nanoseconds.
		Weight::from_ref_time(49_483_170)
			// Standard Error: 79
			.saturating_add(Weight::from_ref_time(12).saturating_mul(b.into()))
			// Standard Error: 845
			.saturating_add(Weight::from_ref_time(46_955).saturating_mul(m.into()))
			// Standard Error: 824
			.saturating_add(Weight::from_ref_time(99_511).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion Voting (r:1 w:1)
	// Storage: AllianceMotion Members (r:1 w:0)
	// Storage: AllianceMotion Prime (r:1 w:0)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: AllianceMotion ProposalOf (r:0 w:1)
	// Storage: Alliance Rule (r:0 w:1)
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 47_785 nanoseconds.
		Weight::from_ref_time(49_064_492)
			// Standard Error: 3_243
			.saturating_add(Weight::from_ref_time(96_267).saturating_mul(m.into()))
			// Standard Error: 3_203
			.saturating_add(Weight::from_ref_time(121_736).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion Voting (r:1 w:1)
	// Storage: AllianceMotion Members (r:1 w:0)
	// Storage: AllianceMotion Prime (r:1 w:0)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: AllianceMotion ProposalOf (r:0 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[5, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(_b: u32, m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 41_807 nanoseconds.
		Weight::from_ref_time(40_124_977)
			// Standard Error: 665
			.saturating_add(Weight::from_ref_time(40_824).saturating_mul(m.into()))
			// Standard Error: 641
			.saturating_add(Weight::from_ref_time(89_499).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Alliance Members (r:2 w:2)
	// Storage: AllianceMotion Members (r:1 w:1)
	/// The range of component `m` is `[1, 100]`.
	/// The range of component `z` is `[0, 100]`.
	fn init_members(m: u32, z: u32, ) -> Weight {
		// Minimum execution time: 37_806 nanoseconds.
		Weight::from_ref_time(26_654_829)
			// Standard Error: 571
			.saturating_add(Weight::from_ref_time(131_979).saturating_mul(m.into()))
			// Standard Error: 564
			.saturating_add(Weight::from_ref_time(114_315).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Alliance Members (r:2 w:2)
	// Storage: AllianceMotion Proposals (r:1 w:0)
	// Storage: Alliance DepositOf (r:101 w:50)
	// Storage: System Account (r:50 w:50)
	// Storage: AllianceMotion Members (r:0 w:1)
	// Storage: AllianceMotion Prime (r:0 w:1)
	/// The range of component `x` is `[1, 100]`.
	/// The range of component `y` is `[0, 100]`.
	/// The range of component `z` is `[0, 50]`.
	fn disband(x: u32, y: u32, z: u32, ) -> Weight {
		// Minimum execution time: 226_805 nanoseconds.
		Weight::from_ref_time(228_436_000)
			// Standard Error: 18_416
			.saturating_add(Weight::from_ref_time(484_425).saturating_mul(x.into()))
			// Standard Error: 18_327
			.saturating_add(Weight::from_ref_time(384_513).saturating_mul(y.into()))
			// Standard Error: 36_622
			.saturating_add(Weight::from_ref_time(8_974_720).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(x.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(y.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(z.into())))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(z.into())))
	}
	// Storage: Alliance Rule (r:0 w:1)
	fn set_rule() -> Weight {
		// Minimum execution time: 16_471 nanoseconds.
		Weight::from_ref_time(16_726_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Alliance Announcements (r:1 w:1)
	fn announce() -> Weight {
		// Minimum execution time: 17_782 nanoseconds.
		Weight::from_ref_time(18_057_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Alliance Announcements (r:1 w:1)
	fn remove_announcement() -> Weight {
		// Minimum execution time: 20_072 nanoseconds.
		Weight::from_ref_time(20_395_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Alliance Members (r:3 w:1)
	// Storage: Alliance UnscrupulousAccounts (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Alliance DepositOf (r:0 w:1)
	fn join_alliance() -> Weight {
		// Minimum execution time: 47_327 nanoseconds.
		Weight::from_ref_time(47_908_000)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Alliance Members (r:3 w:1)
	// Storage: Alliance UnscrupulousAccounts (r:1 w:0)
	fn nominate_ally() -> Weight {
		// Minimum execution time: 34_263 nanoseconds.
		Weight::from_ref_time(34_687_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Alliance Members (r:2 w:2)
	// Storage: AllianceMotion Proposals (r:1 w:0)
	// Storage: AllianceMotion Members (r:0 w:1)
	// Storage: AllianceMotion Prime (r:0 w:1)
	fn elevate_ally() -> Weight {
		// Minimum execution time: 29_783 nanoseconds.
		Weight::from_ref_time(30_668_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Alliance Members (r:4 w:2)
	// Storage: AllianceMotion Proposals (r:1 w:0)
	// Storage: AllianceMotion Members (r:0 w:1)
	// Storage: AllianceMotion Prime (r:0 w:1)
	// Storage: Alliance RetiringMembers (r:0 w:1)
	fn give_retirement_notice() -> Weight {
		// Minimum execution time: 37_967 nanoseconds.
		Weight::from_ref_time(38_605_000)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: Alliance RetiringMembers (r:1 w:1)
	// Storage: Alliance Members (r:1 w:1)
	// Storage: Alliance DepositOf (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn retire() -> Weight {
		// Minimum execution time: 40_362 nanoseconds.
		Weight::from_ref_time(41_038_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Alliance Members (r:3 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:0)
	// Storage: Alliance DepositOf (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Storage: AllianceMotion Members (r:0 w:1)
	// Storage: AllianceMotion Prime (r:0 w:1)
	fn kick_member() -> Weight {
		// Minimum execution time: 118_050 nanoseconds.
		Weight::from_ref_time(119_080_000)
			.saturating_add(T::DbWeight::get().reads(13))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	// Storage: Alliance UnscrupulousAccounts (r:1 w:1)
	// Storage: Alliance UnscrupulousWebsites (r:1 w:1)
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `l` is `[0, 255]`.
	fn add_unscrupulous_items(n: u32, l: u32, ) -> Weight {
		// Minimum execution time: 14_919 nanoseconds.
		Weight::from_ref_time(15_135_000)
			// Standard Error: 3_921
			.saturating_add(Weight::from_ref_time(1_221_966).saturating_mul(n.into()))
			// Standard Error: 1_535
			.saturating_add(Weight::from_ref_time(80_157).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Alliance UnscrupulousAccounts (r:1 w:1)
	// Storage: Alliance UnscrupulousWebsites (r:1 w:1)
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `l` is `[0, 255]`.
	fn remove_unscrupulous_items(n: u32, l: u32, ) -> Weight {
		// Minimum execution time: 14_606 nanoseconds.
		Weight::from_ref_time(14_847_000)
			// Standard Error: 218_175
			.saturating_add(Weight::from_ref_time(15_210_084).saturating_mul(n.into()))
			// Standard Error: 85_447
			.saturating_add(Weight::from_ref_time(717_495).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Alliance Members (r:3 w:2)
	// Storage: AllianceMotion Proposals (r:1 w:0)
	// Storage: AllianceMotion Members (r:0 w:1)
	// Storage: AllianceMotion Prime (r:0 w:1)
	fn abdicate_fellow_status() -> Weight {
		// Minimum execution time: 36_974 nanoseconds.
		Weight::from_ref_time(37_428_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
}
