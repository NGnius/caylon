use serde::{Serialize, Deserialize};

use super::{ButtonConfig, ToggleConfig, SliderConfig, ReadingConfig};

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "element")]
pub enum ElementConfig {
    #[serde(rename = "button")]
    Button(ButtonConfig),
    #[serde(rename = "toggle")]
    Toggle(ToggleConfig),
    #[serde(rename = "slider")]
    Slider(SliderConfig),
    #[serde(rename = "reading")]
    Reading(ReadingConfig),
}
