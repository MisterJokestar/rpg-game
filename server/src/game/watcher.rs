use std::{sync::Arc, time::Duration};
use tokio::{
    sync::Notify,
    time::timeout,    
};

const TIMEOUT_DURATION: Duration = Duration::from_secs(60 * 15);

#[derive(Clone)]
pub struct Watcher {
    pub ping: Arc<Notify>,
    pub stop: Arc<Notify>,
}

impl Watcher {
    pub async fn run(&self) {
        loop {
            tokio::select! {
                res = timeout(TIMEOUT_DURATION, self.ping.notified()) => {
                    if res.is_err() {
                        //timed out - call cleanup
                        break;
                    }
                    // ping recieved - loop restarts, timer resets
                }
                _ = self.stop.notified() => {
                    // told to stop - call cleanup
                    break;
                }
            }
        }
    }
}
