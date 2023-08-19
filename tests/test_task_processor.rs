use std::sync::mpsc::channel;
use rust_register_center::task::task_processor::TaskProcessor;

#[test]
fn test_async_task_processor() {
    // Create task processor
    let task_processor = TaskProcessor::new();

    let task_processor_t = task_processor.clone();

    // Start task processor
    task_processor_t.start();

    // Submit some tasks
    let (tx, rx) = channel();
    for i in 0..10 {
        let tx = tx.clone();
        let task = Box::new(move || {
            println!("Task {} executed.", i);
            tx.send(()).unwrap();
        });
        let processor = task_processor.clone();
        // processor.submit_task(task);
        processor.submit_task(task)
    }

    // Waiting for task finish
    for _ in 0..10 {
        rx.recv().unwrap();
    }
}
