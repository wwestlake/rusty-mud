use std::io;
use std::io::{Read, Write};


pub struct Repl<'a> {
    prompt: String,
    stdout: &'a mut dyn io::Write,
    stdin: &'a mut dyn io::Read,
    running: bool
}

impl<'a> Repl<'a> {
    pub fn new(prompt: String, stdout: &'a mut dyn io::Write, stdin: &'a mut dyn io::Read) -> Self {
        Self {
            prompt,
            stdout,
            stdin,
            running: false,
        }
    }

    pub fn run(&mut self) {
        let mut buffer = "".to_string();
        self.running = true;
        while self.running {
            self.stdout.write(self.prompt.as_bytes());
            self.stdout.flush();
            self.stdin.read_to_string(&mut buffer);
            self.stdout.write(buffer.as_bytes());
        }
    }

}
