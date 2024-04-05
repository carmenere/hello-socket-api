use std::net::TcpListener;

// std::net::TcpStream implements Read and Write traits
use std::io::{Read, Write, Error};

pub fn run() -> Result<(), std::io::Error> {
    let lsocket = TcpListener::bind("127.0.0.1:5555")?;

    for con in lsocket.incoming() { // incoming() returns an iterator over the connections being received on this listener.
        let mut stream = match con {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error while receiving new connection. Details: {}", e.to_string());
                return Err(e)
            }
        };

        println!("Got new connection: src ip: {:?}, src port: {:?}", stream.peer_addr()?.ip(), stream.peer_addr()?.port());

        let mut buf = [0u8; 1024];

        let r = stream.read(&mut buf)?;
        println!("read {r} bytes");
        let r = stream.write(&mut buf)?;
        println!("send {r} bytes");
    }

    Ok(())
}