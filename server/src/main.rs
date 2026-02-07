use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

const DB_DIR: &str = "db";

#[tokio::main]
async fn main() {
    let db_path = std::env::var("DB_PATH").unwrap_or_else(|_| DB_DIR.to_string());
    let app = Router::new()
        .route("/api/list", get(list_files))
        .route("/api/save", post(save_file))
        .route("/api/files/*path", get(get_file))
        .nest_service("/", ServeDir::new("nekokan_music_wa/dist"))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
        .with_state(AppState { db_path: PathBuf::from(db_path) });

    let listener = tokio::net::TcpListener::bind("127.0.0.1:12989").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
struct AppState {
    db_path: PathBuf,
}

async fn list_files(axum::extract::State(state): axum::extract::State<AppState>) -> impl IntoResponse {
    let dir = state.db_path;
    let Ok(entries) = fs::read_dir(&dir) else {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!([]))).into_response();
    };
    let mut names: Vec<String> = entries
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let n = e.file_name();
            let s = n.to_string_lossy();
            if s.ends_with(".json") {
                Some(s.to_string())
            } else {
                None
            }
        })
        .collect();
    names.sort();
    (StatusCode::OK, Json(names)).into_response()
}

async fn get_file(
    axum::extract::State(state): axum::extract::State<AppState>,
    Path(path): Path<String>,
) -> impl IntoResponse {
    let path = path.trim_start_matches('/');
    if path.contains("..") || path.contains('\\') {
        return (StatusCode::BAD_REQUEST, Json(Value::Null)).into_response();
    }
    let full = state.db_path.join(path);
    if full.strip_prefix(&state.db_path).is_err() {
        return (StatusCode::FORBIDDEN, Json(Value::Null)).into_response();
    }
    let Ok(data) = fs::read_to_string(&full) else {
        return (StatusCode::NOT_FOUND, Json(Value::Null)).into_response();
    };
    let Ok(json) = serde_json::from_str::<Value>(&data) else {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(Value::Null)).into_response();
    };
    (StatusCode::OK, Json(json)).into_response()
}

#[derive(serde::Deserialize)]
struct SaveBody {
    filename: String,
    data: Value,
}

async fn save_file(
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(body): Json<SaveBody>,
) -> impl IntoResponse {
    let mut filename = body.filename.trim().to_string();
    if filename.ends_with(".json") {
        filename = filename.strip_suffix(".json").unwrap_or(&filename).to_string();
    }
    filename = filename
        .replace("..", "")
        .replace('/', "")
        .replace('\\', "")
        .replace(':', "");
    if filename.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "invalid filename"}))).into_response();
    }
    let filename = format!("{}.json", filename);
    let full = state.db_path.join(&filename);
    if full.strip_prefix(&state.db_path).is_err() {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({"error": "forbidden"}))).into_response();
    }
    let Ok(json_str) = serde_json::to_string_pretty(&body.data) else {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "invalid json"}))).into_response();
    };
    if let Err(e) = fs::write(&full, json_str) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response();
    }
    (StatusCode::OK, Json(serde_json::json!({"ok": true}))).into_response()
}
