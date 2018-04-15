extern crate futures;
extern crate hyper;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_yaml;

mod call;
mod error;
pub use call::call_api;
pub use error::Error;
