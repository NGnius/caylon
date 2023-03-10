use serde::{Serialize, Deserialize};

use super::TopLevelActionConfig;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct ToggleConfig {
    pub title: String,
    pub description: Option<String>,
    pub on_toggle: TopLevelActionConfig,
}
