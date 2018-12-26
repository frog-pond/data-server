// use super::*;

use rocket::http::Status;
use rocket::local::Client;

use data_server::routes::*;

mod ping;
mod healthz;
