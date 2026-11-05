//! Delegation grants only recorded CRM capabilities and revokes on the next request.
use super::{Adapter, request};
use axum::{Router, http::StatusCode};
use serde_json::{Value, json};
use sqlx::PgPool;
use uuid::Uuid;
const ROOT: &str = "/api/admin/crm";
fn grant(module: &str, action: &str) -> Value {
    json!({"module":module,"action":action,"allowed":true})
}
async fn update_role(api: &Router, token: &str, role: &mut Value, permissions: Value) {
    let response = request(
        api,
        "PUT",
        &format!("{ROOT}/manager-roles/{}", role["id"].as_str().unwrap()),
        Some(token),
        json!({"expected_version":role["version"],"permissions":permissions}),
    )
    .await;
    assert_eq!(response.0, StatusCode::OK, "{response:?}");
    *role = response.1;
}
pub async fn proof(api: &Router, pool: &PgPool, tokens: &[String], adapter: &Adapter) {
    let admin = Some(tokens[0].as_str());
    let delegated = Some(tokens[1].as_str());
    let capabilities = format!("{ROOT}/permissions");
    assert_eq!(
        request(api, "GET", &capabilities, None, Value::Null)
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );
    let empty = request(api, "GET", &capabilities, delegated, Value::Null).await;
    assert_eq!(empty.0, StatusCode::OK, "{empty:?}");
    assert_eq!(empty.1["permissions"], json!([]));
    assert_eq!(empty.1["global_admin"], false);
    let global = request(api, "GET", &capabilities, admin, Value::Null).await;
    assert_eq!(global.0, StatusCode::OK, "{global:?}");
    assert_eq!(global.1["global_admin"], true);
    assert_eq!(global.1["policy_version"], 1);
    assert_eq!(global.1["policy_hash"], empty.1["policy_hash"]);
    assert_eq!(global.1["policy_hash"].as_str().unwrap().len(), 64);
    assert_eq!(global.1["permissions"].as_array().unwrap().len(), 100);
    let modules = request(
        api,
        "GET",
        &format!("{ROOT}/manager-roles/modules"),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(modules.0, StatusCode::OK, "{modules:?}");
    assert_eq!(modules.1.as_array().unwrap().len(), 20);
    let catalog = request(
        api,
        "GET",
        &format!("{ROOT}/manager-roles?per_page=100"),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(catalog.0, StatusCode::OK, "{catalog:?}");
    let original_default = catalog.1["data"]
        .as_array()
        .unwrap()
        .iter()
        .find(|role| role["is_default"] == true)
        .unwrap()
        .clone();
    assert_eq!(original_default["permissions"], json!([]));
    for permissions in [
        json!([grant("*", "manage")]),
        json!([grant("contacts", "send")]),
        json!([grant("contacts", "view"), grant("contacts", "view")]),
    ] {
        let rejected = request(
            api,
            "POST",
            &format!("{ROOT}/manager-roles"),
            admin,
            json!({"name":"Invalid permission fixture","permissions":permissions}),
        )
        .await;
        assert_eq!(rejected.0, StatusCode::BAD_REQUEST, "{rejected:?}");
    }
    let structural = request(
        api,
        "POST",
        &format!("{ROOT}/manager-roles"),
        admin,
        json!({"name":"Unknown policy field","permissions":[],"global_role":"admin"}),
    )
    .await;
    assert_eq!(
        structural.0,
        StatusCode::UNPROCESSABLE_ENTITY,
        "{structural:?}"
    );
    let saved=request(api,"POST",&format!("{ROOT}/manager-roles"),admin,json!({"name":"Scoped CRM proof","slug":"scoped-crm-proof","permissions":[grant("contacts","view")]})).await;
    assert_eq!(saved.0, StatusCode::OK, "{saved:?}");
    let mut role = saved.1;
    let count_before: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM crm_manager_roles")
        .fetch_one(pool)
        .await
        .unwrap();
    for body in [
        json!({"name":" scoped CRM PROOF ","slug":"another-manager-proof-slug","permissions":[]}),
        json!({"name":"Different manager proof name","slug":"scoped-crm-proof","permissions":[]}),
    ] {
        let duplicate = request(api, "POST", &format!("{ROOT}/manager-roles"), admin, body).await;
        assert_eq!(
            duplicate.0,
            StatusCode::CONFLICT,
            "Case-insensitive names and canonical slugs are actual uniqueness boundaries: {duplicate:?}"
        );
    }
    let count_after: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM crm_manager_roles")
        .fetch_one(pool)
        .await
        .unwrap();
    assert_eq!(count_after, count_before);
    let role_id = role["id"].as_str().unwrap().to_string();
    let user: i64 =
        sqlx::query_scalar("SELECT id FROM users WHERE email='user@email-completion.test'")
            .fetch_one(pool)
            .await
            .unwrap();
    let assignment = request(
        api,
        "POST",
        &format!("{ROOT}/managers"),
        admin,
        json!({"user_id":user.to_string(),"role_id":role_id}),
    )
    .await;
    assert_eq!(assignment.0, StatusCode::OK, "{assignment:?}");
    let mut assignment = assignment.1;
    let manager_id = assignment["id"].as_str().unwrap().to_string();
    assert_eq!(assignment["role_id"], role_id);
    assert_eq!(assignment["version"], 1);
    assert_eq!(assignment["role"]["users_count"], 1);
    assert_eq!(
        request(
            api,
            "POST",
            &format!("{ROOT}/managers"),
            admin,
            json!({"user_id":user,"role_id":role_id})
        )
        .await
        .0,
        StatusCode::CONFLICT
    );
    let contact=request(api,"POST",&format!("{ROOT}/contacts"),admin,json!({"email":"manager-record@example.test","first_name":"Actual CRM delegation","status":"customer"})).await;
    assert_eq!(contact.0, StatusCode::OK, "{contact:?}");
    let contact = contact.1["id"].as_i64().unwrap();
    let contact_path = format!("{ROOT}/contacts/{contact}");
    assert_eq!(
        request(
            api,
            "GET",
            &format!("{ROOT}/contacts"),
            delegated,
            Value::Null
        )
        .await
        .0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        request(api, "GET", &format!("{ROOT}/leads"), delegated, Value::Null)
            .await
            .0,
        StatusCode::FORBIDDEN
    );
    for endpoint in [
        contact_path.clone(),
        format!("{contact_path}/timeline"),
        format!("{ROOT}/campaigns"),
        format!("{ROOT}/logs"),
        format!("{ROOT}/settings"),
        format!("/api/admin/member-management/{user}"),
        "/api/admin/email/templates".to_string(),
        "/api/admin/email/campaigns".to_string(),
    ] {
        let denied = request(api, "GET", &endpoint, delegated, Value::Null).await;
        assert_eq!(denied.0, StatusCode::FORBIDDEN, "{endpoint}: {denied:?}");
    }
    for endpoint in [
        "stats",
        "deals",
        "pipelines",
        "lists",
        "contact-tags",
        "segments",
        "sequences",
        "recurring-campaigns",
        "templates",
        "automations",
        "smart-links",
        "webhooks",
        "companies",
        "imports",
        "exports",
        "managers",
        "manager-roles",
    ] {
        let denied = request(
            api,
            "GET",
            &format!("{ROOT}/{endpoint}"),
            delegated,
            Value::Null,
        )
        .await;
        assert_eq!(
            denied.0,
            StatusCode::FORBIDDEN,
            "The contact-only role must not read the {endpoint} module: {denied:?}"
        );
    }
    for (method, endpoint) in [
        ("POST", format!("{contact_path}/notes")),
        ("POST", format!("{contact_path}/recalculate-score")),
        ("POST", format!("{ROOT}/leads/bulk-delete")),
        ("POST", format!("{ROOT}/contact-tags/bulk-apply")),
        ("POST", format!("{ROOT}/lists/1/contacts")),
        ("POST", format!("{ROOT}/campaigns/1/send")),
        ("POST", format!("{ROOT}/sequences/1/test")),
        ("POST", format!("{ROOT}/webhooks/1/test")),
        (
            "POST",
            format!("{ROOT}/imports/{}/start", uuid::Uuid::new_v4()),
        ),
        (
            "GET",
            format!("{ROOT}/exports/{}/download", uuid::Uuid::new_v4()),
        ),
        ("POST", format!("{ROOT}/logs/clear")),
        ("DELETE", format!("{ROOT}/templates/1")),
    ] {
        let denied = request(api, method, &endpoint, delegated, json!({})).await;
        assert_eq!(
            denied.0,
            StatusCode::FORBIDDEN,
            "Restricted action {method} {endpoint} must stop before validation or mutation: {denied:?}"
        );
    }
    for panel in ["notes", "emails"] {
        assert_eq!(
            request(
                api,
                "GET",
                &format!("{contact_path}/{panel}"),
                delegated,
                Value::Null
            )
            .await
            .0,
            StatusCode::OK
        );
    }
    let before = adapter.requests.lock().unwrap().len();
    let denied=request(api,"POST",&format!("{contact_path}/send-email"),delegated,json!({"request_id":uuid::Uuid::new_v4(),"expected_email":"manager-record@example.test","subject":"Unauthorized send","body_html":"<p>Never submitted</p>"})).await;
    assert_eq!(denied.0, StatusCode::FORBIDDEN, "{denied:?}");
    assert_eq!(adapter.requests.lock().unwrap().len(), before);
    update_role(
        api,
        &tokens[0],
        &mut role,
        json!([
            grant("contacts", "view"),
            grant("contacts", "edit"),
            grant("deals", "view"),
            grant("tags", "view"),
            grant("lists", "view"),
            grant("managers", "manage")
        ]),
    )
    .await;
    let detail = request(api, "GET", &contact_path, delegated, Value::Null).await;
    assert_eq!(detail.0, StatusCode::OK, "{detail:?}");
    for kind in ["tags", "lists"] {
        let options = request(
            api,
            "GET",
            &format!("{ROOT}/imports/options?kind={kind}&per_page=1"),
            delegated,
            Value::Null,
        )
        .await;
        assert_eq!(
            options.0,
            StatusCode::OK,
            "Contact assignment pickers use the selected catalog permission, without an import grant: {options:?}"
        );
    }
    let edited = request(
        api,
        "PUT",
        &contact_path,
        delegated,
        json!({"job_title":"Delegated editor persisted this"}),
    )
    .await;
    assert_eq!(edited.0, StatusCode::OK, "{edited:?}");
    assert_eq!(edited.1["job_title"], "Delegated editor persisted this");
    let journal:Option<i64>=sqlx::query_scalar("SELECT created_by FROM crm_contact_events WHERE contact_id=$1 AND kind='updated' ORDER BY id DESC LIMIT 1").bind(contact).fetch_one(pool).await.unwrap();
    assert_eq!(journal, Some(user));
    let escalation = request(
        api,
        "PUT",
        &format!("{ROOT}/manager-roles/{role_id}"),
        delegated,
        json!({"expected_version":role["version"],"permissions":[grant("contacts","manage")]}),
    )
    .await;
    assert_eq!(escalation.0, StatusCode::FORBIDDEN, "{escalation:?}");
    let escalation = request(
        api,
        "POST",
        &format!("{ROOT}/managers"),
        delegated,
        json!({"user_id":user,"role_id":original_default["id"]}),
    )
    .await;
    assert_eq!(escalation.0, StatusCode::FORBIDDEN, "{escalation:?}");
    assert_eq!(
        request(
            api,
            "GET",
            &format!("{ROOT}/managers/users"),
            delegated,
            Value::Null
        )
        .await
        .0,
        StatusCode::FORBIDDEN
    );
    let unchanged: String = sqlx::query_scalar("SELECT role FROM users WHERE id=$1")
        .bind(user)
        .fetch_one(pool)
        .await
        .unwrap();
    assert_eq!(
        unchanged, "user",
        "CRM grants never modify a global application role"
    );
    let tag = request(
        api,
        "POST",
        &format!("{ROOT}/contact-tags"),
        admin,
        json!({"title":"Scoped manager tag","color":"#112233"}),
    )
    .await;
    assert_eq!(tag.0, StatusCode::OK, "{tag:?}");
    let tag = tag.1["id"].clone();
    assert_eq!(
        request(
            api,
            "POST",
            &format!("{contact_path}/tags"),
            delegated,
            json!({"tag_id":tag})
        )
        .await
        .0,
        StatusCode::FORBIDDEN
    );
    let mut grants = role["permissions"].as_array().unwrap().clone();
    grants.push(grant("tags", "edit"));
    update_role(api, &tokens[0], &mut role, json!(grants)).await;
    let tagged = request(
        api,
        "POST",
        &format!("{contact_path}/tags"),
        delegated,
        json!({"tag_id":tag}),
    )
    .await;
    assert_eq!(tagged.0, StatusCode::OK, "{tagged:?}");
    assert_eq!(tagged.1["applied_count"], 1);
    let mut grants = role["permissions"].as_array().unwrap().clone();
    grants.push(grant("contacts", "manage"));
    update_role(api, &tokens[0], &mut role, json!(grants)).await;
    let request_id = uuid::Uuid::new_v4();
    let before = adapter.requests.lock().unwrap().len();
    let submitted=request(api,"POST",&format!("{contact_path}/send-email"),delegated,json!({"request_id":request_id,"expected_email":"manager-record@example.test","subject":"CRM delegated personal message","body_html":"<p>Explicit authorized personal message</p>"})).await;
    assert_eq!(submitted.0, StatusCode::OK, "{submitted:?}");
    assert_eq!(submitted.1["data"]["submissions"][0]["status"], "accepted");
    assert_eq!(adapter.requests.lock().unwrap().len(), before + 1);
    let actual_actor: Option<i64> =
        sqlx::query_scalar("SELECT created_by FROM crm_contact_mail_requests WHERE request_id=$1")
            .bind(request_id)
            .fetch_one(pool)
            .await
            .unwrap();
    assert_eq!(actual_actor, Some(user));
    assert_eq!(
        request(
            api,
            "GET",
            "/api/admin/email/campaigns",
            delegated,
            Value::Null
        )
        .await
        .0,
        StatusCode::FORBIDDEN,
        "A working CRM mail grant does not grant native-email admin access"
    );
    conditional_boundaries(api, pool, tokens, adapter, &mut role, contact, user).await;
    catalog_boundaries(api, pool, tokens, &mut role, &mut assignment).await;
    active_account(api, pool, tokens, user).await;
    policy_repair(api, pool, tokens, &mut role).await;
    default_and_audit(api, pool, tokens, &mut role, &assignment, &original_default).await;
    update_role(
        api,
        &tokens[0],
        &mut role,
        json!([
            grant("contacts", "manage"),
            grant("deals", "view"),
            grant("campaigns", "view"),
            grant("sequences", "view"),
            grant("templates", "view"),
            grant("lists", "view"),
            grant("tags", "view"),
            grant("segments", "view"),
            grant("imports", "manage")
        ]),
    )
    .await;
    let single_job = upload_fixture(
        api,
        &tokens[0],
        "manager-single-pool.json",
        json!([{"email":"manager-single-pool@example.test","first_name":"One connection"}]),
    )
    .await;
    let one = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_with((*pool.connect_options()).clone())
        .await
        .unwrap();
    let single = super::app(revolution_api::db::Database { pool: one.clone() }, None).await;
    let protected = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        request(
            &single,
            "GET",
            &format!("{ROOT}/contacts"),
            delegated,
            Value::Null,
        ),
    )
    .await
    .unwrap();
    assert_eq!(
        protected.0,
        StatusCode::OK,
        "Fresh authentication and role evaluation must not hold a nested pool lease: {protected:?}"
    );
    let campaign: i64 =
        sqlx::query_scalar("SELECT id FROM crm_campaigns WHERE title='Manager113 CRM-only source'")
            .fetch_one(pool)
            .await
            .unwrap();
    let sequence: i64 = sqlx::query_scalar(
        "SELECT id FROM crm_sequences WHERE title='Manager113 authoring persisted'",
    )
    .fetch_one(pool)
    .await
    .unwrap();
    for (method, path, body) in [
        (
            "GET",
            format!("{ROOT}/contacts/native-templates?search=Manager113NativePicker"),
            Value::Null,
        ),
        ("GET", format!("{ROOT}/campaigns/{campaign}"), Value::Null),
        (
            "GET",
            format!("{ROOT}/sequences/{sequence}/tests"),
            Value::Null,
        ),
        (
            "POST",
            format!("{ROOT}/imports/{single_job}/start"),
            json!({}),
        ),
    ] {
        let result = tokio::time::timeout(
            std::time::Duration::from_secs(5),
            request(&single, method, &path, delegated, body),
        )
        .await
        .unwrap();
        assert_eq!(
            result.0,
            StatusCode::OK,
            "Current grants/source checks must not acquire a nested pool lease: {method} {path}: {result:?}"
        );
    }
    one.close().await;
    let stale = request(
        api,
        "PUT",
        &format!("{ROOT}/managers/{manager_id}"),
        admin,
        json!({"role_id":role_id,"expected_version":0}),
    )
    .await;
    assert_eq!(stale.0, StatusCode::BAD_REQUEST, "{stale:?}");
    let stale = request(
        api,
        "PUT",
        &format!("{ROOT}/managers/{manager_id}"),
        admin,
        json!({"role_id":role_id,"expected_version":assignment["version"].as_i64().unwrap()-1}),
    )
    .await;
    assert_eq!(stale.0, StatusCode::CONFLICT, "{stale:?}");
    let removed = request(
        api,
        "DELETE",
        &format!(
            "{ROOT}/managers/{manager_id}?expected_version={}",
            assignment["version"]
        ),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(removed.0, StatusCode::OK, "{removed:?}");
    assert_eq!(removed.1["deleted"], true);
    assert_eq!(
        request(
            api,
            "GET",
            &format!("{ROOT}/contacts"),
            delegated,
            Value::Null
        )
        .await
        .0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        request(api, "GET", &capabilities, delegated, Value::Null)
            .await
            .1["permissions"],
        json!([]),
        "A configured default role never automatically grants access after explicit assignment removal"
    );
    let deleted = request(
        api,
        "DELETE",
        &format!(
            "{ROOT}/manager-roles/{role_id}?expected_version={}",
            role["version"]
        ),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(deleted.0, StatusCode::OK, "{deleted:?}");
}

async fn catalog_boundaries(
    api: &Router,
    pool: &PgPool,
    tokens: &[String],
    role: &mut Value,
    assignment: &mut Value,
) {
    let admin = Some(tokens[0].as_str());
    let delegated = Some(tokens[1].as_str());
    update_role(
        api,
        &tokens[0],
        role,
        json!([grant("campaigns", "view"), grant("sequences", "view")]),
    )
    .await;
    for endpoint in ["campaigns", "sequences"] {
        assert_eq!(
            request(
                api,
                "GET",
                &format!("{ROOT}/{endpoint}"),
                delegated,
                Value::Null
            )
            .await
            .0,
            StatusCode::FORBIDDEN
        );
    }
    let sequence: i64 = sqlx::query_scalar("SELECT id FROM crm_sequences ORDER BY id LIMIT 1")
        .fetch_one(pool)
        .await
        .unwrap();
    for suffix in ["tests", "subscribers", "triggers"] {
        let denied = request(
            api,
            "GET",
            &format!("{ROOT}/sequences/{sequence}/{suffix}"),
            delegated,
            Value::Null,
        )
        .await;
        assert_eq!(denied.0, StatusCode::FORBIDDEN, "{denied:?}");
    }
    let native:i64=sqlx::query_scalar("SELECT id FROM crm_campaigns WHERE native_email_campaign_id IS NOT NULL ORDER BY id LIMIT 1").fetch_one(pool).await.unwrap();
    assert_eq!(
        request(
            api,
            "GET",
            &format!("{ROOT}/campaigns/{native}"),
            delegated,
            Value::Null
        )
        .await
        .0,
        StatusCode::CONFLICT
    );
    update_role(
        api,
        &tokens[0],
        role,
        json!([
            grant("contacts", "view"),
            grant("deals", "view"),
            grant("campaigns", "view"),
            grant("sequences", "view")
        ]),
    )
    .await;
    for suffix in ["recipients", "deliveries"] {
        assert_eq!(
            request(
                api,
                "GET",
                &format!("{ROOT}/campaigns/{native}/{suffix}"),
                delegated,
                Value::Null
            )
            .await
            .0,
            StatusCode::CONFLICT
        );
    }
    let engine: i64 = sqlx::query_scalar(
        "SELECT campaign_id FROM crm_sequence_engines ORDER BY campaign_id LIMIT 1",
    )
    .fetch_one(pool)
    .await
    .unwrap();
    assert_eq!(
        request(
            api,
            "GET",
            &format!("{ROOT}/campaigns/{engine}"),
            delegated,
            Value::Null
        )
        .await
        .0,
        StatusCode::CONFLICT
    );
    let stale = request(
        api,
        "PUT",
        &format!("{ROOT}/manager-roles/{}", role["id"].as_str().unwrap()),
        admin,
        json!({"expected_version":1,"permissions":[]}),
    )
    .await;
    assert_eq!(stale.0, StatusCode::CONFLICT, "{stale:?}");
    let options = request(
        api,
        "GET",
        &format!("{ROOT}/managers/users?search=user%40email-completion.test&per_page=1"),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(options.0, StatusCode::OK, "{options:?}");
    assert_eq!(options.1["meta"]["total"], 1);
    let same = request(
        api,
        "PUT",
        &format!("{ROOT}/managers/{}", assignment["id"].as_str().unwrap()),
        admin,
        json!({"role_id":role["id"],"expected_version":assignment["version"]}),
    )
    .await;
    assert_eq!(same.0, StatusCode::OK, "{same:?}");
    *assignment = same.1;
}
async fn active_account(api: &Router, pool: &PgPool, _tokens: &[String], source_user: i64) {
    let user:i64=sqlx::query_scalar("INSERT INTO users(name,email,password_hash,role,is_active) VALUES('Authority state probe',$1,'not-a-login-password','user',true) RETURNING id").bind(format!("manager-authority-{}@example.test",Uuid::new_v4())).fetch_one(pool).await.unwrap();
    sqlx::query("INSERT INTO crm_managers(user_id,role_id) SELECT $1,role_id FROM crm_managers WHERE user_id=$2").bind(user).bind(source_user).execute(pool).await.unwrap();
    let secret = std::env::var("JWT_SECRET").unwrap();
    let old = revolution_api::utils::create_jwt_versioned(user, &secret, 1, 0).unwrap();
    sqlx::query("UPDATE users SET is_active=false WHERE id=$1")
        .bind(user)
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(
        request(
            api,
            "GET",
            &format!("{ROOT}/contacts"),
            Some(&old),
            Value::Null
        )
        .await
        .0,
        StatusCode::UNAUTHORIZED
    );
    sqlx::query("UPDATE users SET is_active=true,deleted_at=NOW() WHERE id=$1")
        .bind(user)
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(
        request(
            api,
            "GET",
            &format!("{ROOT}/permissions"),
            Some(&old),
            Value::Null
        )
        .await
        .0,
        StatusCode::UNAUTHORIZED
    );
    sqlx::query("UPDATE users SET deleted_at=NULL WHERE id=$1")
        .bind(user)
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(
        request(
            api,
            "GET",
            &format!("{ROOT}/contacts"),
            Some(&old),
            Value::Null
        )
        .await
        .0,
        StatusCode::UNAUTHORIZED,
        "Reactivation never restores a revoked token"
    );
    let version: i64 = sqlx::query_scalar("SELECT auth_version FROM users WHERE id=$1")
        .bind(user)
        .fetch_one(pool)
        .await
        .unwrap();
    let fresh = revolution_api::utils::create_jwt_versioned(user, &secret, 1, version).unwrap();
    assert_eq!(
        request(
            api,
            "GET",
            &format!("{ROOT}/contacts"),
            Some(&fresh),
            Value::Null
        )
        .await
        .0,
        StatusCode::OK
    );
    sqlx::query("DELETE FROM users WHERE id=$1")
        .bind(user)
        .execute(pool)
        .await
        .unwrap();
}

async fn policy_repair(api: &Router, pool: &PgPool, tokens: &[String], role: &mut Value) {
    let saved = role["permissions"].clone();
    sqlx::query("UPDATE crm_manager_roles SET permissions=$2 WHERE id=$1")
        .bind(role["id"].as_str().unwrap().parse::<i64>().unwrap())
        .bind(json!([grant("*", "manage")]))
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(
        request(
            api,
            "GET",
            &format!("{ROOT}/contacts"),
            Some(&tokens[1]),
            Value::Null
        )
        .await
        .0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        request(
            api,
            "GET",
            &format!("{ROOT}/permissions"),
            Some(&tokens[1]),
            Value::Null
        )
        .await
        .0,
        StatusCode::FORBIDDEN
    );
    update_role(api, &tokens[0], role, saved).await;
}
async fn default_and_audit(
    api: &Router,
    pool: &PgPool,
    tokens: &[String],
    role: &mut Value,
    assignment: &Value,
    original_default: &Value,
) {
    let admin = Some(tokens[0].as_str());
    let role_path = format!("{ROOT}/manager-roles/{}", role["id"].as_str().unwrap());
    let assigned = request(
        api,
        "DELETE",
        &format!("{role_path}?expected_version={}", role["version"]),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(assigned.0, StatusCode::CONFLICT, "{assigned:?}");
    let changed = request(
        api,
        "PUT",
        &role_path,
        admin,
        json!({"expected_version":role["version"],"is_default":true}),
    )
    .await;
    assert_eq!(changed.0, StatusCode::OK, "{changed:?}");
    *role = changed.1;
    let defaults: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM crm_manager_roles WHERE is_default")
            .fetch_one(pool)
            .await
            .unwrap();
    assert_eq!(defaults, 1);
    let default_path = format!(
        "{ROOT}/manager-roles/{}",
        original_default["id"].as_str().unwrap()
    );
    let previous = request(api, "GET", &default_path, admin, Value::Null)
        .await
        .1;
    assert_eq!(previous["is_default"], false);
    let restored=request(api,"PUT",&default_path,admin,json!({"expected_version":previous["version"],"is_default":true,"name":"Explicitly configured default CRM proof","description":"Default designation grants no automatic assignment","permissions":[grant("contacts","view")]})).await;
    assert_eq!(restored.0, StatusCode::OK, "{restored:?}");
    assert_eq!(
        restored.1["permissions"],
        json!([grant("contacts", "view")])
    );
    *role = request(api, "GET", &role_path, admin, Value::Null).await.1;
    let blocked = request(
        api,
        "DELETE",
        &format!("{default_path}?expected_version={}", restored.1["version"]),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(blocked.0, StatusCode::CONFLICT, "{blocked:?}");
    let unassigned:i64=sqlx::query_scalar("INSERT INTO users(email,name,password_hash,role,is_active) VALUES('manager-unassigned-audit@example.test','Unassigned audit fixture','not-a-login-password','user',true) RETURNING id").fetch_one(pool).await.unwrap();
    sqlx::query("ALTER TABLE crm_operational_logs RENAME TO manager_permission_audit_unavailable")
        .execute(pool)
        .await
        .unwrap();
    let attempted = request(
        api,
        "PUT",
        &role_path,
        admin,
        json!({"expected_version":role["version"],"permissions":[],"is_default":true}),
    )
    .await;
    let new_role = request(
        api,
        "POST",
        &format!("{ROOT}/manager-roles"),
        admin,
        json!({"name":"Must roll back CRM role","permissions":[]}),
    )
    .await;
    let removed = request(
        api,
        "DELETE",
        &format!(
            "{ROOT}/managers/{}?expected_version={}",
            assignment["id"].as_str().unwrap(),
            assignment["version"]
        ),
        admin,
        Value::Null,
    )
    .await;
    let changed_assignment = request(
        api,
        "PUT",
        &format!("{ROOT}/managers/{}", assignment["id"].as_str().unwrap()),
        admin,
        json!({"role_id":original_default["id"],"expected_version":assignment["version"]}),
    )
    .await;
    let added = request(
        api,
        "POST",
        &format!("{ROOT}/managers"),
        admin,
        json!({"user_id":unassigned,"role_id":role["id"]}),
    )
    .await;
    sqlx::query("ALTER TABLE manager_permission_audit_unavailable RENAME TO crm_operational_logs")
        .execute(pool)
        .await
        .unwrap();
    for failed in [attempted, new_role, removed, added, changed_assignment] {
        assert_eq!(failed.0, StatusCode::SERVICE_UNAVAILABLE, "{failed:?}");
    }
    let unchanged_assignment = request(
        api,
        "GET",
        &format!("{ROOT}/managers/{}", assignment["id"].as_str().unwrap()),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(
        unchanged_assignment.0,
        StatusCode::OK,
        "{unchanged_assignment:?}"
    );
    assert_eq!(unchanged_assignment.1["role_id"], assignment["role_id"]);
    assert_eq!(unchanged_assignment.1["version"], assignment["version"]);
    let unchanged = request(api, "GET", &role_path, admin, Value::Null).await;
    assert_eq!(
        unchanged.1, *role,
        "Failed permission audit rolls back the grant change and its version"
    );
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM crm_manager_roles WHERE name='Must roll back CRM role'",
    )
    .fetch_one(pool)
    .await
    .unwrap();
    assert_eq!(count, 0);
    let unassigned_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM crm_managers WHERE user_id=$1")
            .bind(unassigned)
            .fetch_one(pool)
            .await
            .unwrap();
    assert_eq!(unassigned_count, 0);
    let current_default: i64 =
        sqlx::query_scalar("SELECT id FROM crm_manager_roles WHERE is_default")
            .fetch_one(pool)
            .await
            .unwrap();
    assert_eq!(
        current_default,
        original_default["id"]
            .as_str()
            .unwrap()
            .parse::<i64>()
            .unwrap(),
        "Failed permission audit also restores the prior default atomically"
    );
    assert_eq!(
        request(
            api,
            "GET",
            &format!("{ROOT}/contacts"),
            Some(&tokens[1]),
            Value::Null
        )
        .await
        .0,
        StatusCode::OK,
        "Failed removal audit preserves actual delegated access"
    );
}
fn provider_count(adapter: &Adapter) -> usize {
    let requests = adapter.requests.lock().unwrap();
    requests.len()
}
async fn durable_state(pool: &PgPool) -> Value {
    let mut tx = pool.begin().await.unwrap();
    sqlx::query("SET LOCAL TIME ZONE 'UTC'")
        .execute(&mut *tx)
        .await
        .unwrap();
    let state=sqlx::query_scalar(concat!(
        "SELECT jsonb_build_object(",
        "'contacts',(SELECT md5(COALESCE(jsonb_agg(to_jsonb(t) ORDER BY id)::text,'')) FROM contacts t),",
        "'events',(SELECT md5(COALESCE(jsonb_agg(to_jsonb(t) ORDER BY id)::text,'')) FROM crm_contact_events t),",
        "'notes',(SELECT md5(COALESCE(jsonb_agg(to_jsonb(t) ORDER BY id)::text,'')) FROM crm_contact_notes t),",
        "'mail',(SELECT md5(COALESCE(jsonb_agg(to_jsonb(t) ORDER BY request_id)::text,'')) FROM crm_contact_mail_requests t),",
        "'logs',(SELECT md5(COALESCE(jsonb_agg(to_jsonb(t) ORDER BY id)::text,'')) FROM email_logs t),",
        "'audit',(SELECT md5(COALESCE(jsonb_agg(to_jsonb(t) ORDER BY id)::text,'')) FROM crm_operational_logs t),",
        "'transfers',(SELECT md5(COALESCE(jsonb_agg(to_jsonb(t) ORDER BY id)::text,'')) FROM crm_transfer_jobs t),",
        "'sequences',(SELECT md5(COALESCE(jsonb_agg(to_jsonb(t) ORDER BY id)::text,'')) FROM crm_sequences t),",
        "'qa_requests',(SELECT md5(COALESCE(jsonb_agg(to_jsonb(t) ORDER BY request_id)::text,'')) FROM crm_sequence_test_requests t),",
        "'engines',(SELECT md5(COALESCE(jsonb_agg(to_jsonb(t) ORDER BY campaign_id)::text,'')) FROM crm_sequence_engines t),",
        "'sequence_revisions',(SELECT md5(COALESCE(jsonb_agg(to_jsonb(t) ORDER BY id)::text,'')) FROM crm_sequence_revisions t),",
        "'trackers',(SELECT md5(COALESCE(jsonb_agg(to_jsonb(t) ORDER BY id)::text,'')) FROM crm_sequence_trackers t),",
        "'campaigns',(SELECT md5(COALESCE(jsonb_agg(to_jsonb(t) ORDER BY id)::text,'')) FROM crm_campaigns t),",
        "'campaign_revisions',(SELECT md5(COALESCE(jsonb_agg(to_jsonb(t) ORDER BY id)::text,'')) FROM crm_campaign_revisions t),",
        "'deliveries',(SELECT md5(COALESCE(jsonb_agg(to_jsonb(t) ORDER BY id)::text,'')) FROM crm_campaign_deliveries t),",
        "'tags',(SELECT md5(COALESCE(jsonb_agg(to_jsonb(t) ORDER BY id)::text,'')) FROM crm_tags t),",
        "'lists',(SELECT md5(COALESCE(jsonb_agg(to_jsonb(t) ORDER BY id)::text,'')) FROM crm_lists t),",
        "'members',(SELECT md5(COALESCE(jsonb_agg(to_jsonb(t) ORDER BY list_id,contact_id)::text,'')) FROM crm_list_members t),",
        "'deals',(SELECT md5(COALESCE(jsonb_agg(to_jsonb(t) ORDER BY id)::text,'')) FROM crm_deals t),",
        "'companies',(SELECT md5(COALESCE(jsonb_agg(to_jsonb(t) ORDER BY id)::text,'')) FROM crm_companies t))"
    )).fetch_one(&mut *tx).await.unwrap();
    tx.commit().await.unwrap();
    state
}
async fn denied_unchanged(
    api: &Router,
    pool: &PgPool,
    adapter: &Adapter,
    token: &str,
    method: &str,
    path: &str,
    body: Value,
) {
    let before = durable_state(pool).await;
    let submissions = provider_count(adapter);
    let response = request(api, method, path, Some(token), body).await;
    assert_eq!(
        response.0,
        StatusCode::FORBIDDEN,
        "{method} {path}: {response:?}"
    );
    assert_eq!(
        durable_state(pool).await,
        before,
        "Forbidden {method} {path} must not persist any source, revision, mutation, intent, receipt or audit"
    );
    assert_eq!(
        provider_count(adapter),
        submissions,
        "Forbidden {method} {path} must not contact a provider"
    );
}
async fn native_picker_boundary(
    api: &Router,
    pool: &PgPool,
    tokens: &[String],
    adapter: &Adapter,
    role: &mut Value,
    contact: i64,
) {
    let admin = Some(tokens[0].as_str());
    let token = &tokens[1];
    let picker =
        format!("{ROOT}/contacts/native-templates?search=Manager113NativePicker&per_page=1");
    let template=request(api,"POST","/api/admin/email/templates",admin,json!({"name":"Manager113NativePicker","slug":"manager113-native-picker","subject":"Manager {{contact.first_name}}","body":"<p>Native origin {{contact.first_name}}</p>","body_text":"Native origin {{name}}","is_active":true})).await;
    assert_eq!(template.0, StatusCode::OK, "{template:?}");
    let id = template.1["data"]["id"].as_i64().unwrap();
    for permissions in [
        json!([grant("contacts", "manage")]),
        json!([grant("templates", "view")]),
    ] {
        update_role(api, &tokens[0], role, permissions).await;
        denied_unchanged(api, pool, adapter, token, "GET", &picker, Value::Null).await;
    }
    update_role(api, &tokens[0], role, json!([grant("contacts", "manage")])).await;
    let key = uuid::Uuid::new_v4();
    let message = json!({"request_id":key,"expected_email":"manager-record@example.test","subject":"Manager {{contact.first_name}}","body_html":"<p>Native origin {{contact.first_name}}</p>","text_body":"Native origin {{name}}","template_id":id.to_string()});
    let send = format!("{ROOT}/contacts/{contact}/send-email");
    denied_unchanged(api, pool, adapter, token, "POST", &send, message.clone()).await;
    update_role(
        api,
        &tokens[0],
        role,
        json!([grant("contacts", "manage"), grant("templates", "view")]),
    )
    .await;
    let selected = request(api, "GET", &picker, Some(token), Value::Null).await;
    assert_eq!(selected.0, StatusCode::OK, "{selected:?}");
    assert_eq!(selected.1["metrics_scope"], "contact_template_picker");
    assert_eq!(selected.1["meta"]["total"], 1);
    let row = &selected.1["data"][0];
    assert_eq!(row["id"], id.to_string());
    assert_eq!(
        row.as_object().unwrap().len(),
        5,
        "The scoped adapter exposes only its exact creative contract"
    );
    for native in ["/api/admin/email/templates", "/api/admin/email/campaigns"] {
        denied_unchanged(api, pool, adapter, token, "GET", native, Value::Null).await;
    }
    let before = provider_count(adapter);
    let sent = request(api, "POST", &send, Some(token), message.clone()).await;
    assert_eq!(sent.0, StatusCode::OK, "{sent:?}");
    assert_eq!(sent.1["data"]["submissions"][0]["status"], "accepted");
    assert_eq!(sent.1["data"]["template_id"], id.to_string());
    assert_eq!(provider_count(adapter), before + 1);
    update_role(api, &tokens[0], role, json!([grant("contacts", "manage")])).await;
    denied_unchanged(api, pool, adapter, token, "POST", &send, message.clone()).await;
    update_role(
        api,
        &tokens[0],
        role,
        json!([grant("contacts", "manage"), grant("templates", "view")]),
    )
    .await;
    let edited=request(api,"PUT",&format!("/api/admin/email/templates/{id}"),admin,json!({"subject":"Changed native source","body":"<p>Changed after frozen intent</p>","is_active":false})).await;
    assert_eq!(edited.0, StatusCode::OK, "{edited:?}");
    let replay = request(api, "POST", &send, Some(token), message).await;
    assert_eq!(replay.0, StatusCode::OK, "{replay:?}");
    assert_eq!(
        replay.1, sent.1,
        "Replay preserves the actual source snapshot and receipt after native edits/deactivation"
    );
    assert_eq!(provider_count(adapter), before + 1);
    let hidden = request(api, "GET", &picker, Some(token), Value::Null).await;
    assert_eq!(hidden.0, StatusCode::OK, "{hidden:?}");
    assert_eq!(hidden.1["data"], json!([]));
}

async fn catalog_effect_boundaries(
    api: &Router,
    pool: &PgPool,
    tokens: &[String],
    adapter: &Adapter,
    role: &mut Value,
    contact: i64,
    user: i64,
) {
    let admin = Some(tokens[0].as_str());
    let token = &tokens[1];
    let contact_path = format!("{ROOT}/contacts/{contact}");
    update_role(
        api,
        &tokens[0],
        role,
        json!([
            grant("contacts", "edit"),
            grant("contacts", "view"),
            grant("deals", "view")
        ]),
    )
    .await;
    for body in [json!({"tags":[]}), json!({"owner_id":user.to_string()})] {
        denied_unchanged(api, pool, adapter, token, "PUT", &contact_path, body).await;
    }
    update_role(api, &tokens[0], role, json!([grant("contacts", "create")])).await;
    denied_unchanged(
        api,
        pool,
        adapter,
        token,
        "POST",
        &format!("{ROOT}/contacts"),
        json!({"email":"manager-forbidden-tags@example.test","tags":["Must not assign"]}),
    )
    .await;
    let tag = request(
        api,
        "POST",
        &format!("{ROOT}/contact-tags"),
        admin,
        json!({"title":"Manager113 rename","color":"#223344"}),
    )
    .await;
    assert_eq!(tag.0, StatusCode::OK, "{tag:?}");
    let tag = tag.1["id"].as_i64().unwrap();
    let assigned = request(
        api,
        "POST",
        &format!("{contact_path}/tags"),
        admin,
        json!({"tag_id":tag}),
    )
    .await;
    assert_eq!(assigned.0, StatusCode::OK, "{assigned:?}");
    update_role(
        api,
        &tokens[0],
        role,
        json!([
            grant("tags", "edit"),
            grant("tags", "view"),
            grant("contacts", "view")
        ]),
    )
    .await;
    let path = format!("{ROOT}/contact-tags/{tag}");
    let same = request(
        api,
        "PUT",
        &path,
        Some(token),
        json!({"title":"Manager113 rename","color":"#334455"}),
    )
    .await;
    assert_eq!(
        same.0,
        StatusCode::OK,
        "An unchanged tag title does not rewrite contact assignments: {same:?}"
    );
    denied_unchanged(
        api,
        pool,
        adapter,
        token,
        "PUT",
        &path,
        json!({"title":"Forbidden assignment cascade","color":"#334455"}),
    )
    .await;
    let company = request(
        api,
        "POST",
        &format!("{ROOT}/companies"),
        admin,
        json!({"name":"Manager113 company"}),
    )
    .await;
    assert_eq!(company.0, StatusCode::OK, "{company:?}");
    let company = company.1["id"].as_i64().unwrap();
    let (pipeline,stage):(i64,i64)=sqlx::query_as("SELECT p.id,s.id FROM crm_pipelines p JOIN crm_pipeline_stages s ON s.pipeline_id=p.id WHERE p.is_active=true AND NOT COALESCE(s.is_closed_won,false) AND NOT COALESCE(s.is_closed_lost,false) ORDER BY p.id,s.position,s.id LIMIT 1").fetch_one(pool).await.unwrap();
    let body = json!({"name":"Manager113 linked deal","pipeline_id":pipeline,"stage_id":stage,"contact_id":contact,"company_id":company,"owner_id":user,"amount_cents":100,"currency":"USD"});
    for omitted in ["contacts", "companies", "managers"] {
        let permissions = ["deals", "pipelines", "contacts", "companies", "managers"]
            .iter()
            .filter(|module| **module != omitted)
            .map(|module| grant(module, if *module == "deals" { "manage" } else { "view" }))
            .collect::<Vec<_>>();
        update_role(api, &tokens[0], role, json!(permissions)).await;
        denied_unchanged(
            api,
            pool,
            adapter,
            token,
            "POST",
            &format!("{ROOT}/deals"),
            body.clone(),
        )
        .await;
    }
    update_role(
        api,
        &tokens[0],
        role,
        json!([
            grant("deals", "manage"),
            grant("pipelines", "view"),
            grant("contacts", "view"),
            grant("companies", "view"),
            grant("managers", "view")
        ]),
    )
    .await;
    let deal = request(api, "POST", &format!("{ROOT}/deals"), Some(token), body).await;
    assert_eq!(deal.0, StatusCode::OK, "{deal:?}");
    let deal_id = deal.1["id"].as_str().unwrap();
    update_role(
        api,
        &tokens[0],
        role,
        json!([
            grant("deals", "edit"),
            grant("deals", "view"),
            grant("pipelines", "view"),
            grant("contacts", "view"),
            grant("companies", "view")
        ]),
    )
    .await;
    denied_unchanged(
        api,
        pool,
        adapter,
        token,
        "PUT",
        &format!("{ROOT}/deals/{deal_id}"),
        json!({"name":"Must not leak owner acknowledgement"}),
    )
    .await;
    update_role(
        api,
        &tokens[0],
        role,
        json!([
            grant("companies", "delete"),
            grant("contacts", "view"),
            grant("deals", "view")
        ]),
    )
    .await;
    denied_unchanged(
        api,
        pool,
        adapter,
        token,
        "DELETE",
        &format!("{ROOT}/companies/{company}"),
        Value::Null,
    )
    .await;
    let list:i64=sqlx::query_scalar("INSERT INTO crm_lists(name,title,slug) VALUES('Manager113 protected list','Manager113 protected list','manager113-protected-list') RETURNING id").fetch_one(pool).await.unwrap();
    sqlx::query("INSERT INTO crm_list_members(list_id,contact_id) VALUES($1,$2)")
        .bind(list)
        .bind(contact)
        .execute(pool)
        .await
        .unwrap();
    update_role(api, &tokens[0], role, json!([grant("lists", "delete")])).await;
    denied_unchanged(
        api,
        pool,
        adapter,
        token,
        "DELETE",
        &format!("{ROOT}/lists/{list}"),
        Value::Null,
    )
    .await;
}
async fn authoring_boundaries(
    api: &Router,
    pool: &PgPool,
    tokens: &[String],
    adapter: &Adapter,
    role: &mut Value,
) {
    let admin = Some(tokens[0].as_str());
    let token = &tokens[1];
    let sequence=request(api,"POST",&format!("{ROOT}/sequences"),admin,json!({"title":"Manager113 authoring source","emails":[{"title":"Actual saved mail","subject":"Manager113 content","html_content":"<p>Saved content</p>","status":"active","delay_value":0,"delay_unit":"minutes"}]})).await;
    assert_eq!(sequence.0, StatusCode::OK, "{sequence:?}");
    let sequence = sequence.1;
    let path = format!("{ROOT}/sequences/{}", sequence["id"].as_str().unwrap());
    update_role(
        api,
        &tokens[0],
        role,
        json!([grant("sequences", "view"), grant("sequences", "edit")]),
    )
    .await;
    for status in ["draft", "active", "paused"] {
        denied_unchanged(
            api,
            pool,
            adapter,
            token,
            "PUT",
            &path,
            json!({"expected_version":sequence["authoring_version"],"status":status}),
        )
        .await;
    }
    let edited=request(api,"PUT",&path,Some(token),json!({"expected_version":sequence["authoring_version"],"title":"Manager113 authoring persisted"})).await;
    assert_eq!(
        edited.0,
        StatusCode::OK,
        "Authoring without an execution status remains usable: {edited:?}"
    );
    assert_eq!(edited.1["title"], "Manager113 authoring persisted");
    let template=request(api,"POST",&format!("{ROOT}/templates"),admin,json!({"title":"Manager113 CRM template","subject":"Saved CRM source","content":"<p>Actual CRM template</p>"})).await;
    assert_eq!(template.0, StatusCode::OK, "{template:?}");
    let template = template.1["id"].as_str().unwrap().parse::<i64>().unwrap();
    update_role(api, &tokens[0], role, json!([grant("campaigns", "manage")])).await;
    denied_unchanged(api,pool,adapter,token,"POST",&format!("{ROOT}/campaigns"),json!({"title":"Forbidden source reference","email_subject":"Never submitted","email_body":"<p>Never submitted</p>","template_id":template})).await;
    let plain=request(api,"POST",&format!("{ROOT}/campaigns"),Some(token),json!({"title":"Manager113 CRM-only source","email_subject":"Actual CRM authoring","email_body":"<p>Custom stored source</p>"})).await;
    assert_eq!(
        plain.0,
        StatusCode::OK,
        "Custom campaign composition does not inherit template authority: {plain:?}"
    );
    for audience in [
        json!({"members":true}),
        json!({"members":true,"member_segment":1}),
        json!({"subscribers":[{"list":"9999999"}]}),
        json!({"excluded_subscribers":[{"tag":"9999999"}]}),
        json!({"dynamic_segment":"9999999"}),
    ] {
        denied_unchanged(api,pool,adapter,token,"POST",&format!("{ROOT}/campaigns"),json!({"title":"Forbidden audience reference","email_subject":"Never submitted","email_body":"<p>Never submitted</p>","subscriber_settings":audience})).await;
    }
    update_role(api, &tokens[0], role, json!([grant("sequences", "manage")])).await;
    for audience in [
        json!({"members":true}),
        json!({"member_segment":1}),
        json!({"subscribers":[{"list":"9999999"}]}),
        json!({"excluded_subscribers":[{"tag":"9999999"}]}),
        json!({"dynamic_segment":"9999999"}),
    ] {
        denied_unchanged(
            api,
            pool,
            adapter,
            token,
            "POST",
            &format!("{ROOT}/sequences"),
            json!({"title":"Forbidden sequence source","subscriber_settings":audience}),
        )
        .await;
    }
    update_role(
        api,
        &tokens[0],
        role,
        json!([grant("automations", "create")]),
    )
    .await;
    for field in ["template_id", "actions"] {
        let mut body = json!({"title":"Unsupported automation source","trigger_type":"manual"});
        body[field] = if field == "actions" {
            json!([])
        } else {
            json!(template)
        };
        let before = durable_state(pool).await;
        let response = request(
            api,
            "POST",
            &format!("{ROOT}/automations"),
            Some(token),
            body,
        )
        .await;
        assert_eq!(
            response.0,
            StatusCode::UNPROCESSABLE_ENTITY,
            "Unsupported source fields are structural errors, not silently ignored: {response:?}"
        );
        assert_eq!(durable_state(pool).await, before);
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM crm_automations WHERE name='Unsupported automation source'",
        )
        .fetch_one(pool)
        .await
        .unwrap();
        assert_eq!(count, 0);
    }
}
async fn upload_fixture(api: &Router, token: &str, name: &str, data: Value) -> String {
    let saved = request(
        api,
        "POST",
        &format!("{ROOT}/imports"),
        Some(token),
        json!({"type":"contacts","file_name":name,"file_type":"json","data":data}),
    )
    .await;
    assert_eq!(saved.0, StatusCode::OK, "{saved:?}");
    saved.1["id"].as_str().unwrap().to_string()
}
async fn transfer_boundaries(
    api: &Router,
    pool: &PgPool,
    tokens: &[String],
    adapter: &Adapter,
    role: &mut Value,
) {
    let token = &tokens[1];
    let create = upload_fixture(
        api,
        &tokens[0],
        "manager-create.json",
        json!([{"email":"manager-plan-create@example.test","first_name":"Actual creation"}]),
    )
    .await;
    update_role(
        api,
        &tokens[0],
        role,
        json!([grant("imports", "manage"), grant("contacts", "view")]),
    )
    .await;
    denied_unchanged(
        api,
        pool,
        adapter,
        token,
        "POST",
        &format!("{ROOT}/imports/{create}/start"),
        json!({}),
    )
    .await;
    denied_unchanged(
        api,
        pool,
        adapter,
        token,
        "POST",
        &format!("{ROOT}/import"),
        json!({"file_type":"json","data":[{"email":"manager-legacy-denied@example.test"}]}),
    )
    .await;
    update_role(
        api,
        &tokens[0],
        role,
        json!([
            grant("imports", "manage"),
            grant("contacts", "view"),
            grant("contacts", "create")
        ]),
    )
    .await;
    let created = request(
        api,
        "POST",
        &format!("{ROOT}/imports/{create}/start"),
        Some(token),
        json!({}),
    )
    .await;
    assert_eq!(created.0, StatusCode::OK, "{created:?}");
    assert_eq!(created.1["created_count"], 1);
    let legacy=request(api,"POST",&format!("{ROOT}/import"),Some(token),json!({"file_type":"json","data":[{"email":"manager-legacy-authorized@example.test","first_name":"Atomic actual creation"}]})).await;
    assert_eq!(legacy.0, StatusCode::OK, "{legacy:?}");
    assert_eq!(legacy.1["created_count"], 1);
    let update = upload_fixture(
        api,
        &tokens[0],
        "manager-update.json",
        json!([{"email":"manager-plan-create@example.test","first_name":"Actual changed name"}]),
    )
    .await;
    let settings = json!({"settings":{"update_existing":true,"skip_duplicates":false}});
    denied_unchanged(
        api,
        pool,
        adapter,
        token,
        "POST",
        &format!("{ROOT}/imports/{update}/start"),
        settings.clone(),
    )
    .await;
    update_role(
        api,
        &tokens[0],
        role,
        json!([
            grant("imports", "manage"),
            grant("contacts", "view"),
            grant("contacts", "edit")
        ]),
    )
    .await;
    let updated = request(
        api,
        "POST",
        &format!("{ROOT}/imports/{update}/start"),
        Some(token),
        settings.clone(),
    )
    .await;
    assert_eq!(updated.0, StatusCode::OK, "{updated:?}");
    assert_eq!(updated.1["updated_count"], 1);
    let implicit = upload_fixture(
        api,
        &tokens[0],
        "manager-implicit-tags.json",
        json!([{"email":"manager-implicit-tags@example.test","tags":["Implicit raw tag"]}]),
    )
    .await;
    update_role(
        api,
        &tokens[0],
        role,
        json!([
            grant("imports", "manage"),
            grant("contacts", "view"),
            grant("contacts", "create")
        ]),
    )
    .await;
    denied_unchanged(
        api,
        pool,
        adapter,
        token,
        "POST",
        &format!("{ROOT}/imports/{implicit}/start"),
        json!({}),
    )
    .await;
    let null_tags:i64=sqlx::query_scalar("INSERT INTO contacts(email,first_name,tags) VALUES('manager-null-tags@example.test','Stored unknown tags',NULL) RETURNING id").fetch_one(pool).await.unwrap();
    let clearing=upload_fixture(api,&tokens[0],"manager-clear-tags.json",json!([{"email":"manager-null-tags@example.test","first_name":"Normalized full-row update"}])).await;
    update_role(
        api,
        &tokens[0],
        role,
        json!([
            grant("imports", "manage"),
            grant("contacts", "view"),
            grant("contacts", "edit")
        ]),
    )
    .await;
    denied_unchanged(
        api,
        pool,
        adapter,
        token,
        "POST",
        &format!("{ROOT}/imports/{clearing}/start"),
        settings.clone(),
    )
    .await;
    update_role(
        api,
        &tokens[0],
        role,
        json!([
            grant("imports", "manage"),
            grant("contacts", "view"),
            grant("contacts", "edit"),
            grant("tags", "edit")
        ]),
    )
    .await;
    let cleared = request(
        api,
        "POST",
        &format!("{ROOT}/imports/{clearing}/start"),
        Some(token),
        settings,
    )
    .await;
    assert_eq!(cleared.0, StatusCode::OK, "{cleared:?}");
    let actual: Value = sqlx::query_scalar("SELECT tags FROM contacts WHERE id=$1")
        .bind(null_tags)
        .fetch_one(pool)
        .await
        .unwrap();
    assert_eq!(actual, json!([]));
    let list:i64=sqlx::query_scalar("INSERT INTO crm_lists(name,title,slug) VALUES('Manager113 transfer list','Manager113 transfer list','manager113-transfer-list') RETURNING id").fetch_one(pool).await.unwrap();
    let tag:i64=sqlx::query_scalar("INSERT INTO crm_tags(name,title,slug) VALUES('Manager113 transfer tag','Manager113 transfer tag','manager113-transfer-tag') RETURNING id").fetch_one(pool).await.unwrap();
    let references = upload_fixture(
        api,
        &tokens[0],
        "manager-reference-effects.json",
        json!([{"email":"manager-reference-effects@example.test"}]),
    )
    .await;
    update_role(
        api,
        &tokens[0],
        role,
        json!([
            grant("imports", "manage"),
            grant("contacts", "view"),
            grant("contacts", "create")
        ]),
    )
    .await;
    for selected in [
        json!({"settings":{"add_to_lists":[list.to_string()]}}),
        json!({"settings":{"apply_tags":[tag.to_string()]}}),
    ] {
        denied_unchanged(
            api,
            pool,
            adapter,
            token,
            "POST",
            &format!("{ROOT}/imports/{references}/start"),
            selected,
        )
        .await;
    }
    update_role(
        api,
        &tokens[0],
        role,
        json!([grant("exports", "manage"), grant("contacts", "view")]),
    )
    .await;
    for filters in [
        json!({"list_id":list.to_string()}),
        json!({"tag":"Manager113 transfer tag"}),
    ] {
        denied_unchanged(
            api,
            pool,
            adapter,
            token,
            "POST",
            &format!("{ROOT}/exports"),
            json!({"type":"contacts","filters":filters,"fields":["email"],"file_type":"json"}),
        )
        .await;
    }
    update_role(
        api,
        &tokens[0],
        role,
        json!([
            grant("exports", "manage"),
            grant("contacts", "view"),
            grant("lists", "view"),
            grant("tags", "view")
        ]),
    )
    .await;
    let saved=request(api,"POST",&format!("{ROOT}/exports"),Some(token),json!({"type":"contacts","filters":{"list_id":list.to_string(),"tag":"Manager113 transfer tag"},"fields":["email"],"file_type":"json"})).await;
    assert_eq!(saved.0, StatusCode::OK, "{saved:?}");
    let download = saved.1["file_url"].as_str().unwrap();
    let allowed = request(api, "GET", download, Some(token), Value::Null).await;
    assert_eq!(allowed.0, StatusCode::OK, "{allowed:?}");
    assert_eq!(allowed.1, json!([]));
    update_role(
        api,
        &tokens[0],
        role,
        json!([grant("exports", "manage"), grant("contacts", "view")]),
    )
    .await;
    denied_unchanged(api, pool, adapter, token, "GET", download, Value::Null).await;
    update_role(
        api,
        &tokens[0],
        role,
        json!([
            grant("exports", "manage"),
            grant("lists", "view"),
            grant("tags", "view")
        ]),
    )
    .await;
    denied_unchanged(api, pool, adapter, token, "GET", download, Value::Null).await;
}
async fn member_source_boundaries(
    api: &Router,
    pool: &PgPool,
    tokens: &[String],
    adapter: &Adapter,
    role: &mut Value,
    contact: i64,
) {
    let admin = Some(tokens[0].as_str());
    let token = &tokens[1];
    update_role(api, &tokens[0], role, json!([grant("sequences", "manage")])).await;
    let denied_event = uuid::Uuid::new_v4();
    denied_unchanged(api,pool,adapter,token,"POST",&format!("{ROOT}/sequences/events"),json!({"event_id":denied_event,"contact_id":contact,"event_name":"manager.denied.source","properties":{}})).await;
    let event_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM crm_trigger_events WHERE source_key=$1")
            .bind(format!("custom:{denied_event}"))
            .fetch_one(pool)
            .await
            .unwrap();
    assert_eq!(
        event_count, 0,
        "A sequence execution grant cannot resolve a foreign contact or persist its event without contacts.view"
    );
    let permissions = json!([
        grant("campaigns", "manage"),
        grant("sequences", "manage"),
        grant("contacts", "view"),
        grant("deals", "view"),
        grant("templates", "view"),
        grant("lists", "view"),
        grant("tags", "view"),
        grant("segments", "view"),
        grant("dashboard", "view")
    ]);
    update_role(api, &tokens[0], role, permissions).await;
    let capabilities = request(
        api,
        "GET",
        &format!("{ROOT}/permissions"),
        Some(token),
        Value::Null,
    )
    .await;
    assert_eq!(capabilities.0, StatusCode::OK, "{capabilities:?}");
    assert_eq!(
        capabilities.1["global_admin"], false,
        "A complete CRM bundle never becomes a global member-directory identity"
    );
    sqlx::query("INSERT INTO users(email,name,password_hash,role,is_active,email_verified_at) VALUES('manager-member-history@example.test','Recorded member source','not-a-login-password','user',true,NOW())").execute(pool).await.unwrap();
    sqlx::query("INSERT INTO newsletter_subscribers(email,name,status,confirmed_at) VALUES('manager-member-history@example.test','Recorded consent','confirmed',NOW())").execute(pool).await.unwrap();
    let member=request(api,"POST",&format!("{ROOT}/campaigns"),admin,json!({"title":"Manager113 captured member campaign","email_subject":"Captured member-source intent","email_body":"<p>Recorded source</p>","subscriber_settings":{"members":true}})).await;
    assert_eq!(member.0, StatusCode::OK, "{member:?}");
    let campaign = member.1["id"].as_str().unwrap();
    let campaign_path = format!("{ROOT}/campaigns/{campaign}");
    denied_unchanged(
        api,
        pool,
        adapter,
        token,
        "GET",
        &campaign_path,
        Value::Null,
    )
    .await;
    denied_unchanged(
        api,
        pool,
        adapter,
        token,
        "POST",
        &format!("{campaign_path}/send"),
        Value::Null,
    )
    .await;
    let submitted = request(
        api,
        "POST",
        &format!("{campaign_path}/send"),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(submitted.0, StatusCode::OK, "{submitted:?}");
    assert!(submitted.1["recipients_count"].as_i64().unwrap() > 0);
    let paused = request(
        api,
        "POST",
        &format!("{campaign_path}/pause"),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(paused.0, StatusCode::OK, "{paused:?}");
    let changed = request(
        api,
        "PUT",
        &campaign_path,
        admin,
        json!({"subscriber_settings":{}}),
    )
    .await;
    assert_eq!(changed.0, StatusCode::OK, "{changed:?}");
    assert_eq!(changed.1["subscriber_settings"], json!({}));
    let captured:i64=sqlx::query_scalar("SELECT COUNT(*) FROM crm_campaign_revisions WHERE campaign_id=$1 AND definition->'subscriber_settings'->'members'='true'::jsonb").bind(campaign.parse::<i64>().unwrap()).fetch_one(pool).await.unwrap();
    assert!(captured > 0);
    for suffix in ["", "/recipients", "/deliveries"] {
        denied_unchanged(
            api,
            pool,
            adapter,
            token,
            "GET",
            &format!("{campaign_path}{suffix}"),
            Value::Null,
        )
        .await;
    }
    denied_unchanged(
        api,
        pool,
        adapter,
        token,
        "PUT",
        &campaign_path,
        json!({"title":"Must not edit protected history"}),
    )
    .await;
    denied_unchanged(
        api,
        pool,
        adapter,
        token,
        "DELETE",
        &campaign_path,
        Value::Null,
    )
    .await;
    for suffix in ["send", "pause", "resume"] {
        denied_unchanged(
            api,
            pool,
            adapter,
            token,
            "POST",
            &format!("{campaign_path}/{suffix}"),
            Value::Null,
        )
        .await;
    }
    for path in [
        format!("{ROOT}/stats"),
        format!("{ROOT}/campaigns/stats"),
        format!("{ROOT}/campaigns"),
    ] {
        denied_unchanged(api, pool, adapter, token, "GET", &path, Value::Null).await;
    }
    let sequence=request(api,"POST",&format!("{ROOT}/sequences"),admin,json!({"title":"Manager113 captured member sequence","subscriber_settings":{"members":true},"emails":[{"title":"Captured QA mail","subject":"Manager113 member sample","html_content":"<p>Captured QA source {{contact.first_name}}</p>","status":"active","delay_value":0,"delay_unit":"minutes"}]})).await;
    assert_eq!(sequence.0, StatusCode::OK, "{sequence:?}");
    let sequence = sequence.1;
    let id = sequence["id"].as_str().unwrap();
    let path = format!("{ROOT}/sequences/{id}");
    let qa = json!({"request_id":uuid::Uuid::new_v4(),"expected_version":sequence["authoring_version"],"email_id":sequence["emails"][0]["id"],"email":"manager-record@example.test"});
    denied_unchanged(
        api,
        pool,
        adapter,
        token,
        "POST",
        &format!("{path}/test"),
        qa.clone(),
    )
    .await;
    let before = provider_count(adapter);
    let sample = request(api, "POST", &format!("{path}/test"), admin, qa.clone()).await;
    assert_eq!(sample.0, StatusCode::OK, "{sample:?}");
    assert_eq!(sample.1["data"]["submissions"][0]["status"], "accepted");
    assert_eq!(provider_count(adapter), before + 1);
    let changed = request(
        api,
        "PUT",
        &path,
        admin,
        json!({"expected_version":sequence["authoring_version"],"subscriber_settings":{}}),
    )
    .await;
    assert_eq!(changed.0, StatusCode::OK, "{changed:?}");
    assert_eq!(changed.1["subscriber_settings"], json!({}));
    let captured:i64=sqlx::query_scalar("SELECT COUNT(*) FROM crm_sequence_revisions WHERE sequence_id=$1 AND definition->'subscriber_settings'->'members'='true'::jsonb").bind(id.parse::<i64>().unwrap()).fetch_one(pool).await.unwrap();
    assert!(captured > 0);
    for suffix in [
        "",
        "/stats",
        "/emails",
        "/subscribers",
        "/tests",
        "/triggers",
    ] {
        denied_unchanged(
            api,
            pool,
            adapter,
            token,
            "GET",
            &format!("{path}{suffix}"),
            Value::Null,
        )
        .await;
    }
    denied_unchanged(
        api,
        pool,
        adapter,
        token,
        "POST",
        &format!("{path}/test"),
        qa,
    )
    .await;
    denied_unchanged(api,pool,adapter,token,"PUT",&path,json!({"expected_version":changed.1["authoring_version"],"title":"Must not overwrite captured source"})).await;
    denied_unchanged(api,pool,adapter,token,"POST",&format!("{path}/emails"),json!({"title":"Forbidden mail","subject":"No source write","html_content":"<p>No new revision</p>"})).await;
    for suffix in ["subscribe", "unsubscribe"] {
        denied_unchanged(
            api,
            pool,
            adapter,
            token,
            "POST",
            &format!("{path}/{suffix}"),
            json!({"contact_ids":[contact]}),
        )
        .await;
    }
    denied_unchanged(api, pool, adapter, token, "DELETE", &path, Value::Null).await;
    let event = uuid::Uuid::new_v4();
    let captured=request(api,"POST",&format!("{ROOT}/sequences/events"),Some(token),json!({"event_id":event,"contact_id":contact,"event_name":"manager.permission.recorded","properties":{"recorded":"explicit custom source"}})).await;
    assert_eq!(
        captured.0,
        StatusCode::OK,
        "Actual consented event capture remains distinct from member-directory selection: {captured:?}"
    );
    assert_eq!(captured.1["event_recorded"], true);
    let replay=request(api,"POST",&format!("{ROOT}/sequences/events"),Some(token),json!({"event_id":event,"contact_id":contact,"event_name":"manager.permission.recorded","properties":{"recorded":"explicit custom source"}})).await;
    assert_eq!(replay.0, StatusCode::OK, "{replay:?}");
    assert_eq!(replay.1, captured.1);
}

async fn conditional_boundaries(
    api: &Router,
    pool: &PgPool,
    tokens: &[String],
    adapter: &Adapter,
    role: &mut Value,
    contact: i64,
    user: i64,
) {
    native_picker_boundary(api, pool, tokens, adapter, role, contact).await;
    catalog_effect_boundaries(api, pool, tokens, adapter, role, contact, user).await;
    authoring_boundaries(api, pool, tokens, adapter, role).await;
    transfer_boundaries(api, pool, tokens, adapter, role).await;
    member_source_boundaries(api, pool, tokens, adapter, role, contact).await;
}
