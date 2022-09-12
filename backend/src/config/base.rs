use serde::{Serialize, Deserialize};

use super::{ElementConfig, AboutConfig};

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "api-version")]
pub enum BaseConfig {
    #[serde(rename = "v0.0.0")]
    V0 {
        items: Vec<ElementConfig>,
        about: AboutConfig,
    },
}

impl BaseConfig {
    pub fn load<P: AsRef<std::path::Path>>(path: P) -> Self {
        //let path = std::path::Path::new("./").join(path);
        let path = path.as_ref();
        match std::fs::File::open(&path) {
            Ok(file) => {
                let reader = std::io::BufReader::new(file);
                match serde_json::from_reader(reader) {
                    Ok(conf) => return conf,
                    Err(e) => log::error!("Failed to deserialize {}: {}", path.display(), e),
                }
            },
            Err(e) => log::error!("Failed to open {}: {}", path.display(), e),
        }
        panic!("Cannot open {}", path.display())
    }

    #[inline]
    pub fn get_about(&self) -> &AboutConfig {
        match self {
            Self::V0 {about, ..} => about,
        }
    }

    #[inline]
    pub fn get_item(&self, index: usize) -> Option<&ElementConfig> {
        match self {
            Self::V0 {items, ..} => items.get(index),
        }
    }

    #[inline]
    pub fn items(&self) -> &Vec<ElementConfig> {
        match self {
            Self::V0 {items, ..} => items,
        }
    }
}
