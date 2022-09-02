#[macro_use] extern crate rocket;
use std::net::IpAddr;
use std::str;
use std::process::Stdio;
use std::vec::Vec;

use dns_lookup::lookup_addr;

use rocket::http::HeaderMap;
use rocket::outcome::Outcome;
use rocket::request::{self, Request, FromRequest};

struct ClientData<'r> {
    headers: HeaderMap<'r>,
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

async fn index(data: ClientData<'_>) -> String {
    let host = data.headers.get_one("host").unwrap_or("");
    if host.contains("trace") {
        info!("TRACE: tracing {}...", data.ip);
        let trace_text = trace(data.ip).await;
        info!("TRACE: {} has been traced.", data.ip);
        format!("{trace_text}\n")
    } else if host.contains("ptr") {
        info!("PTR: looking up {}...", data.ip);
        let address = lookup_addr(&data.ip).unwrap();
        info!("PTR: {} resolved to {}", data.ip, address);
        format!("{address}\n")
    } else if host.contains("header") {
        let mut header_data: Vec<String> = Vec::new();
        for header in data.headers.iter() {
            header_data.push(format!("{}: {}", header.name, header.value));
        }
        format!("{}\n", header_data.join("\n"))
    } else {
        info!("IP: {} returned", data.ip);
        format!("{}\n", data.ip)
    }
}

async fn trace<'r>(ip: IpAddr) -> String {
    use std::process::Command;

    let process = Command::new("sh") 
        .arg("-c")
        .arg(format!("traceroute {}", ip))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to start process");

    let output = process.wait_with_output().expect("process failed");

    let stderr = str::from_utf8(&output.stderr).expect("can't process stderr");
    let stdout = str::from_utf8(&output.stdout).expect("can't process stdout");

    if output.status.success() {
        format!("{}{}", stderr, stdout)
    } else {
        "".to_owned()
    }
}

#[get("/")]
async fn get (data: ClientData<'_>) -> String {
    index(data).await
}

#[put("/")]
async fn put (data: ClientData<'_>) -> String {
    index(data).await
}

#[post("/")]
async fn post (data: ClientData<'_>) -> String {
    index(data).await
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get, put, post])
}
