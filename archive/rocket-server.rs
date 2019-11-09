#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::json::JsonValue;

#[post("/api")]
fn root_api() -> &'static str {
    handler()
}

#[post("/test")]
fn test_api() -> String {
    greet("shinsaku".to_string())
}

// curl -X POST http://localhost:5000/method -d "hello"
#[post("/check", data = "<task>")]
fn check_api(task: String) -> String {
    greet(task)
}

#[get("/json")]
fn get_json() -> JsonValue {
    json!({
        "id": 83,
        "values": [1, 2, 3, 4]
    })
}

fn handler() -> &'static str {
    "Hello, world!"
}

fn greet(name: String) -> String {
	format!("Hi {}", name)
}

fn main() {
    rocket::ignite().mount("/", routes![root_api, test_api, check_api, get_json]).launch();
}
