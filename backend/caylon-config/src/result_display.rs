use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct ResultDisplayConfig {
    pub title: String,
    /// Index of element who's action's result will be used
    pub result_of: usize,
}
