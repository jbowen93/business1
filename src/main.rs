#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate jsonwebtoken as jwt;
#[macro_use]
extern crate serde_derive;

use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};

use jwt::{encode, decode, decode_header, Header, Algorithm, Validation};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String
}

struct ApiKey(String);

fn is_valid(key: &str) -> bool {
    // let validation = Validation {
    //     algorithms: Some(vec!(Algorithm::RS256)), 
    //     ..Default::default()
    // };
    // let token_data = decode::<Claims>(&key, "test".as_bytes(), &validation);
    let header = decode_header(key);
    println!("{:?}", header);
    true
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiKey, ()> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }
        let parts: Vec<&str> = keys[0].split(' ').collect();
        if parts[0] != "Bearer" {
            return Outcome::Failure((Status::BadRequest, ()));
        }
        if !is_valid(parts[1]) {
            return Outcome::Forward(());
        }

        return Outcome::Success(ApiKey(parts[1].to_string()));
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/test")]
fn test() -> &'static str {
    "Test!"
}

#[get("/test2")]
fn test2() -> &'static str {
    "New Route!"
}

#[get("/test3")]
fn test3() -> &'static str {
    "Newer Route!"
}

#[get("/sensitive")]
fn senesitive(key: ApiKey) -> &'static str {
    "sensitive data"
}

fn main() {
    rocket::ignite().mount("/", routes![index, test, test2, test3, senesitive]).launch();
}
