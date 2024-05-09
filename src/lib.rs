//! Multithreaded web server.

#![warn(missing_docs)]

use std::{fmt, thread};

/// An error thrown when an invalid size is given during creation of a new ThreadPool
#[derive(Debug)]
pub struct PoolCreationError {
    given_size: usize,
}
impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error creating ThreadPool: Invalid size. Given size: {}",
            &self.given_size
        ) // user output
    }
}

/// A list of worker threads.
pub struct ThreadPool {
    workers: Vec<Worker>,
}
impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// `new` panics if invalid size given; compare behaviour to [ThreadPool::build]
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_tutorial_webserver::ThreadPool;
    /// let my_thread_pool = ThreadPool::new(8);
    /// ```
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        ThreadPool {
            workers: Self::gen_thread_list(size),
        }
    }

    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// `build` returns [PoolCreationError] if invalid size given; compare behaviour to [ThreadPool::new]
    ///
    /// # Examples
    /// ```
    /// use rust_tutorial_webserver::ThreadPool;
    /// let my_thread_pool = ThreadPool::build(4).unwrap();
    /// ```
    /// Checking for invalid ThreadPool:
    /// ```
    /// use rust_tutorial_webserver::ThreadPool;
    /// let thread_creation_status: &'static str = match ThreadPool::build(0) {
    ///     Ok(tp) => "good!",
    ///     Err(pce) => "bad.",
    /// };
    /// assert_eq!("bad.", thread_creation_status);
    /// ```
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size > 0 {
            Ok(ThreadPool {
                workers: Self::gen_thread_list(size),
            })
        } else {
            Err(PoolCreationError { given_size: size })
        }
    }

    fn gen_thread_list(size: usize) -> Vec<Worker> {
        // Preallocating vector space is more efficient than Vec::new
        let mut workers = Vec::with_capacity(size);

        for n in 0..size {
            // Create some threads and store them in the vector.
            workers.push(Worker::new(n));
        }

        workers
    }

    /// Select a worker and execute a given closure.
    // use FnOnce as trait bound on F; eventually pass argument
    // received in execute to spawn. additionally, a thread
    // running a request will only execute that request's
    // closure once.
    pub fn execute<F>(&self, f: F)
    // F has trait bounds FnOnce & Send and has static lifetime

    // FnOnce() = closure that takes no params and returns unit
    // type ().
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

/// A worker with a given id which can be assigned tasks to do
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}
impl Worker {
    pub fn new(id: usize) -> Worker {
        Worker {
            id,
            thread: thread::spawn(|| {}),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn new_ok() {
        ThreadPool::new(2);
    }

    #[test]
    fn new_4() {
        let tp = ThreadPool::new(4);
        assert_eq!(4, tp.workers.len());
    }

    #[test]
    #[should_panic]
    fn new_0() {
        ThreadPool::new(0);
    }

    #[test]
    fn build_ok() {
        ThreadPool::build(4).unwrap();
    }

    #[test]
    fn build_2() {
        let tp = ThreadPool::build(2).unwrap();
        assert_eq!(2, tp.workers.len());
    }

    #[test]
    #[should_panic]
    fn build_0() {
        ThreadPool::build(0).unwrap();
    }

    #[test]
    fn pool_creation_error_display() {
        match ThreadPool::build(0) {
            Err(pce) => {
                assert_eq!(
                    "Error creating ThreadPool: Invalid size. Given size: 0",
                    pce.to_string()
                )
            }
            _ => panic!("Should have returned error"),
        }
    }
}
