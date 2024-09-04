mod components;

use actix_web::{web, App, HttpServer};
use components::{database::MongoDB, routes, utils};
use std::io::Error;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    let mongo_uri: String = utils::get_env_var("MONGO_URI");
    let mongo_db: String = utils::get_env_var("MONGO_DB");

    println!(
        "💽 Connecting to MongoDB at {} with database {}",
        mongo_uri, mongo_db
    );
    let db: MongoDB = MongoDB::new(&mongo_uri, &mongo_db);
    let db_ref: Arc<MongoDB> = Arc::new(db);

    let server_host: String = utils::get_env_var("SERVER_HOST");
    let server_port: String = utils::get_env_var("SERVER_PORT");
    let server_uri: String = format!("{}:{}", server_host, server_port);

    println!("🚀 Starting server on {}", server_uri);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(db_ref.clone()))
            .service(routes::index)
            .service(routes::health)
            .service(routes::create_task)
    })
    .bind(server_uri)
    .unwrap()
    .run()
    .await
}
