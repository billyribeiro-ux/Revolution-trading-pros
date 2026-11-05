//! CRM delegation uses current database identity and the shared committed policy.
use crate::{
    AppState,
    middleware::admin::is_admin_role,
    models::User,
    services::crm_logs,
    utils::errors::{self, ApiError},
};
use axum::{
    Json, Router,
    extract::{FromRequestParts, MatchedPath, Path, Query, State},
    http::{StatusCode, request::Parts},
    routing::get,
};
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::{PgPool, Postgres, Transaction};
use std::collections::BTreeMap;
#[path = "manager_policy.rs"]
mod policy;
use policy::Permission;

struct Grants {
    global: bool,
    permissions: Vec<Permission>,
    route: Option<&'static policy::Route>,
}
pub(crate) struct CrmUser(pub User, Grants);
fn forbidden(message: &str) -> ApiError {
    ApiError::new(StatusCode::FORBIDDEN, message)
}
fn db(error: sqlx::Error) -> ApiError {
    tracing::error!(%error,"CRM permission persistence unavailable");
    errors::service_unavailable("CRM permission records are unavailable")
}
fn policy_error(error: &policy::PolicyError) -> ApiError {
    match error.kind {
        policy::ErrorKind::Denied => forbidden("Your CRM role does not allow this action"),
        policy::ErrorKind::InvalidPolicy | policy::ErrorKind::InvalidFacts => {
            tracing::error!(%error,"CRM authorization policy could not be evaluated");
            errors::service_unavailable(
                "CRM authorization is unavailable; no action was authorized",
            )
        }
    }
}
fn current_policy() -> Result<&'static policy::CompiledPolicy, ApiError> {
    policy::current().map_err(policy_error)
}
async fn current_user(pool: &PgPool, id: i64) -> Result<User, ApiError> {
    sqlx::query_as("SELECT id,email,password_hash,name,role,is_active,email_verified_at,avatar_url,mfa_enabled,created_at,updated_at FROM users WHERE id=$1 AND deleted_at IS NULL AND COALESCE(is_active,true)")
        .bind(id).fetch_optional(pool).await.map_err(db)?.ok_or_else(||forbidden("An active account is required for CRM access"))
}
fn global(state: &AppState, user: &User) -> bool {
    is_admin_role(user.role.as_deref())
        || state.config.is_superadmin_email_strict(
            &user.email,
            user.role.as_deref(),
            user.email_verified_at.is_some(),
        )
}
fn security_administrator(state: &AppState, user: &User) -> Result<(), ApiError> {
    if global(state, user) {
        Ok(())
    } else {
        Err(forbidden(
            "A global administrator must manage CRM security roles and assignments",
        ))
    }
}
#[derive(sqlx::FromRow)]
struct Delegation {
    manager_id: i64,
    role_id: i64,
    role_version: i64,
    assignment_version: i64,
    permissions: Value,
}
async fn delegation(pool: &PgPool, user: i64) -> Result<Option<Delegation>, ApiError> {
    sqlx::query_as("SELECT m.id AS manager_id,m.role_id,r.version AS role_version,m.version AS assignment_version,r.permissions FROM crm_managers m JOIN crm_manager_roles r ON r.id=m.role_id WHERE m.user_id=$1")
        .bind(user).fetch_optional(pool).await.map_err(db)
}
fn parse_permissions(value: Value) -> Result<Vec<Permission>, ApiError> {
    let document = &current_policy()?.document;
    serde_json::from_value::<Vec<Permission>>(value)
        .ok()
        .filter(|permissions| document.valid_permissions(permissions))
        .ok_or_else(|| forbidden("The stored CRM role requires administrator review"))
}
impl FromRequestParts<AppState> for CrmUser {
    type Rejection = ApiError;
    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, ApiError> {
        let authenticated = User::from_request_parts(parts, state)
            .await
            .map_err(|(status, message)| ApiError::new(status, message))?;
        let user = current_user(&state.db.pool, authenticated.id).await?;
        let document = &current_policy()?.document;
        let selectors = Query::<policy::Selectors>::try_from_uri(&parts.uri)
            .map_err(|_| ApiError::validation_error("Provide unambiguous CRM query parameters"))?
            .0
            .0;
        let route = parts.extensions.get::<MatchedPath>().and_then(|path| {
            document.matched_route(parts.method.as_str(), path.as_str(), &selectors)
        });
        let is_global = global(state, &user);
        let permissions = if is_global {
            Vec::new()
        } else {
            let assigned = delegation(&state.db.pool, user.id)
                .await?
                .ok_or_else(|| forbidden("A CRM manager assignment is required"))?;
            parse_permissions(assigned.permissions)?
        };
        if let Some(route) = route {
            document
                .authorize_base(route, is_global, &permissions)
                .map_err(|error| policy_error(&error))?;
        } else if !is_global {
            return Err(forbidden("This route does not allow delegated CRM access"));
        }
        Ok(Self(
            user,
            Grants {
                global: is_global,
                permissions,
                route,
            },
        ))
    }
}
fn member_audience(value: &Value) -> bool {
    value["members"] == true || !value["member_segment"].is_null()
}
fn audience_reference(value: &Value, key: &str) -> bool {
    ["subscribers", "excluded_subscribers"].iter().any(|group| {
        value[group].as_array().is_some_and(|rows| {
            rows.iter()
                .any(|row| row[key].as_str().is_some_and(|value| !value.is_empty()))
        })
    })
}
fn nonempty_array(value: &Value) -> bool {
    value.as_array().is_some_and(|values| !values.is_empty())
}
impl CrmUser {
    pub(super) fn is_delegated(&self) -> bool {
        !self.1.global
    }
    fn check(
        &self,
        context: policy::Context,
        mut evaluate: impl FnMut(&str) -> Result<bool, ApiError>,
    ) -> Result<(), ApiError> {
        if self.1.global {
            return Ok(());
        }
        let Some(route) = self.1.route else {
            return if self.1.global {
                Ok(())
            } else {
                Err(errors::service_unavailable(
                    "CRM route authorization is unavailable",
                ))
            };
        };
        let document = &current_policy()?.document;
        let mut actual = BTreeMap::new();
        for identity in &route.conditional {
            let fact = document.fact(identity).ok_or_else(|| {
                errors::service_unavailable("CRM conditional authorization is unavailable")
            })?;
            if fact.context == context {
                actual.insert(identity.clone(), evaluate(identity)?);
            }
        }
        document
            .authorize_facts(route, context, &actual, self.1.global, &self.1.permissions)
            .map_err(|error| policy_error(&error))
    }
    fn unknown_fact() -> ApiError {
        errors::service_unavailable("CRM authorization fact is not implemented")
    }
    fn check_input(
        &self,
        context: policy::Context,
        value: &Value,
        mut evaluate: impl FnMut(&str) -> Result<bool, ApiError>,
    ) -> Result<(), ApiError> {
        self.check(context, |identity| {
            let document = &current_policy()?.document;
            let fact = document.fact(identity).ok_or_else(Self::unknown_fact)?;
            for field in &fact.source_fields {
                let top = field
                    .strip_prefix('/')
                    .and_then(|field| field.split('/').next())
                    .ok_or_else(Self::unknown_fact)?;
                if !value
                    .as_object()
                    .is_some_and(|object| object.contains_key(top))
                {
                    return Err(errors::service_unavailable(
                        "CRM authorization source facts are incomplete",
                    ));
                }
            }
            evaluate(identity)
        })
    }
    pub(crate) fn body(&self, value: &Value) -> Result<(), ApiError> {
        self.check_input(policy::Context::Body, value, |identity| {
            Ok(match identity {
                "body_native_template" | "body_crm_template" => !value["template_id"].is_null(),
                "body_contact_tags" => !value["tags"].is_null(),
                "body_owner_reference" => !value["owner_id"].is_null(),
                "body_sequence_execution_status" => !value["status"].is_null(),
                "body_member_audience" => member_audience(&value["subscriber_settings"]),
                "body_audience_lists" => audience_reference(&value["subscriber_settings"], "list"),
                "body_audience_tags" => audience_reference(&value["subscriber_settings"], "tag"),
                "body_audience_segment" => value["subscriber_settings"]["dynamic_segment"]
                    .as_str()
                    .is_some_and(|value| !value.is_empty()),
                "body_import_tags" => {
                    nonempty_array(&value["settings"]["apply_tags"])
                        || value["mapping"]
                            .as_object()
                            .is_some_and(|mapping| mapping.values().any(|field| field == "tags"))
                }
                "body_import_lists" => nonempty_array(&value["settings"]["add_to_lists"]),
                "body_export_lists" => !value["filters"]["list_id"].is_null(),
                "body_export_tags" => value["filters"]["tag"]
                    .as_str()
                    .is_some_and(|value| !value.is_empty()),
                _ => return Err(Self::unknown_fact()),
            })
        })
    }
    pub(crate) fn record(&self, value: &Value) -> Result<(), ApiError> {
        self.check_input(policy::Context::Record, value, |identity| {
            Ok(match identity {
                "record_template" => {
                    !value["template_id"].is_null() || !value["email_template_id"].is_null()
                }
                "record_member_audience" => member_audience(&value["subscriber_settings"]),
                "record_audience_lists" => {
                    audience_reference(&value["subscriber_settings"], "list")
                }
                "record_audience_tags" => audience_reference(&value["subscriber_settings"], "tag"),
                "record_audience_segment" => value["subscriber_settings"]["dynamic_segment"]
                    .as_str()
                    .is_some_and(|value| !value.is_empty()),
                "record_tag_rename" => value["old_title"] != value["new_title"],
                _ => return Err(Self::unknown_fact()),
            })
        })
    }
    pub(crate) fn history_member_source(&self, has_member_source: bool) -> Result<(), ApiError> {
        self.check(policy::Context::History, |identity| match identity {
            "history_member_audience" => Ok(has_member_source),
            _ => Err(Self::unknown_fact()),
        })
    }
    pub(crate) fn import_plan(
        &self,
        creates: bool,
        updates: bool,
        tags: bool,
        lists: bool,
    ) -> Result<(), ApiError> {
        self.check(policy::Context::ImportPlan, |identity| match identity {
            "import_plan_creates" => Ok(creates),
            "import_plan_updates" => Ok(updates),
            "import_plan_tags" => Ok(tags),
            "import_plan_lists" => Ok(lists),
            _ => Err(Self::unknown_fact()),
        })
    }
    pub(crate) fn export_record(&self, value: &Value) -> Result<(), ApiError> {
        self.check_input(
            policy::Context::ExportRecord,
            value,
            |identity| match identity {
                "export_record_export_lists" => Ok(!value["filters"]["list_id"].is_null()),
                "export_record_export_tags" => Ok(value["filters"]["tag"]
                    .as_str()
                    .is_some_and(|value| !value.is_empty())),
                _ => Err(Self::unknown_fact()),
            },
        )
    }
    pub(crate) fn needs_history(&self) -> Result<bool, ApiError> {
        let document = &current_policy()?.document;
        Ok(self.1.route.is_some_and(|route| {
            route.conditional.iter().any(|identity| {
                document
                    .fact(identity)
                    .is_some_and(|fact| fact.context == policy::Context::History)
            })
        }))
    }
}
async fn permissions(
    State(state): State<AppState>,
    authenticated: User,
) -> Result<Json<Value>, ApiError> {
    let user = current_user(&state.db.pool, authenticated.id).await?;
    let compiled = current_policy()?;
    let document = &compiled.document;
    let is_global = global(&state, &user);
    let assigned = delegation(&state.db.pool, user.id).await?;
    let declared = if is_global {
        None
    } else {
        assigned
            .as_ref()
            .map(|assigned| parse_permissions(assigned.permissions.clone()))
            .transpose()?
    };
    let mut effective = Vec::new();
    for module in &document.modules {
        for action in &document.actions {
            if is_global
                || declared
                    .as_ref()
                    .is_some_and(|permissions| document.allows(permissions, &module.id, *action))
            {
                effective.push(json!({"module":module.id,"action":action,"allowed":true}));
            }
        }
    }
    Ok(Json(
        json!({"user_id":user.id.to_string(),"global_admin":is_global,"manager_id":assigned.as_ref().map(|value|value.manager_id.to_string()),"role_id":assigned.as_ref().map(|value|value.role_id.to_string()),"role_version":assigned.as_ref().map(|value|value.role_version),"assignment_version":assigned.as_ref().map(|value|value.assignment_version),"permissions":effective,"policy_version":document.policy_version,"policy_hash":compiled.policy_hash,"security_management_requires_global_admin":true,"default_role_is_automatic":false,"metrics_scope":"current_crm_capabilities"}),
    ))
}

#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct Filters {
    page: Option<i64>,
    per_page: Option<i64>,
    search: Option<String>,
}
impl Filters {
    fn bounds(&self) -> Result<(i64, i64, i64, Option<String>), ApiError> {
        let page = self.page.unwrap_or(1);
        let size = self.per_page.unwrap_or(50);
        if !(1..=100_000).contains(&page)
            || !(1..=100).contains(&size)
            || self
                .search
                .as_ref()
                .is_some_and(|text| text.len() > 500 || text.contains('\0'))
        {
            return Err(ApiError::validation_error(
                "Provide bounded manager filters",
            ));
        }
        Ok((
            page,
            size,
            (page - 1) * size,
            self.search
                .as_ref()
                .map(|search| format!("%{}%", crate::utils::escape_like(search))),
        ))
    }
}
fn envelope(rows: Vec<Value>, page: i64, size: i64, total: i64) -> Json<Value> {
    Json(
        json!({"data":rows,"meta":{"current_page":page,"per_page":size,"total":total,"last_page":((total+size-1)/size).max(1)}}),
    )
}
macro_rules! role_sql {($tail:literal)=>{concat!("SELECT jsonb_build_object('id',r.id::text,'name',r.name,'slug',r.slug,'description',r.description,'permissions',r.permissions,'users_count',(SELECT COUNT(*) FROM crm_managers m WHERE m.role_id=r.id),'is_default',r.is_default,'version',r.version,'created_at',r.created_at,'updated_at',r.updated_at) FROM crm_manager_roles r ",$tail)}}
async fn role_on(connection: &mut sqlx::PgConnection, id: i64) -> Result<Value, ApiError> {
    sqlx::query_scalar(role_sql!("WHERE r.id=$1"))
        .bind(id)
        .fetch_optional(connection)
        .await
        .map_err(db)?
        .ok_or_else(|| errors::not_found("CRM role"))
}
async fn roles(
    State(state): State<AppState>,
    _: CrmUser,
    Query(filters): Query<Filters>,
) -> Result<Json<Value>, ApiError> {
    let (page, size, offset, search) = filters.bounds()?;
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *tx)
        .await
        .map_err(db)?;
    let total:i64=sqlx::query_scalar("SELECT COUNT(*) FROM crm_manager_roles WHERE $1::text IS NULL OR name ILIKE $1 OR description ILIKE $1").bind(&search).fetch_one(&mut *tx).await.map_err(db)?;
    let rows=sqlx::query_scalar(role_sql!("WHERE $1::text IS NULL OR r.name ILIKE $1 OR r.description ILIKE $1 ORDER BY r.is_default DESC,r.id LIMIT $2 OFFSET $3"))
        .bind(search).bind(size).bind(offset).fetch_all(&mut *tx).await.map_err(db)?;
    tx.commit().await.map_err(db)?;
    Ok(envelope(rows, page, size, total))
}
async fn role(
    State(state): State<AppState>,
    _: CrmUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    Ok(Json(
        role_on(&mut *state.db.pool.acquire().await.map_err(db)?, id).await?,
    ))
}
async fn modules(_: CrmUser) -> Result<Json<Value>, ApiError> {
    let compiled = current_policy()?;
    let document = &compiled.document;
    Ok(Json(json!(document.modules.iter().map(|module|json!({"id":module.id,"name":module.name,"actions":document.actions,"manage_implies":document.manage_implies,"notes":module.notes,"policy_version":document.policy_version,"policy_hash":compiled.policy_hash})).collect::<Vec<_>>())))
}
fn patch<'de, D, T>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    Option::<T>::deserialize(deserializer).map(Some)
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RoleInput {
    name: Option<String>,
    slug: Option<String>,
    #[serde(default, deserialize_with = "patch")]
    description: Option<Option<String>>,
    permissions: Option<Vec<Permission>>,
    is_default: Option<bool>,
    expected_version: Option<i64>,
}
impl RoleInput {
    fn validate(&self, create: bool) -> Result<(), ApiError> {
        let document = &current_policy()?.document;
        if (create && (self.name.is_none() || self.permissions.is_none()))
            || self.name.as_ref().is_some_and(|text| {
                text.trim().is_empty()
                    || text.chars().count() > 100
                    || text.chars().any(char::is_control)
            })
            || self
                .description
                .as_ref()
                .and_then(Option::as_ref)
                .is_some_and(|text| text.chars().count() > 2000 || text.contains('\0'))
            || self
                .permissions
                .as_ref()
                .is_some_and(|permissions| !document.valid_permissions(permissions))
            || self.slug.as_ref().is_some_and(|slug| !valid_slug(slug))
            || (!create && self.expected_version.is_none_or(|version| version <= 0))
            || (create && self.expected_version.is_some())
        {
            return Err(ApiError::validation_error(
                "Provide a bounded name, distinct supported permissions, valid slug and current version",
            ));
        }
        Ok(())
    }
}
fn valid_slug(slug: &str) -> bool {
    !slug.is_empty()
        && slug.len() <= 100
        && !slug.starts_with('-')
        && !slug.ends_with('-')
        && !slug.contains("--")
        && slug
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}
fn slug(name: &str) -> String {
    name.to_ascii_lowercase()
        .split(|character: char| !character.is_ascii_alphanumeric())
        .filter(|piece| !piece.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}
async fn security_lock(tx: &mut Transaction<'_, Postgres>) -> Result<(), ApiError> {
    sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended('crm-manager-security',0))")
        .execute(&mut **tx)
        .await
        .map_err(db)?;
    Ok(())
}
async fn clear_default(
    tx: &mut Transaction<'_, Postgres>,
    except: Option<i64>,
) -> Result<Vec<i64>, ApiError> {
    sqlx::query_scalar("UPDATE crm_manager_roles SET is_default=false,version=version+1,updated_at=clock_timestamp() WHERE is_default AND ($1::bigint IS NULL OR id<>$1) RETURNING id")
        .bind(except).fetch_all(&mut **tx).await.map_err(db)
}
fn write_error(error: sqlx::Error) -> ApiError {
    if error
        .as_database_error()
        .is_some_and(|error| error.is_unique_violation())
    {
        errors::conflict("A CRM role or assignment already uses that identity")
    } else {
        db(error)
    }
}
async fn create_role(
    State(state): State<AppState>,
    actor: CrmUser,
    Json(input): Json<RoleInput>,
) -> Result<Json<Value>, ApiError> {
    security_administrator(&state, &actor.0)?;
    input.validate(true)?;
    let name = input.name.as_deref().unwrap().trim();
    let slug = input.slug.clone().unwrap_or_else(|| slug(name));
    if !valid_slug(&slug) {
        return Err(ApiError::validation_error(
            "Provide an explicit lowercase ASCII role slug",
        ));
    }
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    security_lock(&mut tx).await?;
    let defaults = if input.is_default == Some(true) {
        clear_default(&mut tx, None).await?
    } else {
        Vec::new()
    };
    let id:i64=sqlx::query_scalar("INSERT INTO crm_manager_roles(name,slug,description,permissions,is_default,created_by) VALUES($1,$2,$3,$4,$5,$6) RETURNING id")
        .bind(name).bind(slug).bind(input.description.flatten()).bind(json!(input.permissions.unwrap())).bind(input.is_default.unwrap_or(false)).bind(actor.0.id).fetch_one(&mut *tx).await.map_err(write_error)?;
    let role = role_on(&mut tx, id).await?;
    crm_logs::record(
        &mut tx,
        "info",
        "api",
        "CRM security role created",
        json!({"role_id":id,"role_snapshot":role,"replaced_default_role_ids":defaults}),
        Some(actor.0.id),
    )
    .await
    .map_err(db)?;
    tx.commit().await.map_err(db)?;
    Ok(Json(role))
}
async fn update_role(
    State(state): State<AppState>,
    actor: CrmUser,
    Path(id): Path<i64>,
    Json(input): Json<RoleInput>,
) -> Result<Json<Value>, ApiError> {
    security_administrator(&state, &actor.0)?;
    input.validate(false)?;
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    security_lock(&mut tx).await?;
    let old: Value =
        sqlx::query_scalar("SELECT to_jsonb(r) FROM crm_manager_roles r WHERE id=$1 FOR UPDATE")
            .bind(id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(db)?
            .ok_or_else(|| errors::not_found("CRM role"))?;
    if old["version"].as_i64() != input.expected_version {
        return Err(errors::conflict(
            "CRM role changed; reload before saving permissions",
        ));
    }
    if input.is_default == Some(false) && old["is_default"] == true {
        return Err(errors::conflict(
            "Choose another default role before clearing the current default",
        ));
    }
    let defaults = if input.is_default == Some(true) {
        clear_default(&mut tx, Some(id)).await?
    } else {
        Vec::new()
    };
    sqlx::query("UPDATE crm_manager_roles SET name=COALESCE($2,name),slug=COALESCE($3,slug),description=CASE WHEN $4 THEN $5::text ELSE description END,permissions=COALESCE($6,permissions),is_default=COALESCE($7,is_default),version=version+1,updated_at=clock_timestamp() WHERE id=$1")
        .bind(id).bind(input.name.as_deref().map(str::trim)).bind(input.slug).bind(input.description.is_some()).bind(input.description.flatten()).bind(input.permissions.map(|permissions|json!(permissions))).bind(input.is_default).execute(&mut *tx).await.map_err(write_error)?;
    let role = role_on(&mut tx, id).await?;
    crm_logs::record(&mut tx,"info","api","CRM security role updated",json!({"role_id":id,"previous_version":old["version"],"previous_role":old,"role_snapshot":role,"replaced_default_role_ids":defaults}),Some(actor.0.id)).await.map_err(db)?;
    tx.commit().await.map_err(db)?;
    Ok(Json(role))
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Version {
    expected_version: i64,
}
async fn delete_role(
    State(state): State<AppState>,
    actor: CrmUser,
    Path(id): Path<i64>,
    Query(version): Query<Version>,
) -> Result<Json<Value>, ApiError> {
    security_administrator(&state, &actor.0)?;
    if version.expected_version <= 0 {
        return Err(ApiError::validation_error(
            "Provide the current role version",
        ));
    }
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    security_lock(&mut tx).await?;
    let old: Value =
        sqlx::query_scalar("SELECT to_jsonb(r) FROM crm_manager_roles r WHERE id=$1 FOR UPDATE")
            .bind(id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(db)?
            .ok_or_else(|| errors::not_found("CRM role"))?;
    if old["version"].as_i64() != Some(version.expected_version) {
        return Err(errors::conflict("CRM role changed; reload before deleting"));
    }
    let assigned: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM crm_managers WHERE role_id=$1)")
            .bind(id)
            .fetch_one(&mut *tx)
            .await
            .map_err(db)?;
    if old["is_default"] == true || assigned {
        return Err(errors::conflict(
            "The current default or an assigned CRM role cannot be deleted",
        ));
    }
    sqlx::query("DELETE FROM crm_manager_roles WHERE id=$1")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(db)?;
    crm_logs::record(
        &mut tx,
        "info",
        "api",
        "CRM security role deleted",
        json!({"role_id":id,"previous_version":version.expected_version,"previous_role":old}),
        Some(actor.0.id),
    )
    .await
    .map_err(db)?;
    tx.commit().await.map_err(db)?;
    Ok(Json(
        json!({"deleted":true,"id":id.to_string(),"version":version.expected_version}),
    ))
}

pub(super) fn router() -> Router<AppState> {
    Router::new()
        .route("/contacts/native-templates", get(contact_templates))
        .route("/permissions", get(permissions))
        .route("/manager-roles/modules", get(modules))
        .route("/manager-roles", get(roles).post(create_role))
        .route(
            "/manager-roles/{id}",
            get(role).put(update_role).delete(delete_role),
        )
        .route("/managers/users", get(users))
        .route("/managers", get(managers).post(add_manager))
        .route(
            "/managers/{id}",
            get(manager).put(update_manager).delete(remove_manager),
        )
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Picker {
    page: Option<i64>,
    per_page: Option<i64>,
    search: Option<String>,
}
async fn contact_templates(
    State(state): State<AppState>,
    _: CrmUser,
    Query(query): Query<Picker>,
) -> Result<Json<Value>, ApiError> {
    let page = query.page.unwrap_or(1);
    let size = query.per_page.unwrap_or(25);
    if !(1..=10_000).contains(&page)
        || !(1..=100).contains(&size)
        || query
            .search
            .as_ref()
            .is_some_and(|value| value.len() > 500 || value.contains('\0'))
    {
        return Err(ApiError::validation_error(
            "Provide bounded template search and pagination",
        ));
    }
    let result = crate::routes::email_templates::read_catalog(
        &state.db.pool,
        &crate::routes::email_templates::TemplateListQuery {
            page: Some(page),
            per_page: Some(size),
            search: query.search,
            category: None,
        },
        true,
    )
    .await
    .map_err(|(status, Json(value))| {
        if status.is_server_error() {
            errors::service_unavailable("Contact template picker is unavailable")
        } else {
            ApiError::new(
                status,
                value["error"]
                    .as_str()
                    .unwrap_or("Template picker request is invalid"),
            )
        }
    })?;
    let rows=result.rows.into_iter().map(|template|json!({"id":template.id.to_string(),"name":template.name,"subject":template.subject,"body_html":template.body_html,"body_text":template.body_text})).collect::<Vec<_>>();
    Ok(Json(
        json!({"metrics_scope":"contact_template_picker","data":rows,"meta":{"current_page":result.page,"per_page":result.per_page,"total":result.total,"last_page":((result.total+result.per_page-1)/result.per_page).max(1)}}),
    ))
}

macro_rules! manager_sql {($tail:literal)=>{concat!("SELECT jsonb_build_object('id',m.id::text,'user_id',m.user_id::text,'role_id',m.role_id::text,'name',u.name,'email',u.email,'avatar',u.avatar_url,'assigned_at',m.assigned_at,'version',m.version,'is_active',COALESCE(u.is_active,true) AND u.deleted_at IS NULL,'global_role',u.role,'role',jsonb_build_object('id',r.id::text,'name',r.name,'slug',r.slug,'description',r.description,'permissions',r.permissions,'users_count',(SELECT COUNT(*) FROM crm_managers others WHERE others.role_id=r.id),'is_default',r.is_default,'version',r.version,'created_at',r.created_at,'updated_at',r.updated_at)) FROM crm_managers m JOIN users u ON u.id=m.user_id JOIN crm_manager_roles r ON r.id=m.role_id ",$tail)}}
async fn manager_on(connection: &mut sqlx::PgConnection, id: i64) -> Result<Value, ApiError> {
    sqlx::query_scalar(manager_sql!("WHERE m.id=$1"))
        .bind(id)
        .fetch_optional(connection)
        .await
        .map_err(db)?
        .ok_or_else(|| errors::not_found("CRM manager assignment"))
}
async fn managers(
    State(state): State<AppState>,
    _: CrmUser,
    Query(filters): Query<Filters>,
) -> Result<Json<Value>, ApiError> {
    let (page, size, offset, search) = filters.bounds()?;
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *tx)
        .await
        .map_err(db)?;
    let total:i64=sqlx::query_scalar("SELECT COUNT(*) FROM crm_managers m JOIN users u ON u.id=m.user_id JOIN crm_manager_roles r ON r.id=m.role_id WHERE $1::text IS NULL OR u.name ILIKE $1 OR u.email ILIKE $1 OR r.name ILIKE $1").bind(&search).fetch_one(&mut *tx).await.map_err(db)?;
    let rows=sqlx::query_scalar(manager_sql!("WHERE $1::text IS NULL OR u.name ILIKE $1 OR u.email ILIKE $1 OR r.name ILIKE $1 ORDER BY m.id LIMIT $2 OFFSET $3"))
        .bind(search).bind(size).bind(offset).fetch_all(&mut *tx).await.map_err(db)?;
    tx.commit().await.map_err(db)?;
    Ok(envelope(rows, page, size, total))
}
async fn manager(
    State(state): State<AppState>,
    _: CrmUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    Ok(Json(
        manager_on(&mut *state.db.pool.acquire().await.map_err(db)?, id).await?,
    ))
}
async fn users(
    State(state): State<AppState>,
    actor: CrmUser,
    Query(filters): Query<Filters>,
) -> Result<Json<Value>, ApiError> {
    security_administrator(&state, &actor.0)?;
    let (page, size, offset, search) = filters.bounds()?;
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *tx)
        .await
        .map_err(db)?;
    let total:i64=sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE deleted_at IS NULL AND COALESCE(is_active,true) AND ($1::text IS NULL OR name ILIKE $1 OR email ILIKE $1)")
        .bind(&search).fetch_one(&mut *tx).await.map_err(db)?;
    let rows=sqlx::query_scalar("SELECT jsonb_build_object('id',u.id::text,'name',u.name,'email',u.email,'avatar',u.avatar_url,'global_role',u.role,'manager_id',m.id::text,'role_id',m.role_id::text) FROM users u LEFT JOIN crm_managers m ON m.user_id=u.id WHERE u.deleted_at IS NULL AND COALESCE(u.is_active,true) AND ($1::text IS NULL OR u.name ILIKE $1 OR u.email ILIKE $1) ORDER BY u.id LIMIT $2 OFFSET $3")
        .bind(search).bind(size).bind(offset).fetch_all(&mut *tx).await.map_err(db)?;
    tx.commit().await.map_err(db)?;
    Ok(envelope(rows, page, size, total))
}
#[derive(Deserialize)]
#[serde(untagged)]
enum Id {
    Number(i64),
    Text(String),
}
impl Id {
    fn value(&self) -> Result<i64, ApiError> {
        let number = match self {
            Self::Number(number) => Some(*number),
            Self::Text(text) if text.bytes().all(|byte| byte.is_ascii_digit()) => text.parse().ok(),
            _ => None,
        };
        number.filter(|number| *number > 0).ok_or_else(|| {
            ApiError::validation_error("Choose a positive user and CRM role identity")
        })
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct NewManager {
    user_id: Id,
    role_id: Id,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct UpdateManager {
    role_id: Id,
    expected_version: i64,
}
async fn eligible(
    tx: &mut Transaction<'_, Postgres>,
    user: i64,
    role: i64,
) -> Result<(), ApiError> {
    let user=sqlx::query_scalar::<_,i64>("SELECT id FROM users WHERE id=$1 AND deleted_at IS NULL AND COALESCE(is_active,true) FOR KEY SHARE").bind(user).fetch_optional(&mut **tx).await.map_err(db)?;
    if user.is_none() {
        return Err(ApiError::validation_error("Choose an active existing user"));
    }
    let role =
        sqlx::query_scalar::<_, i64>("SELECT id FROM crm_manager_roles WHERE id=$1 FOR KEY SHARE")
            .bind(role)
            .fetch_optional(&mut **tx)
            .await
            .map_err(db)?;
    if role.is_none() {
        return Err(ApiError::validation_error("Choose an existing CRM role"));
    }
    Ok(())
}
async fn add_manager(
    State(state): State<AppState>,
    actor: CrmUser,
    Json(input): Json<NewManager>,
) -> Result<Json<Value>, ApiError> {
    security_administrator(&state, &actor.0)?;
    let user = input.user_id.value()?;
    let role = input.role_id.value()?;
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    security_lock(&mut tx).await?;
    eligible(&mut tx, user, role).await?;
    let id: i64 = sqlx::query_scalar(
        "INSERT INTO crm_managers(user_id,role_id,assigned_by) VALUES($1,$2,$3) RETURNING id",
    )
    .bind(user)
    .bind(role)
    .bind(actor.0.id)
    .fetch_one(&mut *tx)
    .await
    .map_err(write_error)?;
    let manager = manager_on(&mut tx, id).await?;
    crm_logs::record(
        &mut tx,
        "info",
        "api",
        "CRM manager assigned",
        json!({"manager_id":id,"user_id":user,"role_id":role,"assignment_snapshot":manager}),
        Some(actor.0.id),
    )
    .await
    .map_err(db)?;
    tx.commit().await.map_err(db)?;
    Ok(Json(manager))
}
async fn update_manager(
    State(state): State<AppState>,
    actor: CrmUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdateManager>,
) -> Result<Json<Value>, ApiError> {
    security_administrator(&state, &actor.0)?;
    let role = input.role_id.value()?;
    if input.expected_version <= 0 {
        return Err(ApiError::validation_error(
            "Provide the current assignment version",
        ));
    }
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    security_lock(&mut tx).await?;
    let old: (i64, i64, i64) =
        sqlx::query_as("SELECT user_id,role_id,version FROM crm_managers WHERE id=$1 FOR UPDATE")
            .bind(id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(db)?
            .ok_or_else(|| errors::not_found("CRM manager assignment"))?;
    if old.2 != input.expected_version {
        return Err(errors::conflict(
            "CRM assignment changed; reload before saving",
        ));
    }
    eligible(&mut tx, old.0, role).await?;
    sqlx::query("UPDATE crm_managers SET role_id=$2,assigned_by=$3,version=version+1,updated_at=clock_timestamp() WHERE id=$1")
        .bind(id).bind(role).bind(actor.0.id).execute(&mut *tx).await.map_err(db)?;
    let manager = manager_on(&mut tx, id).await?;
    crm_logs::record(&mut tx,"info","api","CRM manager role updated",json!({"manager_id":id,"user_id":old.0,"previous_role_id":old.1,"role_id":role,"previous_version":old.2,"assignment_snapshot":manager}),Some(actor.0.id)).await.map_err(db)?;
    tx.commit().await.map_err(db)?;
    Ok(Json(manager))
}
async fn remove_manager(
    State(state): State<AppState>,
    actor: CrmUser,
    Path(id): Path<i64>,
    Query(version): Query<Version>,
) -> Result<Json<Value>, ApiError> {
    security_administrator(&state, &actor.0)?;
    if version.expected_version <= 0 {
        return Err(ApiError::validation_error(
            "Provide the current assignment version",
        ));
    }
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    security_lock(&mut tx).await?;
    let old: (i64, i64, i64) =
        sqlx::query_as("SELECT user_id,role_id,version FROM crm_managers WHERE id=$1 FOR UPDATE")
            .bind(id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(db)?
            .ok_or_else(|| errors::not_found("CRM manager assignment"))?;
    if old.2 != version.expected_version {
        return Err(errors::conflict(
            "CRM assignment changed; reload before removing",
        ));
    }
    let previous = manager_on(&mut tx, id).await?;
    sqlx::query("DELETE FROM crm_managers WHERE id=$1")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(db)?;
    crm_logs::record(
        &mut tx,
        "info",
        "api",
        "CRM manager removed",
        json!({"manager_id":id,"user_id":old.0,"previous_role_id":old.1,"previous_version":old.2,"previous_assignment":previous}),
        Some(actor.0.id),
    )
    .await
    .map_err(db)?;
    tx.commit().await.map_err(db)?;
    Ok(Json(
        json!({"deleted":true,"id":id.to_string(),"user_id":old.0.to_string(),"version":old.2}),
    ))
}

#[cfg(test)]
mod authorization_facts_tests {
    use super::*;

    fn actor(method: &str, path: &str, permissions: &[(&str, &str)]) -> CrmUser {
        let time =
            chrono::NaiveDateTime::parse_from_str("2026-11-05 00:00:00", "%Y-%m-%d %H:%M:%S")
                .unwrap();
        let route = current_policy()
            .unwrap()
            .document
            .matched_route(method, &format!("/api/admin/crm{path}"), &BTreeMap::new())
            .unwrap();
        CrmUser(
            User {
                id: 17,
                email: "manager@example.test".into(),
                password_hash: String::new(),
                name: "Fixture Manager".into(),
                role: Some("user".into()),
                is_active: Some(true),
                email_verified_at: None,
                avatar_url: None,
                mfa_enabled: None,
                created_at: time,
                updated_at: time,
            },
            Grants {
                global: false,
                route: Some(route),
                permissions: permissions
                    .iter()
                    .map(|(module, action)| Permission {
                        module: (*module).into(),
                        action: (*action).into(),
                        allowed: true,
                    })
                    .collect(),
            },
        )
    }

    fn status(result: Result<(), ApiError>, expected: &str) {
        assert_eq!(result.unwrap_err().status, expected);
    }

    #[test]
    fn personal_custom_composition_does_not_grant_template_resolution() {
        let path = "/contacts/{id}/send-email";
        let sender = actor("POST", path, &[("contacts", "manage")]);
        sender.body(&json!({"template_id":null})).unwrap();
        status(sender.body(&json!({"template_id":"4"})), "403");
        status(sender.body(&json!({})), "503");
        let reader = actor(
            "POST",
            path,
            &[("contacts", "manage"), ("templates", "view")],
        );
        reader.body(&json!({"template_id":"4"})).unwrap();
        let mut disabled = reader;
        disabled.1.permissions[1].allowed = false;
        status(disabled.body(&json!({"template_id":"4"})), "403");
    }

    #[test]
    fn clearing_contact_tags_and_linking_an_owner_are_real_conditional_actions() {
        let editor = actor("PUT", "/contacts/{id}", &[("contacts", "edit")]);
        editor.body(&json!({"tags":null,"owner_id":null})).unwrap();
        status(editor.body(&json!({"tags":[],"owner_id":null})), "403");
        status(editor.body(&json!({"tags":null,"owner_id":"17"})), "403");
        let allowed = actor(
            "PUT",
            "/contacts/{id}",
            &[("contacts", "edit"), ("tags", "edit"), ("managers", "view")],
        );
        allowed.body(&json!({"tags":[],"owner_id":"17"})).unwrap();
        status(allowed.body(&json!({"tags":[]})), "503");
    }

    #[test]
    fn any_supplied_sequence_status_needs_execution_management() {
        let editor = actor("PUT", "/sequences/{id}", &[("sequences", "edit")]);
        editor
            .body(&json!({"status":null,"subscriber_settings":null}))
            .unwrap();
        for selected in ["draft", "active", "paused"] {
            status(
                editor.body(&json!({"status":selected,"subscriber_settings":null})),
                "403",
            );
        }
        let executor = actor("PUT", "/sequences/{id}", &[("sequences", "manage")]);
        executor
            .body(&json!({"status":"paused","subscriber_settings":null}))
            .unwrap();
    }

    #[test]
    fn explicit_member_directory_source_and_immutable_history_stay_global_only() {
        let editor = actor("PUT", "/sequences/{id}", &[("sequences", "manage")]);
        for audience in [json!({"members":true}), json!({"member_segment":"paying"})] {
            status(
                editor.body(&json!({"status":null,"subscriber_settings":audience})),
                "403",
            );
        }
        editor
            .record(&json!({"subscriber_settings":{"members":false,"member_segment":null}}))
            .unwrap();
        editor.history_member_source(false).unwrap();
        status(editor.history_member_source(true), "403");
    }

    #[test]
    fn current_audience_references_need_each_explicit_catalog_grant() {
        let selected = json!({"subscriber_settings":{
            "subscribers":[{"list":"7"}],"excluded_subscribers":[{"tag":"8"}],"dynamic_segment":"9"
        }});
        let reader = actor(
            "GET",
            "/sequences/{id}",
            &[("sequences", "view"), ("contacts", "view")],
        );
        status(reader.record(&selected), "403");
        let allowed = actor(
            "GET",
            "/sequences/{id}",
            &[("lists", "view"), ("tags", "view"), ("segments", "view")],
        );
        allowed.record(&selected).unwrap();
        status(allowed.record(&json!({})), "503");
    }

    #[test]
    fn normalized_import_plan_does_not_borrow_contact_or_catalog_authority() {
        let importer = actor("POST", "/import", &[("imports", "manage")]);
        importer.import_plan(false, false, false, false).unwrap();
        for plan in [
            (true, false, false, false),
            (false, true, false, false),
            (false, false, true, false),
            (false, false, false, true),
        ] {
            status(importer.import_plan(plan.0, plan.1, plan.2, plan.3), "403");
        }
        let allowed = actor(
            "POST",
            "/import",
            &[
                ("contacts", "create"),
                ("contacts", "edit"),
                ("tags", "edit"),
                ("lists", "edit"),
            ],
        );
        allowed.import_plan(true, true, true, true).unwrap();
        status(
            importer.body(&json!({"settings":{},"mapping":{"Column A":"tags"}})),
            "403",
        );
        status(
            importer.body(&json!({"settings":{"add_to_lists":["7"]},"mapping":{}})),
            "403",
        );
        importer.body(&json!({"settings":{},"mapping":{}})).unwrap();
    }

    #[test]
    fn saved_export_download_rechecks_actual_frozen_list_and_tag_references() {
        let exporter = actor(
            "GET",
            "/exports/{id}/download",
            &[("exports", "manage"), ("contacts", "view")],
        );
        exporter.export_record(&json!({"filters":{}})).unwrap();
        status(
            exporter.export_record(&json!({"filters":{"list_id":"7"}})),
            "403",
        );
        status(
            exporter.export_record(&json!({"filters":{"tag":"retained"}})),
            "403",
        );
        status(exporter.export_record(&json!({})), "503");
        let reader = actor(
            "GET",
            "/exports/{id}/download",
            &[("lists", "view"), ("tags", "view")],
        );
        reader
            .export_record(&json!({"filters":{"list_id":"7","tag":"retained"}}))
            .unwrap();
    }

    #[test]
    fn genuine_global_staff_keep_existing_repair_rights_without_synthetic_facts() {
        let mut staff = actor("PUT", "/sequences/{id}", &[]);
        staff.1.global = true;
        staff.body(&json!({})).unwrap();
        staff.record(&json!({})).unwrap();
        staff.history_member_source(true).unwrap();
    }
}
