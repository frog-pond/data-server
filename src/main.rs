#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/ping")]
fn ping() -> &'static str {
	"pong"
}


fn rocket() -> rocket::Rocket {
	rocket::ignite()
		.mount("/", routes![ping])
}

fn main() {
	rocket()
		.launch();
}
