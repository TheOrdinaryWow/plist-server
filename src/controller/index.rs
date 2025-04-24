use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

pub async fn get_index() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({"message": "fetch '/genPlist' to get data."})),
    )
}
