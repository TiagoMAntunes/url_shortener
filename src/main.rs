use std::clone;

use actix_web::{get, post, web, App, HttpServer, Responder};
use serde::Deserialize;

mod database;

#[derive(Clone)]
struct AppState {
    db: database::Database,
}

#[get("/{shortenedUrl}")]
async fn handle_fetch_url(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let url = path.into_inner();

    match data.db.fetch_url(&url).await {
        Ok(original_url) => original_url,
        Err(_) => "".to_string(),
    }
}

#[post("/login")]
async fn handle_login() -> impl Responder {
    "Login"
}

#[derive(Deserialize)]
struct URLSubmit {
    url: String,
}

fn is_valid_url(url: &str) -> bool {
    true
}

#[post("/")]
async fn handle_submit_url(
    url_form: web::Form<URLSubmit>,
    data: web::Data<AppState>,
) -> impl Responder {
    let url = &url_form.url;
    if !is_valid_url(url) {
        todo!()
    }

    match data.db.save_url(url).await {
        Ok(shortened_url) => shortened_url,
        Err(_) => "".to_string(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse::<u16>()
        .expect("Failed to parse port");

    let state = AppState {
        db: database::Database::new(),
    };

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
