use std::error::Error;

#[cfg(feature = "single-thread")]
mod single_thread;

#[cfg(feature = "thread-pool")]
mod thread_pool;

fn main() -> Result<(), Box<dyn Error>> {
    println!("tcp-server.rs");

    #[cfg(feature = "single-thread")] 
    {
        single_thread::server::run()?
    }
    
    #[cfg(feature = "thread-pool")] {
        thread_pool::server::run()?
    }

    Ok(())
}