use std::{collections::HashMap, sync::Arc};
use tokio::sync::{mpsc, Mutex};

pub struct Events {
    subscribers: Arc<Mutex<HashMap<String, mpsc::Sender<String>>>>,
}

impl Events {
    #[allow(unused)]
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    #[allow(unused)]
    pub async fn subscribe(&self, event: &str) -> mpsc::Receiver<String> {
        let (tx, rx) = mpsc::channel(32);
        self.subscribers.lock().await.insert(event.to_owned(), tx);
        rx
    }

    #[allow(unused)]
    pub async fn publish(&self, event: &str, data: &str) {
        if let Some(subscriber) = self.subscribers.lock().await.get(event) {
            let _ = subscriber.send(data.to_owned()).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{self, Duration};

    #[tokio::test]
    async fn test_subscribe_publish() {
        let events = Events::new();

        let event_name = "test_event";
        let data = "test_data";

        let mut rx = events.subscribe(event_name).await;
        events.publish(event_name, data).await;

        // Use tokio's timeout to await the result of receiving data
        let received_data = time::timeout(Duration::from_secs(1), rx.recv()).await;

        assert!(received_data.is_ok());
        assert_eq!(received_data.unwrap().unwrap(), data);
    }
}
