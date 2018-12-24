use super::*;

#[test]
fn responds_with_pong() {
	let rocket = rocket::ignite().mount("/", routes![ping]);
	let client = Client::new(rocket).expect("valid rocket instance");
	let mut response = client.get("/ping").dispatch();

	assert_eq!(response.status(), Status::Ok);
	assert_eq!(response.body_string(), Some("pong".into()));
}
