use std::sync::mpsc::{self, Receiver, RecvError, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use tracing::{debug, error, info, warn};

type Job = Box<dyn FnOnce() + Send + 'static>;
type SharedReceiver = Arc<Mutex<Receiver<Job>>>;
type ReceiverGuard<'a> = std::sync::MutexGuard<'a, std::sync::mpsc::Receiver<Job>>;
type PoisonError<'a> = std::sync::PoisonError<ReceiverGuard<'a>>;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: SharedReceiver) -> Self {
        let thread: thread::JoinHandle<()> = thread::spawn(move || {
            while let Some(job) = Self::fetch_job(id, &receiver) {
                debug!("Worker {id} got a job");
                job();
            }

            debug!("Worker {id} finished");
        });

        Self {
            id,
            thread: Some(thread),
        }
    }

    fn fetch_job(id: usize, receiver: &SharedReceiver) -> Option<Job> {
        let guard: ReceiverGuard = receiver
            .lock()
            .inspect_err(|e: &PoisonError| error!("Worker {id} poison error: {e}"))
            .ok()?;

        guard
            .recv()
            .inspect_err(|e: &RecvError| warn!("Worker {id} recv error: {e}"))
            .ok()
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

        let mut workers: Vec<Worker> = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

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
