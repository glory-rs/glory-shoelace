use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Notification {
    pub id: u64,
    pub brief: String,
    pub crated_at: String,
    pub is_read: bool,
}