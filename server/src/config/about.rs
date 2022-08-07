use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct AboutConfig {
    pub name: String,
    pub version: String,
    pub description: String,
    pub url: Option<String>,
    pub author: Option<String>,
    pub license: Option<String>,
}
