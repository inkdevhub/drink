use std::path::PathBuf;

use sp_runtime::AccountId32;

pub struct AppState {
    pub pwd: PathBuf,
    pub block_height: u64,
    pub deployed_contracts: u16,
    pub current_contract_address: AccountId32,
}
