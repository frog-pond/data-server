use std::io::Cursor;
use rocket::response::{Response, Result};
use rocket::http::ContentType;

#[derive(Serialize)]
pub struct Health {
	code: bool
}

pub fn get_health() -> Health {
	Health { code: true }
}

#[get("/healthz")]
pub fn healthz() -> Result<'static> {
	Response::build()
		.header(ContentType::JSON)
		.sized_body(Cursor::new(serde_json::to_string(&get_health()).unwrap()))
		.ok()
}
