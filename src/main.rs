mod models;
mod utils;

use crate::models::Status;
use crate::utils::config::ServerConfig;
use ::config::Config;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv;
use tokio_postgres::NoTls;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

async fn status() -> impl Responder {
    // "{\"status\": \"UP\"}"
    HttpResponse::Ok().json(Status {
        status: "Ok".to_string(),
    })
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let config_ = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();

    let config: ServerConfig = config_.try_deserialize().unwrap();
    let pool = config.pg.create_pool(None, NoTls).unwrap();

    println!("server on port: http://127.0.0:8080/");
    println!("Server running at http://{}/", config.server_addr);

    HttpServer::new(move || {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(greet)
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(status))
    })
    .bind(config.server_addr.clone())?
    .run()
    .await
    // println!("Server running at http://{}/", config.server_addr);
}
