//! SMTP settings use the same privileged policy as credential configuration.
use crate::{
    AppState,
    middleware::admin::{AdminUser, SuperAdminUser},
    services::smtp,
};
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use serde_json::{Value, json};
type Error = (StatusCode, Json<Value>);
fn unavailable(error: anyhow::Error) -> Error {
    tracing::error!(error=%error,"SMTP settings operation failed");
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(json!({"error":"SMTP settings storage is unavailable"})),
    )
}
fn operation_error(error: anyhow::Error) -> Error {
    if error
        .chain()
        .any(|cause| cause.downcast_ref::<sqlx::Error>().is_some())
    {
        unavailable(error)
    } else {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error":error.to_string()})),
        )
    }
}
async fn get_settings(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<Value>, Error> {
    let stored = smtp::load(&state).await.map_err(unavailable)?;
    let mut value=stored.as_ref().map(|stored|stored.settings.public_view()).unwrap_or_else(||json!({"provider":"smtp","host":"","port":587,"username":"","has_password":false,"encryption":"tls","from_address":"","from_name":""}));
    value["environment"] = json!(crate::services::credential_resolver::runtime_environment(
        &state.config
    ));
    value["status"] = stored
        .as_ref()
        .map(|s| s.value["status"].clone())
        .unwrap_or(json!("pending"));
    value["last_verified_at"] = stored
        .as_ref()
        .map(|s| s.value["last_verified_at"].clone())
        .unwrap_or(Value::Null);
    value["last_error"] = stored
        .as_ref()
        .map(|s| s.value["last_error"].clone())
        .unwrap_or(Value::Null);
    Ok(Json(value))
}
async fn save_settings(
    State(state): State<AppState>,
    admin: SuperAdminUser,
    Json(input): Json<smtp::SmtpSettings>,
) -> Result<Json<Value>, Error> {
    smtp::save(&state, input, admin.0.id).await
        .map(|settings| Json(json!({"success":true,"message":"SMTP settings saved; verify the connection before use","settings":settings})))
        .map_err(operation_error)
}
async fn test_settings(
    State(state): State<AppState>,
    admin: SuperAdminUser,
    Json(input): Json<smtp::SmtpSettings>,
) -> Result<Json<Value>, Error> {
    smtp::verify(&state, input, admin.0.id)
        .await
        .map(Json)
        .map_err(operation_error)
}
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/settings", get(get_settings).post(save_settings))
        .route("/settings/test", post(test_settings))
}
