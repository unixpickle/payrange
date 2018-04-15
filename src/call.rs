use futures::{Future, IntoFuture, Stream};
use hyper;
use hyper::{Client, Method, Request};
use hyper::client::Connect;
use hyper::header::ContentType;
use serde::ser::Serialize;
use serde::de::DeserializeOwned;
use serde_yaml;

use super::error::Error;

const API_PATH: &str = "https://api.payrange.com";

pub fn call_api<A: Serialize, B: DeserializeOwned + 'static, C: Connect>(
    client: Client<C>,
    path: &str,
    payload: &A
) -> Box<Future<Item = B, Error = Error>> {
    let future = create_request(path, payload).into_future()
        .and_then(move |request| {
            client.request(request).map_err(From::from)
        })
        .and_then(|response| {
            response.body().concat2().map_err(From::from)
        })
        .and_then(|chunk| {
            let dec_err = serde_yaml::from_slice(&chunk);
            if let Ok(ErrorResponse{status: Some(status), reason: r}) = dec_err {
                Err(Error::Remote{status: status, reason: r})
            } else {
                serde_yaml::from_slice(&chunk).map_err(From::from)
            }
        });
    Box::new(future)
}

fn create_request<T: Serialize>(path: &str, payload: &T) -> Result<Request, Error> {
    let uri = format!("{}{}", API_PATH, path).parse().map_err(hyper::Error::from)?;
    let mut req = Request::new(Method::Post, uri);
    req.set_body(serde_yaml::to_string(payload)?);
    req.headers_mut().set(ContentType("text/plain".parse().unwrap()));
    Ok(req)
}

#[derive(Deserialize)]
struct ErrorResponse {
    status: Option<i32>,
    reason: Option<String>
}
