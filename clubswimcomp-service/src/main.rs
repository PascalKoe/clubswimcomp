use anyhow::Context;
use api::AppState;
use axum::Router;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

mod api;
mod conversions;
mod db;
mod infra;
mod services;

pub struct Config {
    pub typst_bin: String,
    pub typst_assets: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect("postgres://postgres:changeme@localhost:3001/clubswimcomp")
        .await
        .context("Failed to create database connection pool")?;

    let config = Config {
        typst_bin: std::env::var("CLUBSWIMCOMP_TYPST_BIN")
            .expect("Missing 'CLUBSWIMCOMP_TYPST_BIN'"),
        typst_assets: std::env::var("CLUBSWIMCOMP_TYPST_ASSETS")
            .expect("Missing 'CLUBSWIMCOMP_TYPST_ASSETS'"),
    };
    let app_state = AppState::new(config, pool);

    let app = Router::new()
        .nest("/", api::routes())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}
