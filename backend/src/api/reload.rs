use std::sync::{Mutex, mpsc::{Sender, channel}};

use super::ApiParameterType;

use crate::runtime::{QueueAction, QueueItem};

pub fn reload(sender: Sender<QueueItem>) -> impl Fn(ApiParameterType) -> ApiParameterType {
    let sender = Mutex::new(sender);
    move |_| {
        log::debug!("API: reload");
        let (rx, tx) = channel();
        sender.lock().unwrap().send(
            QueueItem {
                action: QueueAction::DoReload {
                    respond_to: rx,
                }
            }
        ).unwrap();
        log::info!("waiting for JSON reload");
        vec![
            usdpl_back::core::serdes::Primitive::Json(
                serde_json::to_string(&tx.recv().unwrap()).unwrap()
            )
        ]
    }
}
