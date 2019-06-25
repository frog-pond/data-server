#![feature(custom_attribute, decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde;

#[cfg(test)]
extern crate mocktopus;

pub mod routes;

/// Prepares a `rocket::Rocket` for usage.
///
/// # Examples
///
/// ```
/// use data_server::*;
/// let rocket: rocket::Rocket = server();
/// ```
pub fn server() -> rocket::Rocket {
	rocket::ignite().mount("/", routes![routes::healthz::healthz, routes::ping::ping])
}
