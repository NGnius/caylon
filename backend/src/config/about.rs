use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct AboutConfig {
    pub name: String,
    pub version: String,
    pub description: String,
    pub url: Option<String>,
    pub authors: Vec<String>,
    pub license: Option<String>,
}

impl Default for AboutConfig {
    fn default() -> Self {
        Self {
            name: env!("CARGO_PKG_NAME").to_owned(),
            version: env!("CARGO_PKG_VERSION").to_owned(),
            description: env!("CARGO_PKG_DESCRIPTION").to_owned(),
            url: Some(env!("CARGO_PKG_HOMEPAGE").to_owned()),
            authors: env!("CARGO_PKG_AUTHORS").split(':').map(|x| x.to_owned()).collect(),
            license: Some(env!("CARGO_PKG_LICENSE").to_owned())
        }
    }
}
