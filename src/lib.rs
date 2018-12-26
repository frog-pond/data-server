#![feature(decl_macro, proc_macro_hygiene, uniform_paths)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_derive;

pub mod routes;

use routes::*;

/// Prepares a rocket::Rocket for usage.
///
/// # Examples
///
/// ```
/// use data_server::*;
/// let rocket: rocket::Rocket = server();
/// ```
pub fn server() -> rocket::Rocket {
	rocket::ignite().mount("/", routes![healthz, ping])
}
