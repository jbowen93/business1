#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

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

#[get("/test4")]
fn test4() -> &'static str {
    "Newest Route!"
}

fn main() {
    rocket::ignite().mount("/", routes![index, test, test2, test3, test4]).launch();
}
