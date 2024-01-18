pub mod rpc;
pub mod state;
pub mod node;

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


fn main() {
    println!("Hello, world!");
}
