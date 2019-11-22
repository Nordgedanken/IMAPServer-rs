#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use diesel_migrations::embed_migrations;

pub mod mailbox;

mod database;
mod models;
mod schema;

embed_migrations!("../migrations");

pub fn setup() {
    let connection = database::establish_connection();
    embedded_migrations::run(&connection).expect("failed to run database migrations");
}
