mod actor;
mod command_actor;
mod javascript_actor;
mod json_actor;
mod periodic_actor;
mod sequential_actor;
mod transform_actor;

pub use actor::{Actor, Act, ActError, ActorType, SeqAct, SeqActor, TopLevelActorType};
#[cfg(test)]
pub use actor::{Expected, SeqActTestHarness, /*ActTestHarness*/};
pub use command_actor::CommandActor;
pub use javascript_actor::JavascriptActor;
pub use json_actor::JsonActor;
pub use periodic_actor::PeriodicActor;
pub use sequential_actor::SequenceActor;
pub use transform_actor::TransformActor;

pub const VALUE_VAR: &str = "CAYLON_VALUE";
