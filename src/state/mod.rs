use std::collections::{HashMap, BTreeMap};
use rand::{thread_rng, Rng};

use crate::rpc::Responder;
use crate::rpc::raft_rpc::LogEntry;

mod leader;
mod candidate;

#[derive(Debug)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

#[derive(Debug)]
pub struct LeaderState {
    pub next_index: HashMap<u64, u64>,
    pub match_index: HashMap<u64, u64>,
}

#[derive(Debug)]
pub struct CandidateState {
    pub votes: BTreeMap<u64, ()>,
}

#[derive(Debug)]
pub enum NodeType {
    Leader(leader::LeaderState),
    Follower,
    Candidate(candidate::CandidateState),
}

#[derive(Debug)]
pub struct State {
    pub id: u64,
    current_term: u64,
    pub voted_for: u64,
    log: Vec<KeyValue>,
    commit_index: u64,
    last_applied: u64,
    current_type: NodeType,
}

impl State {
    pub fn push_log(&mut self, key: String, value: String) {
        self.log.push(KeyValue {
            key,
            value
        });
    }

    pub fn update_current_term(&mut self, term: u64) -> u64 {
        if term > self.current_term {
            self.current_term = term;
        }
        self.current_term
    }
}

impl Responder for State {
    fn new() -> State {
        State {
            id: thread_rng().gen(),
            current_term: 0,
            voted_for: 0,
            log: Vec::new(),
            commit_index: 0,
            last_applied: 0,
            current_type: NodeType::Follower,
        }
    }

    fn append_entries(
        &self,
        term: u64,
        leader_id: u64,
        prev_log_index: u64,
        prev_log_term: u64,
        entry: Option<LogEntry>,
        leader_commit: u64,
    ) -> Option<u64>{
        todo!()
    }

    fn request_vote(
        &self,
        term: u64,
        candidate_id: u64,
        last_log_index: u64,
        last_log_term: u64,
    ) -> (u64, bool) {
        todo!()
    }
}