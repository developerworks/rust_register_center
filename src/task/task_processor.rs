use crossbeam_channel::{unbounded, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

type Task = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    Task(Task),
    Quit,
}

#[derive(Clone, Debug)]
struct InnerTaskProcessor {
    sender: Sender<Message>,
    receiver: Receiver<Message>,
}

#[allow(unused)]
#[derive(Clone, Debug)]
struct TaskProcessor {
    inner: Arc<Mutex<InnerTaskProcessor>>,
}

#[allow(unused)]
impl TaskProcessor {
    pub fn new() -> Self {
        TaskProcessor {
            inner: Arc::new(Mutex::new(InnerTaskProcessor::new())),
        }
    }

    pub fn start(&self) {
        let inner = self.inner.lock().unwrap();
        inner.clone().start();
    }

    fn submit_task(&self, task: Task) {
        let inner = self.inner.lock().unwrap();
        inner.clone().submit_task(task);
    }
}

#[allow(unused)]
impl InnerTaskProcessor {
    /// Initialize TaskProcess
    fn new() -> Self {
        let (sender, receiver) = unbounded();
        Self { sender, receiver }
    }

    /// Start task processor
    fn start(self) {
        thread::spawn(move || {
            while let Ok(message) = self.receiver.recv() {
                match message {
                    Message::Task(task) => {
                        task();
                    }
                    Message::Quit => {
                        break;
                    }
                }
            }
        });
    }

    /// Submit task to it
    fn submit_task(&self, task: Task) {
        self.sender
            .send(Message::Task(task))
            .expect("Failed to submit task");
    }

    /// Shutdown the task processor and exit the task thread
    fn shutdown(&self) {
        self.sender
            .send(Message::Quit)
            .expect("Failed to send quit message");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::channel;

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
}
