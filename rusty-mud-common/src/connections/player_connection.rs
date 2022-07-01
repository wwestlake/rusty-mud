

use std::{net::{ TcpListener, TcpStream }, io::Write};
use bufstream::BufStream;
use std::io::BufRead;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

pub struct UserConnection {
    stream: BufStream< TcpStream >,
}

impl UserConnection {

    pub fn new(stream: TcpStream) -> Self {
        Self { stream: BufStream::new(stream) }
    }

    pub fn read_line(&mut self) -> String {
        let mut line = String::new();
        self.stream.read_line(&mut line);
        line.to_owned()
    }

    pub fn write(&mut self, msg: &str) {
        self.stream.write_fmt(format_args!("{}", msg));
        self.stream.flush().unwrap();
    }
    pub fn writeln(&mut self, msg: &str) {
        self.stream.write_fmt(format_args!("{}{}", msg, LINE_ENDING));
        self.stream.flush().unwrap();
    }

    pub fn run(&mut self) {
        println!("user connected 1");
        self.writeln("Welcome to Rusty MUD!");
        loop {
            let input = self.read_line();
            println!("{}", &input);
            self.write(&input);
        }
    }


}
