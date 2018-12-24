extern crate rocket;

mod routes {
	mod ping {
		use rocket::http::Status;
		use rocket::local::Client;

		#[test]
		fn responds_with_pong() {
			let client = Client::new(data_server::server()).expect("valid rocket instance");
			let mut response = client.get("/ping").dispatch();

			assert_eq!(response.status(), Status::Ok);
			assert_eq!(response.body_string(), Some("pong".into()));
		}
	}
}
