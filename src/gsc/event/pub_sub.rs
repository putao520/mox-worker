use std::sync::Arc;

use anyhow::Result;
use dashmap::DashMap;
use futures::StreamExt;
use redis::{AsyncCommands, FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use tokio::select;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::task::JoinHandle;

use crate::gsc::mdl::redis::{get_redis_pub, get_redis_sub};

/**
 * 事件订阅
 */

type EventConsumer<String> = fn(&String) -> Result<()>;

enum PubSubCmd {
    Subscribe(String),
    Unsubscribe(String),
}

pub struct EventSystem {
    sender: Sender<PubSubCmd>,
    handle_map: Arc<DashMap<String, Vec<EventConsumer<String>>>>,
    join_handle: JoinHandle<()>,
}

impl Drop for EventSystem {
    fn drop(&mut self) {
        self.join_handle.abort();
    }
}
impl EventSystem {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<PubSubCmd>(32);
        let m = Arc::new(DashMap::new());
        EventSystem {
            sender: tx,
            handle_map: m.clone(),
            join_handle: listen(m, rx),
        }
    }

    pub async fn subscribe(&mut self, event_name: &str, consumer: EventConsumer<String>) -> Result<()> {
        if !self.handle_map.contains_key(event_name) {
            self.handle_map.insert(event_name.to_string(), Vec::new());
        }
        let mut consumers = self.handle_map.get_mut(event_name).unwrap();
        consumers.push(consumer);
        self.sender
            .send(PubSubCmd::Subscribe(event_name.to_string()))
            .await?;
        Ok(())
    }

    pub async fn unsubscribe(&mut self, event_name: &str) -> Result<()> {
        self.handle_map.remove(event_name);
        self.sender
            .send(PubSubCmd::Unsubscribe(event_name.to_string()))
            .await?;
        Ok(())
    }
}

fn listen(m: Arc<DashMap<String, Vec<EventConsumer<String>>>>, mut receiver: Receiver<PubSubCmd>) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut redis = get_redis_sub().await.expect("get_redis_pub_sub error");
        loop {
            let cmd: Option<PubSubCmd>;
            loop {
                let mut stream = redis.on_message();
                select! {
                        value = stream.next() => {
                            if let Some(msg) = value {
                                let event_name = msg.get_channel_name();
                                let consumers_res = m.get(event_name);
                                if let Some(consumers) = consumers_res {
                                    if let Ok(payload) = msg.get_payload::<String>() {
                                        for consumer in consumers.iter() {
                                            consumer(&payload).expect("consumer error");
                                        }
                                    }
                                }
                            } else {
                                // The Redis pub/sub connection was dropped and we can exit.
                                cmd = None;
                                break;
                            }
                        }
                        value = receiver.recv() => {
                            // If value is Some: Break out of the inner loop, update the connection, then listen again.
                            // If value is None: The cmd_tx was dropped and we can exit.
                            cmd = value;
                            break;
                        }
                    }
            }
            match cmd {
                Some(PubSubCmd::Subscribe(channel)) => redis.subscribe(channel).await,
                Some(PubSubCmd::Unsubscribe(channel)) => redis.unsubscribe(channel).await,
                None => return,
            }
            .expect("PubSubCmd error");
        }
    })
}

// 发送订阅事件
pub async fn publish_event(event_name: String, payload: String) -> Result<()> {
    let mut con = get_redis_pub().await?;
    con.publish(event_name, payload).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use once_cell::sync::Lazy;

    use super::*;

    static EVENT_TEST_RECEIVER_PAYLOAD: Lazy<Mutex<String>> =
        Lazy::new(|| Mutex::new("".to_string()));
    fn event_consumer(msg: &String) -> Result<()> {
        println!("收到消息: {}", msg.clone());
        EVENT_TEST_RECEIVER_PAYLOAD
            .lock()
            .unwrap()
            .push_str(msg.as_str());
        Ok(())
    }

    #[tokio::test]
    async fn test_event_consumer() {
        let event_name = "test_event";
        let mut es = EventSystem::new();
        es.subscribe(event_name, event_consumer)
            .await
            .expect("subscribe error");
        // 等待5秒
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        publish_event(event_name.to_string(), "hello putao520".to_string())
            .await
            .expect("publish_event error");
        // 等待5秒
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        es.unsubscribe(event_name).await.expect("unsubscribe error");
        loop {
            let payload = EVENT_TEST_RECEIVER_PAYLOAD.lock().unwrap();
            if payload.len() > 0 {
                break;
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
        let payload = EVENT_TEST_RECEIVER_PAYLOAD.lock().unwrap();
        assert_eq!(payload.to_string(), "hello putao520".to_string());
    }
}
