mod args;
mod exec;

use args::RustyMudArgs;
use clap::Parser;
use exec::run_command;

fn main() {
    let args = RustyMudArgs::parse();
    run_command(&args);
}
