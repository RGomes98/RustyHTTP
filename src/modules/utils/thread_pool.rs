use crate::modules::utils::Logger;

use std::process;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let thread: thread::JoinHandle<()> = thread::spawn(move || {
            loop {
                let job: Job = receiver
                    .lock()
                    .expect("Worker failed to lock receiver.")
                    .recv()
                    .expect("Worker failed to receive job.");
                Logger::debug(&format!("Worker {id} picked up a new job."));
                job();
            }
        });

        Self { id, thread }
    }
}

pub struct ThreadPool {
    sender: Sender<Job>,
    workers: Vec<Worker>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        if size < 1 {
            Logger::error("Failed to initialize thread pool: size must be greater than zero.");
            process::exit(1);
        }

        let (sender, receiver): (Sender<Job>, Receiver<Job>) = channel();
        let receiver: Arc<Mutex<Receiver<Job>>> = Arc::new(Mutex::new(receiver));

        let workers: Vec<Worker> = (0..size)
            .map(|id: usize| Worker::new(id, Arc::clone(&receiver)))
            .collect::<Vec<Worker>>();

        Self { workers, sender }
    }

    pub fn schedule<T>(&self, job: T)
    where
        T: FnOnce() + Send + 'static,
    {
        let job: Box<T> = Box::new(job);
        self.sender
            .send(job)
            .expect("Failed to send job to worker thread.");
    }
}
