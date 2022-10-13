use serde::{Serialize, Deserialize};

use super::TopLevelActionConfig;

#[derive(Serialize, Deserialize, Clone)]
pub struct ReadingConfig {
    pub title: String,
    /// Period in milliseconds, or None/null for non-repeating actions
    pub period_ms: Option<u64>,
    /// Action to perform on every period
    pub on_period: TopLevelActionConfig,
}
