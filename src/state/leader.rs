use std::collections::HashMap;

#[derive(Debug)]
pub struct LeaderState{
    pub next_index: HashMap<u64, u64>,
    pub match_index: HashMap<u64, u64>,
}
