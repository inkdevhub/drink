mod runtime;

use sp_io::TestExternalities;

use crate::runtime::*;

pub struct Sandbox {
    externalities: TestExternalities,
}

impl Sandbox {
    pub fn new() -> Self {
        let storage = frame_system::GenesisConfig::default()
            .build_storage::<SandboxRuntime>()
            .unwrap();

        Self {
            externalities: TestExternalities::new(storage),
        }
    }

    pub fn deploy_contract(&mut self, contract_bytes: &[u8]) {}
}
