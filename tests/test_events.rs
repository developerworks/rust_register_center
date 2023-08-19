use rust_register_center::events::default::Events;
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
