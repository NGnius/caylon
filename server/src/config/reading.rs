use serde::{Serialize, Deserialize};

use super::ActionConfig;

#[derive(Serialize, Deserialize)]
pub struct ReadingConfig {
    pub title: String,
    pub period_ms: usize,
    pub on_period: ActionConfig,
}
