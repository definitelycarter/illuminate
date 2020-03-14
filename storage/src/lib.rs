#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod models;
mod schema;
mod storage;

pub use crate::storage::Storage;
pub use models::*;
