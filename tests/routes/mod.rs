// use super::*;

use rocket::http::Status;
use rocket::local::Client;

// In order to get your test module to run, you need to add a mod <file.rs>.
// Note that .rs files in this directory are each given their own "module" that
// is named appropriately, so by adding a .rs file you add a module.  You
// should really like that.
//
// Adding `mod` <filename sans .rs> will do.
mod healthz;
mod ping;
