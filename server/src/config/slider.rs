use serde::{Serialize, Deserialize};

use super::ActionConfig;

#[derive(Serialize, Deserialize)]
pub struct SliderConfig {
    pub title: String,
    pub on_set: ActionConfig,
}
