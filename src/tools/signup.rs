//! Create a PayRange account.

extern crate clap;
extern crate payrange;
extern crate serde_yaml;
extern crate tokio_core;

use std::process::exit;

use clap::{App, Arg};
use payrange::Client;
use tokio_core::reactor::Core;

fn main() {
    let matches = App::new("payrange-signup")
        .arg(Arg::with_name("phone")
            .short("p")
            .long("phone")
            .value_name("PHONE")
            .help("Set the user's phone number (default is blank)")
            .takes_value(true))
        .arg(Arg::with_name("email")
            .help("Set the login email")
            .required(true)
            .index(1))
        .arg(Arg::with_name("password")
            .help("Set the login password")
            .required(true)
            .index(2))
        .arg(Arg::with_name("name")
            .help("Set the user's name")
            .required(true)
            .index(3))
        .get_matches();
    let email = matches.value_of("email").unwrap().to_owned();
    let password = matches.value_of("password").unwrap().to_owned();
    let name = matches.value_of("name").unwrap().to_owned();
    let phone = matches.value_of("phone").unwrap_or("").to_owned();

    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());
    match core.run(client.create_user(email, password, name, phone)) {
        Ok(info) => {
            println!("{}", serde_yaml::to_string(&info).unwrap());
        },
        Err(err) => {
            eprintln!("failed to create user: {}", err);
            exit(1);
        }
    }
}
