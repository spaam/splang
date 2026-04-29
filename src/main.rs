use lingua::LanguageDetector;
use lingua::LanguageDetectorBuilder;
use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};

use serde::{Deserialize, Serialize};

struct AppState {
    detector: LanguageDetector,
}

#[derive(Deserialize)]
struct Message {
    query: String,
}

#[derive(Serialize)]
struct Langdetected {
    language: String,
}

async fn root() -> &'static str {
    "huh?"
}

async fn detect_lang(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Message>,
) -> (StatusCode, Json<Langdetected>) {
    let lang = match state.detector.detect_language_of(payload.query) {
        None => "und".to_string(),
        Some(lang) => lang.iso_code_639_3().to_string(),
    };
    (StatusCode::OK, Json(Langdetected { language: lang }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let detector = LanguageDetectorBuilder::from_all_languages()
        .with_low_accuracy_mode()
        .build();
    let app_state = Arc::new(AppState { detector });
    let app = Router::new()
        .route("/", get(root))
        .route("/", post(detect_lang))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8344").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
