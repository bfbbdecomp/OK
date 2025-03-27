use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub id: String,
    pub email: String,
    pub time: String, // Consider using chrono::DateTime if you need to parse this
    pub message: String,
}
