mod actor;
mod command_actor;
mod periodic_actor;
mod transform_actor;

pub use actor::{Actor, Act, ActError, ActorType};
pub use command_actor::CommandActor;
pub use periodic_actor::PeriodicActor;
pub use transform_actor::TransformActor;

pub const VALUE_VAR: &str = "KAYLON_VALUE";
