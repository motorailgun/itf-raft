use crate::rpc::Responder;
use crate::rpc::raft_rpc::LogEntry;

mod leader;
mod candidate;
mod follower;
mod inner_state;

#[derive(Debug)]
pub enum NodeType {
    Leader(leader::LeaderState),
    Follower(follower::FollowerState),
    Candidate(candidate::CandidateState),
}
use NodeType::*;

#[derive(Debug)]
pub struct State {
    inner: NodeType,
}

impl Responder for State {
    fn new() -> State {
        State {
            inner: NodeType::Follower(follower::FollowerState::new()),
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
        let mut inner = self.inner;
        match &self.inner {
            Follower(state) => {
                let (new_inner, new_term, success) = state.append_entries(term, prev_log_index, prev_log_term, entry);
                self.inner = new_inner;
                
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
