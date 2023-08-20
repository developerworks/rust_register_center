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
pub struct TaskProcessor {
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
        let inner: std::sync::MutexGuard<'_, InnerTaskProcessor> = self.inner.lock().unwrap();
        inner.clone().start();
    }

    pub fn submit_task(&self, task: Task) {
        let inner: std::sync::MutexGuard<'_, InnerTaskProcessor> = self.inner.lock().unwrap();
        inner.clone().submit_task(task);
    }
}

impl Default for TaskProcessor {
    fn default() -> Self {
        Self::new()
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
