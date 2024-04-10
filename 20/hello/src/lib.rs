use std::{sync::mpsc, thread};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Job;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl ThreadPool {
    /// Create a new `ThreadPool`.
    ///
    /// The `size` is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// This function will panic if `size` is zero.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, receiver));
        }

        Self { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

impl Worker {
    fn new(id: usize, receiver: mpsc::Receiver<Job>) -> Self {
        let thread = thread::spawn(|| {
            receiver;
        });

        Self { id, thread }
    }
}
