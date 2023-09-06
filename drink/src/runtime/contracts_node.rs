//! This module provides the runtime of substrate-contracts-node in a form that can be used by
//! the drink framework.

use crate::runtime::{AccountId, Runtime};
pub use contracts_node_runtime::Runtime as ContractsNodeRuntime;

impl Runtime for ContractsNodeRuntime {
    fn default_actor() -> AccountId<Self> {
        todo!()
    }
}
