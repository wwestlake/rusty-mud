mod args;

use args::RustyMudArgs;
use clap::Parser;

fn main() {
    let args = RustyMudArgs::parse();
    println!("{:#?}", args);
}
