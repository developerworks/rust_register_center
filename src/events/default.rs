// mod event_bus;

use std::{collections::HashMap, sync::Arc};
use tokio::sync::{mpsc, Mutex};

#[derive(Default)]
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
