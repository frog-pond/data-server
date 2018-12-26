#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate rocket;

pub mod routes;

use routes::*;

pub fn server() -> rocket::Rocket {
	rocket::ignite().mount("/", routes![ping])
}
