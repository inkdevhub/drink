#![allow(missing_docs)] // `construct_macro` doesn't allow doc comments for the runtime type.

use frame_support::{
    parameter_types,
    sp_runtime::{
        testing::{Header, H256},
        traits::{BlakeTwo256, Convert, IdentityLookup},
        AccountId32,
    },
    traits::{ConstBool, ConstU128, ConstU32, ConstU64, Currency, Randomness},
    weights::Weight,
};
use pallet_contracts::{DefaultAddressGenerator, Frame, Schedule};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<MinimalRuntime>;
type Block = frame_system::mocking::MockBlock<MinimalRuntime>;

frame_support::construct_runtime!(
    pub enum MinimalRuntime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
        Contracts: pallet_contracts::{Pallet, Call, Storage, Event<T>},
    }
);

impl frame_system::Config for MinimalRuntime {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId32;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = ConstU64<250>;
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
    type HoldIdentifier = ();
    type FreezeIdentifier = ();
    type MaxLocks = ();
    type MaxReserves = ();
    type MaxHolds = ();
    type MaxFreezes = ();
}

impl pallet_timestamp::Config for MinimalRuntime {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = ConstU64<1>;
    type WeightInfo = ();
}

pub enum SandboxRandomness {}
impl Randomness<H256, u64> for SandboxRandomness {
    fn random(_subject: &[u8]) -> (H256, u64) {
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
}

use std::time::SystemTime;

use frame_support::{sp_runtime::Storage, traits::Hooks};

use crate::{Runtime, DEFAULT_ACTOR};

/// Default initial balance for the default account.
pub const INITIAL_BALANCE: u128 = 1_000_000_000_000_000;

impl Runtime for MinimalRuntime {
    fn initialize_storage(storage: &mut Storage) -> Result<(), String> {
        pallet_balances::GenesisConfig::<Self> {
            balances: vec![(DEFAULT_ACTOR, INITIAL_BALANCE)],
        }
        .assimilate_storage(storage)
    }

    fn initialize_block(height: u64, parent_hash: H256) -> Result<(), String> {
        System::reset_events();

        if height > 0 {
            System::initialize(&height, &parent_hash, &Default::default());
        }

        Balances::on_initialize(height);
        Timestamp::set_timestamp(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
        );
        Timestamp::on_initialize(height);
        Contracts::on_initialize(height);

        Ok(())
    }

    fn finalize_block(height: u64) -> Result<H256, String> {
        Contracts::on_finalize(height);
        Timestamp::on_finalize(height);
        Balances::on_finalize(height);

        Ok(System::finalize().hash())
    }
}
