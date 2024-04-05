use std::error::Error;

mod single_thread;

fn main() -> Result<(), Box<dyn Error>> {
    println!("tcp-client.rs");
    let r = single_thread::client::run()?;
    Ok(())
}