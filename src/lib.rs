#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/ping")]
fn ping() -> &'static str {
	"pong"
}

pub fn server() -> rocket::Rocket {
	rocket::ignite()
		.mount("/", routes![ping])
}
