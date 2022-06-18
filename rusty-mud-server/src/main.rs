extern crate zxcvbn;

use rusty_mud_common::player::Password;
use zxcvbn::zxcvbn;
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use rusty_mud_common::{
    dungeon::*,
    player::*,
    menus::*,
    repl::*,
};
use handlebars::Handlebars;
use std::collections::HashMap;

fn main() {
    let s = include_str!("../../Data/MOTD.txt");
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string("motd", s)
        .unwrap();
    let mut data = HashMap::new();
    data.insert("announcements", "These are some announcements!");
    
    println!("{}", handlebars.render("motd", &data).unwrap());

    let mut menu = Menu::new("Login".to_string(), ">".to_string(), output, read);
    menu
        .add("Login".to_string(), 2, login)
        .add("Create Account".to_string(), 1, createAccount);

    menu.show();
    menu.run();

    let mut repl = Repl::new("login> ".to_string(), &mut std::io::stdout().by_ref(), &mut std::io::stdin().by_ref());
    &repl.run();
}

fn output(text: String) {
    println!("{}", text);
}

fn read() -> String {
    "3".to_string()
}

fn createAccount() {
    println!("Creating Account!");
}

fn login() {
    println!("Logging In!");
}

