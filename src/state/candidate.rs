use std::collections::BTreeMap;
use super::NodeType;
use super::inner_state::InnerState;

#[derive(Debug)]
pub struct CandidateState {
    pub votes: BTreeMap<u64, ()>,
    inner: InnerState,
}

impl CandidateState {
    pub fn init(inner: InnerState) -> CandidateState {
        CandidateState {
            votes: BTreeMap::new(),
            inner,
        }
    }

    pub fn append_entries(self, inner: InnerState, term: u64, leader_id: u64) -> NodeType {
        todo!()
    }
}
