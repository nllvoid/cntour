use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use std::env;

mod api;
mod image_processing;

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().json("API is up and running!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("{}:{}", host, port);

    let allowed_origin = env::var("ALLOWED_ORIGIN").unwrap_or_else(|_| "*".to_string());

    log::info!("Server running at http://{}", address);

    HttpServer::new(move || {
        let cors = if allowed_origin == "*" {
            Cors::default()
                .allow_any_origin()
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![actix_web::http::header::CONTENT_TYPE])
                .max_age(3600)
        } else {
            Cors::default()
                .allowed_origin(&allowed_origin)
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![actix_web::http::header::CONTENT_TYPE])
                .max_age(3600)
        };

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .route("/images/health", web::get().to(health))
            .route("/image/generate/single", web::post().to(api::generate_single))
            .route("/image/generate/blended", web::post().to(api::generate_blended))
    })
        .bind(&address)?
        .run()
        .await
}