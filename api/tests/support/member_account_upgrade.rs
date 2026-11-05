//! A genuine pre-117 upgrade, with owned legacy identities preserved for later audience proofs.
use axum::http::StatusCode;
use revolution_api::{services::account_links, utils};
use serde_json::{Value, json};
use sqlx::{PgPool, Postgres, QueryBuilder, postgres::PgPoolOptions};
use uuid::Uuid;

pub struct UpgradeFixture {
    pub pool: PgPool,
    pub duplicate_ids: [i64; 2],
    pub duplicate_emails: [String; 2],
    pub database_name: String,
}
async fn legacy_user(pool: &PgPool, email: &str, created: &str) -> i64 {
    sqlx::query_scalar("INSERT INTO users(name,email,password_hash,role,is_active,email_verified_at,created_at,updated_at) VALUES('Owned legacy upgrade account',$1,'not-a-login-password','user',true,'2020-01-02 03:04:05'::timestamp,$2::text::timestamp,'2020-01-02 03:04:05'::timestamp) RETURNING id")
        .bind(email).bind(created).fetch_one(pool).await.unwrap()
}
async fn dates(pool: &PgPool) -> Value {
    sqlx::query_scalar("SELECT jsonb_build_object('users',(SELECT jsonb_agg(jsonb_build_array(id,created_at::text,updated_at::text,email_verified_at::text) ORDER BY id) FROM users),'orders',(SELECT jsonb_agg(jsonb_build_array(id,created_at::text,completed_at::text) ORDER BY id) FROM orders))")
        .fetch_one(pool).await.unwrap()
}
pub async fn proof(main: &PgPool) -> UpgradeFixture {
    let options = (*main.connect_options()).clone();
    assert!(
        options
            .get_database()
            .unwrap()
            .starts_with("admin_move_verify")
    );
    assert_eq!(options.get_host(), "127.0.0.1");
    let database_name = format!("admin_move_verify_upgrade_{}", Uuid::new_v4().simple());
    assert!(
        database_name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_')
    );
    // The only dynamic identifier is the generated owned fixture name above.
    QueryBuilder::<Postgres>::new("CREATE DATABASE ")
        .push(&database_name)
        .build()
        .execute(main)
        .await
        .unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .after_connect(|connection, _| {
            Box::pin(async move {
                sqlx::query("SET TIME ZONE 'America/New_York'")
                    .execute(connection)
                    .await?;
                Ok(())
            })
        })
        .connect_with(options.database(&database_name))
        .await
        .unwrap();
    let mut connection = pool.acquire().await.unwrap();
    sqlx::raw_sql(include_str!("../../migrations/schema.sql"))
        .execute(&mut *connection)
        .await
        .unwrap();
    sqlx::query("SET search_path TO public")
        .execute(&mut *connection)
        .await
        .unwrap();
    sqlx::raw_sql(include_str!("../../migrations/sqlx-seed.sql"))
        .execute(&mut *connection)
        .await
        .unwrap();
    drop(connection);
    // SQLx 0.9's real run_to applies and validates the actual immutable historical migrations.
    sqlx::migrate!("./migrations")
        .run_to(115, &pool)
        .await
        .unwrap();
    let version: i64 =
        sqlx::query_scalar("SELECT MAX(version) FROM _sqlx_migrations WHERE success")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(version, 115);
    let suffix = Uuid::new_v4().simple().to_string();
    let duplicate_emails = [
        format!("Legacy-{suffix}@example.test"),
        format!("legacy-{suffix}@example.test"),
    ];
    let duplicate_ids = [
        legacy_user(&pool, &duplicate_emails[0], "2020-01-02 03:04:05").await,
        legacy_user(&pool, &duplicate_emails[1], "2021-02-03 04:05:06").await,
    ];
    let known_hash = utils::hash_password_async("UPGRADECODE1234").await.unwrap();
    let mut material = Vec::new();
    for (index, codes) in [json!(["ABCD-1234", "EFGH-5678"]), json!([known_hash.clone()]), json!(["$argon2bad"]),
        json!(["$argon2i$v=19$m=65536,t=3,p=4$aaaaaaaaaaaaaaaaaaaaaa$bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"]),
        json!(["ABCD-1234", "not a recovery code"]), json!([17]), json!({"legacy":"bad"})].into_iter().enumerate() {
        let id = legacy_user(&pool, &format!("upgrade-mfa-{index}-{suffix}@example.test"), "2020-01-02 03:04:05").await;
        sqlx::query("INSERT INTO user_mfa_secrets(user_id,totp_secret,totp_verified_at,backup_codes,backup_codes_used,backup_codes_generated_at) VALUES($1,'JBSWY3DPEHPK3PXPJBSWY3DPEHPK3PXP',NOW(),$2,0,NOW())")
            .bind(id).bind(codes).execute(&pool).await.unwrap();
        sqlx::query("UPDATE users SET mfa_enabled=true WHERE id=$1").bind(id).execute(&pool).await.unwrap();
        material.push(id);
    }
    let gone = legacy_user(
        &pool,
        &format!("legacy-deleted-{suffix}@example.test"),
        "2020-01-02 03:04:05",
    )
    .await;
    sqlx::query(
        "UPDATE users SET role='deleted',password_hash='DELETED',is_active=false WHERE id=$1",
    )
    .bind(gone)
    .execute(&pool)
    .await
    .unwrap();
    let infinite = legacy_user(
        &pool,
        &format!("upgrade-infinite-{suffix}@example.test"),
        "infinity",
    )
    .await;
    let negative = legacy_user(
        &pool,
        &format!("upgrade-negative-{suffix}@example.test"),
        "-infinity",
    )
    .await;
    for (id, completed) in [(infinite, "infinity"), (negative, "-infinity")] {
        sqlx::query("INSERT INTO orders(user_id,order_number,status,subtotal,total,currency,created_at,completed_at) VALUES($1,$2,'paid',1,1,'USD','2020-01-01'::timestamp,$3::text::timestamp)")
            .bind(id).bind(format!("upgrade-{id}-{suffix}")).bind(completed).execute(&pool).await.unwrap();
    }
    let (legacy_verify, legacy_verify_hash) = utils::generate_verification_token();
    sqlx::query("INSERT INTO email_verification_tokens(user_id,token,expires_at) VALUES($1,$2,NOW()+INTERVAL '1 hour')").bind(duplicate_ids[0]).bind(legacy_verify_hash).execute(&pool).await.unwrap();
    let (legacy_reset, legacy_reset_hash) = utils::generate_verification_token();
    sqlx::query("INSERT INTO password_resets(email,token,expires_at) VALUES($1,$2,(NOW() AT TIME ZONE 'UTC')+INTERVAL '1 hour')").bind(&duplicate_emails[0]).bind(legacy_reset_hash).execute(&pool).await.unwrap();
    let now = chrono::Utc::now().timestamp();
    let old_jwt=jsonwebtoken::encode(&jsonwebtoken::Header::default(),&json!({"sub":duplicate_ids[0],"iat":now,"exp":now+3600,"token_type":"access","tv":0,"jti":Uuid::new_v4()}),&jsonwebtoken::EncodingKey::from_secret(std::env::var("JWT_SECRET").unwrap().as_bytes())).unwrap();
    let before = dates(&pool).await;
    sqlx::migrate!("./migrations")
        .run_to(117, &pool)
        .await
        .unwrap();
    assert_eq!(
        before,
        dates(&pool).await,
        "No historical user/order wall time was backfilled"
    );
    let claims =
        utils::verify_jwt(&old_jwt, &std::env::var("JWT_SECRET").unwrap(), "access").unwrap();
    let denied = revolution_api::services::account_authority::authenticate(&pool, &claims)
        .await
        .err()
        .expect("Pre-rollout JWT must require reauthentication");
    assert_eq!(denied.0, StatusCode::UNAUTHORIZED);
    let latest: i64 = sqlx::query_scalar("SELECT MAX(version) FROM _sqlx_migrations WHERE success")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(latest, 117);
    let both: Vec<(i64, String)> =
        sqlx::query_as("SELECT id,email FROM users WHERE LOWER(email)=LOWER($1) ORDER BY id")
            .bind(&duplicate_emails[0])
            .fetch_all(&pool)
            .await
            .unwrap();
    assert_eq!(
        both,
        duplicate_ids
            .into_iter()
            .zip(duplicate_emails.iter().cloned())
            .collect::<Vec<_>>()
    );
    let conflict = sqlx::query("INSERT INTO users(name,email,password_hash,role) VALUES('Must reject third identity',$1,'not-a-login-password','user')")
        .bind(duplicate_emails[0].to_uppercase()).execute(&pool).await.unwrap_err();
    assert!(conflict.as_database_error().unwrap().is_unique_violation());
    for (index, id) in material.into_iter().enumerate() {
        let hashes: Option<Value> =
            sqlx::query_scalar("SELECT backup_codes FROM user_mfa_secrets WHERE user_id=$1")
                .bind(id)
                .fetch_one(&pool)
                .await
                .unwrap();
        if index == 0 {
            let values = hashes.unwrap();
            let array = values.as_array().unwrap();
            assert_eq!(array.len(), 2);
            assert!(
                utils::verify_password_async("ABCD1234", array[0].as_str().unwrap())
                    .await
                    .unwrap()
            );
            assert!(!values.to_string().contains("ABCD"));
        } else if index == 1 {
            let values = hashes.unwrap();
            assert_eq!(
                values,
                json!([known_hash]),
                "A complete existing Argon2id hash is preserved"
            );
        } else {
            assert!(
                hashes.is_none(),
                "Malformed array {index} is unavailable, not guessed valid"
            );
        }
    }
    for id in [infinite, negative] {
        let value: Value = sqlx::query_scalar("SELECT jsonb_build_object('signup',days_since_signup,'order',days_since_last_order) FROM admin_member_facts WHERE id=$1").bind(id).fetch_one(&pool).await.unwrap();
        assert!(value["signup"].is_null() && value["order"].is_null());
    }
    let removed: (bool, bool) = sqlx::query_as("SELECT deleted_at IS NULL,NOT EXISTS(SELECT 1 FROM admin_member_facts WHERE id=$1) FROM users WHERE id=$1").bind(gone).fetch_one(&pool).await.unwrap();
    assert_eq!(
        removed,
        (true, true),
        "Explicit legacy marker has no invented deleted_at"
    );
    assert_eq!(
        account_links::verify(&pool, &legacy_verify)
            .await
            .unwrap_err()
            .status,
        StatusCode::BAD_REQUEST.as_str()
    );
    let hash = utils::hash_password_async("NewCedar8!JuniperRidge")
        .await
        .unwrap();
    assert_eq!(
        account_links::reset(&pool, &legacy_reset, &duplicate_emails[0], hash)
            .await
            .unwrap_err()
            .status,
        StatusCode::BAD_REQUEST.as_str()
    );
    let context = json!({"schema_version":1,"database_name":database_name,"duplicate_ids":duplicate_ids,"duplicate_emails":duplicate_emails,"upgraded_from":115,"upgraded_to":117,"cluster_owned_by_runner":true});
    let fixture_dir =
        std::env::var("ADMIN_VERIFY_FIXTURE_DIR").expect("Owned fixture directory is required");
    let fixture_dir = std::path::Path::new(&fixture_dir);
    assert!(
        fixture_dir.is_absolute()
            && fixture_dir
                .file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("revolution-email-proof.")
    );
    std::fs::write(
        fixture_dir.join("member-upgrade-context.json"),
        serde_json::to_vec_pretty(&context).unwrap(),
    )
    .unwrap();
    eprintln!("Retained real legacy upgrade context: {}", context);
    UpgradeFixture {
        pool,
        duplicate_ids,
        duplicate_emails,
        database_name,
    }
}
