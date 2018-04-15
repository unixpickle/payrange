//! Get a PayRange auth token.

extern crate clap;
extern crate payrange;
extern crate tokio_core;

use std::process::exit;

use clap::{App, Arg};
use payrange::Client;
use tokio_core::reactor::Core;

fn main() {
    let matches = App::new("payrange-auth")
        .arg(Arg::with_name("email")
            .help("Set the login email")
            .required(true)
            .index(1))
        .arg(Arg::with_name("password")
            .help("Set the login password")
            .required(true)
            .index(2))
        .get_matches();
    let email = matches.value_of("email").unwrap().to_owned();
    let password = matches.value_of("password").unwrap().to_owned();

    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());
    match core.run(client.get_token_email(email, password)) {
        Ok(token) => {
            println!("{}", token);
        },
        Err(err) => {
            eprintln!("failed to get token: {}", err);
            exit(1);
        }
    }
}
