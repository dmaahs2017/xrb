use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

fn main() {
    stub_server();
}

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50];
    while match stream.read(&mut data) {
        Ok(size) => {
            stream.write(&data[0..size]).unwrap();
            true
        },
            Err(_) => {
                println!("Terminating TCPSTREAM");
                stream.shutdown(Shutdown::Both).unwrap();
                false
            }
        
    } {}
}

fn stub_server() {
    let listener = TcpListener::bind("0.0.0.0:6000").unwrap();

    println!("Server listen on port 6000");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // connection succeeded
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    drop(listener);

}
