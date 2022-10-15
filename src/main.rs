use actix_web::{get, post, web, App, HttpServer, Responder};
use log::log_enabled;
use serde::Deserialize;

mod database;

#[derive(Clone)]
struct AppState {
    db: database::Database,
}

#[get("/{shortenedUrl}")]
async fn handle_fetch_url(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let url = path.into_inner();

    log::debug!("Fetching shortened URL {}", url);

    match data.db.fetch_url(&url).await {
        Ok(original_url) => {
            log::debug!("Short URL {} has original {}", url, original_url);
            original_url
        }
        Err(e) => {
            log::error!("Failed to fetch URL {}, err: {}", url, e);
            "".to_string()
        }
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
    regex::Regex::new(
        r#"https?://((www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}|localhost:\d+)\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)"#,
    ).unwrap().is_match(url)
}

#[post("/")]
async fn handle_submit_url(
    url_form: web::Form<URLSubmit>,
    data: web::Data<AppState>,
) -> impl Responder {
    let url = &url_form.url;
    if !is_valid_url(url) {
        log::debug!("{} is not a valid URL", url);
        todo!()
    }

    log::debug!("Saving URL {}", url);
    match data.db.save_url(url).await {
        Ok(shortened_url) => {
            log::debug!("URL {} is now {}", shortened_url, url);
            shortened_url
        }
        Err(e) => {
            log::error!("Failed to save URL {}, err: {}", url, e);
            "".to_string()
        }
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
    
    env_logger::init();
    log::info!("Listening on port {}", port);

    HttpServer::new(move || {
        App::new()
            // .wrap(actix_web::middleware::Logger::default())
            .app_data(web::Data::new(state.clone()))
            .service(handle_fetch_url)
            .service(handle_login)
            .service(handle_submit_url)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
