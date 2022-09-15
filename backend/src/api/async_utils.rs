use std::sync::mpsc::{RecvError, Receiver};

/// Receive on a blocking channel in an async manner
pub async fn channel_recv<T: Send + 'static>(rx: Receiver<T>) -> Result<T, RecvError> {
    tokio::task::spawn_blocking(move || rx.recv()).await.map_err(|e| {
        log::error!("Async JoinError while receiving from sync channel: {}", e);
        RecvError
    })?
}
