use std::error::Error;

pub trait ThreadPool {
    fn execute<F>(&self, f: F) -> Result<(), Box<dyn Error>>
    where 
        F: FnOnce() -> Result<(), Box<dyn Error>> + Send + 'static;
}
