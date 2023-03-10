//! HTML versions of UI elements that Caylon supports.
//! These do not aim to exactly clone the Steam Deck style,
//! as long as they give the right impression.

mod add_element;
mod button;
mod elements;
mod event_display;
mod msg_common;
mod reading_display;
mod remove_element;
mod result_display;
mod slider;
mod toggle;

pub mod fake;
pub mod edit;

pub use add_element::AddElementComponent;
pub use elements::ElementsComponent;
pub use msg_common::{ElementMessage, ElementCtx, ElementContext};
pub use remove_element::{RemoveElementComponent, RemoveElementProps};
