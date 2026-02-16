use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

mod handlers;
mod persistence;

pub struct AppState {
    pub config: Mutex<nekotimer_shared::TimerConfigFile>,
    pub config_path: String,
}

const DEFAULT_CONFIG_PATH: &str = "timer.config";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config_path = std::env::var("TIMER_CONFIG_PATH").unwrap_or_else(|_| DEFAULT_CONFIG_PATH.to_string());
    let config = persistence::load_config(&config_path).unwrap_or_default();

    println!("nekotimer backend starting on http://127.0.0.1:14990");

    let data = web::Data::new(AppState {
        config: Mutex::new(config),
        config_path,
    });

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(data.clone())
            .route("/api/timers", web::get().to(handlers::list_timers))
            .route("/api/timers", web::post().to(handlers::create_timer))
            .route("/api/timers/{id}", web::get().to(handlers::get_timer))
            .route("/api/timers/{id}", web::put().to(handlers::update_timer))
            .route("/api/timers/{id}", web::delete().to(handlers::delete_timer))
    })
    .bind("127.0.0.1:14990")?
    .run()
    .await
}
