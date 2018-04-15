//! Utilities for PayRange CLI tools.

use std::env;
use std::process::exit;

use clap::{Arg, ArgMatches};

pub fn token_arg() -> Arg<'static, 'static> {
    Arg::with_name("token")
        .short("t")
        .long("token")
        .value_name("TOKEN")
        .help("Set the auth token from payrange-auth")
        .takes_value(true)
}

pub fn get_token<'a>(args: &ArgMatches<'a>) -> String {
    if let Some(value) = args.value_of("token") {
        value.to_owned()
    } else {
        if let Ok(value) = env::var("PAYRANGE_TOKEN") {
            value
        } else {
            eprintln!("Must pass -token or set $PAYRANGE_TOKEN");
            exit(1);
        }
    }
}
