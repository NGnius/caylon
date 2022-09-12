use std::sync::{Mutex, mpsc::{Sender, channel, self}};

use usdpl_back::core::serdes::Primitive;
use usdpl_back::AsyncCallable;

use super::ApiParameterType;

use crate::runtime::{QueueAction, QueueItem};

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
            if let Ok(_) = send_result {
                // TODO: don't poll for response
                log::info!("waiting for display for item #{}", index);
                let sleep_duration = std::time::Duration::from_millis(10);
                let receiver = Mutex::new(receiver);
                loop {
                    let received = receiver.lock().unwrap().try_recv();
                    match received {
                        Err(mpsc::TryRecvError::Disconnected) => {
                            log::info!("Failed to response for get_display for #{}", index);
                            return vec![Primitive::Empty];
                        },
                        Err(_) => {},
                        Ok(x) => {
                            log::debug!("got display for item #{}", index);
                            return vec![x];
                        },
                    }
                    tokio::time::sleep(sleep_duration).await;
                }
            } else {
                log::info!("Failed to get_display for #{}", index);
                vec![Primitive::Empty]
            }

        } else {
            vec![Primitive::Empty]
        }
    }
}
