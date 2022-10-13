use serde::{Serialize, Deserialize};

use super::TopLevelActionConfig;

#[derive(Serialize, Deserialize, Clone)]
pub struct EventDisplayConfig {
    pub title: String,
    /// Type of event to listen for
    pub event: EventType,
    /// Action to perform when the event occurs
    pub on_event: TopLevelActionConfig,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum EventType {
    #[serde(rename = "achievement")]
    Achievement,
    #[serde(rename = "airplane", alias = "airplane mode")]
    Airplane,
    #[serde(rename = "bluetooth")]
    Bluetooth,
    #[serde(rename = "brightness")]
    Brightness,
    #[serde(rename = "screenshot")]
    Screenshot,
    #[serde(rename = "game-start", alias = "game start")]
    GameStart,
    #[serde(rename = "game-lifetime", alias = "game lifetime")]
    GameLifetime,
}

impl EventType {
    #[inline]
    pub fn is_achievement(&self) -> bool {
        matches!(self, Self::Achievement)
    }

    #[inline]
    pub fn is_airplane(&self) -> bool {
        matches!(self, Self::Airplane)
    }

    #[inline]
    pub fn is_bluetooth(&self) -> bool {
        matches!(self, Self::Bluetooth)
    }

    #[inline]
    pub fn is_brightness(&self) -> bool {
        matches!(self, Self::Brightness)
    }

    #[inline]
    pub fn is_screenshot(&self) -> bool {
        matches!(self, Self::Screenshot)
    }

    #[inline]
    pub fn is_game_start(&self) -> bool {
        matches!(self, Self::GameStart)
    }

    #[inline]
    pub fn is_game_lifetime(&self) -> bool {
        matches!(self, Self::GameLifetime)
    }
}
