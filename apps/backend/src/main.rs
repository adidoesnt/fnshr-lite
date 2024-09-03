mod components;

use components::{database::Database, utils, routes};
use actix_web::{App, HttpServer};
use std::io::Error;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    let mongo_uri: String = utils::get_env_var("MONGO_URI");

    println!("💽 Connecting to MongoDB at {}", mongo_uri);
    let _db: Database = Database::new(&mongo_uri);

    let server_host: String = utils::get_env_var("SERVER_HOST");
    let server_port: String = utils::get_env_var("SERVER_PORT");
    let server_uri: String = format!("{}:{}", server_host, server_port);

    println!("🚀 Starting server on {}", server_uri);
    HttpServer::new(move || {
        App::new()
            .service(routes::index)
            .service(routes::health)
    })
    .bind(server_uri)
    .unwrap()
    .run()
    .await
}
