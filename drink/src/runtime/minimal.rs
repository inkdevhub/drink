#![allow(missing_docs)] // `construct_macro` doesn't allow doc comments for the runtime type.

macro_rules! create_minimal_runtime {
    ($name:ident) => {
        create_minimal_runtime!($name, ());
    };
    ($name:ident, $chain_extension: ty) => {

// ------------ Put all the boilerplate into an auxiliary module -----------------------------------
mod construct_runtime {

    // ------------ Bring some common types into the scope -----------------------------------------
    use $crate::frame_support::{
        parameter_types,
        sp_runtime::{
            testing::H256,
            traits::{BlakeTwo256, Convert, IdentityLookup},
            AccountId32, Perbill,
        },
        traits::{ConstBool, ConstU128, ConstU32, ConstU64, Currency, Randomness},
        weights::Weight,
    };
    use $crate::runtime::pallet_contracts_debugging::DrinkDebug;

    // ------------ Define the runtime type as a collection of pallets -----------------------------
    $crate::frame_support::construct_runtime!(
        pub enum $name {
            System: $crate::frame_system,
            Balances: $crate::pallet_balances,
            Timestamp: $crate::pallet_timestamp,
            Contracts: $crate::pallet_contracts,
        }
    );

    // ------------ Configure pallet system --------------------------------------------------------
    impl frame_system::Config for $name {
        type BaseCallFilter = $crate::frame_support::traits::Everything;
        type BlockWeights = ();
        type BlockLength = ();
        type Block = $crate::frame_system::mocking::MockBlockU32<$name>;
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
        type AccountData = $crate::pallet_balances::AccountData<u128>;
        type OnNewAccount = ();
        type OnKilledAccount = ();
        type SystemWeightInfo = ();
        type SS58Prefix = ();
        type OnSetCode = ();
        type MaxConsumers = ConstU32<16>;
    }

    // ------------ Configure pallet balances ------------------------------------------------------
    impl pallet_balances::Config for $name {
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
        type MaxHolds = ConstU32<1>;
        type MaxFreezes = ();
        type RuntimeHoldReason = RuntimeHoldReason;
    }

    // ------------ Configure pallet timestamp -----------------------------------------------------
    impl pallet_timestamp::Config for $name {
        type Moment = u64;
        type OnTimestampSet = ();
        type MinimumPeriod = ConstU64<1>;
        type WeightInfo = ();
    }

    // ------------ Configure pallet contracts -----------------------------------------------------
    pub enum SandboxRandomness {}
    impl Randomness<H256, u32> for SandboxRandomness {
        fn random(_subject: &[u8]) -> (H256, u32) {
            unreachable!("No randomness")
        }
    }

    type BalanceOf = <Balances as Currency<AccountId32>>::Balance;
    impl Convert<Weight, BalanceOf> for $name {
        fn convert(w: Weight) -> BalanceOf {
            w.ref_time().into()
        }
    }

    parameter_types! {
        pub SandboxSchedule: $crate::pallet_contracts::Schedule<$name> = {
            <$crate::pallet_contracts::Schedule<$name>>::default()
        };
        pub DeletionWeightLimit: Weight = Weight::zero();
        pub DefaultDepositLimit: BalanceOf = 10_000_000;
        pub CodeHashLockupDepositPercent: Perbill = Perbill::from_percent(0);
        pub MaxDelegateDependencies: u32 = 32;
    }

    impl pallet_contracts::Config for $name {
        type Time = Timestamp;
        type Randomness = SandboxRandomness;
        type Currency = Balances;
        type RuntimeEvent = RuntimeEvent;
        type RuntimeCall = RuntimeCall;
        type CallFilter = ();
        type WeightPrice = Self;
        type WeightInfo = ();
        type ChainExtension = $chain_extension;
        type Schedule = SandboxSchedule;
        type CallStack = [$crate::pallet_contracts::Frame<Self>; 5];
        type DepositPerByte = ConstU128<1>;
        type DepositPerItem = ConstU128<1>;
        type AddressGenerator = $crate::pallet_contracts::DefaultAddressGenerator;
        type MaxCodeLen = ConstU32<{ 123 * 1024 }>;
        type MaxStorageKeyLen = ConstU32<128>;
        type UnsafeUnstableInterface = ConstBool<false>;
        type MaxDebugBufferLen = ConstU32<{ 2 * 1024 * 1024 }>;
        type Migrations = ();
        type DefaultDepositLimit = DefaultDepositLimit;
        type Debug = DrinkDebug;
        type CodeHashLockupDepositPercent = CodeHashLockupDepositPercent;
        type MaxDelegateDependencies = MaxDelegateDependencies;
        type RuntimeHoldReason = RuntimeHoldReason;
        type Environment = ();
    }
}

// ------------ Export runtime type itself, pallets and useful types from the auxiliary module -----
pub use construct_runtime::{
    $name, Balances, Contracts, PalletInfo, RuntimeCall, RuntimeEvent, RuntimeHoldReason,
    RuntimeOrigin, System, Timestamp,
};
    };
}

create_minimal_runtime!(MinimalRuntime);

use std::time::SystemTime;

use frame_support::{
    sp_runtime::{testing::H256, traits::Dispatchable, AccountId32, BuildStorage, Storage},
    traits::Hooks,
};

use crate::{AccountIdFor, Runtime, RuntimeMetadataPrefixed};

/// Default initial balance for the default account.
pub const INITIAL_BALANCE: u128 = 1_000_000_000_000_000;

impl Runtime for MinimalRuntime {
    fn initialize_storage(storage: &mut Storage) -> Result<(), String> {
        pallet_balances::GenesisConfig::<Self> {
            balances: vec![(Self::default_actor(), INITIAL_BALANCE)],
        }
        .assimilate_storage(storage)
    }

    fn initialize_block(
        height: frame_system::pallet_prelude::BlockNumberFor<Self>,
        parent_hash: H256,
    ) -> Result<(), String> {
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

    fn finalize_block(
        height: frame_system::pallet_prelude::BlockNumberFor<Self>,
    ) -> Result<H256, String> {
        Contracts::on_finalize(height);
        Timestamp::on_finalize(height);
        Balances::on_finalize(height);

        Ok(System::finalize().hash())
    }

    fn default_actor() -> AccountIdFor<Self> {
        AccountId32::new([1u8; 32])
    }

    fn get_metadata() -> RuntimeMetadataPrefixed {
        Self::metadata()
    }

    fn convert_account_to_origin(
        account: AccountIdFor<Self>,
    ) -> <<Self as frame_system::Config>::RuntimeCall as Dispatchable>::RuntimeOrigin {
        Some(account).into()
    }
}
