
pub mod player;
pub mod database;
pub mod email;
pub mod rooms;
pub mod items;
pub mod network;
pub mod threads;
pub mod process;
pub mod connections;


pub mod general {

}


pub mod init_entities {
    use reindeer::Db;
    use crate::network::TcpServerSpec;

    use super::player::*;

    pub fn init_all(db: &Db) {
        PlayerAccount::init(db).expect("Unable to initialize PlayerAccount with Database");
        TcpServerSpec::init(db).expect("Unable to initialize TcpServerSpec with Database");
    }

}
