use super::*;
use mocktopus::mocking::*;

use data_server::routes::healthz::*;

#[test]
fn when_healthy_responds_healthily() {
	routes::healthz::get_health.mock_safe(|| MockResult::Return(Health { code: true }));

	let rocket = rocket::ignite().mount("/", routes![healthz]);
	let client = Client::new(rocket).expect("valid rocket instance");
	let mut response = client.get("/healthz").dispatch();

	assert_eq!(response.status(), Status::Ok);

	let response: String = response.body_string().unwrap();
	let response: Health = serde_json::from_str(&response).unwrap();

	assert_eq!(response.code, true);
}

#[test]
fn responds_with_correct_headers() {
	let rocket = rocket::ignite().mount("/", routes![healthz]);
	let client = Client::new(rocket).expect("valid rocket instance");
	let response = client.get("/healthz").dispatch();

	assert_eq!(response.status(), Status::Ok);
	assert!(response.headers().contains("Content-Type"));
	assert_eq!(
		response.headers().get_one("Content-Type"),
		Some("application/json")
	);
}
