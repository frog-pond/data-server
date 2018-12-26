#![feature(decl_macro, proc_macro_hygiene, uniform_paths)]

#[macro_use]
extern crate rocket;

pub mod routes;

use routes::*;

pub fn server() -> rocket::Rocket {
	rocket::ignite().mount("/", routes![ping])
}
