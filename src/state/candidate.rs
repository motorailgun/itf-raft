use std::collections::BTreeMap;
use super::NodeType;
use super::inner_state::InnerState;

#[derive(Debug)]
pub struct CandidateState {
    pub votes: BTreeMap<u64, ()>,
    inner: super::inner_state::InnerState,
}

impl CandidateState {
    fn append_entries(&self, inner: InnerState, term: u64, leader_id: u64) -> NodeType {
        unimplemented!()
    }
}
