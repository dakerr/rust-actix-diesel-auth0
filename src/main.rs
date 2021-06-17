extern crate openssl;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

use crate::server::server;

mod auth;
mod config;
mod database;
mod errors;
mod helpers;
mod models;
mod routes;
mod schema;
mod server;
mod state;
mod tests;
pub mod handlers;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    server().await
}
