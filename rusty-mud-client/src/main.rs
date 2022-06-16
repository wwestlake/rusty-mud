extern crate pancurses;
use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use rusty_mud_common::*;
use std::thread;

fn main() {
    connect();
}

fn handle_server(mut stream: TcpStream) {
    loop {
        let my_message = Message::new(1, "hello".to_string());
        let msg = my_message.serialize().unwrap();
        stream.write(msg.as_bytes()).unwrap();
        println!("Sent Hello, awaiting reply...");

        let mut data = [0 as u8; 1024]; // using 6 byte buffer
        match stream.read(&mut data) {
            Ok(size) => {
                if &data[0..size] == msg.as_bytes() {
                    let text = from_utf8(&data).unwrap();
                    println!("Reply is ok!\n{}", text);
                } else {
                    let text = from_utf8(&data).unwrap();
                    println!("Unexpected reply: {} -> expected: {}", text, &msg[0..size]);
                }
            },
            Err(e) => {
                println!("Failed to receive data: {}", e);
            }
        }

    }
}

fn connect() {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");

            let msg = b"Hello!";

            let handle = thread::spawn(move|| {
                // connection succeeded
                handle_server(stream)
            });
            
            handle.join();
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}