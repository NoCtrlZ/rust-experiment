#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[post("/api")]
fn root_api() -> &'static str {
    handler()
}

#[post("/test")]
fn test_api() -> String {
    greet("shinsaku".to_string())
}

#[post("/check", data = "<task>")]
fn check_api(task: String) -> String {
    greet(task)
}

fn handler() -> &'static str {
    "Hello, world!"
}

fn greet(name: String) -> String {
	format!("Hi {}", name)
}

fn main() {
    rocket::ignite().mount("/", routes![root_api, test_api, check_api]).launch();
}
