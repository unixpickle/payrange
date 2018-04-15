//! Watch a set of devices and log every change in the update timestamp.

extern crate clap;
extern crate futures;
extern crate payrange;
extern crate serde_yaml;
extern crate tokio_core;

mod util;

use std::sync::mpsc::{Sender, channel};
use std::thread::spawn;
use std::time::Duration;

use clap::{App, Arg};
use futures::future::{Future, Loop, join_all, loop_fn};
use payrange::Client;
use tokio_core::reactor::{Core, Handle, Timeout};

use util::{get_token, token_arg};

fn main() {
    let matches = App::new("payrange-track")
        .arg(token_arg())
        .arg(Arg::with_name("interval")
            .short("i")
            .long("interval")
            .value_name("SECONDS")
            .help("Set the poll interval")
            .takes_value(true))
        .arg(Arg::with_name("id")
            .help("Set the device IDs to track")
            .required(true)
            .multiple(true))
        .get_matches();
    let auth_token = get_token(&matches);
    let poll_interval = matches.value_of("interval").unwrap_or("60").parse().unwrap();

    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());

    let (logger, receiver) = channel();
    spawn(|| {
        for (id, timestamp) in receiver {
            println!("{},{}", id, timestamp);
        }
    });

    let mut futures = Vec::new();
    for id in matches.values_of("id").unwrap() {
        futures.push(track_device(client.clone(), auth_token.clone(), id.to_owned(),
            logger.clone(), core.handle(), poll_interval));
    }
    core.run(join_all(futures)).unwrap();
}

fn track_device(
    client: Client,
    token: String,
    id: String,
    logger: Sender<(String, u64)>,
    handle: Handle,
    poll_interval: u64
) -> Box<Future<Item = (), Error = ()>> {
    Box::new(loop_fn(None, move |last_time| {
        let sub_id = id.clone();
        let sub_logger = logger.clone();
        let sub_handle = handle.clone();
        client.get_device(token.clone(), id.clone())
            .then(move |x| {
                Ok(Loop::Continue(match x {
                    Err(e) => {
                        eprintln!("{}", e);
                        last_time
                    },
                    Ok(device_info) => {
                        if let Some(time) = last_time {
                            if time != device_info.updated {
                                sub_logger.send((sub_id, time)).unwrap();
                            }
                        }
                        Some(device_info.updated)
                    }
                }))
            })
            .and_then(move |loop_res| {
                Timeout::new(Duration::from_secs(poll_interval), &sub_handle).unwrap()
                    .map_err(|_| ())
                    .map(|_| loop_res)
            })
    }))
}
