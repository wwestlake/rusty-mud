use reindeer::{Db, Error, ErrorKind};
use directories::UserDirs;
use std::path::{Path, PathBuf};

//use crate::player::PlayerAccount;

pub fn database_dir() -> PathBuf {
    let user_dirs = UserDirs::new();
    match user_dirs {
        Some(dir) => 
            dir.home_dir().join(".rustymud_db"),
        None => Path::new("./.rustymud_db").to_owned()
    }

}

pub fn open_database() -> Option<Db> {
     match reindeer::open(database_dir()) {
         Ok(db) => Some(db),
         Err(err) => None
     }
}
