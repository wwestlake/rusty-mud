
pub mod player;
pub mod database;


pub mod init_entities {
    use reindeer::Db;
    use super::player::*;

    pub fn init_all(db: &Db) {
        PlayerAccount::init(db).expect("Unable to initalize PlayerAccount with Database");
    }

}
