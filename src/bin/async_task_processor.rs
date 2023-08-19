use std::thread;
use crossbeam_channel::{unbounded, Receiver, Sender};

// 定义一个任务类型，这里用简单的闭包来代表一个任务
type Task = Box<dyn FnOnce() + Send + 'static>;

// 定义一个异步任务处理器
#[derive(Clone)]
struct AsyncTaskProcessor {
    sender: Sender<Task>,
    receiver: Receiver<Task>,
}

impl AsyncTaskProcessor {
    fn new() -> Self {
        let (sender, receiver) = unbounded();
        AsyncTaskProcessor { sender, receiver }
    }

    fn start(self) {
        // 启动处理任务的线程
        thread::spawn(move || {
            while let Ok(task) = self.receiver.recv() {
                // 执行任务
                task();
            }
        });
    }

    fn submit_task(&mut self, task: Task) {
        // 提交任务到通道
        self.sender.send(task).expect("Failed to submit task");
    }
}

fn main() {
    // 创建异步任务处理器
    let async_task_processor = AsyncTaskProcessor::new();

    let async_task_processor1 = async_task_processor.clone();
    // 启动任务处理线程
    async_task_processor1.start();

    // 提交一些任务
    for i in 0..10 {
        let task = Box::new(move || {
            println!("Task {} executed.", i);
        });
        let mut async_task_processor2 = async_task_processor.clone();
        async_task_processor2.submit_task(task);
    }

    // 等待一段时间，确保所有任务都被处理
    thread::sleep(std::time::Duration::from_secs(2));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::channel;

    #[test]
    fn test_async_task_processor() {
        // 创建异步任务处理器
        let async_task_processor = AsyncTaskProcessor::new();

        let async_task_processor_t = async_task_processor.clone();
        // 启动任务处理线程
        async_task_processor_t.start();

        // 提交一些任务
        let (tx, rx) = channel();
        for i in 0..10 {
            let tx = tx.clone();
            let task = Box::new(move || {
                println!("Task {} executed.", i);
                tx.send(()).unwrap();
            });
            let mut processor = async_task_processor.clone();
            processor.submit_task(task);
        }

        // 等待所有任务完成
        for _ in 0..10 {
            rx.recv().unwrap();
        }
    }
}
