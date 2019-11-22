#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use diesel_migrations::embed_migrations;

pub mod config;
pub mod mailbox;

mod database;
mod models;
mod schema;

#[cfg(test)]
mod tests;

embed_migrations!("../migrations");

pub fn setup() {
    let connection = database::establish_connection();
    embedded_migrations::run(&connection).expect("failed to run shared migrations");
}
