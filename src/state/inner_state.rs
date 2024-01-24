use crate::rpc::raft_rpc::LogEntry;
use rand::{thread_rng, Rng};
use thiserror::Error;
use tokio::task::LocalEnterGuard;

#[derive(Debug)]
pub struct InnerState {
    id: u64,
    pub current_term: u64,
    pub voted_for: Option<u64>,
    pub log: Vec<LogEntry>,
    pub commit_index: u64,
    pub last_applied: u64,
}

#[derive(Debug, Error)]
pub enum LogAcceptanceError {
    #[error("no matching entry in log")]
    InvalidLog,
    #[error("term is old")]
    OldTerm,
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
                prev_index: 0,
                prev_term: 0,
            }],
            commit_index: 0,
            last_applied: 0,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    fn acceptable_log(&self, entry: &LogEntry) -> Result<(), LogAcceptanceError> {
        let &LogEntry {
            prev_index, prev_term, term, ..
        } = entry;
        if term < self.current_term {
            Err(LogAcceptanceError::OldTerm)
        } else if let Some(entry_at) = self.log.get(prev_index as usize) {
            if entry_at.term == prev_term {
                Ok(())
            } else {
                Err(LogAcceptanceError::InvalidLog)
            }
        } else {
            panic!("log[] is empty: invalid internal state")
        }
    }

    pub fn append_entry(&mut self, entry: LogEntry) -> Result<(), LogAcceptanceError> {
        match self.acceptable_log(&entry) {
            Ok(_) => {
                self.current_term = entry.term;
                self.log.split_off((entry.prev_index + 1) as usize);
                self.log.push(entry);
                Ok(())
            },
            Err(e) => Err(e),
        }
    } 
}
