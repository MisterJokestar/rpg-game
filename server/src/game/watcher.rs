use std::{sync::Arc, time::Duration};
use tokio::{
    sync::Notify,
    time::timeout,    
};
use tokio_util::sync::CancellationToken;

const TIMEOUT_DURATION: Duration = Duration::from_secs(60 * 15);

#[derive(Clone)]
pub struct Watcher {
    cancel_runner: CancellationToken,
    pub ping: Arc<Notify>,
    pub stop: Arc<Notify>,
}

impl Watcher {
    pub fn new(
        cancel_runner:CancellationToken,
        ping: Arc<Notify>,
        stop:Arc<Notify>
    ) -> Self {
        Watcher {
            cancel_runner,
            ping,
            stop
        }
    }

    pub async fn run(&self) {
        loop {
            tokio::select! {
                res = timeout(TIMEOUT_DURATION, self.ping.notified()) => {
                    if res.is_err() {
                        //timed out
                        self.cancel_runner.cancel();
                        break;
                    }
                    // ping recieved - loop restarts, timer resets
                }
                _ = self.stop.notified() => {
                    // told to stop 
                    self.cancel_runner.cancel();
                    break;
                }
            }
        }
    }
}
