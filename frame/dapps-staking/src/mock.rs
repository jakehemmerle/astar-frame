use crate::{self as pallet_dapps_staking, weights};

use frame_support::{
    construct_runtime, parameter_types,
    traits::{Currency, OnFinalize, OnInitialize},
    PalletId,
};
use sp_core::{H160, H256};

use codec::{Decode, Encode};
use sp_io::TestExternalities;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};

pub(crate) type AccountId = u64;
pub(crate) type BlockNumber = u64;
pub(crate) type Balance = u128;
pub(crate) type EraIndex = u32;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;
type Block = frame_system::mocking::MockBlock<TestRuntime>;

/// Value shouldn't be less than 2 for testing purposes, otherwise we cannot test certain corner cases.
pub(crate) const EXISTENTIAL_DEPOSIT: Balance = 2;
pub(crate) const MAX_NUMBER_OF_STAKERS: u32 = 4;
/// Value shouldn't be less than 2 for testing purposes, otherwise we cannot test certain corner cases.
pub(crate) const MINIMUM_STAKING_AMOUNT: Balance = 10;
pub(crate) const MINIMUM_STAKING_AMOUNT_PER_CALL: Balance = 2;
pub(crate) const MINIMUM_REMAINING_AMOUNT: Balance = 1;
pub(crate) const MAX_UNLOCKING_CHUNKS: u32 = 4;
pub(crate) const UNBONDING_PERIOD: EraIndex = 3;
pub(crate) const MAX_ERA_STAKE_VALUES: u32 = 8;

// Do note that this needs to at least be 3 for tests to be valid. It can be greater but not smaller.
pub(crate) const BLOCKS_PER_ERA: BlockNumber = 3;

pub(crate) const REGISTER_DEPOSIT: Balance = 10;

pub(crate) const STAKER_BLOCK_REWARD: Balance = 531911;
pub(crate) const DAPP_BLOCK_REWARD: Balance = 773333;

construct_runtime!(
    pub enum TestRuntime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
        DappsStaking: pallet_dapps_staking::{Pallet, Call, Storage, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::simple_max(1024);
}

impl frame_system::Config for TestRuntime {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type Origin = Origin;
    type Index = u64;
    type Call = Call;
    type BlockNumber = BlockNumber;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type DbWeight = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
    pub const MaxLocks: u32 = 4;
    pub const ExistentialDeposit: Balance = EXISTENTIAL_DEPOSIT;
}

impl pallet_balances::Config for TestRuntime {
    type MaxLocks = MaxLocks;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type Balance = Balance;
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
}

parameter_types! {
    pub const MinimumPeriod: u64 = 3;
}

impl pallet_timestamp::Config for TestRuntime {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

parameter_types! {
    pub const RegisterDeposit: Balance = REGISTER_DEPOSIT;
    pub const BlockPerEra: BlockNumber = BLOCKS_PER_ERA;
    pub const MaxNumberOfStakersPerContract: u32 = MAX_NUMBER_OF_STAKERS;
    pub const MinimumStakingAmount: Balance = MINIMUM_STAKING_AMOUNT;
    pub const MinimumStakingAmountPerCall: Balance = MINIMUM_STAKING_AMOUNT_PER_CALL; // in practice should be any value between 0 and MINIMUM_STAKING_AMOUNT.
    pub const DappsStakingPalletId: PalletId = PalletId(*b"mokdpstk");
    pub const MinimumRemainingAmount: Balance = MINIMUM_REMAINING_AMOUNT;
    pub const MaxUnlockingChunks: u32 = MAX_UNLOCKING_CHUNKS;
    pub const UnbondingPeriod: EraIndex = UNBONDING_PERIOD;
    pub const MaxEraStakeValues: u32 = MAX_ERA_STAKE_VALUES;
}

impl pallet_dapps_staking::Config for TestRuntime {
    type Event = Event;
    type Currency = Balances;
    type BlockPerEra = BlockPerEra;
    type RegisterDeposit = RegisterDeposit;
    type SmartContract = MockSmartContract<AccountId>;
    type WeightInfo = weights::SubstrateWeight<TestRuntime>;
    type MaxNumberOfStakersPerContract = MaxNumberOfStakersPerContract;
    type MinimumStakingAmount = MinimumStakingAmount;
    type MinimumStakingAmountPerCall = MinimumStakingAmountPerCall;
    type PalletId = DappsStakingPalletId;
    type MinimumRemainingAmount = MinimumRemainingAmount;
    type MaxUnlockingChunks = MaxUnlockingChunks;
    type UnbondingPeriod = UnbondingPeriod;
    type MaxEraStakeValues = MaxEraStakeValues;
}

#[derive(PartialEq, Eq, Copy, Clone, Encode, Decode, Debug, scale_info::TypeInfo)]
pub enum MockSmartContract<AccountId> {
    Evm(sp_core::H160),
    Wasm(AccountId),
}

impl<AccountId> Default for MockSmartContract<AccountId> {
    fn default() -> Self {
        MockSmartContract::Evm(H160::repeat_byte(0x01))
    }
}

impl<AccountId> pallet_dapps_staking::IsContract for MockSmartContract<AccountId> {
    fn is_valid(&self) -> bool {
        match self {
            MockSmartContract::Wasm(_account) => false,
            MockSmartContract::Evm(_account) => true,
        }
    }
}

pub struct ExternalityBuilder;

impl ExternalityBuilder {
    pub fn build() -> TestExternalities {
        let mut storage = frame_system::GenesisConfig::default()
            .build_storage::<TestRuntime>()
            .unwrap();

        pallet_balances::GenesisConfig::<TestRuntime> {
            balances: vec![
                (1, 9000),
                (2, 800),
                (3, 10000),
                (4, 4900),
                (5, 3800),
                (6, 10),
                (7, 1000),
                (8, 2000),
                (9, 10000),
                (10, 300),
                (11, 400),
                (20, 10),
                (540, EXISTENTIAL_DEPOSIT),
                (1337, 1_000_000_000_000),
            ],
        }
        .assimilate_storage(&mut storage)
        .ok();

        let mut ext = TestExternalities::from(storage);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}

/// Used to run to the specified block number
pub fn run_to_block(n: u64) {
    while System::block_number() < n {
        DappsStaking::on_finalize(System::block_number());
        System::set_block_number(System::block_number() + 1);
        // This is performed outside of dapps staking but we expect it before on_initialize
        payout_block_rewards();
        DappsStaking::on_initialize(System::block_number());
    }
}

/// Used to run the specified number of blocks
pub fn run_for_blocks(n: u64) {
    run_to_block(System::block_number() + n);
}

/// Advance blocks to the beginning of an era.
///
/// Function has no effect if era is already passed.
pub fn advance_to_era(n: EraIndex) {
    while DappsStaking::current_era() < n {
        run_for_blocks(1);
    }
}

/// Initialize first block.
/// This method should only be called once in a UT otherwise the first block will get initialized multiple times.
pub fn initialize_first_block() {
    // This assert prevents method misuse
    assert_eq!(System::block_number(), 1 as BlockNumber);

    // This is performed outside of dapps staking but we expect it before on_initialize
    payout_block_rewards();
    DappsStaking::on_initialize(System::block_number());
    run_to_block(2);
}

/// Returns total block rewards that goes to dapps-staking.
/// Contains both `dapps` reward and `stakers` reward.
pub fn joint_block_reward() -> Balance {
    STAKER_BLOCK_REWARD + DAPP_BLOCK_REWARD
}

/// Payout block rewards to stakers & dapps
fn payout_block_rewards() {
    DappsStaking::rewards(
        Balances::issue(STAKER_BLOCK_REWARD.into()),
        Balances::issue(DAPP_BLOCK_REWARD.into()),
    );
}

// Used to get a vec of all dapps staking events
pub fn dapps_staking_events() -> Vec<crate::Event<TestRuntime>> {
    System::events()
        .into_iter()
        .map(|r| r.event)
        .filter_map(|e| {
            if let Event::DappsStaking(inner) = e {
                Some(inner)
            } else {
                None
            }
        })
        .collect()
}
