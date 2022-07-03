
use rusty_mud_common::character::{ 
    stats 
};

fn main() {
    let stats = stats::characterStats::new();

    stats.to_table().printstd();

}