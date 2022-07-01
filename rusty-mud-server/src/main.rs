use rusty_mud_common::network::*;
use rusty_mud_common::process::*;

fn main() {
    let server = TcpServerSpec::new("local", "Local connection", "localhost", 9876);
    server.start();
}