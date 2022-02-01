#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
mod cli;
mod execution_service;
mod functor;
mod layers;
mod model_service;
mod models;
mod tests;
mod types;
mod types_helper;
mod wc;
mod ws;
extern crate prettytable;

fn main() {
    cli::evaluate();
}
