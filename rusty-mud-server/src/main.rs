use clap::Parser;
use rusty_mud_common::network::*;
use rusty_mud_common::process::*;

mod args;

use args::*;

fn main() {

    let args = ServerConfig::parse();

    let server = TcpServerSpec::new(&args.name, &args.description, &args.address, &args.port);
    server.start();
}