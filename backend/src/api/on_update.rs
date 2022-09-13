use std::sync::{Mutex, mpsc::Sender};

use usdpl_back::core::serdes::Primitive;

use super::ApiParameterType;

use crate::runtime::{QueueAction, QueueItem};

/// API web method to notify the back-end of an update event (e.g. click, slider slide, toggle)
pub fn on_update(sender: Sender<QueueItem>) -> impl Fn(ApiParameterType) -> ApiParameterType {
    let sender = Mutex::new(sender);
    move |mut params: ApiParameterType| {
        log::debug!("API: on_update");
        if params.len() == 2 {
            if let Primitive::F64(index) = params.remove(0) {
                let index = index as usize;
                let val = params.remove(0);
                sender.lock().unwrap().send(
                    QueueItem {
                        action: QueueAction::DoUpdate {
                            index,
                            value: val,
                        }
                    }
                ).unwrap();
                log::info!("Sent update for #{}", index);
                vec![true.into()]
            } else {
                vec![false.into()]
            }
        } else {
            vec![false.into()]
        }
    }
}
