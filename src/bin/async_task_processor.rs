use crossbeam_channel::{unbounded, Receiver, Sender};
use futures;
use std::thread;

type Task = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    Task(Task),
    Quit,
}

#[derive(Clone)]
struct TaskProcessor {
    sender: Sender<Message>,
    receiver: Receiver<Message>,
}

impl TaskProcessor {
    fn new() -> Self {
        let (sender, receiver) = unbounded();
        TaskProcessor { sender, receiver }
    }

    fn start(self) {
        // 启动工作线程
        println!("启动工作线程");
        thread::spawn(move || {
            println!("监听管道, 并接受消息");
            while let Ok(message) = self.receiver.recv() {
                match message {
                    Message::Task(task) => {
                        // 执行任务
                        println!("消息类型: Task");
                        task();
                    }
                    Message::Quit => {
                        println!("消息类型: Quit");
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
    // 创建异步任务处理器
    let async_task_processor = TaskProcessor::new();

    // 启动任务处理线程
    let t = async_task_processor.clone();
    t.start();

    let tasks = async {
        // 提交一些任务
        for i in 0..10 {
            let task = Box::new(move || {
                println!("Task {} executed.", i);
            });
            let t2 = async_task_processor.clone();
            t2.submit_task(task);
        }
    };
    // 等待一段时间，确保所有任务都被处理
    thread::sleep(std::time::Duration::from_secs(2));

    // 关闭任务处理器
    let shutdown = async {
        async_task_processor.shutdown();
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
