
use crate::args::{self, RustyMudArgs, EntityType, UserSubcommand};
use rusty_mud_common::{
    player::*,
    database::open_database
};

use prettytable::{Table, Row, Cell};

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
                    println!("Add player:");
                    player.store(&db);
                    let list = vec![player];
                    list_players(&list);
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
                            list_players(&l);
                        },
                        Err(err) => print!("{}", err)
                    }
                }
            }
        }
    };


    Ok(())
}

pub fn list_players(player_accounts: &Vec<PlayerAccount>) {
    let mut table = Table::new();
    table.add_row(row!["ID", "EMAIL", "NAME", "ROLE"]);
    for player in player_accounts {
        table.add_row(Row::new(vec![
            Cell::new(&player.id),
            Cell::new(&player.email.to_string()),
            Cell::new(&player.nickname),
            Cell::new(&player.role.to_string()),
        ]));
    }
    table.printstd();
}