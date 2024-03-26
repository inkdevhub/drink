use std::{collections::BTreeMap, sync::Arc};

use contract_transcode::ContractMessageTranscoder;

pub struct TranscoderRegistry<Contract: Ord> {
    transcoders: BTreeMap<Contract, Arc<ContractMessageTranscoder>>,
}

impl<Contract: Ord> TranscoderRegistry<Contract> {
    pub fn new() -> Self {
        Self {
            transcoders: BTreeMap::new(),
        }
    }

    pub fn register(&mut self, contract: Contract, transcoder: &Arc<ContractMessageTranscoder>) {
        self.transcoders.insert(contract, Arc::clone(transcoder));
    }

    pub fn get(&self, contract: &Contract) -> Option<Arc<ContractMessageTranscoder>> {
        self.transcoders.get(contract).map(Arc::clone)
    }
}
