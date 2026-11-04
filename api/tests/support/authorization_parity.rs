use super::request;
use axum::{Router, http::StatusCode};
use serde_json::Value;
use sqlx::PgPool;
pub(super) async fn proof(api: &Router, pool: &PgPool) {
    use tower::ServiceExt;
    let anonymous = api
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .uri("/api/auth/me")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(anonymous.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(anonymous.headers()["cache-control"], "no-store");
    let id:i64=sqlx::query_scalar("INSERT INTO users(email,name,password_hash,role,email_verified_at) VALUES('configured-auth-proof@example.test','Configured authorization fixture','not-a-login-password','user',NOW()) RETURNING id").fetch_one(pool).await.unwrap();
    let token =
        revolution_api::utils::create_jwt(id, &std::env::var("JWT_SECRET").unwrap(), 1).unwrap();
    for role in [
        "user",
        "member",
        "editor",
        "marketing",
        "manager",
        "admin",
        "ADMIN",
        "administrator",
        "developer",
        "DEVELOPER",
        "super_admin",
        "super-admin",
        "superadmin",
    ] {
        for verified in [false, true] {
            sqlx::query("UPDATE users SET role=$2,email_verified_at=CASE WHEN $3 THEN NOW() ELSE NULL END WHERE id=$1").bind(id).bind(role).bind(verified).execute(pool).await.unwrap();
            let (status, user) =
                request(api, "GET", "/api/auth/me", Some(&token), Value::Null).await;
            assert_eq!(status, StatusCode::OK, "{user}");
            assert_eq!(
                user["role"], role,
                "The effective capability must not rewrite the account's DB role"
            );
            let admin = matches!(
                role,
                "admin"
                    | "ADMIN"
                    | "administrator"
                    | "developer"
                    | "DEVELOPER"
                    | "super_admin"
                    | "super-admin"
                    | "superadmin"
            );
            let base_super = matches!(role, "super_admin" | "super-admin" | "superadmin");
            let superadmin = base_super || (admin && verified);
            let developer = matches!(role, "developer" | "DEVELOPER") || (admin && verified);
            assert_eq!(user["is_admin"], admin, "{role} verified={verified}");
            assert_eq!(
                user["is_superadmin"], superadmin,
                "{role} verified={verified}"
            );
            assert_eq!(
                user["is_developer"], developer,
                "{role} verified={verified}"
            );
            assert_eq!(
                request(api, "GET", "/fixture/superadmin", Some(&token), Value::Null)
                    .await
                    .0,
                if superadmin {
                    StatusCode::NO_CONTENT
                } else {
                    StatusCode::FORBIDDEN
                },
                "{role} verified={verified}"
            );
            assert_eq!(
                request(
                    api,
                    "GET",
                    "/api/admin/email/subscribers/stats",
                    Some(&token),
                    Value::Null
                )
                .await
                .0,
                if admin {
                    StatusCode::OK
                } else {
                    StatusCode::FORBIDDEN
                },
                "{role}"
            );
        }
    }
    let authenticated = api
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .uri("/api/auth/me")
                .header("Authorization", format!("Bearer {token}"))
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(authenticated.status(), StatusCode::OK);
    assert_eq!(authenticated.headers()["cache-control"], "no-store");
    // A role revocation is effective on the next request with the same JWT.
    sqlx::query("UPDATE users SET role='user',email_verified_at=NOW() WHERE id=$1")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
    let (_, user) = request(api, "GET", "/api/auth/me", Some(&token), Value::Null).await;
    assert_eq!(user["is_superadmin"], false);
    assert_eq!(user["is_developer"], false);
    assert_eq!(
        request(api, "GET", "/fixture/superadmin", Some(&token), Value::Null)
            .await
            .0,
        StatusCode::FORBIDDEN
    );
}
