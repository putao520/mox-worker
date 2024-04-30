use std::future::Future;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use log::{debug, info};

use tokio::sync::Notify;

pub struct Transaction {
    blocking: Arc<AtomicBool>,
    notify: Arc<Notify>,
}

impl Transaction {
    pub fn new() -> Self {
        Transaction {
            blocking: Arc::new(AtomicBool::new(false)),     // 是否在更新中
            notify: Arc::new(Notify::new()),
        }
    }

    pub async fn run<'a, F, T, R>(&self, f: F, rs: R) -> Option<T>
        where
            // F: Future + Send + 'static,
            F: Future<Output = Option<T>> + Send + 'static,
            // F::Output: T + Send + 'static,
            // R: Future + Send + 'static,
            R: Future<Output = Option<()>> + Send + 'static,
            // R::Output: T + Send + 'static,
            T: Send + 'a,
    {
        // 如果正在更新中，等待
        if self.blocking.load(Ordering::SeqCst) {
            info!("Transaction is blocking, waiting...");
            self.notify.notified().await;
        }

        match f.await {
            Some(result) => {
                Some(result)
            },
            None => {
                if let Ok(_) = self.blocking.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst) {
                    rs.await;
                    self.blocking.store(false, Ordering::SeqCst);
                    self.notify.notify_waiters();
                }
                None
            }
        }
    }
}