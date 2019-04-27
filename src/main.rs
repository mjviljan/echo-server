#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::json::{Json};
use serde_json::Value;

#[post("/json", format = "json", data = "<msg>")]
fn echo(msg: Json<Value>) -> Json<Value> {
    msg
}

fn main() {
    rocket::ignite().mount("/echo", routes![echo]).launch();
}
