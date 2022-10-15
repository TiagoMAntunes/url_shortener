use std::clone;

use actix_web::{get, post, web, App, HttpServer, Responder};

#[derive(Clone)]
struct AppState {}

#[get("/{shortenedUrl}")]
async fn handle_fetch_url(data: web::Data<AppState>) -> impl Responder {
    "Fething url"
}

#[post("/login")]
async fn handle_login() -> impl Responder {
    "Login"
}

#[post("/")]
async fn handle_submit_url(data: web::Data<AppState>) -> impl Responder {
    "Submitting url"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse::<u16>()
        .expect("Failed to parse port");

    let state = AppState {};

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(handle_fetch_url)
            .service(handle_login)
            .service(handle_submit_url)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
