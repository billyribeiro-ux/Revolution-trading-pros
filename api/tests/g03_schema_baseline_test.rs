//! G0.3 — proves the fresh-environment schema baseline mechanism.
//!
//! Audit item G0.3: the historical sqlx migration chain is not reproducible
//! from zero. The remediation ships `api/migrations/schema.sql` (a full
//! reconstructed baseline) plus a `_sqlx_migrations` seed so fresh
//! environments can stand up the current schema in one shot WITHOUT
//! changing how production loads migrations.
//!
//! This test is the definitive proof that the mechanism satisfies the
//! EXACT call the application makes at startup
//! (`sqlx::migrate!("./migrations").run(&pool)` in `src/db/mod.rs`):
//! after loading `schema.sql` and seeding `_sqlx_migrations` with the
//! real SHA-384 checksums of the committed migration files, the embedded
//! migrator must validate the baseline and execute forward migrations
//! (including their schema changes and structural defaults) — i.e. production's migration path is fully honored.
//!
//! Gated on `G03_VERIFY_DB_URL` (a throwaway empty database) so it does
//! not run in the standard no-DB test set. It creates only its own
//! objects in that throwaway DB.
//!
//!   G03_VERIFY_DB_URL=postgres://postgres@/g03verify?host=/tmp&port=55433 \
//!     cargo test --test g03_schema_baseline_test -- --nocapture

use std::path::Path;

#[tokio::test]
async fn schema_and_seed_apply_forward_schema_and_defaults() {
    let Ok(db_url) = std::env::var("G03_VERIFY_DB_URL") else {
        eprintln!("G03_VERIFY_DB_URL not set — skipping (gated DB proof).");
        return;
    };

    let mig_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");
    let schema_sql = std::fs::read_to_string(mig_dir.join("schema.sql"))
        .expect("api/migrations/schema.sql must exist");

    // Single connection so the session-local search_path we set after
    // loading schema.sql (which resets it to '') stays in effect for the
    // seed + the embedded-migrator run.
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(&db_url)
        .await
        .expect("connect to G03_VERIFY_DB_URL");

    // Clean slate in the throwaway DB.
    sqlx::raw_sql("DROP SCHEMA public CASCADE; CREATE SCHEMA public;")
        .execute(&pool)
        .await
        .expect("reset public schema");

    // Step 1: load the reconstructed baseline schema.
    sqlx::raw_sql(sqlx::AssertSqlSafe(schema_sql.as_str()))
        .execute(&pool)
        .await
        .expect("schema.sql must load with zero errors on an empty DB");

    // schema.sql ends with `set_config('search_path','')`; restore it so
    // unqualified object names resolve, and so this matches how the app
    // (default search_path) sees the schema.
    sqlx::raw_sql("SELECT pg_catalog.set_config('search_path', 'public', false);")
        .execute(&pool)
        .await
        .expect("restore search_path");

    // Seed only the versions represented by schema.sql. Never mark new
    // forward migrations applied without executing their DDL/default inserts.
    let seed_sql = std::fs::read_to_string(mig_dir.join("sqlx-seed.sql"))
        .expect("baseline migration ledger must exist");
    sqlx::raw_sql(sqlx::AssertSqlSafe(seed_sql.as_str()))
        .execute(&pool)
        .await
        .expect("seed the historical ledger");

    // Step 3: run the EXACT embedded migrator the app runs at startup.
    // If any seeded checksum disagreed with the embedded migration sqlx
    // would return VersionMismatch here; success proves the chain is
    // treated as fully applied and nothing is re-run.
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("embedded migrator must validate the baseline and execute forward migrations");

    let applied: i64 = sqlx::query_scalar("SELECT count(*) FROM public._sqlx_migrations")
        .fetch_one(&pool)
        .await
        .expect("count _sqlx_migrations");
    let expected = sqlx::migrate!("./migrations").iter().count() as i64;
    assert_eq!(
        applied, expected,
        "all baseline and forward migrations applied"
    );
    let defaults: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM cms_site_settings")
        .fetch_one(&pool)
        .await
        .expect("structural defaults");
    assert_eq!(
        defaults, 1,
        "forward defaults migration must actually execute"
    );
    let scan_column: bool = sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='media' AND column_name='scan_status')")
        .fetch_one(&pool).await.expect("forward schema column");
    assert!(
        scan_column,
        "forward schema migration must actually execute"
    );

    let tables: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM information_schema.tables
         WHERE table_schema = 'public' AND table_name <> '_sqlx_migrations'",
    )
    .fetch_one(&pool)
    .await
    .expect("count tables");
    assert!(
        tables >= 200,
        "expected the full reconstructed schema (~204 tables), got {tables}"
    );

    eprintln!(
        "G0.3 PROOF OK: schema.sql + seed -> baseline validation and forward migration; \
               {applied} versions applied, {tables} tables."
    );
}
