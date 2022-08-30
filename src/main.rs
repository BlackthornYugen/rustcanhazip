#[macro_use] extern crate rocket;
use std::net::IpAddr;

use rocket::http::HeaderMap;
use rocket::outcome::Outcome;
use rocket::request::{self, Request, FromRequest};

struct ClientData<'a> {
    headers: HeaderMap<'a>,
    ip: IpAddr,
}


#[rocket::async_trait]
impl<'r> FromRequest<'r> for ClientData<'r> {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let client_data = ClientData {
            headers: request.headers().clone(),
            ip: request.real_ip().unwrap_or(request.client_ip().unwrap()).clone(),
        };
        Outcome::Success(client_data)
    }
}

#[get("/")]
fn index(data: ClientData) -> String {
    let host = data.headers.get_one("host").unwrap_or("");
    if host.contains("trace") {
        data.ip.to_string() + "trace\n"
    } else if host.contains("ptr") {
        data.ip.to_string() + " ptr\n"
    } else {
        data.ip.to_string() + "\n"        
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}