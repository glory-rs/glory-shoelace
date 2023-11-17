use serde::{Deserialize, Serialize};

use glory::Cage;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Notification {
    pub id: u64,
    pub brief: String,
    pub crated_at: String,
    pub is_read: Cage<bool>,
}
