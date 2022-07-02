
use crate::args::{self, RustyMudArgs, EntityType, UserSubcommand};
use rusty_mud_common::{
    player::PlayerAccount,
    player::PlayerRoles,
    database::open_database
};

pub fn run_command(args: &RustyMudArgs) -> Result<(), String> {
    match &args.entity_type {
        EntityType::DatabaseManager(cmd) => {}
        EntityType::GameManager(cmd) => {}
        EntityType::Repl(cmd) => {}
        EntityType::ServerManager(cmd) => {}
        EntityType::UserManager(cmd) => {
            match &cmd.command {
                UserSubcommand::Create(user_data) => {
                    let db = open_database().unwrap();
                    let player = PlayerAccount::new(
                        &user_data.email,
                        &user_data.password,
                        &user_data.name,
                        PlayerRoles::from_str(user_data.role.as_ref()).unwrap()
                    );
                    player.store(&db);
                }
                UserSubcommand::Delete(user_data) => {}
                UserSubcommand::Auth(user_data) => {}
                UserSubcommand::Jail(user_data) => {}
                UserSubcommand::Unjail(user_data) => {}
                UserSubcommand::List => {
                    println!("Listing users");
                    let db = open_database().unwrap();
                    let list = PlayerAccount::all(&db);
                    match list {
                        Ok(l) => {
                            for user in l {
                                println!("{:?}", user);
                            }
                        },
                        Err(err) => print!("{}", err)
                    }
                }
            }
        }
    };


    Ok(())
}

