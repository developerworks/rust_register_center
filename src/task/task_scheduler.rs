use crossbeam_channel::{unbounded, Receiver, Sender};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::thread;

type Task = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    Task(Task),
    Quit,
}

pub enum ScheduleStrategy {
    FIFO,
    LIFO,
    RoundRobin,
    PriorityQueue,
}

pub struct AsyncTaskScheduler {
    sender: Sender<Message>,
    receiver: Receiver<Message>,
    strategy: ScheduleStrategy,
    tasks: Vec<Task>,
    task_queue: BinaryHeap<Reverse<Task>>,
}

impl AsyncTaskScheduler {
    pub fn new(strategy: ScheduleStrategy) -> Self {
        let (sender, receiver) = unbounded();
        AsyncTaskScheduler {
            sender,
            receiver,
            strategy,
            tasks: Vec::new(),
            task_queue: BinaryHeap::new(),
        }
    }

    pub fn start(self) {
        thread::spawn(move || loop {
            if let Ok(message) = self.receiver.recv() {
                match message {
                    Message::Task(task) => match self.strategy {
                        ScheduleStrategy::FIFO => self.tasks.push(task),
                        ScheduleStrategy::LIFO => self.tasks.insert(0, task),
                        ScheduleStrategy::RoundRobin => self.tasks.push(task),
                        ScheduleStrategy::PriorityQueue => {
                            self.task_queue.push(Reverse(task));
                        }
                    },
                    Message::Quit => {
                        break;
                    }
                }
            }
            self.process_tasks();
        });
    }

    pub fn submit_task(&self, task: Task) {
        self.sender
            .send(Message::Task(task))
            .expect("Failed to submit task");
    }

    pub fn shutdown(&self) {
        self.sender
            .send(Message::Quit)
            .expect("Failed to send quit message");
    }

    fn process_tasks(&mut self) {
        while let Some(task) = self.get_next_task() {
            task();
        }
    }

    fn get_next_task(&mut self) -> Option<Task> {
        match self.strategy {
            ScheduleStrategy::FIFO | ScheduleStrategy::LIFO => self.tasks.pop(),
            ScheduleStrategy::RoundRobin => self.tasks.pop().or_else(|| {
                self.tasks.rotate_left(1);
                self.tasks.pop()
            }),
            ScheduleStrategy::PriorityQueue => self.task_queue.pop().map(|rev_task| rev_task.0),
        }
    }
}
