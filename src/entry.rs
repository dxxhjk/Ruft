use serde::{Deserialize, Serialize};

#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
pub struct Entry {
    pub index: usize,
    pub term: u32,
    pub command: String,
}