pub mod rpc_server;

use std::collections::HashMap;

// State machine must not be thread safe as it is not designed to be shared
struct StateMachine {
    state: HashMap<String, String>,
}

impl StateMachine {
    fn new() -> StateMachine {
        StateMachine {
            state: HashMap::new(),
        }
    }

    fn apply(&mut self, key: String, value: String) {
        self.state.insert(key, value);
    }
}

struct LogEntry {
    key: String,
    value: String,
}

enum NodeType {
    Leader,
    Follower,
    Candidate,
}

struct NodeState {
    node_type: NodeType,
    state_machine: StateMachine,
    log_queue: Vec<LogEntry>,
    term: u64,
}

fn main() {
    println!("Hello, world!");
}
