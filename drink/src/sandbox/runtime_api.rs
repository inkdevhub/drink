//! Basic chain API.

use frame_support::sp_runtime::{
    traits::{One, Saturating},
    BuildStorage,
};
use frame_system::{pallet_prelude::BlockNumberFor, GenesisConfig};
use sp_io::TestExternalities;

use super::Sandbox;
use crate::{DrinkResult, Error};

impl<Config: crate::SandboxConfig> Sandbox<Config> {
    /// Creates a new sandbox.
    ///
    /// Returns an error if the storage could not be initialized.
    ///
    /// The storage is initialized with a genesis block with a single account `R::default_actor()` with
    /// `INITIAL_BALANCE`.
    pub fn new() -> DrinkResult<Self> {
        let mut storage = GenesisConfig::<Config::Runtime>::default()
            .build_storage()
            .map_err(Error::StorageBuilding)?;

        Config::initialize_storage(&mut storage).map_err(Error::StorageBuilding)?;

        let mut sandbox = Self {
            externalities: TestExternalities::new(storage),
            _phantom: Default::default(),
        };

        sandbox
            .externalities
            // We start the chain from the 1st block, so that events are collected (they are not
            // recorded for the genesis block...).
            .execute_with(|| {
                Config::initialize_block(
                    BlockNumberFor::<Config::Runtime>::one(),
                    Default::default(),
                )
            })
            .map_err(Error::BlockInitialize)?;

        Ok(sandbox)
    }
    /// Build a new empty block and return the new height.
    pub fn build_block(&mut self) -> DrinkResult<BlockNumberFor<Config::Runtime>> {
        self.execute_with(|| {
            let mut current_block = frame_system::Pallet::<Config::Runtime>::block_number();
            let block_hash = Config::finalize_block(current_block).map_err(Error::BlockFinalize)?;
            current_block.saturating_inc();
            Config::initialize_block(current_block, block_hash).map_err(Error::BlockInitialize)?;
            Ok(current_block)
        })
    }
    /// Build `n` empty blocks and return the new height.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of blocks to build.
    pub fn build_blocks(&mut self, n: u32) -> DrinkResult<BlockNumberFor<Config::Runtime>> {
        let mut last_block = None;
        for _ in 0..n {
            last_block = Some(self.build_block()?);
        }
        Ok(last_block.unwrap_or_else(|| self.block_number()))
    }
}
