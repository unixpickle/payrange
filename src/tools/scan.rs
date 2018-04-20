//! Search for PayRange devices.

extern crate clap;
extern crate futures;
extern crate payrange;
extern crate tokio_core;

mod util;

use clap::{App, Arg};
use futures::{Future, Stream};
use futures::stream::iter_ok;
use payrange::{Client, Error};
use payrange::response::{Category, Geocode};
use tokio_core::reactor::Core;

use util::{get_token, token_arg};

struct BasicDeviceInfo {
    id: String,
    created: u64,
    updated: u64,
    latitude: f64,
    longitude: f64,
    category_top: String,
    category_sub: String
}

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
        .get_matches();
    let token = get_token(&matches);
    let start = matches.value_of("start").unwrap_or("10000000").parse().unwrap();
    let end = matches.value_of("end").unwrap_or("100000000").parse().unwrap();
    let concurrency = matches.value_of("concurrency").unwrap_or("100").parse().unwrap();

    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());

    println!("id,created,updated,latitude,longitude,category_top,category_sub");
    let process_stream = iter_ok(start..end)
        .map(|id| basic_device_info(&client, &token, id))
        .buffer_unordered(concurrency)
        .then(|res| -> Result<(), ()> {
            if let Ok(res) = res {
                println!("{},{},{},{},{},{},{}", res.id, res.created, res.updated, res.latitude,
                    res.longitude, res.category_top, res.category_sub);
            }
            Ok(())
        }).for_each(Ok);

    core.run(process_stream).unwrap();
}

fn basic_device_info(
    client: &Client,
    token: &str,
    id: u32
) -> Box<Future<Item = BasicDeviceInfo, Error = Error>> {
    Box::new(client.get_device(token.to_owned(), format!("{:08}", id))
        .map(move |raw_info| {
            let (lat, lon) = if let Some(Geocode{coordinates: coords, ..}) = raw_info.geocode {
                if coords.len() == 2 {
                    (coords[1], coords[0])
                } else {
                    (0.0, 0.0)
                }
            } else {
                (0.0, 0.0)
            };
            let (cat_top, cat_sub) = match raw_info.category {
                Some(Category{top_level: x, sub_level: y, ..}) => (x, y),
                _ => (None, None)
            };
            BasicDeviceInfo{
                id: raw_info.id,
                created: raw_info.created,
                updated: raw_info.updated.unwrap_or(raw_info.created),
                latitude: lat,
                longitude: lon,
                category_top: cat_top.unwrap_or(String::new()),
                category_sub: cat_sub.unwrap_or(String::new())
            }
        }))
}
