#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[post("/api")]
fn root_api() -> &'static str {
    handler()
}

fn handler() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![root_api]).launch();
}
