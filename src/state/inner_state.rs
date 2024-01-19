use crate::rpc::raft_rpc::LogEntry;

#[derive(Debug)]
pub struct InnerState {
    id: u64,
    current_term: u64,
    voted_for: u64,
    log: Vec<LogEntry>,
    commit_index: u64,
    last_applied: u64,
}
