use std::sync::mpsc::Sender;

use usdpl_back::core::serdes::Primitive;

use crate::config::{AboutConfig, ElementConfig};

pub enum QueueAction {
    GetAbout {
        respond_to: Sender<AboutConfig>,
    },
    DoUpdate {
        index: usize,
        value: Primitive,
    },
    DoReload {
        respond_to: Sender<Vec<ElementConfig>>
    },
    SetCallback {
        index: usize,
        respond_to: Sender<Primitive>,
    }
}

pub struct QueueItem {
    pub action: QueueAction,
}
