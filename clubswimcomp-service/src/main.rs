use anyhow::Context;
use api::AppState;
use axum::Router;
use tower_http::trace::TraceLayer;

mod api;
mod conversions;
mod db;
mod infra;
mod services;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect("postgres://postgres:changeme@localhost:3001/clubswimcomp")
        .await
        .context("Failed to create database connection pool")?;

    let app_state = AppState::new(pool);

    let app = Router::new()
        .nest("/", api::routes())
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}
