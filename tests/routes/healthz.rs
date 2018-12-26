use super::*;

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
