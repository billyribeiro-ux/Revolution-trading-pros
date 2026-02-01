//! CMS v2 Routes - Apple ICT 7+ Principal Engineer Grade
//!
//! Comprehensive API routes for the custom CMS:
//! - Content CRUD with block-based editing
//! - Digital Asset Management
//! - Workflow management
//! - Navigation and redirects
//! - Site settings
//! - Audit logging
//!
//! @version 2.0.0 - January 2026

use axum::{
    extract::{Multipart, Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use uuid::Uuid;

use crate::{
    models::{
        cms::{
            AssetListQuery, CmsAsset, CmsAssetFolder, CmsAssetSummary, CmsComment, CmsContent,
            CmsContentStatus, CmsContentSummary, CmsContentType, CmsNavigationMenu, CmsRedirect,
            CmsRevision, CmsSiteSettings, CmsTag, CmsUser, CmsUserRole, ContentListQuery,
            CreateAssetFolderRequest, CreateAssetRequest, CreateCommentRequest,
            CreateContentRequest, CreateNavigationMenuRequest, CreateRedirectRequest,
            CreateRelationRequest, CreateTagRequest, PaginatedResponse, TransitionStatusRequest,
            UpdateAssetFolderRequest, UpdateAssetRequest, UpdateContentRequest,
            UpdateNavigationMenuRequest, UpdateSiteSettingsRequest,
        },
        User,
    },
    services::cms_content,
    AppState,
};

// ═══════════════════════════════════════════════════════════════════════════════════════
// AUTHORIZATION HELPERS
// ═══════════════════════════════════════════════════════════════════════════════════════

type ApiResult<T> = Result<Json<T>, (StatusCode, Json<JsonValue>)>;
type ApiResultEmpty = Result<Json<JsonValue>, (StatusCode, Json<JsonValue>)>;

fn api_error(status: StatusCode, message: &str) -> (StatusCode, Json<JsonValue>) {
    (status, Json(json!({ "error": message })))
}

fn require_cms_admin(user: &User) -> Result<(), (StatusCode, Json<JsonValue>)> {
    let role = user.role.as_deref().unwrap_or("user");
    if matches!(role, "admin" | "super-admin" | "super_admin") {
        Ok(())
    } else {
        Err(api_error(StatusCode::FORBIDDEN, "Admin access required"))
    }
}

fn require_cms_editor(user: &User) -> Result<(), (StatusCode, Json<JsonValue>)> {
    let role = user.role.as_deref().unwrap_or("user");
    if matches!(
        role,
        "admin" | "super-admin" | "super_admin" | "editor" | "marketing"
    ) {
        Ok(())
    } else {
        Err(api_error(StatusCode::FORBIDDEN, "Editor access required"))
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// ASSET FOLDERS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// List all asset folders
async fn list_asset_folders(
    State(state): State<AppState>,
    user: User,
) -> ApiResult<Vec<CmsAssetFolder>> {
    require_cms_editor(&user)?;

    let folders = cms_content::list_asset_folders(&state.db.pool)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(folders))
}

/// Get asset folder by ID
async fn get_asset_folder(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResult<CmsAssetFolder> {
    require_cms_editor(&user)?;

    let folder = cms_content::get_asset_folder(&state.db.pool, id)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?
        .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "Folder not found"))?;

    Ok(Json(folder))
}

/// Create asset folder
async fn create_asset_folder(
    State(state): State<AppState>,
    user: User,
    Json(request): Json<CreateAssetFolderRequest>,
) -> ApiResult<CmsAssetFolder> {
    require_cms_editor(&user)?;

    let cms_user = cms_content::get_cms_user_by_user_id(&state.db.pool, user.id)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    let folder = cms_content::create_asset_folder(&state.db.pool, request, cms_user.map(|u| u.id))
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(folder))
}

/// Update asset folder
async fn update_asset_folder(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateAssetFolderRequest>,
) -> ApiResult<CmsAssetFolder> {
    require_cms_editor(&user)?;

    let cms_user = cms_content::get_cms_user_by_user_id(&state.db.pool, user.id)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    let folder =
        cms_content::update_asset_folder(&state.db.pool, id, request, cms_user.map(|u| u.id))
            .await
            .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(folder))
}

/// Delete asset folder
#[derive(Deserialize)]
struct DeleteFolderQuery {
    move_to: Option<Uuid>,
}

async fn delete_asset_folder(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
    Query(query): Query<DeleteFolderQuery>,
) -> ApiResultEmpty {
    require_cms_admin(&user)?;

    cms_content::delete_asset_folder(&state.db.pool, id, query.move_to)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(json!({ "message": "Folder deleted successfully" })))
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// ASSETS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// List assets with pagination and filtering
async fn list_assets(
    State(state): State<AppState>,
    user: User,
    Query(query): Query<AssetListQuery>,
) -> ApiResult<PaginatedResponse<CmsAssetSummary>> {
    require_cms_editor(&user)?;

    let assets = cms_content::list_assets(&state.db.pool, query)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(assets))
}

/// Get asset by ID
async fn get_asset(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResult<CmsAsset> {
    require_cms_editor(&user)?;

    let asset = cms_content::get_asset(&state.db.pool, id)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?
        .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "Asset not found"))?;

    Ok(Json(asset))
}

/// Create asset (metadata only - upload handled separately)
async fn create_asset(
    State(state): State<AppState>,
    user: User,
    Json(request): Json<CreateAssetRequest>,
) -> ApiResult<CmsAsset> {
    require_cms_editor(&user)?;

    let cms_user = cms_content::get_cms_user_by_user_id(&state.db.pool, user.id)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    let asset = cms_content::create_asset(&state.db.pool, request, cms_user.map(|u| u.id))
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(asset))
}

/// Update asset metadata
async fn update_asset(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateAssetRequest>,
) -> ApiResult<CmsAsset> {
    require_cms_editor(&user)?;

    let cms_user = cms_content::get_cms_user_by_user_id(&state.db.pool, user.id)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    let asset = cms_content::update_asset(&state.db.pool, id, request, cms_user.map(|u| u.id))
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(asset))
}

/// Delete asset (soft delete)
async fn delete_asset(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResultEmpty {
    require_cms_editor(&user)?;

    cms_content::delete_asset(&state.db.pool, id)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(json!({ "message": "Asset deleted successfully" })))
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// CONTENT
// ═══════════════════════════════════════════════════════════════════════════════════════

/// List content with pagination and filtering
async fn list_content(
    State(state): State<AppState>,
    user: User,
    Query(query): Query<ContentListQuery>,
) -> ApiResult<PaginatedResponse<CmsContentSummary>> {
    require_cms_editor(&user)?;

    let content = cms_content::list_content(&state.db.pool, query)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(content))
}

/// Get content by ID
async fn get_content(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResult<CmsContent> {
    require_cms_editor(&user)?;

    let content = cms_content::get_content(&state.db.pool, id)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?
        .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "Content not found"))?;

    Ok(Json(content))
}

/// Get content by type and slug (public endpoint)
#[derive(Deserialize)]
struct ContentBySlugQuery {
    locale: Option<String>,
}

async fn get_content_by_slug(
    State(state): State<AppState>,
    Path((content_type, slug)): Path<(String, String)>,
    Query(query): Query<ContentBySlugQuery>,
) -> ApiResult<CmsContent> {
    let content_type = parse_content_type(&content_type)?;

    let content = cms_content::get_content_by_slug(
        &state.db.pool,
        content_type,
        &slug,
        query.locale.as_deref(),
    )
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?
    .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "Content not found"))?;

    // Only return published content for public endpoint
    if content.status != CmsContentStatus::Published {
        return Err(api_error(StatusCode::NOT_FOUND, "Content not found"));
    }

    Ok(Json(content))
}

/// Create content
async fn create_content(
    State(state): State<AppState>,
    user: User,
    Json(request): Json<CreateContentRequest>,
) -> ApiResult<CmsContent> {
    require_cms_editor(&user)?;

    let cms_user = cms_content::get_cms_user_by_user_id(&state.db.pool, user.id)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    let content = cms_content::create_content(&state.db.pool, request, cms_user.map(|u| u.id))
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(content))
}

/// Update content
async fn update_content(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateContentRequest>,
) -> ApiResult<CmsContent> {
    require_cms_editor(&user)?;

    let cms_user = cms_content::get_cms_user_by_user_id(&state.db.pool, user.id)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    let content = cms_content::update_content(&state.db.pool, id, request, cms_user.map(|u| u.id))
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(content))
}

/// Transition content status (publish, archive, etc.)
async fn transition_content_status(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
    Json(request): Json<TransitionStatusRequest>,
) -> ApiResult<CmsContent> {
    require_cms_editor(&user)?;

    // Check permissions for publish
    if request.status == CmsContentStatus::Published {
        require_cms_admin(&user)?;
    }

    let cms_user = cms_content::get_cms_user_by_user_id(&state.db.pool, user.id)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    let content =
        cms_content::transition_content_status(&state.db.pool, id, request, cms_user.map(|u| u.id))
            .await
            .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(content))
}

/// Delete content (soft delete)
async fn delete_content(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResultEmpty {
    require_cms_admin(&user)?;

    let cms_user = cms_content::get_cms_user_by_user_id(&state.db.pool, user.id)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    cms_content::delete_content(&state.db.pool, id, cms_user.map(|u| u.id))
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(json!({ "message": "Content deleted successfully" })))
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// REVISIONS
// ═══════════════════════════════════════════════════════════════════════════════════════

#[derive(Deserialize)]
struct RevisionListQuery {
    limit: Option<i64>,
}

/// Get revisions for content
async fn get_content_revisions(
    State(state): State<AppState>,
    user: User,
    Path(content_id): Path<Uuid>,
    Query(query): Query<RevisionListQuery>,
) -> ApiResult<Vec<CmsRevision>> {
    require_cms_editor(&user)?;

    let revisions =
        cms_content::get_revisions(&state.db.pool, content_id, query.limit.unwrap_or(25))
            .await
            .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(revisions))
}

/// Restore a revision
async fn restore_revision(
    State(state): State<AppState>,
    user: User,
    Path((content_id, revision_number)): Path<(Uuid, i32)>,
) -> ApiResult<CmsContent> {
    require_cms_admin(&user)?;

    let cms_user = cms_content::get_cms_user_by_user_id(&state.db.pool, user.id)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    let content = cms_content::restore_revision(
        &state.db.pool,
        content_id,
        revision_number,
        cms_user.map(|u| u.id),
    )
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(content))
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// TAGS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// List all tags (requires editor access)
async fn list_tags(State(state): State<AppState>, user: User) -> ApiResult<Vec<CmsTag>> {
    require_cms_editor(&user)?;

    let tags = cms_content::list_tags(&state.db.pool)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(tags))
}

/// Create tag
async fn create_tag(
    State(state): State<AppState>,
    user: User,
    Json(request): Json<CreateTagRequest>,
) -> ApiResult<CmsTag> {
    require_cms_editor(&user)?;

    let cms_user = cms_content::get_cms_user_by_user_id(&state.db.pool, user.id)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    let tag = cms_content::create_tag(&state.db.pool, request, cms_user.map(|u| u.id))
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(tag))
}

/// Get tags for content
async fn get_content_tags(
    State(state): State<AppState>,
    user: User,
    Path(content_id): Path<Uuid>,
) -> ApiResult<Vec<CmsTag>> {
    require_cms_editor(&user)?;

    let tags = cms_content::get_content_tags(&state.db.pool, content_id)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(tags))
}

/// Add tag to content
async fn add_tag_to_content(
    State(state): State<AppState>,
    user: User,
    Path((content_id, tag_id)): Path<(Uuid, Uuid)>,
) -> ApiResultEmpty {
    require_cms_editor(&user)?;

    let cms_user = cms_content::get_cms_user_by_user_id(&state.db.pool, user.id)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    cms_content::add_tag_to_content(&state.db.pool, content_id, tag_id, cms_user.map(|u| u.id))
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(json!({ "message": "Tag added successfully" })))
}

/// Remove tag from content
async fn remove_tag_from_content(
    State(state): State<AppState>,
    user: User,
    Path((content_id, tag_id)): Path<(Uuid, Uuid)>,
) -> ApiResultEmpty {
    require_cms_editor(&user)?;

    cms_content::remove_tag_from_content(&state.db.pool, content_id, tag_id)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(json!({ "message": "Tag removed successfully" })))
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// COMMENTS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Get comments for content
async fn get_content_comments(
    State(state): State<AppState>,
    user: User,
    Path(content_id): Path<Uuid>,
) -> ApiResult<Vec<CmsComment>> {
    require_cms_editor(&user)?;

    let comments = cms_content::get_content_comments(&state.db.pool, content_id)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(comments))
}

/// Create comment
async fn create_comment(
    State(state): State<AppState>,
    user: User,
    Path(content_id): Path<Uuid>,
    Json(request): Json<CreateCommentRequest>,
) -> ApiResult<CmsComment> {
    require_cms_editor(&user)?;

    let cms_user = cms_content::get_cms_user_by_user_id(&state.db.pool, user.id)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    let comment =
        cms_content::create_comment(&state.db.pool, content_id, request, cms_user.map(|u| u.id))
            .await
            .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(comment))
}

/// Resolve comment
async fn resolve_comment(
    State(state): State<AppState>,
    user: User,
    Path(comment_id): Path<Uuid>,
) -> ApiResultEmpty {
    require_cms_editor(&user)?;

    let cms_user = cms_content::get_cms_user_by_user_id(&state.db.pool, user.id)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?
        .ok_or_else(|| api_error(StatusCode::FORBIDDEN, "CMS user not found"))?;

    cms_content::resolve_comment(&state.db.pool, comment_id, cms_user.id)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(json!({ "message": "Comment resolved" })))
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// SITE SETTINGS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Get site settings
async fn get_site_settings(State(state): State<AppState>) -> ApiResult<CmsSiteSettings> {
    let settings = cms_content::get_site_settings(&state.db.pool)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(settings))
}

/// Update site settings
async fn update_site_settings(
    State(state): State<AppState>,
    user: User,
    Json(request): Json<UpdateSiteSettingsRequest>,
) -> ApiResult<CmsSiteSettings> {
    require_cms_admin(&user)?;

    let cms_user = cms_content::get_cms_user_by_user_id(&state.db.pool, user.id)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    let settings =
        cms_content::update_site_settings(&state.db.pool, request, cms_user.map(|u| u.id))
            .await
            .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(settings))
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// NAVIGATION MENUS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// List navigation menus
async fn list_navigation_menus(State(state): State<AppState>) -> ApiResult<Vec<CmsNavigationMenu>> {
    let menus = cms_content::list_navigation_menus(&state.db.pool)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(menus))
}

/// Get navigation menu by slug
async fn get_navigation_menu(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> ApiResult<CmsNavigationMenu> {
    let menu = cms_content::get_navigation_menu(&state.db.pool, &slug)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?
        .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "Menu not found"))?;

    Ok(Json(menu))
}

/// Create navigation menu
async fn create_navigation_menu(
    State(state): State<AppState>,
    user: User,
    Json(request): Json<CreateNavigationMenuRequest>,
) -> ApiResult<CmsNavigationMenu> {
    require_cms_admin(&user)?;

    let cms_user = cms_content::get_cms_user_by_user_id(&state.db.pool, user.id)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    let menu = cms_content::create_navigation_menu(&state.db.pool, request, cms_user.map(|u| u.id))
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(menu))
}

/// Update navigation menu
async fn update_navigation_menu(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateNavigationMenuRequest>,
) -> ApiResult<CmsNavigationMenu> {
    require_cms_admin(&user)?;

    let cms_user = cms_content::get_cms_user_by_user_id(&state.db.pool, user.id)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    let menu =
        cms_content::update_navigation_menu(&state.db.pool, id, request, cms_user.map(|u| u.id))
            .await
            .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(menu))
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// REDIRECTS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// List redirects
async fn list_redirects(State(state): State<AppState>, user: User) -> ApiResult<Vec<CmsRedirect>> {
    require_cms_admin(&user)?;

    let redirects = cms_content::list_redirects(&state.db.pool)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(redirects))
}

/// Create redirect
async fn create_redirect(
    State(state): State<AppState>,
    user: User,
    Json(request): Json<CreateRedirectRequest>,
) -> ApiResult<CmsRedirect> {
    require_cms_admin(&user)?;

    let cms_user = cms_content::get_cms_user_by_user_id(&state.db.pool, user.id)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    let redirect = cms_content::create_redirect(&state.db.pool, request, cms_user.map(|u| u.id))
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(redirect))
}

/// Delete redirect
async fn delete_redirect(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResultEmpty {
    require_cms_admin(&user)?;

    cms_content::delete_redirect(&state.db.pool, id)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(json!({ "message": "Redirect deleted successfully" })))
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// CMS STATS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Get CMS overview statistics
async fn get_cms_stats(State(state): State<AppState>, user: User) -> ApiResultEmpty {
    require_cms_editor(&user)?;

    let total_content: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM cms_content WHERE deleted_at IS NULL")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    let published_content: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM cms_content WHERE status = 'published' AND deleted_at IS NULL",
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    let draft_content: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM cms_content WHERE status = 'draft' AND deleted_at IS NULL",
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    let total_assets: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM cms_assets WHERE deleted_at IS NULL")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    let pending_review: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM cms_content WHERE status = 'in_review' AND deleted_at IS NULL",
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    Ok(Json(json!({
        "content": {
            "total": total_content.0,
            "published": published_content.0,
            "draft": draft_content.0,
            "pending_review": pending_review.0
        },
        "assets": {
            "total": total_assets.0
        }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// HELPERS
// ═══════════════════════════════════════════════════════════════════════════════════════

fn parse_content_type(s: &str) -> Result<CmsContentType, (StatusCode, Json<JsonValue>)> {
    match s {
        "page" => Ok(CmsContentType::Page),
        "blog_post" | "blog-post" => Ok(CmsContentType::BlogPost),
        "alert_service" | "alert-service" => Ok(CmsContentType::AlertService),
        "trading_room" | "trading-room" => Ok(CmsContentType::TradingRoom),
        "indicator" => Ok(CmsContentType::Indicator),
        "course" => Ok(CmsContentType::Course),
        "lesson" => Ok(CmsContentType::Lesson),
        "testimonial" => Ok(CmsContentType::Testimonial),
        "faq" => Ok(CmsContentType::Faq),
        "author" => Ok(CmsContentType::Author),
        "topic_cluster" | "topic-cluster" => Ok(CmsContentType::TopicCluster),
        "weekly_watchlist" | "weekly-watchlist" => Ok(CmsContentType::WeeklyWatchlist),
        "resource" => Ok(CmsContentType::Resource),
        _ => Err(api_error(StatusCode::BAD_REQUEST, "Invalid content type")),
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Admin CMS v2 routes (requires authentication)
pub fn admin_router() -> Router<AppState> {
    Router::new()
        // Stats
        .route("/stats", get(get_cms_stats))
        // Asset Folders
        .route(
            "/folders",
            get(list_asset_folders).post(create_asset_folder),
        )
        .route(
            "/folders/:id",
            get(get_asset_folder)
                .put(update_asset_folder)
                .delete(delete_asset_folder),
        )
        // Assets
        .route("/assets", get(list_assets).post(create_asset))
        .route(
            "/assets/:id",
            get(get_asset).put(update_asset).delete(delete_asset),
        )
        // Content
        .route("/content", get(list_content).post(create_content))
        .route(
            "/content/:id",
            get(get_content).put(update_content).delete(delete_content),
        )
        .route("/content/:id/status", post(transition_content_status))
        .route("/content/:id/revisions", get(get_content_revisions))
        .route(
            "/content/:content_id/revisions/:revision_number/restore",
            post(restore_revision),
        )
        .route("/content/:content_id/tags", get(get_content_tags))
        .route(
            "/content/:content_id/tags/:tag_id",
            post(add_tag_to_content).delete(remove_tag_from_content),
        )
        .route(
            "/content/:content_id/comments",
            get(get_content_comments).post(create_comment),
        )
        .route("/comments/:comment_id/resolve", post(resolve_comment))
        // Tags
        .route("/tags", get(list_tags).post(create_tag))
        // Site Settings
        .route(
            "/settings",
            get(get_site_settings).put(update_site_settings),
        )
        // Navigation Menus
        .route(
            "/menus",
            get(list_navigation_menus).post(create_navigation_menu),
        )
        .route(
            "/menus/:id",
            get(get_navigation_menu).put(update_navigation_menu),
        )
        // Redirects
        .route("/redirects", get(list_redirects).post(create_redirect))
        .route("/redirects/:id", delete(delete_redirect))
}

/// Public CMS routes (no auth required)
pub fn public_router() -> Router<AppState> {
    Router::new()
        // Public content access by type and slug
        .route("/content/:content_type/:slug", get(get_content_by_slug))
        // Public site settings
        .route("/settings", get(get_site_settings))
        // Public navigation menus
        .route("/menus", get(list_navigation_menus))
        .route("/menus/:slug", get(get_navigation_menu))
}
