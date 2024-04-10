pub struct ThreadPool;

pub struct PoolCreationError;

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

        Self
    }

    pub fn build(size: usize) -> Result<Self, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError);
        }

        Ok(Self)
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
