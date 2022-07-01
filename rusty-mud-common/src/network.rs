use std::str;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use uuid::Uuid;
use reindeer::{Db, Serialize, Deserialize, Entity, Error};
use std::fmt;
use crate::connections::player_connection::*;


use crate::threads::ThreadPool;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TcpServerSpec {
    id: String,
    name: String,
    description: String,
    address: String,
    port: u32,

}

impl Entity for TcpServerSpec {
        type Key = String;

        fn store_name() -> &'static str {
            "ServerSpec"
        }

        fn get_key(&self) -> &Self::Key {
            &self.id
        }

        fn set_key(&mut self, key : &Self::Key) {
            
            self.id = key.clone();
        } 
}

impl TcpServerSpec {
    pub fn new(name: &str, description: &str, address: &str, port: u32) -> Self {
        Self { 
            id: Uuid::new_v4().to_string(), 
            name: name.to_owned(), 
            description: description.to_owned(), 
            address: address.to_owned(), 
            port: port 
        }
    }

    pub fn init(db: &Db) -> Result<(), Error> {
        Self::register(db)
    }

    pub fn store(&self, db: &Db) -> Result<(), Error> {
        self.save(db)
    }

    pub fn all(db: &Db) -> Result<Vec<TcpServerSpec>, Error> {
        TcpServerSpec::get_all(db)
    }

    pub fn start(&self) {
        let address = format!("{}:{}", self.address, self.port);
        let listener = TcpListener::bind(address);
        match listener {
            Ok(l) => {
                let pool = ThreadPool::new(10);
                for stream in l.incoming() {
                    match stream {
                        Ok(s) =>
                            pool.execute(|| {
                                TcpServerSpec::handle_connection(s);
                            }),
                        Err(err) => println!("{:#?}", err),
                    }
                }
            },
            Err(err) => println!("{:#?}", err),
        }
    }

    pub fn handle_connection(mut stream: TcpStream) {
        let mut pc = UserConnection::new(stream);
        pc.run();
    }

}


