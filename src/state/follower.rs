use log::*;

use crate::rpc::raft_rpc::LogEntry;
use super::{inner_state::InnerState, NodeType};
use std::time::Instant;

#[derive(Debug)]
pub struct FollowerState {
    last_heartbeat: Instant,
}

impl FollowerState {
    pub fn new() -> FollowerState {
        FollowerState {
            last_heartbeat: Instant::now(),
        }
    }

    pub fn append_entries(&mut self, state: &mut InnerState, incoming_entry: Option<LogEntry>, term: u64) -> (NodeType, u64, bool) {
        if state.current_term < term {
            return (NodeType::Follower(FollowerState::new()), state.current_term, false)
        }
        if let Some(incoming_entry) = incoming_entry {
            match state.append_entry(incoming_entry) {
                Ok(_) => {
                    (NodeType::Follower(FollowerState::new()), state.current_term, true)
                },
                Err(e) => {
                    log::error!("{}", e);
                    (NodeType::Follower(FollowerState::new()), state.current_term, false)
                }
            }
        } else {
            state.current_term = term;
            (NodeType::Follower(FollowerState::new()), state.current_term, true)
        }
    }
}
