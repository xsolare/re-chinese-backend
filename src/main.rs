#[macro_use]
extern crate diesel;

#[macro_use]
extern crate log;

use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
mod app;
mod middleware;
mod routes;
mod schema;
mod utils;

use dotenv::dotenv;
use std::env;

pub struct AppState {
    pub pool: utils::db::DbPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=trace");
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let address = env::var(utils::constants::env_key::ADDRESS).expect("ADDRESS must be set");
    print!("Address {}", address);

    HttpServer::new(move || {
        let logger = Logger::default();
        let pool = utils::db::establish_connection();

        App::new()
            .wrap(logger)
            .app_data(Data::new(AppState { pool: pool }))
            .wrap(middleware::cors::cors())
            .wrap(middleware::auth::Authentication)
            .service(web::scope("").configure(routes::api))
    })
    .bind(address)?
    .run()
    .await
}
