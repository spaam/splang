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
    let detected_language = state.detector.detect_language_of(payload.query);
    let lang = match detected_language {
        None => "und",
        lang => &lang.unwrap().iso_code_639_3().to_string(),
    };
    (
        StatusCode::OK,
        Json(Langdetected {
            language: lang.into(),
        }),
    )
}

#[tokio::main]
async fn main() {
    let detector = LanguageDetectorBuilder::from_all_languages()
        .with_low_accuracy_mode()
        .build();
    let app_state = Arc::new(AppState { detector });
    let app = Router::new()
        .route("/", get(root))
        .route("/", post(detect_lang))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8344").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
