use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "action")]
pub enum ActionConfig {
    #[serde(rename = "command")]
    Command(CommandAction),
    #[serde(rename = "transform")]
    Transform(super::TransformAction),
    #[serde(rename = "mirror")]
    Mirror(MirrorAction),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CommandAction {
    pub run: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MirrorAction;
