//! Temporary place holder for email
//! 
use handlebars::{Handlebars, RenderError};
use std::collections::HashMap;
use reindeer::{Db, Error, ErrorKind};
use crate::player::PlayerAccount;
use rand::prelude::*;

fn token() -> String {
    let symbols: Vec<char> = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let mut token: String = "".to_owned();
    let mut rand = rand::thread_rng();
    for i in 0..9 {
        let r = rand.gen_range(0..symbols.len());

        token.insert(i, symbols[r]);
    }

    token.to_owned()
}


pub fn send_verification_email(db: &Db, player: &mut PlayerAccount) -> Result<String, RenderError> {
    let template = "
Hello {{nickname}},
    Welcome to RustyMUD!  This is a verification email, if you did not create
    an account on RustyMUD please ignore this email.  If you want to report this
    action you can send an email to rustymud@lagdaemon.com and let me know that
    you did not create this account.
   
    If you did create this account, your verification token is:

        Token: {{token}}

    Please connect to the RustyMUD server and enter the token when requested.

    Thanks for playing RustyMUD!

    --- RustyMUD Administration
    ";
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string("email", template)
        .unwrap();
    let mut data = HashMap::new();
    let token = token();
    player.add_token(&db, &token);
    data.insert("nickname", player.get_nickname());
    data.insert("token", token);

    handlebars.render("email", &data)    

}
