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
    List,
    Create(CreateUser),
    Delete(DeleteUser),
    Auth(AuthUser),
    Jail(JailUser),
    Unjail(UnjailUser)
}

#[derive(Debug, Args)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

#[derive(Debug, Args)]
pub struct DeleteUser {
    pub id: String
}

#[derive(Debug, Args)]
pub struct AuthUser {
    pub id: String,
    pub auth: String
}

#[derive(Debug, Args)]
pub struct JailUser {
    pub id: String
}

#[derive(Debug, Args)]
pub struct UnjailUser {
    pub id: String
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
