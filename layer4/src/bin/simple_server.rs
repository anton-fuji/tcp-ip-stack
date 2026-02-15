use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    loop {
        // Read data from the client
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("The client has disconnected");
                break;
            }
            Ok(n) => {
                let recv = String::from_utf8_lossy(&buffer[..n]);
                println!("recved : {}", recv);

                // Rerturn the received data as is
                if let Err(e) = stream.write(&buffer[..n]) {
                    eprintln!("Write Err: {}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Write Err : {}", e);
                break;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to connect");
    println!("Start: 127.0.0.1:7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New Connect: {}", stream.peer_addr().unwrap());
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Failed to connect: {}", e);
            }
        }
    }
}
