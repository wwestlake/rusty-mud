use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct ServerConfig {

    /// The name of the server
    #[clap(short, long, value_parser)]
    pub name: String,

    /// Description of the server
    #[clap(short, long, value_parser, default_value = "rusty-mud tcp listening server")]
    pub description: String,    

    /// The address to listen on
    #[clap(short, long, value_parser, default_value = "localhost")]
    pub address: String,

    /// The port to listen on
    #[clap(short, long, value_parser, default_value_t = 9876)]
    pub port: u32,
    
    /// Use SSL for sercure server
    #[clap(short, long, value_parser, default_value_t = false)]
    pub use_ssl: bool

}
