#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/ping")]
fn ping() -> &'static str {
	"pong"
}

fn main() {
	rocket::ignite()
		.mount("/", routes![ping])
		.launch();
}
