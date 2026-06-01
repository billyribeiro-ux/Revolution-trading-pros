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
//! migrator must run as a clean no-op (no `VersionMismatch`, nothing
//! pending) — i.e. production's migration path is fully honored.
//!
//! Gated on `G03_VERIFY_DB_URL` (a throwaway empty database) so it does
//! not run in the standard no-DB test set. It creates only its own
//! objects in that throwaway DB.
//!
//!   G03_VERIFY_DB_URL=postgres://postgres@/g03verify?host=/tmp&port=55433 \
//!     cargo test --test g03_schema_baseline_test -- --nocapture

use std::path::Path;

#[tokio::test]
async fn schema_sql_plus_seed_makes_embedded_migrator_a_noop() {
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
    sqlx::raw_sql(&schema_sql)
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

    // Step 2: seed _sqlx_migrations with the REAL SHA-384 of every
    // committed migration file (exactly what sqlx::migrate! embedded).
    sqlx::query(
        r"CREATE TABLE IF NOT EXISTS public._sqlx_migrations (
               version        BIGINT PRIMARY KEY,
               description    TEXT        NOT NULL,
               installed_on   TIMESTAMPTZ NOT NULL DEFAULT now(),
               success        BOOLEAN     NOT NULL,
               checksum       BYTEA       NOT NULL,
               execution_time BIGINT      NOT NULL
           )",
    )
    .execute(&pool)
    .await
    .expect("create _sqlx_migrations");

    let mut files: Vec<_> = std::fs::read_dir(&mig_dir)
        .expect("read migrations dir")
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| {
            n.ends_with(".sql")
                && n != "schema.sql"
                && n.chars()
                    .next()
                    .map(|c| c.is_ascii_digit())
                    .unwrap_or(false)
        })
        .collect();
    files.sort();
    assert_eq!(files.len(), 60, "expected exactly 60 committed migrations");

    for f in &files {
        let bytes = std::fs::read(mig_dir.join(f)).expect("read migration");
        let digits: String = f.chars().take_while(|c| c.is_ascii_digit()).collect();
        let version: i64 = digits.parse().expect("version prefix");
        let desc = f
            .trim_start_matches(|c: char| c.is_ascii_digit())
            .trim_start_matches('_')
            .trim_end_matches(".sql")
            .to_string();
        // sqlx uses SHA-384 of the migration SQL for its checksum.
        use sha2::{Digest, Sha384};
        let checksum = Sha384::digest(&bytes).to_vec();
        sqlx::query(
            "INSERT INTO public._sqlx_migrations
                 (version, description, success, checksum, execution_time)
             VALUES ($1, $2, true, $3, 0)
             ON CONFLICT (version) DO NOTHING",
        )
        .bind(version)
        .bind(&desc)
        .bind(&checksum)
        .execute(&pool)
        .await
        .expect("seed _sqlx_migrations row");
    }

    // Step 3: run the EXACT embedded migrator the app runs at startup.
    // If any seeded checksum disagreed with the embedded migration sqlx
    // would return VersionMismatch here; success proves the chain is
    // treated as fully applied and nothing is re-run.
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("embedded migrator must be a clean no-op after baseline+seed");

    let applied: i64 = sqlx::query_scalar("SELECT count(*) FROM public._sqlx_migrations")
        .fetch_one(&pool)
        .await
        .expect("count _sqlx_migrations");
    assert_eq!(applied, 60, "all 60 historical versions marked applied");

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
        "G0.3 PROOF OK: schema.sql + seed -> embedded migrator no-op; \
               {applied} versions applied, {tables} tables."
    );
}
