use std::fmt;

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

pub struct ThreadPool;
impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// `new` panics if invalid size given; compare behaviour to [ThreadPool::build]
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        ThreadPool
    }

    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// `build` returns [PoolCreationError] if invalid size given; compare behaviour to [ThreadPool::new]
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size > 0 {
            Ok(ThreadPool)
        } else {
            Err(PoolCreationError { given_size: size })
        }
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn new_ok() {
        ThreadPool::new(2);
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
