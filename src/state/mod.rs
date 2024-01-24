use crate::rpc::Responder;
use crate::rpc::raft_rpc::LogEntry;

mod leader;
mod candidate;
mod follower;
mod inner_state;

use self::inner_state::InnerState;

#[derive(Debug)]
pub enum NodeType {
    Leader(leader::LeaderState),
    Follower(follower::FollowerState),
    Candidate(candidate::CandidateState),
}
use NodeType::*;

#[derive(Debug)]
pub struct State {
    node_type: NodeType,
    inner: InnerState
}

impl Responder for State {
    fn new() -> State {
        State {
            node_type: NodeType::Follower(follower::FollowerState::new()),
            inner: InnerState::init(),
        }
    }

    fn append_entries(
        &mut self,
        term: u64,
        leader_id: u64,
        prev_log_index: u64,
        prev_log_term: u64,
        entry: Option<LogEntry>,
        leader_commit: u64,
    ) -> (u64, bool) {
        match &mut self.node_type {
            Follower(state) => {
                let (new_type, new_term, success) = state.append_entries(&mut self.inner, entry, term);
                self.node_type = new_type;
                
                (new_term, success)
            },
            _ => unimplemented!(),
        }
    }

    fn request_vote(
        &mut self,
        term: u64,
        candidate_id: u64,
        last_log_index: u64,
        last_log_term: u64,
    ) -> (u64, bool) {
        todo!()
    }
}
