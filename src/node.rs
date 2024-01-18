use crate::state::State;
use crate::rpc::Responder;
use crate::rpc::raft_rpc::LogEntry;
use std::sync::{Mutex, Arc};

#[derive(Debug)]
pub struct Node {
    inner: Arc<Mutex<State>>,
}

unsafe impl Sync for Node {}
unsafe impl Send for Node {}

impl Node {
    pub fn new() -> Node {
        Node {
            inner: Arc::new(Mutex::new(State::new())),
        }
    }
}

#[tonic::async_trait]
impl Responder for Node {
    async fn append_entries(&self,
        term: u64, leader_id: u64, prev_log_index: u64, prev_log_term: u64,
        entry: Option<LogEntry>, leader_commit: u64) -> Option<u64> {
        todo!();
    }

    async fn request_vote(&self, term: u64, candidate_id: u64, last_log_index: u64, last_log_term: u64) -> (u64, bool) {
        todo!();
    }
}
