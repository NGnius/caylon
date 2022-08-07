use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "action")]
pub enum ActionConfig {
    #[serde(rename = "command")] 
    Command(CommandAction),
}

#[derive(Serialize, Deserialize)]
pub struct CommandAction {
    pub run: String,
}
