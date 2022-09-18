use serde::{Serialize, Deserialize};

use super::TopLevelActionConfig;

#[derive(Serialize, Deserialize, Clone)]
pub struct ButtonConfig {
    pub title: String,
    pub on_click: TopLevelActionConfig,
}
