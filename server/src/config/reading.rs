use serde::{Serialize, Deserialize};

use super::ActionConfig;

#[derive(Serialize, Deserialize, Clone)]
pub struct ReadingConfig {
    pub title: String,
    pub period_ms: u64,
    pub on_period: ActionConfig,
}
