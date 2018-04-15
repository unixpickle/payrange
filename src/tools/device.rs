//! Lookup a PayRange device.

extern crate clap;
extern crate payrange;
extern crate serde_yaml;
extern crate tokio_core;

mod util;

use std::process::exit;

use clap::{App, Arg};
use payrange::Client;
use tokio_core::reactor::Core;

use util::{get_token, token_arg};

fn main() {
    let matches = App::new("payrange-device")
        .arg(token_arg())
        .arg(Arg::with_name("id")
            .help("Set the device ID")
            .required(true)
            .index(1))
        .get_matches();
    let auth_token = get_token(&matches);
    let device_id = matches.value_of("id").unwrap().to_owned();

    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());
    match core.run(client.get_device(auth_token, device_id)) {
        Ok(info) => {
            println!("{}", serde_yaml::to_string(&info).unwrap());
        },
        Err(err) => {
            eprintln!("failed to make request: {}", err);
            exit(1);
        }
    }
}
