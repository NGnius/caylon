use std::sync::{Mutex, mpsc::Sender};

use usdpl_back::core::serdes::Primitive;

use super::ApiParameterType;

use crate::runtime::{QueueAction, QueueItem};

/// API web method to notify the back-end of a Javascript result (initiated by a JS action)
pub fn on_javascript_result(sender: Sender<QueueItem>) -> impl Fn(ApiParameterType) -> ApiParameterType {
    let sender = Mutex::new(sender);
    move |mut params: ApiParameterType| {
        log::debug!("API: on_javascript_result");
        if params.len() == 2 {
            if let Primitive::F64(id) = params.remove(0) {
                let id = id as usize;
                let val = params.remove(0);
                sender.lock().unwrap().send(
                    QueueItem {
                        action: QueueAction::DoJavascriptResult {
                            id,
                            value: val,
                        }
                    }
                ).unwrap();
                log::info!("Sent JS result for #{}", id);
                vec![true.into()]
            } else {
                vec![false.into()]
            }
        } else {
            vec![false.into()]
        }
    }
}
