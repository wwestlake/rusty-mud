
use rusty_mud_common::{
    player::*
};

fn main() {
    let email = "someone@someplace.com".to_string();
    let pw = "mydoglovesmeatpies".to_string();

    let email = Email::process(email);
    let pw = Password::new(pw);

    let account = PlayerAccount::new(1, email, pw);

    println!("{:#?}", account);


}