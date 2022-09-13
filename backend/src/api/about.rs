use std::sync::{Mutex, mpsc::{Sender, channel}};

use super::ApiParameterType;

use crate::runtime::{QueueAction, QueueItem};

/// API web method to retrieve AboutConfig from the back-end, as described in the config file
pub fn get_about(sender: Sender<QueueItem>) -> impl Fn(ApiParameterType) -> ApiParameterType {
    let sender = Mutex::new(sender);
    move |_| {
        log::debug!("API: get_about");
        let (rx, tx) = channel();
        sender.lock().unwrap().send(
            QueueItem {
                action: QueueAction::GetAbout {
                    respond_to: rx,
                }
            }
        ).unwrap();
        vec![
            usdpl_back::core::serdes::Primitive::Json(
                serde_json::to_string(&tx.recv().unwrap()).unwrap()
            )
        ]
    }
}
