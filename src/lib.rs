#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

pub mod routes;

use routes::*;

pub fn server() -> rocket::Rocket {
	rocket::ignite().mount("/", routes![ping])
}
