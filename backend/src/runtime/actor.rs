use usdpl_back::core::serdes::Primitive;

use crate::config::{ElementConfig, ActionConfig};

pub type ActError = String;

/// Something capable of performing an action.
pub trait Act: Sized {
    type Param;
    type Config: ?Sized;
    type Return;
    fn build(config: &Self::Config, parameter: Self::Param) -> Result<Self, ActError>;
    fn run(self) -> Self::Return;
}

/// Action performer for a regular element
pub struct Actor {
    actor_type: ActorType,
    index: usize,
}

impl Act for Actor {
    type Param = (usize, Primitive);
    type Config = ElementConfig;
    type Return = Primitive;

    fn build(config: &ElementConfig, parameter: Self::Param) -> Result<Self, ActError> {
        let a_type = match config {
            ElementConfig::Button(b) => ActorType::build(&b.on_click, parameter.1),
            ElementConfig::Toggle(t) => ActorType::build(&t.on_toggle, parameter.1),
            ElementConfig::Slider(s) => ActorType::build(&s.on_set, parameter.1),
            ElementConfig::ReadingDisplay(r) => ActorType::build(&r.on_period, parameter.1),
            ElementConfig::ResultDisplay(_) => Err(format!("Item #{} is a ResultDisplay, which can't act", parameter.0)),
        }?;
        Ok(Self {
            actor_type: a_type,
            index: parameter.0,
        })
    }

    fn run(self) -> Self::Return {
        log::info!("Running act for item {}", self.index);
        let result = self.actor_type.run();
        log::info!("Completed act for item {}", self.index);
        result
    }
}

pub enum ActorType {
    Command(super::CommandActor),
}

impl Act for ActorType {
    type Param = Primitive;
    type Config = ActionConfig;
    type Return = Primitive;

    fn build(config: &Self::Config, parameter: Self::Param) -> Result<Self, ActError> {
        Ok(match config {
            ActionConfig::Command(c) =>
                Self::Command(super::CommandActor::build(c, parameter)?),
        })
    }

    fn run(self) -> Self::Return {
        match self {
            Self::Command(c) => c.run().into(),
        }
    }
}
