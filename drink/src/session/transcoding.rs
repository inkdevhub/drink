use std::{collections::BTreeMap, rc::Rc};

use contract_transcode::ContractMessageTranscoder;

pub struct TranscoderRegistry<Contract: Ord> {
    transcoders: BTreeMap<Contract, Rc<ContractMessageTranscoder>>,
}

impl<Contract: Ord> TranscoderRegistry<Contract> {
    pub fn new() -> Self {
        Self {
            transcoders: BTreeMap::new(),
        }
    }

    pub fn register(&mut self, contract: Contract, transcoder: &Rc<ContractMessageTranscoder>) {
        self.transcoders.insert(contract, Rc::clone(transcoder));
    }

    pub fn get(&self, contract: &Contract) -> Option<Rc<ContractMessageTranscoder>> {
        self.transcoders.get(contract).map(Rc::clone)
    }
}
