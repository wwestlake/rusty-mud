
use rusty_mud_common::{
    player::*,
    database::*,
    email::*,
    init_entities::{*, self}
};

fn main() {
    let db = open_database();
    match db {
        Some(db) => {
            init_all(&db);
            //let user = PlayerAccount::new("test@tester.com", "thisisapasswordofsomestrength", "LagDaemon", PlayerRoles::Admin);
            //user.store(&db);
            let result = PlayerAccount::authenticate(&db, "test@tester.com", "thisisapas7swordofsomestrength");
            //println!("{}", result);
            let mut player = PlayerAccount::get_player(&db, "test@tester.com");
            println!("{:#?}", &player);
            match player {
                Ok(mut players) => {
                    match send_verification_email(&db, &mut players[0]) {
                        Ok(em) => println!("{}", em),
                        Err(err) => println!("Error {:#?}", err)
                    }
                },
                _ => println!("Error")
            }
        },
        None => todo!(),
    }

    let db = open_database().unwrap();
    let mut player = PlayerAccount::get_player(&db, "test@tester.com");
    println!("{:#?}", player);
}

