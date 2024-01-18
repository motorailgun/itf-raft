use std::collections::BTreeMap;

#[derive(Debug)]
pub struct CandidateState {
    pub votes: BTreeMap<u64, ()>,
}
