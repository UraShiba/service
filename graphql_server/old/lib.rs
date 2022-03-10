#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;
pub mod utils;
pub mod function {
    pub mod delete;
    pub mod insert;
    pub mod query;
    pub mod update;
}
