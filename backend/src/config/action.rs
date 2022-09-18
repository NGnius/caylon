use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "action")]
pub enum TopLevelActionConfig {
    #[serde(rename = "sequence")]
    Sequence(SequenceAction),
    #[serde(rename = "command")]
    Command(CommandAction),
    #[serde(rename = "transform")]
    Transform(super::TransformAction),
    #[serde(rename = "mirror")]
    Mirror(MirrorAction),
    #[serde(rename = "javascript")]
    Javascript(JavascriptAction),
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "action")]
pub enum ActionConfig {
    #[serde(rename = "command")]
    Command(CommandAction),
    #[serde(rename = "transform")]
    Transform(super::TransformAction),
    #[serde(rename = "javascript")]
    Javascript(JavascriptAction),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SequenceAction {
    pub steps: Vec<ActionConfig>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CommandAction {
    pub run: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MirrorAction;

#[derive(Serialize, Deserialize, Clone)]
pub struct JavascriptAction {
    pub run: String,
}
