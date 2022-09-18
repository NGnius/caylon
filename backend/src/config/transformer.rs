//! Transformers, robots in disguise! (or value-based transformations)

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct TransformAction {
    pub transformer: TransformTypeAction,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "rule")]
pub enum TransformTypeAction {
    #[serde(rename = "replace")]
    Replace(ReplaceTransformAction),
    #[serde(rename = "expand")]
    Expand(ExpandTransformAction),
    #[serde(rename = "log")]
    Log(LogTransformAction),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ReplaceTransformAction {
    /// Regex
    pub patterns: Vec<PatternConfig>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PatternConfig {
    /// Regex
    pub pattern: String,
    /// Formatting info https://docs.rs/regex/latest/regex/struct.Regex.html#replacement-string-syntax
    pub format: String,
    // Regex case_insensitive flags
    pub i: Option<bool>,
    // Regex multi_line flags
    pub m: Option<bool>,
    // Regex dot_matches_new_line flags
    pub s: Option<bool>,
    // Regex swap_greed flags
    #[serde(rename = "U")]
    pub u: Option<bool>,
    // Regex ignore_whitespace flags
    pub x: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ExpandTransformAction {
    pub format: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LogTransformAction {
    pub level: LogLevel,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum LogLevel {
    DEBUG,
    INFO,
    WARN,
    ERROR,
}
