use serde::{Serialize, Deserialize};

use super::ActionConfig;

#[derive(Serialize, Deserialize, Clone)]
pub struct ToggleConfig {
    pub title: String,
    pub description: Option<String>,
    pub on_toggle: ActionConfig,
}
