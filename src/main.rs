#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/ping")]
fn ping() -> &'static str {
	"pong"
}

mod test {
	use super::rocket;
	use rocket::http::Status;
	use rocket::local::Client;

	#[test]
	fn ping_responds_with_pong() {
		let client = Client::new(rocket()).expect("valid rocket instance");
		let mut response = client.get("/ping").dispatch();

		assert_eq!(response.status(), Status::Ok);
		assert_eq!(response.body_string(), Some("pong".into()));
	}
}

fn rocket() -> rocket::Rocket {
	rocket::ignite().mount("/", routes![ping])
}

fn main() {
	rocket().launch();
}
