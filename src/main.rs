use axum::Router;
use axum::routing::get;
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

mod handlers;
mod render;
mod server;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  dotenvy::dotenv().ok();

  tracing_subscriber::fmt()
    .with_env_filter(
      EnvFilter::builder().with_default_directive(LevelFilter::INFO.into()).from_env_lossy(),
    )
    .with_target(false)
    .compact()
    .init();

  let app = Router::new() //
    .route("/v0/svg", get(handlers::ogi_svg))
    .route("/v0/png", get(handlers::ogi_png))
    .route("/", get(handlers::index));

  let host = std::env::var("HOST").unwrap_or("127.0.0.1".to_string());
  let port = std::env::var("PORT").unwrap_or("8080".to_string());
  let addr = format!("{}:{}", host, port);
  Ok(server::run_server(&addr, app).await?)
}
