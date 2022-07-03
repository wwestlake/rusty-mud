
use rusty_mud_common::random::*;
use rusty_mud_common::character::{
    stats,
};

fn main() {
    let char = stats::CharStats::new();
    char.as_table().printstd();
}