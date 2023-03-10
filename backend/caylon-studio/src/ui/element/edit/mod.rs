mod action_editor;
mod always_str;
mod always_usize;
mod optional_str;
mod optional_u64;

pub mod actions;

pub use action_editor::{ActionComponent, ActionProps};
pub use always_str::{AlwaysStringComponent, AlwaysStringProps};
pub use always_usize::{AlwaysUsizeComponent, AlwaysUsizeProps};
pub use optional_str::{OptionStringComponent, OptionStringProps};
pub use optional_u64::{OptionU64Component, OptionU64Props};

type EditCallback = yew::prelude::Callback<super::ElementMessage>;
