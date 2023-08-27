use crossbeam_channel::{unbounded, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

type Task = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    Task(Task),
    Quit,
}

#[derive(Clone, Debug)]
struct TaskProcessor {
    sender: Sender<Message>,
    receiver: Receiver<Message>,
}

impl TaskProcessor {
    /// Initialize TaskProcess
    fn new() -> Self {
        let (sender, receiver) = unbounded();
        TaskProcessor { sender, receiver }
    }

    /// Start task processor
    /// ```rust
    /// #[cfg(test)]
    ///
    /// let processor = TaskProcessor::new();
    /// ```
    fn start(self) {
        // 启动工作线程
        println!("启动工作线程");
        thread::spawn(move || {
            println!("监听管道, 并接受消息");
            while let Ok(message) = self.receiver.recv() {
                match message {
                    Message::Task(task) => {
                        // 执行任务
                        print!("消息类型: Task");
                        task();
                    }
                    Message::Quit => {
                        print!("消息类型: Quit");
                        break;
                    }
                }
            }
        });
    }

    fn submit_task(&self, task: Task) {
        // 提交任务到通道
        self.sender
            .send(Message::Task(task))
            .expect("Failed to submit task");
    }

    fn shutdown(&self) {
        // 发送退出消息到工作线程
        // logger::info("Shutdown async task processor");
        println!("Shutdown async task processor");
        self.sender
            .send(Message::Quit)
            .expect("Failed to send quit message");
    }
}

#[actix_web::main]
async fn main() {
    let async_task_processor: Arc<Mutex<TaskProcessor>> = Arc::new(Mutex::new(TaskProcessor::new()));
    let t = async_task_processor.clone();
    t.lock().unwrap().to_owned().start();
    let tasks = async {
        for i in 0..10 {
            let task = Box::new(move || {
                println!("Task {} executed.", i);
            });
            let t2 = async_task_processor.clone();
            t2.lock().unwrap().submit_task(task);
        }
    };
    let shutdown = async {
        async_task_processor.clone().lock().unwrap().shutdown();
    };
    futures::join!(tasks, shutdown);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::channel;

    #[test]
    fn test_async_task_processor() {
        // 创建异步任务处理器
        let task_processor = TaskProcessor::new();

        let task_processor_t = task_processor.clone();
        // 启动任务处理线程
        task_processor_t.start();

        // 提交一些任务
        let (tx, rx) = channel();
        for i in 0..10 {
            let tx = tx.clone();
            let task = Box::new(move || {
                println!("Task {} executed.", i);
                tx.send(()).unwrap();
            });
            let processor = task_processor.clone();
            processor.submit_task(task);
        }

        // 等待所有任务完成
        for _ in 0..10 {
            rx.recv().unwrap();
        }
    }
}
