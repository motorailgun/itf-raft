use crate::rpc::raft_rpc::LogEntry;

use super::inner_state::InnerState;
use super::NodeType;

use std::time::Instant;

#[derive(Debug)]
pub struct FollowerState {
    inner: InnerState,
    last_heartbeat: Instant,
}

impl FollowerState {
    pub fn new() -> FollowerState {
        FollowerState {
            inner: InnerState::init(),
            last_heartbeat: Instant::now(),
        }
    }

    pub fn init(inner: InnerState) -> FollowerState {
        FollowerState {
            inner,
            last_heartbeat: Instant::now(),
        }
    }

    fn acceptable_log(&self, entry: &LogEntry, prev_log_index: u64, prev_log_term: u64) -> bool {
        let inner = &self.inner;
        if entry.term < inner.current_term {
            false
        } else if let Some(entry_at) = inner.log.get(prev_log_index as usize) {
            if entry_at.term == prev_log_term {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn append_entries(self, term: u64, prev_log_index: u64, prev_log_term: u64, incoming_entry: Option<LogEntry>) -> (NodeType, u64, bool) {
        let current_term = self.inner.current_term;

        if term < current_term {
            return (NodeType::Follower(Self::init(self.inner)), current_term, false);
        }

        match incoming_entry {
            Some(entry) => {
                if self.acceptable_log(&entry, prev_log_index, prev_log_term) {
                    let mut inner = self.inner;
                    inner.current_term = term;
                    
                    inner.log.truncate((prev_log_index + 1) as usize);
                    inner.log.push(entry);

                    (NodeType::Follower(Self::init(inner)), term, true)
                } else {
                    (NodeType::Follower(Self::init(self.inner)), current_term, false)
                }
            },
            None => {
                (NodeType::Follower(Self::init(self.inner)), current_term, true)
            }
        }
    }
}
