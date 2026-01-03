use std::sync::mpsc::{self, Receiver, RecvError, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use tracing::{error, info, warn};

type Job = Box<dyn FnOnce() + Send + 'static>;
type ReceiverGuard<'a> = std::sync::MutexGuard<'a, std::sync::mpsc::Receiver<Job>>;
type ReceiverPoisonError<'a> = std::sync::PoisonError<ReceiverGuard<'a>>;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let thread: thread::JoinHandle<()> = thread::spawn(move || {
            loop {
                let job_result: Result<Job, ()> = receiver
                    .lock()
                    .map_err(|e: ReceiverPoisonError| error!("Worker {id} failed to lock receiver: {e}"))
                    .and_then(|receiver: ReceiverGuard| {
                        receiver
                            .recv()
                            .map_err(|e: RecvError| warn!("Worker {id} disconnected from pool: {e}"))
                    });

                if let Ok(job) = job_result {
                    info!("Worker {id} picked up a new job");
                    job();
                } else {
                    warn!("Worker {id} shutting down");
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
        assert!(size > 0, "'POOL_SIZE' must be greater than 0");
        info!("Initializing ThreadPool with {size} workers");

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
            warn!("No sender available, job could not be dispatched");
            return;
        };

        if let Err(e) = sender.send(Box::new(job)) {
            error!("Failed to dispatch job to worker: {e}");
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        info!("Shutting down ThreadPool");
        drop(self.sender.take());

        for Worker { id, thread } in &mut self.workers {
            if let Some(Err(e)) = thread.take().map(|thread: thread::JoinHandle<()>| thread.join()) {
                error!("Worker {id} panicked during shutdown: {e:?}");
            }
        }
    }
}
