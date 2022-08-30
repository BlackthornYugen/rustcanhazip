#[macro_use] extern crate rocket;
use rocket::outcome::Outcome;
use rocket::request::{self, Request, FromRequest};

enum HostType {
    IP,
    PTR,
    TRACE,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for HostType {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let host_headers: Vec<_> = request.headers().get("host").collect();
        let host: String;
        if host_headers.len() != 0 {
            host = host_headers[0].to_lowercase();
        } else {
            host = String::from("ip");
        }

        if host.contains("ip") {
            Outcome::Success(HostType::IP)
        } else if host.contains("ptr") {
            Outcome::Success(HostType::PTR)
        } else if host.contains("trace") {
            Outcome::Success(HostType::TRACE)
        } else {
            Outcome::Success(HostType::IP)
        }
    }
}

#[get("/")]
fn index(action: HostType) -> String {
    match action {
        HostType::IP => do_ip(),
        HostType::PTR => do_ptr(),
        HostType::TRACE => do_trace(),
    }
}

fn do_ip() -> String {
    return String::from("127.0.0.1\n")
}

fn do_trace() -> String {
    return String::from("trace\n")
}

fn do_ptr() -> String {
    return String::from("ptr\n")
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}