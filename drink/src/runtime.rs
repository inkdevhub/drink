use frame_support::{
    parameter_types,
    sp_runtime::{
        testing::{Header, H256},
        traits::{BlakeTwo256, Convert, IdentityLookup},
        AccountId32,
    },
    traits::{ConstBool, ConstU32, ConstU64, Currency, Randomness},
    weights::Weight,
};
use pallet_contracts::{DefaultAddressGenerator, Frame, Schedule};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<SandboxRuntime>;
type Block = frame_system::mocking::MockBlock<SandboxRuntime>;

frame_support::construct_runtime!(
    pub enum SandboxRuntime where
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

impl frame_system::Config for SandboxRuntime {
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
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
}

impl pallet_balances::Config for SandboxRuntime {
    type Balance = u64;
    type DustRemoval = ();
    type RuntimeEvent = RuntimeEvent;
    type ExistentialDeposit = ConstU64<1>;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
}

impl pallet_timestamp::Config for SandboxRuntime {
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
impl Convert<Weight, BalanceOf> for SandboxRuntime {
    fn convert(w: Weight) -> BalanceOf {
        w.ref_time()
    }
}

parameter_types! {
    pub SandboxSchedule: Schedule<SandboxRuntime> = {
        <Schedule<SandboxRuntime>>::default()
    };
    pub DeletionWeightLimit: Weight = Weight::zero();
}

impl pallet_contracts::Config for SandboxRuntime {
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
    type DeletionQueueDepth = ConstU32<10>;
    type DeletionWeightLimit = DeletionWeightLimit;
    type DepositPerByte = ConstU64<0>;
    type DepositPerItem = ConstU64<0>;
    type AddressGenerator = DefaultAddressGenerator;
    type MaxCodeLen = ConstU32<{ 123 * 1024 }>;
    type MaxStorageKeyLen = ConstU32<128>;
    type UnsafeUnstableInterface = ConstBool<false>;
    type MaxDebugBufferLen = ConstU32<{ 2 * 1024 * 1024 }>;
}
