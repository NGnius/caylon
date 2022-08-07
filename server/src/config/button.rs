use serde::{Serialize, Deserialize};

use super::ActionConfig;

#[derive(Serialize, Deserialize)]
pub struct ButtonConfig {
    pub title: String,
    pub on_click: ActionConfig,
}
