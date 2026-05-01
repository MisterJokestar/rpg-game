//! Session inactivity watcher.
//!
//! [`Watcher`] runs as a background task alongside a [`crate::game::runner::Runner`].
//! It monitors the session for inactivity and cancels the runner when the idle
//! timeout elapses or an explicit stop signal is received.
use std::{sync::Arc, time::Duration};
use tokio::{
    sync::Notify,
    time::timeout,
};
use tokio_util::sync::CancellationToken;

/// Timeout duration set to 15 minutes.
const TIMEOUT_DURATION: Duration = Duration::from_secs(60 * 15);

/// Background task that cancels a [`crate::game::runner::Runner`] when the
/// session becomes idle or is explicitly stopped.
///
/// `Watcher` is cheap to clone; both the session record and the spawned task
/// hold a copy.
#[derive(Clone)]
pub struct Watcher {
    cancel_runner: CancellationToken,
    /// Notify handle used to reset the idle timer. Call `ping.notify_one()`
    /// whenever player activity is detected (e.g., an action is submitted).
    pub ping: Arc<Notify>,
    /// Notify handle used to request an explicit shutdown. Call
    /// `stop.notify_one()` to cancel the runner immediately regardless of the
    /// idle timer.
    pub stop: Arc<Notify>,
}

impl Watcher {
    /// Create a new watcher that will cancel `cancel_runner` on timeout or
    /// stop signal.
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

    /// Start the watch loop.
    ///
    /// Waits for either a ping (which resets the 15-minute idle timer) or a
    /// stop signal. If the timer expires without a ping, the runner is
    /// cancelled. Either signal (timeout or explicit stop) causes the task to
    /// exit after cancelling the runner.
    pub async fn run(&self) {
        loop {
            tokio::select! {
                res = timeout(TIMEOUT_DURATION, self.ping.notified()) => {
                    if res.is_err() {
                        // timed out
                        self.cancel_runner.cancel();
                        break;
                    }
                    // ping received — loop restarts, timer resets
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
