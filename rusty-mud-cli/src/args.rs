use clap:: {
    Args,
    Parser,
    Subcommand,
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct RustyMudArgs {
    
    #[clap(subcommand)]
    pub entity_type: EntityType,
    

}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Manage all users of the game
    UserManager(UserCommand),

    /// Manage all listening servers
    ServerManager(ServerCommand),

    /// Manage the databse
    DatabaseManager(DatabaseCommand),

    /// Manage game rules, triggers, etc.
    GameManager(GameCommand),

    Repl(ReplCommand),
}

#[derive(Debug, Args)]
pub struct UserCommand {

    /// Create a user
    #[clap(subcommand)]
    pub command: UserSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum UserSubcommand {
    Create(CreateUser),
    Delete(DeleteUser),
    Auth(AuthUser),
    Jail(JailUser),
    Unjail(UnjailUser)
}

#[derive(Debug, Args)]
pub struct CreateUser {
    name: String,
    email: String,
    password: String
}

#[derive(Debug, Args)]
pub struct DeleteUser {
    id: String
}

#[derive(Debug, Args)]
pub struct AuthUser {
    id: String,
    auth: String
}

#[derive(Debug, Args)]
pub struct JailUser {
    id: String
}

#[derive(Debug, Args)]
pub struct UnjailUser {
    id: String
}




#[derive(Debug, Args)]
pub struct ServerCommand {
    
}

#[derive(Debug, Args)]
pub struct DatabaseCommand {
    
}

#[derive(Debug, Args)]
pub struct GameCommand {
    
}

#[derive(Debug, Args)]
pub struct ReplCommand {
    
}
