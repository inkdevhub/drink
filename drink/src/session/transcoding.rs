use std::{collections::HashMap, hash::Hash, rc::Rc};

use contract_transcode::ContractMessageTranscoder;

#[derive(Default)]
pub struct TranscoderRegistry<Contract: Eq + Hash> {
    transcoders: HashMap<Contract, Rc<ContractMessageTranscoder>>,
}

impl<Contract: Eq + Hash> TranscoderRegistry<Contract> {
    pub fn register(&mut self, contract: Contract, transcoder: Rc<ContractMessageTranscoder>) {
        self.transcoders.insert(contract, Rc::clone(&transcoder));
    }

    pub fn get(&self, contract: &Contract) -> Option<Rc<ContractMessageTranscoder>> {
        self.transcoders.get(contract).map(Rc::clone)
    }
}
