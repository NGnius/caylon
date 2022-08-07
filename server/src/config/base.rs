use serde::{Serialize, Deserialize};

use super::{ElementConfig, AboutConfig};

#[derive(Serialize, Deserialize)]
#[serde(tag = "api-version")]
pub enum BaseConfig {
    #[serde(rename = "v0.0.0")]
    V0 {
        items: Vec<ElementConfig>,
        about: AboutConfig,
    },
}
