use std::sync::{Mutex, mpsc::{Sender, channel}};

use usdpl_back::AsyncCallable;

use super::{ApiParameterType, ApiJavascriptResult};

//use crate::runtime::JavascriptCommand;

use crate::runtime::{QueueAction, QueueItem};

pub struct JavascriptData {
    pub raw: String,
    pub id: usize,
}

impl JavascriptData {
    pub fn from_string(js: String, id: usize) -> Self {
        Self {
            raw: js,
            id
        }
    }
}

/// API web method to retrieve the latest display result for an element,
// or wait for the next display result if no display result is cached
pub struct GetJavascriptEndpoint {
    sender: Mutex<Sender<QueueItem>>,
}

impl GetJavascriptEndpoint {
    pub fn new(tx: Sender<QueueItem>) -> Self {
        //let (async_tx, async_rx) = tokio::sync::mpsc::channel::<SetCallbackAsync>(64);
        Self {
            sender: Mutex::new(tx),
        }
    }
}

#[async_trait::async_trait]
impl AsyncCallable for GetJavascriptEndpoint {
    async fn call(&self, _: ApiParameterType) -> ApiParameterType {
        log::debug!("API: get_javascript");
        let (respond_to, receiver) = channel();
        let send_result = self.sender.lock().unwrap().send(
            QueueItem {
                action: QueueAction::SetJavascriptSubscriber {respond_to}
            }
        );
        match send_result {
            Ok(_) => {
                match super::async_utils::channel_recv(receiver).await {
                    Err(_) => {
                            let msg = "Failed to get response for get_javascript";
                            log::warn!("{}", msg);
                            vec![ApiJavascriptResult::failure(msg, "receiving channel disconnected").to_primitive()]
                    },
                    Ok(x) => {
                        log::debug!("got javascript");
                        vec![ApiJavascriptResult::success(x).to_primitive()]
                    },
                }
            },
            Err(_e) => {
                let msg = format!("Failed to get_javascript");
                log::warn!("{}", msg);
                vec![ApiJavascriptResult::failure(msg, "sending channel disconnected").to_primitive()]
            }
        }

    }
}
