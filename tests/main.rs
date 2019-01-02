#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate data_server;

#[cfg(test)]
extern crate mocktopus;

// Add a `mod` for each test directory with a mod.rs in it.
mod routes;
