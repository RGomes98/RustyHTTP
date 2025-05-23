use crate::modules::utils::Logger;

use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let thread: thread::JoinHandle<()> = thread::spawn(move || {
            loop {
                let job_result: Result<Job, ()> = match receiver.lock() {
                    Ok(receiver) => match receiver.recv() {
                        Ok(job) => {
                            Logger::info(&format!("Worker {id} picked up a new job."));
                            Ok(job)
                        }
                        Err(err) => {
                            Logger::error(&format!("Worker {id} failed to receive job: {err}."));
                            Err(())
                        }
                    },
                    Err(err) => {
                        Logger::error(&format!("Worker {id} failed to lock receiver: {err}."));
                        Err(())
                    }
                };

                if let Ok(job) = job_result {
                    job()
                } else {
                    Logger::warn(&format!("Worker {id} shutting down."));
                    break;
                }
            }
        });

        Self {
            id,
            thread: Some(thread),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver): (Sender<Job>, Receiver<Job>) = mpsc::channel();
        let receiver: Arc<Mutex<Receiver<Job>>> = Arc::new(Mutex::new(receiver));

        let workers: Vec<Worker> = (0..size)
            .map(|id: usize| Worker::new(id, Arc::clone(&receiver)))
            .collect::<Vec<Worker>>();

        Self {
            workers,
            sender: Some(sender),
        }
    }

    pub fn schedule<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let Some(sender) = &self.sender else {
            Logger::warn("No sender available; job could not be dispatched.");
            return;
        };

        if let Err(err) = sender.send(Box::new(job)) {
            Logger::error(&format!("Failed to dispatch job to worker: {err}."));
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for Worker { id, thread } in &mut self.workers {
            let Some(thread) = thread.take() else {
                Logger::warn(&format!("No thread found for worker {id} during shutdown."));
                continue;
            };

            if let Err(err) = thread.join() {
                Logger::error(&format!("Worker {id} panicked during shutdown: {err:?}."));
            }
        }
    }
}
