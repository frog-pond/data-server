#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/ping")]
fn ping() -> &'static str {
	"pong"
}

mod test {
	use super::rocket;
	use rocket::local::Client;
	use rocket::http::Status;

	#[test]
	fn ping_responds_with_pong() {
		let client = Client::new(rocket()).expect("valid rocket instance");
		let mut response = client.get("/ping").dispatch();

		assert_eq!(response.status(), Status::Ok);
		assert_eq!(response.body_string(), Some("pong".into()));
	}
}

fn rocket() -> rocket::Rocket {
	rocket::ignite()
		.mount("/", routes![ping])
}

fn main() {
	rocket()
		.launch();
}
