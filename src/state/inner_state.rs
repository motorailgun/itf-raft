use crate::rpc::raft_rpc::LogEntry;
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct InnerState {
    id: u64,
    pub current_term: u64,
    pub voted_for: Option<u64>,
    pub log: Vec<LogEntry>,
    pub commit_index: u64,
    pub last_applied: u64,
}

impl InnerState {
    pub fn init() -> InnerState {
        InnerState {
            id: thread_rng().gen(),
            current_term: 0,
            voted_for: None,
            log: vec![LogEntry {
                // log must be 1-indexed 
                // because of prev_log_index, which is uint
                key: "".into(),
                value: "".into(),
                term: 0,
                index: 0,
            }],
            commit_index: 0,
            last_applied: 0,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }
}
