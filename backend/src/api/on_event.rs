use std::sync::{Mutex, mpsc::Sender};

use usdpl_back::core::serdes::Primitive;

use super::ApiParameterType;

use crate::runtime::{QueueAction, QueueItem};

/// API web method to notify the back-end of a steam event (i.e. callback through SteamClient API)
pub fn on_event(sender: Sender<QueueItem>) -> impl Fn(ApiParameterType) -> ApiParameterType {
    let sender = Mutex::new(sender);
    move |params: ApiParameterType| {
        log::debug!("API: on_event");
        if let Some(Primitive::Json(event_data)) = params.get(0) {
            match serde_json::from_str(event_data) {
                Ok(event_obj) => {
                    sender.lock().unwrap().send(
                        QueueItem {
                            action: QueueAction::DoSteamEvent {
                                event: event_obj,
                            }
                        }
                    ).unwrap();
                    log::info!("Sent steam event");
                    vec![true.into()]
                },
                Err(e) => {
                    log::error!("Failed to parse event json: {}", e);
                    vec![false.into()]
                }
            }
        } else {
            vec![false.into()]
        }
    }
}
