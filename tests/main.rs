#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate data_server;

#[cfg(test)]
extern crate mocktopus;

mod routes;
