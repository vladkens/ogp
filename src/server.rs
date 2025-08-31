use anyhow::Result;
use axum::Router;
use axum::http::{StatusCode, Uri, header};
use axum::response::{IntoResponse, Json, Response};
use axum::routing::get;
use rust_embed::Embed;
use tokio::net::TcpListener;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

pub struct AppError(anyhow::Error);
pub type Res<T> = Result<T, AppError>;

impl AppError {
  pub fn new<T>(msg: &str) -> Res<T> {
    Err(Self(anyhow::anyhow!(msg.to_string())))
  }
}

impl IntoResponse for AppError {
  fn into_response(self) -> Response {
    let msg = serde_json::json!({ "code": 400, "message": self.0.to_string() });
    (StatusCode::BAD_REQUEST, Json(msg)).into_response()
  }
}

impl<E: Into<anyhow::Error>> From<E> for AppError {
  fn from(err: E) -> Self {
    Self(err.into())
  }
}

// MARK: Default handlers

async fn health() -> impl IntoResponse {
  let msg = serde_json::json!({ "status": "ok", "ver": env!("CARGO_PKG_VERSION") });
  (StatusCode::OK, axum::response::Json(msg))
}

async fn not_found() -> impl IntoResponse {
  let msg = serde_json::json!({ "code": 404, "message": "not found" });
  (StatusCode::NOT_FOUND, Json(msg))
}

#[derive(Embed)]
#[folder = "assets"]
struct Asset;

async fn static_handler(uri: Uri) -> impl IntoResponse {
  let mut path = uri.path().trim_start_matches('/').to_string();
  if path.starts_with("assets/") {
    path = path.replace("assets/", "");
  }

  match Asset::get(path.as_str()) {
    Some(content) => {
      let mime = mime_guess::from_path(path).first_or_octet_stream();
      ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
    }
    None => not_found().await.into_response(),
  }
}

// https://github.com/tokio-rs/axum/discussions/1894
async fn shutdown_signal() {
  use tokio::signal;

  let ctrl_c = async {
    signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
  };

  let terminate = async {
    signal::unix::signal(signal::unix::SignalKind::terminate())
      .expect("failed to install signal handler")
      .recv()
      .await;
  };

  tokio::select! {
      _ = ctrl_c => {},
      _ = terminate => {},
  }
}

pub async fn run_server(addr: &str, app: Router) -> Result<()> {
  let app = app
    .layer(
      TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
    )
    .route("/health", get(health))
    .route("/assets/{*file}", get(static_handler))
    .fallback_service(get(not_found));

  let mut listenfd = listenfd::ListenFd::from_env();
  let listener = match listenfd.take_tcp_listener(0)? {
    Some(listener) => TcpListener::from_std(listener),
    None => TcpListener::bind(&addr).await,
  }?;

  tracing::info!("listening on http://{}", addr);
  axum::serve(listener, app).with_graceful_shutdown(shutdown_signal()).await?;
  Ok(())
}
