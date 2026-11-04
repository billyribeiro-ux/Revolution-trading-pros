//! Admin-authorized manual execution uses the same transactional worker as startup.
use super::{ApiResult, CronExecutionResponse};
use crate::{AppState, middleware::admin::AdminUser, services::cms_scheduler};
use axum::{Json, extract::State};

#[utoipa::path(post,path="/api/cms/schedules/process",responses((status=200,description="Processing complete",body=CronExecutionResponse),(status=401,description="Unauthorized"),(status=403,description="Admin access required")),tag="CMS Scheduling (Internal)",security(("bearer_auth"=[])))]
pub(super) async fn process_pending_schedules(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> ApiResult<CronExecutionResponse> {
    cms_scheduler::process_schedules(&state.db.pool)
        .await
        .map(Json)
        .map_err(|error| {
            super::api_error(
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                &error.to_string(),
            )
        })
}

#[utoipa::path(post,path="/api/cms/releases/process",responses((status=200,description="Processing complete",body=CronExecutionResponse),(status=401,description="Unauthorized"),(status=403,description="Admin access required")),tag="CMS Releases (Internal)",security(("bearer_auth"=[])))]
pub(super) async fn process_pending_releases(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> ApiResult<CronExecutionResponse> {
    cms_scheduler::process_releases(&state.db.pool)
        .await
        .map(Json)
        .map_err(|error| {
            super::api_error(
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                &error.to_string(),
            )
        })
}
