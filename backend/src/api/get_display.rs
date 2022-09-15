use std::sync::{Mutex, mpsc::{Sender, channel}};

use usdpl_back::core::serdes::Primitive;
use usdpl_back::AsyncCallable;

use super::{ApiParameterType, ApiDisplayResult};

use crate::runtime::{QueueAction, QueueItem};

/// API web method to retrieve the latest display result for an element,
// or wait for the next display result if no display result is cached
pub struct GetDisplayEndpoint {
    //sender: tokio::sync::mpsc::Sender<SetCallbackAsync>,
    //receiver: Mutex<Option<tokio::sync::mpsc::Receiver<SetCallbackAsync>>>,
    sync_sender: Mutex<Sender<QueueItem>>,
}

impl GetDisplayEndpoint {
    pub fn new(sender: Sender<QueueItem>) -> Self {
        //let (async_tx, async_rx) = tokio::sync::mpsc::channel::<SetCallbackAsync>(64);
        Self {
            //sender: async_tx,
            //receiver: Mutex::new(Some(async_rx)),
            sync_sender: Mutex::new(sender),
        }
    }
}

#[async_trait::async_trait]
impl AsyncCallable for GetDisplayEndpoint {
    async fn call(&self, params: ApiParameterType) -> ApiParameterType {
        log::debug!("API: get_display");
        if let Some(Primitive::F64(index)) = params.get(0) {
            let index = *index as usize;
            let (respond_to, receiver) = channel();
            log::info!("requesting display for item #{}", index);
            let send_result = self.sync_sender.lock().unwrap().send(
                QueueItem {
                    action: QueueAction::SetCallback {
                        index,
                        respond_to,
                    }
                }
            );
            match send_result {
                Ok(_) => {
                    log::info!("waiting for display for item #{}", index);
                    match super::async_utils::channel_recv(receiver).await {
                        Err(_) => {
                                let msg = format!("Failed to response for get_display for #{}", index);
                                log::warn!("{}", msg);
                                return vec![ApiDisplayResult::failure(msg, "receiving channel disconnected").to_primitive()];
                        },
                        //Err(_) => return vec![], // impossible
                        Ok(x) => {
                            log::debug!("got display for item #{}", index);
                            return vec![ApiDisplayResult::success(x).to_primitive()];
                        },
                    }
                },
                Err(_e) => {
                    let msg = format!("Failed to get_display for #{}", index);
                    log::warn!("{}", msg);
                    vec![ApiDisplayResult::failure(msg, "sending channel disconnected").to_primitive()]
                }
            }
        } else {
            vec![ApiDisplayResult::failure("Failed to get param 0", "invalid call parameters").to_primitive()]
        }
    }
}
