//! Get a PayRange auth token.

extern crate clap;
extern crate payrange;
extern crate tokio_core;

use std::process::exit;

use clap::{App, Arg};
use payrange::{AuthRequest, Error, UserResponse, call_api, make_client};
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
    let client = make_client(&core.handle());
    let resp_future = call_api(client, "/user/gettoken", &AuthRequest::new_email(email, password));
    let result: Result<UserResponse, Error> = core.run(resp_future);
    match result {
        Ok(info) => {
            if info.auth.token.is_some() {
                println!("{}", info.auth.token.unwrap().token_string);
            } else {
                eprintln!("no token string in response");
                exit(1);
            }
        },
        Err(err) => {
            eprintln!("failed to make request: {}", err);
            exit(1);
        }
    }
}
