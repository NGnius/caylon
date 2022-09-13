use std::sync::mpsc::{TryRecvError, Receiver};
use std::sync::Mutex;

/// Receive on a blocking channel in an async manner (by polling conservatively)
pub async fn channel_recv<T>(rx: Receiver<T>) -> Result<T, TryRecvError> {
    let sleep_duration = std::time::Duration::from_millis(10);
    let receiver = Mutex::new(rx);
    loop {
        let received = receiver.lock().unwrap().try_recv();
        match received {
            Err(TryRecvError::Disconnected) => {
                return Err(TryRecvError::Disconnected);
            },
            Err(_) => {},
            Ok(x) => return Ok(x),
        }
        tokio::time::sleep(sleep_duration).await;
    }
}
