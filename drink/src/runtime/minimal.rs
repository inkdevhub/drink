#![allow(missing_docs)] // `construct_macro` doesn't allow doc comments for the runtime type.

/// Macro creating a minimal runtime with the given name. Optionally can take a chain extension
/// type as a second argument.
///
/// The new macro will automatically implement `drink::Runtime`.
#[macro_export]
macro_rules! create_minimal_runtime {
    ($name:ident) => {
        create_minimal_runtime!($name, ());
    };
    ($name:ident, $chain_extension: ty) => {

// ------------ Put all the boilerplate into an auxiliary module -----------------------------------
mod construct_runtime {

    // ------------ Bring some common types into the scope -----------------------------------------
    use $crate::frame_support::{
        construct_runtime,
        derive_impl,
        parameter_types,
        sp_runtime::{
            testing::H256,
            traits::Convert,
            AccountId32, Perbill,
        },
        traits::{ConstBool, ConstU128, ConstU32, ConstU64, Currency, Randomness},
        weights::Weight,
    };
    use $crate::runtime::pallet_contracts_debugging::DrinkDebug;

    // ------------ Define the runtime type as a collection of pallets -----------------------------
    construct_runtime!(
        pub enum $name {
            System: $crate::frame_system,
            Balances: $crate::pallet_balances,
            Timestamp: $crate::pallet_timestamp,
            Contracts: $crate::pallet_contracts,
        }
    );

    // ------------ Configure pallet system --------------------------------------------------------
    #[derive_impl($crate::frame_system::config_preludes::SolochainDefaultConfig as $crate::frame_system::DefaultConfig)]
    impl $crate::frame_system::Config for $name {
        type Block = $crate::frame_system::mocking::MockBlockU32<$name>;
        type Version = ();
        type BlockHashCount = ConstU32<250>;
        type AccountData = $crate::pallet_balances::AccountData<<$name as pallet_balances::Config>::Balance>;
    }

    // ------------ Configure pallet balances ------------------------------------------------------
    impl $crate::pallet_balances::Config for $name {
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
        type RuntimeFreezeReason = RuntimeFreezeReason;
    }

    // ------------ Configure pallet timestamp -----------------------------------------------------
    impl $crate::pallet_timestamp::Config for $name {
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

    impl $crate::pallet_contracts::Config for $name {
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
        type Xcm = ();
    }
}

// ------------ Export runtime type itself, pallets and useful types from the auxiliary module -----
pub use construct_runtime::{
    $name, Balances, Contracts, PalletInfo, RuntimeCall, RuntimeEvent, RuntimeHoldReason,
    RuntimeOrigin, System, Timestamp
};
    };
}

create_minimal_runtime!(MinimalRuntime);

// ------------ Implement `drink::Runtime` trait ---------------------------------------------------

use std::time::SystemTime;

use frame_support::{
    sp_runtime::{traits::Header, BuildStorage, Storage},
    traits::Hooks,
};

use crate::{
    runtime::{AccountIdFor, Runtime},
    AccountId32,
};

/// Default initial balance for the default account.
pub const INITIAL_BALANCE: u128 = 1_000_000_000_000_000;

type SystemOf<T> = frame_system::Pallet<T>;
type BalancesOf<T> = pallet_balances::Pallet<T>;
type TimestampOf<T> = pallet_timestamp::Pallet<T>;
type ContractsOf<T> = pallet_contracts::Pallet<T>;

impl<T> Runtime for T
where
    T: pallet_balances::Config + pallet_timestamp::Config + pallet_contracts::Config,
    <T as frame_system::Config>::AccountId: From<AccountId32>,
    <T as pallet_balances::Config>::Balance: From<u128>,
    <T as pallet_timestamp::Config>::Moment: From<u64>,
{
    fn initialize_storage(storage: &mut Storage) -> Result<(), String> {
        pallet_balances::GenesisConfig::<Self> {
            balances: vec![(Self::default_actor(), INITIAL_BALANCE.into())],
        }
        .assimilate_storage(storage)
    }

    fn initialize_block(
        height: frame_system::pallet_prelude::BlockNumberFor<Self>,
        parent_hash: <Self as frame_system::Config>::Hash,
    ) -> Result<(), String> {
        SystemOf::<T>::reset_events();
        SystemOf::<T>::initialize(&height, &parent_hash, &Default::default());
        BalancesOf::<T>::on_initialize(height);
        TimestampOf::<T>::set_timestamp(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs()
                .into(),
        );
        TimestampOf::<T>::on_initialize(height);
        ContractsOf::<T>::on_initialize(height);
        SystemOf::<T>::note_finished_initialize();

        Ok(())
    }

    fn finalize_block(
        height: frame_system::pallet_prelude::BlockNumberFor<Self>,
    ) -> Result<<Self as frame_system::Config>::Hash, String> {
        ContractsOf::<T>::on_finalize(height);
        TimestampOf::<T>::on_finalize(height);
        BalancesOf::<T>::on_finalize(height);
        Ok(SystemOf::<T>::finalize().hash())
    }

    fn default_actor() -> AccountIdFor<Self> {
        AccountId32::new([1u8; 32]).into()
    }
}
