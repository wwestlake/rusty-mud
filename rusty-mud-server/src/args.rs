use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct ServerConfig {

    #[clap(short, long, value_parser)]
    pub name: String,

    #[clap(short, long, value_parser, default_value = "rusty-mud tcp listening server")]
    pub description: String,    

    #[clap(short, long, value_parser, default_value = "localhost")]
    pub address: String,

    /// Number of times to greet
    #[clap(short, long, value_parser, default_value_t = 9876)]
    pub port: u32,
    
    #[clap(short, long, value_parser, default_value_t = false)]
    pub use_ssl: bool

}
