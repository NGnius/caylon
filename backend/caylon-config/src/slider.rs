use serde::{Serialize, Deserialize};

use super::TopLevelActionConfig;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct SliderConfig {
    pub title: String,
    pub min: u64,
    pub max: u64,
    pub notches: Option<Vec<String>>,
    pub on_set: TopLevelActionConfig,
}
