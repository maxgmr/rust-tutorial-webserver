pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool
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
