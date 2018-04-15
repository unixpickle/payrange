use futures::Future;
use hyper;
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Handle;

use super::call::call_api;
use super::error::Error;
use super::request::{AuthRequest, DeviceRequest};
use super::response::{DeviceResponse, UserResponse};

pub struct Client {
    pub client: hyper::Client<HttpsConnector<HttpConnector>>
}

impl Client {
    pub fn new(h: &Handle) -> Client {
        let client = hyper::Client::configure()
            .connector(HttpsConnector::new(4, h).unwrap())
            .build(h);
        Client{client: client}
    }

    pub fn get_token_email(
        &self,
        email: String,
        password: String
    ) -> Box<Future<Item = String, Error = Error>> {
        let msg = AuthRequest::new_email(email, password);
        let call = call_api::<_, UserResponse, _>(self.client.clone(), "/user/gettoken", &msg);
        Box::new(call.and_then(|info| {
            if info.auth.token.is_some() {
                Ok(info.auth.token.unwrap().token_string)
            } else {
                Err(Error::Other("no auth token in response".to_owned()))
            }
        }))
    }

    pub fn get_device(
        &self,
        token: String,
        id: String
    ) -> Box<Future<Item = DeviceResponse, Error = Error>> {
        let resp_future = call_api(self.client.clone(), "/device", &DeviceRequest{
            auth: token,
            id: id,
            include_loyalty_points_offer: true
        });
        Box::new(resp_future)
    }
}
