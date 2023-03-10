pub(crate) mod get_hits;
mod get_index;
mod get_resources;

pub use get_hits::hits;
pub use get_index::{index, IndexPage};
pub use get_resources::resource;
