pub mod db;
pub mod models;
pub mod pages;

use axum::{routing::get, Router};

use pages::{get_display_video, get_home};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::{sync::Arc, time::Duration};

#[derive(Clone)]
pub struct AppState {
    pool: PgPool,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    dotenv::dotenv().ok();
    let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool: PgPool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let app_state: Arc<AppState> = Arc::new(AppState { pool });

    let app = Router::new()
        .route("/http_test", get(http_test))
        .route("/home", get(get_home))
        .route("/video/:id", get(get_display_video))
        .with_state(app_state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn http_test() -> axum::http::StatusCode {
    return axum::http::StatusCode::OK;
}
