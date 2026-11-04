//! SMTP protocol fixture; NOOP checks may not submit any message.
use super::{app, request};
use axum::{Router, http::StatusCode};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde_json::{Value, json};
use sqlx::PgPool;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

pub(super) async fn proof(api: &Router, pool: &PgPool, tokens: &[String]) {
    assert_eq!(std::env::var("ENVIRONMENT").as_deref(), Ok("development"));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:19025")
        .await
        .unwrap();
    let checks = Arc::new(AtomicUsize::new(0));
    let authenticated = Arc::new(AtomicUsize::new(0));
    let seen = checks.clone();
    let auths = authenticated.clone();
    let server = tokio::spawn(async move {
        loop {
            let (socket, _) = listener.accept().await.unwrap();
            let seen = seen.clone();
            let auths = auths.clone();
            tokio::spawn(async move {
                let (read, mut write) = socket.into_split();
                let mut read = BufReader::new(read);
                write
                    .write_all(b"220 fixture.example.test ESMTP\r\n")
                    .await
                    .unwrap();
                let mut authorized = false;
                loop {
                    let mut line = String::new();
                    let size = tokio::time::timeout(
                        std::time::Duration::from_secs(5),
                        read.read_line(&mut line),
                    )
                    .await
                    .unwrap()
                    .unwrap();
                    if size == 0 {
                        break;
                    }
                    assert!(size < 4096);
                    let response = if line.starts_with("EHLO ") {
                        "250-fixture.example.test\r\n250-AUTH PLAIN\r\n250 SMTPUTF8\r\n"
                    } else if let Some(encoded) = line.trim().strip_prefix("AUTH PLAIN ") {
                        let decoded = STANDARD.decode(encoded).unwrap();
                        authorized = decoded == b"\0fixture-user\0fixture-password";
                        if authorized {
                            auths.fetch_add(1, Ordering::SeqCst);
                            "235 2.7.0 Accepted\r\n"
                        } else {
                            "535 5.7.8 Rejected\r\n"
                        }
                    } else if line.trim() == "NOOP" {
                        assert!(authorized, "SMTP check must authenticate");
                        seen.fetch_add(1, Ordering::SeqCst);
                        "250 2.0.0 OK\r\n"
                    } else if line.trim() == "QUIT" {
                        let _ = write.write_all(b"221 Bye\r\n").await;
                        break;
                    } else {
                        assert!(
                            !line.starts_with("MAIL ") && !line.starts_with("DATA"),
                            "Connection verification cannot send email"
                        );
                        "500 Unsupported\r\n"
                    };
                    if write.write_all(response.as_bytes()).await.is_err() {
                        break;
                    }
                }
            });
        }
    });
    let id:i64=sqlx::query_scalar("INSERT INTO users(email,name,password_hash,role,email_verified_at) VALUES('smtp-super@example.test','SMTP proof','not-a-login-password','super_admin',NOW()) RETURNING id").fetch_one(pool).await.unwrap();
    let token =
        revolution_api::utils::create_jwt(id, &std::env::var("JWT_SECRET").unwrap(), 1).unwrap();
    let fields = json!({"provider":"smtp","host":"127.0.0.1","port":19025,"username":"fixture-user","password":"fixture-password","encryption":"null","from_address":"smtp-sender@example.test","from_name":"Fixture sender"});
    for (credential, status) in [
        (None, StatusCode::UNAUTHORIZED),
        (Some(tokens[0].as_str()), StatusCode::FORBIDDEN),
        (Some(tokens[1].as_str()), StatusCode::FORBIDDEN),
    ] {
        assert_eq!(
            request(
                api,
                "POST",
                "/api/admin/email/settings",
                credential,
                fields.clone()
            )
            .await
            .0,
            status
        );
        assert_eq!(
            request(
                api,
                "POST",
                "/api/admin/email/settings/test",
                credential,
                fields.clone()
            )
            .await
            .0,
            status
        );
    }
    let empty = request(
        api,
        "GET",
        "/api/admin/email/settings",
        Some(tokens[0].as_str()),
        Value::Null,
    )
    .await;
    assert_eq!(empty.0, StatusCode::OK, "{empty:?}");
    assert_eq!(empty.1["has_password"], false);
    assert_eq!(empty.1["status"], "pending");
    let saved = request(
        api,
        "POST",
        "/api/admin/email/settings",
        Some(&token),
        fields.clone(),
    )
    .await;
    assert_eq!(saved.0, StatusCode::OK, "{saved:?}");
    assert_eq!(saved.1["settings"]["has_password"], true);
    assert!(!saved.1.to_string().contains("fixture-password"));
    let encrypted: Value =
        sqlx::query_scalar("SELECT value FROM settings WHERE key='email.smtp.sandbox'")
            .fetch_one(pool)
            .await
            .unwrap();
    assert!(encrypted["encrypted"].as_str().unwrap().starts_with("v1:"));
    assert!(!encrypted.to_string().contains("fixture-password"));
    let mut retained = fields.clone();
    retained.as_object_mut().unwrap().remove("password");
    let retained_save = request(
        api,
        "POST",
        "/api/admin/email/settings",
        Some(&token),
        retained.clone(),
    )
    .await;
    assert_eq!(retained_save.0, StatusCode::OK, "{retained_save:?}");
    let tested = request(
        api,
        "POST",
        "/api/admin/email/settings/test",
        Some(&token),
        retained.clone(),
    )
    .await;
    assert_eq!(tested.0, StatusCode::OK, "{tested:?}");
    assert_eq!(tested.1["success"], true, "{tested:?}");
    assert_eq!(tested.1["email_sent"], false);
    assert!(checks.load(Ordering::SeqCst) > 0);
    assert!(authenticated.load(Ordering::SeqCst) > 0);
    let cold = app(revolution_api::db::Database { pool: pool.clone() }, None).await;
    let loaded = request(
        &cold,
        "GET",
        "/api/admin/email/settings",
        Some(&token),
        Value::Null,
    )
    .await;
    assert_eq!(loaded.0, StatusCode::OK, "{loaded:?}");
    assert_eq!(loaded.1["status"], "verified");
    assert_eq!(loaded.1["has_password"], true);
    assert!(loaded.1.get("password").is_none());
    assert!(loaded.1["last_verified_at"].is_string());
    let mut denied = fields.clone();
    denied["password"] = json!("fixture-reject");
    let failed = request(
        api,
        "POST",
        "/api/admin/email/settings/test",
        Some(&token),
        denied,
    )
    .await;
    assert_eq!(failed.0, StatusCode::OK, "{failed:?}");
    assert_eq!(failed.1["success"], false);
    assert!(!failed.1.to_string().contains("fixture-reject"));
    let mut tls = fields.clone();
    tls["encryption"] = json!("tls");
    let required = request(
        api,
        "POST",
        "/api/admin/email/settings/test",
        Some(&token),
        tls,
    )
    .await;
    assert_eq!(required.0, StatusCode::OK, "{required:?}");
    assert_eq!(
        required.1["success"], false,
        "Missing STARTTLS cannot downgrade to plaintext"
    );
    let before: Value =
        sqlx::query_scalar("SELECT value FROM settings WHERE key='email.smtp.sandbox'")
            .fetch_one(pool)
            .await
            .unwrap();
    sqlx::query("CREATE FUNCTION admin_move_reject_smtp_audit() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN IF NEW.message='SMTP configuration saved' THEN RAISE EXCEPTION 'fixture audit failure'; END IF; RETURN NEW; END $$").execute(pool).await.unwrap();
    sqlx::query("CREATE TRIGGER admin_move_reject_smtp_audit BEFORE INSERT ON crm_operational_logs FOR EACH ROW EXECUTE FUNCTION admin_move_reject_smtp_audit()").execute(pool).await.unwrap();
    let rollback = request(
        api,
        "POST",
        "/api/admin/email/settings",
        Some(&token),
        fields.clone(),
    )
    .await;
    sqlx::query("DROP TRIGGER admin_move_reject_smtp_audit ON crm_operational_logs")
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("DROP FUNCTION admin_move_reject_smtp_audit()")
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(rollback.0, StatusCode::SERVICE_UNAVAILABLE, "{rollback:?}");
    let after: Value =
        sqlx::query_scalar("SELECT value FROM settings WHERE key='email.smtp.sandbox'")
            .fetch_one(pool)
            .await
            .unwrap();
    assert_eq!(before, after);
    sqlx::query("ALTER TABLE settings RENAME TO admin_move_smtp_settings_unavailable")
        .execute(pool)
        .await
        .unwrap();
    let outage = request(
        api,
        "GET",
        "/api/admin/email/settings",
        Some(&token),
        Value::Null,
    )
    .await;
    sqlx::query("ALTER TABLE admin_move_smtp_settings_unavailable RENAME TO settings")
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(outage.0, StatusCode::SERVICE_UNAVAILABLE, "{outage:?}");
    server.abort();
}
