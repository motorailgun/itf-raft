use crate::rpc::raft_rpc::LogEntry;
use super::{inner_state::InnerState, NodeType};
use std::time::{Duration, Instant};

const timeout: Duration = Duration::from_millis(300);

#[derive(Debug)]
pub struct FollowerState {
    pub last_heartbeat: Instant,
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
                    state.voted_for = None;
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

    pub fn request_vote(&mut self, state: &mut InnerState, candidate_id: u64, last_log_index: u64, last_log_term: u64) -> (u64, bool) {
        let now = Instant::now();
        let not_voted = state.voted_for == None;
        let timeouted = now - self.last_heartbeat > timeout;
        let up_to_date_or_newer = last_log_term >= state.current_term && last_log_index >= state.last_log_index();
        let acceptable = up_to_date_or_newer && (timeouted || not_voted);

        if acceptable {
            state.voted_for = Some(candidate_id);
            (state.current_term, true)
        } else {
            if timeouted {
                state.voted_for = None;
            }
            (state.current_term, false)
        }
    }
}
