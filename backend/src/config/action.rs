use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "action")]
pub enum ActionConfig {
    #[serde(rename = "command")] 
    Command(CommandAction),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CommandAction {
    pub run: String,
}
