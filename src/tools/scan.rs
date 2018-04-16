//! Search for PayRange devices.

extern crate clap;
extern crate futures;
extern crate payrange;
extern crate serde_yaml;
extern crate tokio_core;

mod util;

use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::exit;

use clap::{App, Arg};
use futures::Future;
use futures::future::join_all;
use payrange::{Client, Error};
use tokio_core::reactor::Core;

use util::{get_token, token_arg};

fn main() {
    let matches = App::new("payrange-scan")
        .arg(token_arg())
        .arg(Arg::with_name("start")
            .short("s")
            .long("start")
            .value_name("ID")
            .help("Set the start ID for the search (default: 10000000)")
            .takes_value(true))
        .arg(Arg::with_name("end")
            .short("e")
            .long("end")
            .value_name("ID")
            .help("Set the end ID for the search (default: 100000000)")
            .takes_value(true))
        .arg(Arg::with_name("concurrency")
            .short("c")
            .long("concurrency")
            .value_name("INT")
            .help("Set the number of parallel requests to make")
            .takes_value(true))
        .arg(Arg::with_name("out_dir")
            .help("Set the output directory")
            .required(true)
            .index(1))
        .get_matches();
    let token = get_token(&matches);
    let mut start = matches.value_of("start").unwrap_or("10000000").parse().unwrap();
    let end = matches.value_of("end").unwrap_or("100000000").parse().unwrap();
    let concurrency = matches.value_of("concurrency").unwrap_or("100").parse().unwrap();
    let out_dir = matches.value_of("out_dir").unwrap();

    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());
    while start < end {
        let count: u32 = if start + concurrency <= end {
            concurrency
        } else {
            end - start
        };
        if let Err(e) = core.run(fetch_batch(&client, &token, start, count, out_dir)) {
            eprintln!("io error: {}", e);
            exit(1);
        }
        start += count;
    }
}

fn fetch_batch(
    client: &Client,
    token: &str,
    start: u32,
    count: u32,
    out_dir: &str
) -> Box<Future<Item = (), Error = io::Error>> {
    let mut futures = Vec::new();
    for i in start..(start + count) {
        let dir_copy = out_dir.to_owned();
        let sub_future = client.get_device(token.to_owned(), format!("{:08}", i))
            .then(move |res| -> Result<(), io::Error> {
                match res {
                    Err(Error::Remote{status: 3, reason: _}) => Ok(()),
                    Err(e) => {
                        eprintln!("{} {}", i, e);
                        Ok(())
                    },
                    Ok(info) => {
                        let data = serde_yaml::to_vec(&info).unwrap();
                        let path: &Path = Path::new(&dir_copy);
                        let name = format!("{:08}.yaml", i);
                        let mut file = File::create(path.join(name))?;
                        file.write_all(&data)?;
                        file.flush()
                    }
                }
            });
        futures.push(sub_future);
    }
    Box::new(join_all(futures).map(|_| ()))
}
