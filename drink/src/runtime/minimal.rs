#![allow(missing_docs)] // `construct_macro` doesn't allow doc comments for the runtime type.

use std::time::SystemTime;

use frame_support::{
    parameter_types,
    sp_runtime::{
        testing::H256,
        traits::{BlakeTwo256, Convert, IdentityLookup},
        AccountId32, BuildStorage, Storage,
    },
    traits::{ConstBool, ConstU128, ConstU32, ConstU64, Currency, Hooks, Randomness},
    weights::Weight,
};
// Re-export all pallets.
pub use frame_system;
use frame_system::pallet_prelude::BlockNumberFor;
pub use pallet_balances;
pub use pallet_contracts;
use pallet_contracts::{DefaultAddressGenerator, Frame, Schedule};
pub use pallet_timestamp;

use crate::{
    runtime::{pallet_contracts_debugging::DrinkDebug, AccountId},
    Runtime,
};

type Block = frame_system::mocking::MockBlockU32<MinimalRuntime>;

frame_support::construct_runtime!(
    pub enum MinimalRuntime {
        System: frame_system,
        Balances: pallet_balances,
        Timestamp: pallet_timestamp,
        Contracts: pallet_contracts,
    }
);

impl frame_system::Config for MinimalRuntime {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type Block = Block;
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Nonce = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId32;
    type Lookup = IdentityLookup<Self::AccountId>;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = ConstU32<250>;
    type DbWeight = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u128>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
}

impl pallet_balances::Config for MinimalRuntime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type Balance = u128;
    type DustRemoval = ();
    type ExistentialDeposit = ConstU128<1>;
    type AccountStore = System;
    type ReserveIdentifier = [u8; 8];
    type FreezeIdentifier = ();
    type MaxLocks = ();
    type MaxReserves = ();
    type MaxHolds = ();
    type MaxFreezes = ();
    type RuntimeHoldReason = ();
}

impl pallet_timestamp::Config for MinimalRuntime {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = ConstU64<1>;
    type WeightInfo = ();
}

pub enum SandboxRandomness {}
impl Randomness<H256, u32> for SandboxRandomness {
    fn random(_subject: &[u8]) -> (H256, u32) {
        todo!("No randomness")
    }
}

type BalanceOf = <Balances as Currency<AccountId32>>::Balance;
impl Convert<Weight, BalanceOf> for MinimalRuntime {
    fn convert(w: Weight) -> BalanceOf {
        w.ref_time().into()
    }
}

parameter_types! {
    pub SandboxSchedule: Schedule<MinimalRuntime> = {
        <Schedule<MinimalRuntime>>::default()
    };
    pub DeletionWeightLimit: Weight = Weight::zero();
    pub DefaultDepositLimit: BalanceOf = 10_000_000;
}

impl pallet_contracts::Config for MinimalRuntime {
    type Time = Timestamp;
    type Randomness = SandboxRandomness;
    type Currency = Balances;
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type CallFilter = ();
    type WeightPrice = Self;
    type WeightInfo = ();
    type ChainExtension = ();
    type Schedule = SandboxSchedule;
    type CallStack = [Frame<Self>; 5];
    type DepositPerByte = ConstU128<1>;
    type DepositPerItem = ConstU128<1>;
    type AddressGenerator = DefaultAddressGenerator;
    type MaxCodeLen = ConstU32<{ 123 * 1024 }>;
    type MaxStorageKeyLen = ConstU32<128>;
    type UnsafeUnstableInterface = ConstBool<false>;
    type MaxDebugBufferLen = ConstU32<{ 2 * 1024 * 1024 }>;
    type Migrations = ();
    type DefaultDepositLimit = DefaultDepositLimit;
    type Debug = DrinkDebug;
}

/// Default initial balance for the default account.
pub const INITIAL_BALANCE: u128 = 1_000_000_000_000_000;

impl Runtime for MinimalRuntime {
    fn initialize_storage(storage: &mut Storage) -> Result<(), String> {
        pallet_balances::GenesisConfig::<Self> {
            balances: vec![(Self::default_actor(), INITIAL_BALANCE)],
        }
        .assimilate_storage(storage)
    }

    fn initialize_block(height: BlockNumberFor<Self>, parent_hash: H256) -> Result<(), String> {
        System::reset_events();
        System::initialize(&height, &parent_hash, &Default::default());

        Balances::on_initialize(height);
        Timestamp::set_timestamp(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
        );
        Timestamp::on_initialize(height);
        Contracts::on_initialize(height);

        System::note_finished_initialize();

        Ok(())
    }

    fn finalize_block(height: BlockNumberFor<Self>) -> Result<H256, String> {
        Contracts::on_finalize(height);
        Timestamp::on_finalize(height);
        Balances::on_finalize(height);

        Ok(System::finalize().hash())
    }

    fn default_actor() -> AccountId<Self> {
        AccountId32::new([1u8; 32])
    }
}
