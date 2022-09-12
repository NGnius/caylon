use serde::{Serialize, Deserialize};

use super::ActionConfig;

#[derive(Serialize, Deserialize, Clone)]
pub struct SliderConfig {
    pub title: String,
    pub min: u64,
    pub max: u64,
    pub notches: Option<Vec<String>>,
    pub on_set: ActionConfig,
}
