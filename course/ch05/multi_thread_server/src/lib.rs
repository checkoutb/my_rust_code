
use std::thread;
use std::sync::mpsc;
use std::sync::{
    Arc, Mutex
};

// https://course.rs/advance-practice1/multi-threads.html

pub struct ThreadPool {
    // threads: Vec<thread::JoinHandle<()>>,
    workers: Vec<Worker>,
// Worker 结构体需要从线程池 TreadPool 的队列中获取待执行的代码，
// 对于这类场景，消息传递非常适合：我们将使用消息通道( channel )作为任务队列。
    sender: Option<mpsc::Sender<Job>>,
}

struct Worker {
    id: usize,
    // thread: thread::JoinHandle<()>,
    thread: Option<thread::JoinHandle<()>>,
}

// struct Job;
type Job = Box<dyn FnOnce() + Send + 'static>;

impl Worker {
    // fn new(id: usize) -> Worker {
    //     let thread = thread:spawn(|| {});
    //     Worker{id, thread}
    // }


// fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Box<dyn FnOnce() + Send + 'static>>>>) -> Worker
    // fn new(id: usize, receiver: mpsc::Receiver<Job>) -> Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // let thread = thread::spawn(move || {
        //     // receiver;
        //     let job = receiver.lock().unwrap().recv().unwrap();
        //     println!("worker {id} got a job...");
        //     job();
        // });

        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(job) => {
                    println!("worker {id} got a job, executing...");
                    job();
                }
                Err(_) => {
                    println!("worker {id} disconnected; shut down...");
                    break;
                }
            }
        });

        Worker {id, thread: Some(thread)}
    }
}


impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // // let mut threads = Vec::with_capacity(size);
        // let mut workers = Vec::with_capacity(size);

        // for id in 0..size {
        //     workers.push(Worker::new(id));
        // }

        // // ThreadPool{ threads }
        // ThreadPool { workers }


        // let (sender, receiver) = mpsc::channel();
        // let mut workers = Vec::with_capacity(size);
        // for id in 0..size {
        //     workers.push(Worker::new(id, receiver));
        // }
        // ThreadPool {workers, sender}

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {workers, sender: Some(sender)}
    }


// 参考thread::spawn
// pub fn spawn<F, T>(f: F) -> JoinHandle<T>
//     where
//         F: FnOnce() -> T,
//         F: Send + 'static,
//         T: Send + 'static,
    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        // self.sender.send(job).unwrap();
        self.sender.as_ref().unwrap().send(job).unwrap();
    }

}


// 优雅关闭
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("shut down worker {}", worker.id);
            // worker.thread.join().unwrap();       // error, worker.thread move, so need Option
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
