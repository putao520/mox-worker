use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use tokio::sync::Notify;

pub struct Transaction {
    blocking: Arc<AtomicBool>,
    completed: Arc<AtomicBool>,
    notify: Arc<Notify>,
}

impl Transaction {
    pub fn new() -> Self {
        Transaction {
            blocking: Arc::new(AtomicBool::new(false)),
            completed: Arc::new(AtomicBool::new(true)),
            notify: Arc::new(Notify::new()),
        }
    }

    pub async fn run<'a, F, T>(&self, f: F) -> Option<T>
        where
            F: FnOnce() -> T + Send + 'a,
            T: Send + 'a,
    {
        if self.completed.compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
            self.blocking.store(false, Ordering::SeqCst);
        }
        match self.blocking.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst) {
            Ok(_) => {
                let result = f();
                self.completed.store(true, Ordering::SeqCst);
                self.notify.notify_waiters();
                Some(result)
            },
            Err(_) => {
                self.notify.notified().await;
                None
            }
        }
    }
}