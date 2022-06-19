
use rusty_mud_common::{
    player::*,
    database::*,
    init_entities::{*, self}
};

fn main() {
    let db = open_database();
    match db {
        Some(db) => {
            init_all(&db);
            let user = PlayerAccount::new("test@tester.com", "thisisapasswordofsomestrength", PlayerRoles::Admin);
            user.store(&db);
            let all_users = PlayerAccount::all(&db);
            for user in all_users {
                println!("{:#?}", user);
            }
        },
        None => todo!(),
    }


}

