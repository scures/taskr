use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod conf;
pub mod notify;
pub mod utils;

pub use conf::Config;
pub use notify::notify_via_hook;
pub use utils::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub time: chrono::DateTime<Utc>,
    pub command: String,
    pub args: Vec<String>,
    pub id: Uuid,
}
