//! Shared authenticated endpoints for both existing member route families.
use super::{CreateNoteRequest, history, profile};
use crate::{AppState, middleware::admin::AdminUser, utils::errors::ApiError};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::header,
    response::{IntoResponse, Response},
};
use serde_json::Value;

fn response(value: Value) -> Response {
    (
        [
            (header::CACHE_CONTROL, "private, no-store"),
            (header::VARY, "Authorization, Cookie"),
        ],
        Json(value),
    )
        .into_response()
}

pub(crate) async fn show(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(member): Path<i64>,
) -> Result<Response, ApiError> {
    Ok(response(profile::get(&state, member).await?))
}

macro_rules! history_reader {
    ($name:ident, $kind:ident) => {
        pub(crate) async fn $name(
            State(state): State<AppState>,
            _admin: AdminUser,
            Path(member): Path<i64>,
            Query(query): Query<history::HistoryQuery>,
        ) -> Result<Response, ApiError> {
            Ok(response(
                history::read(&state, member, &query, history::Kind::$kind).await?,
            ))
        }
    };
}
history_reader!(notes, Notes);
history_reader!(activity, Activity);
history_reader!(emails, Emails);
history_reader!(memberships, Memberships);
history_reader!(orders, Orders);

pub(crate) async fn timeline(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(member): Path<i64>,
    Query(query): Query<history::HistoryQuery>,
) -> Result<Response, ApiError> {
    if query.action.is_some() {
        return Err(ApiError::validation_error(
            "The timeline does not accept an activity-action filter",
        ));
    }
    let mut tx = state.db.pool.begin().await.map_err(history::db)?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *tx)
        .await
        .map_err(history::db)?;
    let result = profile::timeline_on(
        &mut tx,
        member,
        query.page.unwrap_or(1),
        query.per_page.unwrap_or(25),
    )
    .await?;
    tx.commit().await.map_err(history::db)?;
    Ok(response(result))
}

pub(crate) async fn add_note(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(member): Path<i64>,
    Json(input): Json<CreateNoteRequest>,
) -> Result<Response, ApiError> {
    Ok(response(
        history::add_note(&state, admin.0.id, member, input.content).await?,
    ))
}

pub(crate) async fn remove_note(
    State(state): State<AppState>,
    admin: AdminUser,
    Path((member, note)): Path<(i64, i64)>,
) -> Result<Response, ApiError> {
    Ok(response(
        history::delete_note(&state, admin.0.id, member, note).await?,
    ))
}
