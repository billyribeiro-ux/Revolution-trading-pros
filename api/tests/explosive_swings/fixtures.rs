//! Test Fixtures and Helpers - Explosive Swings Feature
//! ═══════════════════════════════════════════════════════════════════════════════════
//! Apple Principal Engineer ICT Level 7 Grade - January 2026
//!
//! Comprehensive test infrastructure for Explosive Swings integration tests:
//! - Test database setup/teardown with transaction isolation
//! - Factory functions for creating test data
//! - Authentication helpers for admin, member, and unauthenticated scenarios
//! - TOS format string generation utilities
//!
//! Design Principles:
//! - Isolated: Each test runs in its own transaction (rollback on completion)
//! - Deterministic: No flaky tests, predictable data generation
//! - Fast: Uses connection pooling and transaction rollback
//! - Comprehensive: Covers edge cases and error paths

use axum::{body::Body, Router};
use chrono::{Datelike, Duration, NaiveDate, Utc};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use sqlx::PgPool;
use std::sync::{atomic::AtomicI64, Once};

use revolution_api::{
    cache::{CacheInvalidator, CacheService},
    config::Config,
    db::Database,
    routes,
    routes::realtime::EventBroadcaster,
    routes::websocket::WsConnectionManager,
    services::Services,
    utils::{create_jwt, hash_password},
    AppState,
};

// ═══════════════════════════════════════════════════════════════════════════════════
// TEST ENVIRONMENT INITIALIZATION
// ═══════════════════════════════════════════════════════════════════════════════════

static INIT: Once = Once::new();
static TEST_USER_COUNTER: AtomicI64 = AtomicI64::new(1000);

/// Initialize test environment variables once (thread-safe)
pub fn init_test_env() {
    INIT.call_once(|| {
        // Load .env.test first, then .env as fallback
        dotenvy::from_filename(".env.test").ok();
        dotenvy::dotenv().ok();

        // Set required env vars with test defaults if not already set
        if std::env::var("JWT_SECRET").is_err() {
            std::env::set_var(
                "JWT_SECRET",
                "test-jwt-secret-minimum-32-characters-for-testing-explosive-swings",
            );
        }
        if std::env::var("DATABASE_URL").is_err() {
            std::env::set_var(
                "DATABASE_URL",
                "postgres://test:test@localhost:5432/test_db",
            );
        }
        if std::env::var("REDIS_URL").is_err() {
            std::env::set_var("REDIS_URL", "redis://localhost:6379");
        }
        if std::env::var("STRIPE_SECRET_KEY").is_err() {
            std::env::set_var("STRIPE_SECRET_KEY", "sk_test_mock_key");
        }
        if std::env::var("STRIPE_WEBHOOK_SECRET").is_err() {
            std::env::set_var("STRIPE_WEBHOOK_SECRET", "whsec_test_mock_secret");
        }
    });
}

// ═══════════════════════════════════════════════════════════════════════════════════
// TEST APPLICATION SETUP
// ═══════════════════════════════════════════════════════════════════════════════════

/// Test context containing the app, pool, and authentication helpers
#[derive(Clone)]
pub struct TestContext {
    pub app: Router,
    pub pool: PgPool,
    pub config: Config,
    pub jwt_secret: String,
}

impl TestContext {
    /// Create a new test context with initialized database and app
    pub async fn new() -> Self {
        init_test_env();

        let config = Config::from_env().expect("Failed to load test config");
        let jwt_secret = config.jwt_secret.clone();

        let db = Database::new(&config)
            .await
            .expect("Failed to connect to test database");

        let services = Services::new(&config)
            .await
            .expect("Failed to initialize services");

        let event_broadcaster = EventBroadcaster::new();
        let ws_manager = WsConnectionManager::new();
        let cache = CacheService::new(None, None);
        let cache_invalidator = CacheInvalidator::new(cache.clone());

        let state = AppState {
            db: db.clone(),
            services,
            config: config.clone(),
            event_broadcaster,
            ws_manager,
            cache,
            cache_invalidator,
        };

        let app = Router::new()
            .merge(routes::health::router())
            .nest("/api", routes::api_router())
            .with_state(state);

        Self {
            app,
            pool: db.pool,
            config,
            jwt_secret,
        }
    }

    /// Get the database pool for direct queries
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Generate a unique test user ID
    pub fn next_user_id() -> i64 {
        TEST_USER_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════
// TEST USER FACTORY
// ═══════════════════════════════════════════════════════════════════════════════════

/// Represents a test user with authentication capabilities
#[derive(Debug, Clone)]
pub struct TestUser {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub role: String,
    pub token: String,
}

impl TestUser {
    /// Create an admin test user
    pub async fn admin(ctx: &TestContext) -> Self {
        Self::create(ctx, "admin").await
    }

    /// Create a super admin test user
    pub async fn super_admin(ctx: &TestContext) -> Self {
        Self::create(ctx, "super_admin").await
    }

    /// Create a developer test user
    pub async fn developer(ctx: &TestContext) -> Self {
        Self::create(ctx, "developer").await
    }

    /// Create a regular member test user
    pub async fn member(ctx: &TestContext) -> Self {
        Self::create(ctx, "member").await
    }

    /// Create a test user with specified role
    pub async fn create(ctx: &TestContext, role: &str) -> Self {
        let id = TestContext::next_user_id();
        let email = format!("test_{}_{id}@explosiveswings.test", role);
        let name = format!("Test {} User {}", role.to_uppercase(), id);
        let password_hash = hash_password("TestPassword123!@#").expect("Failed to hash password");

        // Insert user into database
        sqlx::query(
            r#"
            INSERT INTO users (id, email, password_hash, name, role, email_verified_at, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, NOW(), NOW(), NOW())
            ON CONFLICT (id) DO UPDATE SET
                email = EXCLUDED.email,
                password_hash = EXCLUDED.password_hash,
                name = EXCLUDED.name,
                role = EXCLUDED.role,
                updated_at = NOW()
            "#,
        )
        .bind(id)
        .bind(&email)
        .bind(&password_hash)
        .bind(&name)
        .bind(role)
        .execute(&ctx.pool)
        .await
        .expect("Failed to create test user");

        // Generate JWT token
        let token = create_jwt(id, &ctx.jwt_secret, 24).expect("Failed to create JWT");

        Self {
            id,
            email,
            name,
            role: role.to_string(),
            token,
        }
    }

    /// Get authorization header value
    pub fn auth_header(&self) -> String {
        format!("Bearer {}", self.token)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════
// TRADING ROOM FACTORY
// ═══════════════════════════════════════════════════════════════════════════════════

/// Represents a test trading room
#[derive(Debug, Clone)]
pub struct TestTradingRoom {
    pub id: i64,
    pub name: String,
    pub slug: String,
}

impl TestTradingRoom {
    /// Create the Explosive Swings trading room (default for tests)
    pub async fn explosive_swings(ctx: &TestContext) -> Self {
        Self::create(ctx, "Explosive Swings", "explosive-swings").await
    }

    /// Create a custom trading room
    pub async fn create(ctx: &TestContext, name: &str, slug: &str) -> Self {
        let result: (i64,) = sqlx::query_as(
            r#"
            INSERT INTO trading_rooms (name, slug, description, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, true, NOW(), NOW())
            ON CONFLICT (slug) DO UPDATE SET name = EXCLUDED.name
            RETURNING id
            "#,
        )
        .bind(name)
        .bind(slug)
        .bind(format!("{} trading room for testing", name))
        .fetch_one(&ctx.pool)
        .await
        .expect("Failed to create test trading room");

        Self {
            id: result.0,
            name: name.to_string(),
            slug: slug.to_string(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════
// ALERT FACTORY
// ═══════════════════════════════════════════════════════════════════════════════════

/// Builder for creating test alerts
#[derive(Debug, Clone)]
pub struct AlertBuilder {
    pub room_slug: String,
    pub alert_type: String,
    pub ticker: String,
    pub title: String,
    pub message: String,
    pub notes: Option<String>,
    pub trade_type: Option<String>,
    pub action: Option<String>,
    pub quantity: Option<i32>,
    pub option_type: Option<String>,
    pub strike: Option<f64>,
    pub expiration: Option<String>,
    pub contract_type: Option<String>,
    pub order_type: Option<String>,
    pub limit_price: Option<f64>,
    pub fill_price: Option<f64>,
    pub tos_string: Option<String>,
    pub entry_alert_id: Option<i64>,
    pub trade_plan_id: Option<i64>,
    pub is_new: Option<bool>,
    pub is_published: Option<bool>,
    pub is_pinned: Option<bool>,
}

impl AlertBuilder {
    /// Create a new alert builder with defaults
    pub fn new(room_slug: &str) -> Self {
        Self {
            room_slug: room_slug.to_string(),
            alert_type: "ENTRY".to_string(),
            ticker: "SPY".to_string(),
            title: "Test Alert".to_string(),
            message: "Test alert message".to_string(),
            notes: None,
            trade_type: Some("options".to_string()),
            action: Some("BUY".to_string()),
            quantity: Some(1),
            option_type: Some("CALL".to_string()),
            strike: Some(500.0),
            expiration: Some(Self::default_expiration()),
            contract_type: Some("Weeklys".to_string()),
            order_type: Some("MKT".to_string()),
            limit_price: None,
            fill_price: Some(2.50),
            tos_string: None,
            entry_alert_id: None,
            trade_plan_id: None,
            is_new: Some(true),
            is_published: Some(true),
            is_pinned: Some(false),
        }
    }

    /// Create an entry alert builder
    pub fn entry(room_slug: &str, ticker: &str) -> Self {
        Self::new(room_slug)
            .with_alert_type("ENTRY")
            .with_ticker(ticker)
            .with_title(format!("ENTRY: {} Call", ticker))
            .with_message(format!("Opening {} call position", ticker))
    }

    /// Create an exit alert builder
    pub fn exit(room_slug: &str, ticker: &str, entry_alert_id: i64) -> Self {
        Self::new(room_slug)
            .with_alert_type("EXIT")
            .with_ticker(ticker)
            .with_title(format!("EXIT: {} Call", ticker))
            .with_message(format!("Closing {} call position", ticker))
            .with_action("SELL")
            .with_entry_alert_id(entry_alert_id)
    }

    /// Create an update alert builder
    pub fn update(room_slug: &str, ticker: &str, entry_alert_id: i64) -> Self {
        Self::new(room_slug)
            .with_alert_type("UPDATE")
            .with_ticker(ticker)
            .with_title(format!("UPDATE: {}", ticker))
            .with_message(format!("Updating {} position info", ticker))
            .with_entry_alert_id(entry_alert_id)
    }

    /// Set alert type
    pub fn with_alert_type(mut self, alert_type: &str) -> Self {
        self.alert_type = alert_type.to_string();
        self
    }

    /// Set ticker
    pub fn with_ticker(mut self, ticker: &str) -> Self {
        self.ticker = ticker.to_string();
        self
    }

    /// Set title
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set message
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    /// Set notes
    pub fn with_notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    /// Set trade type (options/shares)
    pub fn with_trade_type(mut self, trade_type: &str) -> Self {
        self.trade_type = Some(trade_type.to_string());
        self
    }

    /// Set action (BUY/SELL)
    pub fn with_action(mut self, action: &str) -> Self {
        self.action = Some(action.to_string());
        self
    }

    /// Set quantity
    pub fn with_quantity(mut self, quantity: i32) -> Self {
        self.quantity = Some(quantity);
        self
    }

    /// Set option type (CALL/PUT)
    pub fn with_option_type(mut self, option_type: &str) -> Self {
        self.option_type = Some(option_type.to_string());
        self
    }

    /// Set strike price
    pub fn with_strike(mut self, strike: f64) -> Self {
        self.strike = Some(strike);
        self
    }

    /// Set expiration date
    pub fn with_expiration(mut self, expiration: &str) -> Self {
        self.expiration = Some(expiration.to_string());
        self
    }

    /// Set contract type
    pub fn with_contract_type(mut self, contract_type: &str) -> Self {
        self.contract_type = Some(contract_type.to_string());
        self
    }

    /// Set order type (MKT/LMT)
    pub fn with_order_type(mut self, order_type: &str) -> Self {
        self.order_type = Some(order_type.to_string());
        self
    }

    /// Set limit price
    pub fn with_limit_price(mut self, limit_price: f64) -> Self {
        self.limit_price = Some(limit_price);
        self
    }

    /// Set fill price
    pub fn with_fill_price(mut self, fill_price: f64) -> Self {
        self.fill_price = Some(fill_price);
        self
    }

    /// Set TOS format string
    pub fn with_tos_string(mut self, tos_string: impl Into<String>) -> Self {
        self.tos_string = Some(tos_string.into());
        self
    }

    /// Set entry alert ID (for exit/update alerts)
    pub fn with_entry_alert_id(mut self, id: i64) -> Self {
        self.entry_alert_id = Some(id);
        self
    }

    /// Set trade plan ID
    pub fn with_trade_plan_id(mut self, id: i64) -> Self {
        self.trade_plan_id = Some(id);
        self
    }

    /// Set is_new flag
    pub fn with_is_new(mut self, is_new: bool) -> Self {
        self.is_new = Some(is_new);
        self
    }

    /// Set is_published flag
    pub fn with_is_published(mut self, is_published: bool) -> Self {
        self.is_published = Some(is_published);
        self
    }

    /// Set is_pinned flag
    pub fn with_is_pinned(mut self, is_pinned: bool) -> Self {
        self.is_pinned = Some(is_pinned);
        self
    }

    /// Build JSON payload for API request
    pub fn build_json(&self) -> Value {
        json!({
            "room_slug": self.room_slug,
            "alert_type": self.alert_type,
            "ticker": self.ticker,
            "title": self.title,
            "message": self.message,
            "notes": self.notes,
            "trade_type": self.trade_type,
            "action": self.action,
            "quantity": self.quantity,
            "option_type": self.option_type,
            "strike": self.strike,
            "expiration": self.expiration,
            "contract_type": self.contract_type,
            "order_type": self.order_type,
            "limit_price": self.limit_price,
            "fill_price": self.fill_price,
            "tos_string": self.tos_string,
            "entry_alert_id": self.entry_alert_id,
            "trade_plan_id": self.trade_plan_id,
            "is_new": self.is_new,
            "is_published": self.is_published,
            "is_pinned": self.is_pinned
        })
    }

    /// Get default expiration date (next Friday)
    fn default_expiration() -> String {
        let today = Utc::now().date_naive();
        let days_until_friday = (5 - today.weekday().num_days_from_monday() as i64 + 7) % 7;
        let expiration = today
            + Duration::days(if days_until_friday == 0 {
                7
            } else {
                days_until_friday
            });
        expiration.format("%Y-%m-%d").to_string()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════
// TRADE FACTORY
// ═══════════════════════════════════════════════════════════════════════════════════

/// Builder for creating test trades
#[derive(Debug, Clone)]
pub struct TradeBuilder {
    pub room_slug: String,
    pub ticker: String,
    pub trade_type: String,
    pub direction: String,
    pub quantity: i32,
    pub option_type: Option<String>,
    pub strike: Option<f64>,
    pub expiration: Option<String>,
    pub contract_type: Option<String>,
    pub entry_alert_id: Option<i64>,
    pub entry_price: f64,
    pub entry_date: String,
    pub entry_tos_string: Option<String>,
    pub setup: Option<String>,
    pub notes: Option<String>,
}

impl TradeBuilder {
    /// Create a new trade builder with defaults
    pub fn new(room_slug: &str) -> Self {
        Self {
            room_slug: room_slug.to_string(),
            ticker: "SPY".to_string(),
            trade_type: "options".to_string(),
            direction: "long".to_string(),
            quantity: 1,
            option_type: Some("CALL".to_string()),
            strike: Some(500.0),
            expiration: Some(Self::default_expiration()),
            contract_type: Some("Weeklys".to_string()),
            entry_alert_id: None,
            entry_price: 2.50,
            entry_date: Utc::now().date_naive().format("%Y-%m-%d").to_string(),
            entry_tos_string: None,
            setup: Some("Breakout".to_string()),
            notes: None,
        }
    }

    /// Create a long call trade
    pub fn long_call(room_slug: &str, ticker: &str, strike: f64, entry_price: f64) -> Self {
        Self::new(room_slug)
            .with_ticker(ticker)
            .with_direction("long")
            .with_option_type("CALL")
            .with_strike(strike)
            .with_entry_price(entry_price)
    }

    /// Create a long put trade
    pub fn long_put(room_slug: &str, ticker: &str, strike: f64, entry_price: f64) -> Self {
        Self::new(room_slug)
            .with_ticker(ticker)
            .with_direction("long")
            .with_option_type("PUT")
            .with_strike(strike)
            .with_entry_price(entry_price)
    }

    /// Create a shares trade
    pub fn shares(room_slug: &str, ticker: &str, quantity: i32, entry_price: f64) -> Self {
        Self {
            room_slug: room_slug.to_string(),
            ticker: ticker.to_string(),
            trade_type: "shares".to_string(),
            direction: "long".to_string(),
            quantity,
            option_type: None,
            strike: None,
            expiration: None,
            contract_type: None,
            entry_alert_id: None,
            entry_price,
            entry_date: Utc::now().date_naive().format("%Y-%m-%d").to_string(),
            entry_tos_string: None,
            setup: None,
            notes: None,
        }
    }

    /// Set ticker
    pub fn with_ticker(mut self, ticker: &str) -> Self {
        self.ticker = ticker.to_string();
        self
    }

    /// Set direction
    pub fn with_direction(mut self, direction: &str) -> Self {
        self.direction = direction.to_string();
        self
    }

    /// Set quantity
    pub fn with_quantity(mut self, quantity: i32) -> Self {
        self.quantity = quantity;
        self
    }

    /// Set option type
    pub fn with_option_type(mut self, option_type: &str) -> Self {
        self.option_type = Some(option_type.to_string());
        self
    }

    /// Set strike price
    pub fn with_strike(mut self, strike: f64) -> Self {
        self.strike = Some(strike);
        self
    }

    /// Set expiration
    pub fn with_expiration(mut self, expiration: &str) -> Self {
        self.expiration = Some(expiration.to_string());
        self
    }

    /// Set entry price
    pub fn with_entry_price(mut self, entry_price: f64) -> Self {
        self.entry_price = entry_price;
        self
    }

    /// Set entry date
    pub fn with_entry_date(mut self, entry_date: &str) -> Self {
        self.entry_date = entry_date.to_string();
        self
    }

    /// Set entry alert ID
    pub fn with_entry_alert_id(mut self, id: i64) -> Self {
        self.entry_alert_id = Some(id);
        self
    }

    /// Set setup
    pub fn with_setup(mut self, setup: &str) -> Self {
        self.setup = Some(setup.to_string());
        self
    }

    /// Set notes
    pub fn with_notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    /// Build JSON payload for API request
    pub fn build_json(&self) -> Value {
        json!({
            "room_slug": self.room_slug,
            "ticker": self.ticker,
            "trade_type": self.trade_type,
            "direction": self.direction,
            "quantity": self.quantity,
            "option_type": self.option_type,
            "strike": self.strike,
            "expiration": self.expiration,
            "contract_type": self.contract_type,
            "entry_alert_id": self.entry_alert_id,
            "entry_price": self.entry_price,
            "entry_date": self.entry_date,
            "entry_tos_string": self.entry_tos_string,
            "setup": self.setup,
            "notes": self.notes
        })
    }

    /// Get default expiration date (next Friday)
    fn default_expiration() -> String {
        let today = Utc::now().date_naive();
        let days_until_friday = (5 - today.weekday().num_days_from_monday() as i64 + 7) % 7;
        let expiration = today
            + Duration::days(if days_until_friday == 0 {
                7
            } else {
                days_until_friday
            });
        expiration.format("%Y-%m-%d").to_string()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════
// TRADE PLAN FACTORY
// ═══════════════════════════════════════════════════════════════════════════════════

/// Builder for creating test trade plans
#[derive(Debug, Clone)]
pub struct TradePlanBuilder {
    pub room_slug: String,
    pub week_of: Option<String>,
    pub ticker: String,
    pub bias: String,
    pub entry: Option<String>,
    pub target1: Option<String>,
    pub target2: Option<String>,
    pub target3: Option<String>,
    pub runner: Option<String>,
    pub runner_stop: Option<String>,
    pub stop: Option<String>,
    pub options_strike: Option<String>,
    pub options_exp: Option<String>,
    pub notes: Option<String>,
    pub sort_order: Option<i32>,
}

impl TradePlanBuilder {
    /// Create a new trade plan builder with defaults
    pub fn new(room_slug: &str) -> Self {
        Self {
            room_slug: room_slug.to_string(),
            week_of: Some(Self::current_week_monday()),
            ticker: "SPY".to_string(),
            bias: "BULLISH".to_string(),
            entry: Some("495".to_string()),
            target1: Some("500".to_string()),
            target2: Some("505".to_string()),
            target3: Some("510".to_string()),
            runner: None,
            runner_stop: None,
            stop: Some("490".to_string()),
            options_strike: Some("500".to_string()),
            options_exp: Some(Self::default_expiration()),
            notes: None,
            sort_order: Some(0),
        }
    }

    /// Create a bullish trade plan
    pub fn bullish(room_slug: &str, ticker: &str) -> Self {
        Self::new(room_slug)
            .with_ticker(ticker)
            .with_bias("BULLISH")
    }

    /// Create a bearish trade plan
    pub fn bearish(room_slug: &str, ticker: &str) -> Self {
        Self::new(room_slug)
            .with_ticker(ticker)
            .with_bias("BEARISH")
    }

    /// Create a neutral trade plan
    pub fn neutral(room_slug: &str, ticker: &str) -> Self {
        Self::new(room_slug)
            .with_ticker(ticker)
            .with_bias("NEUTRAL")
    }

    /// Set ticker
    pub fn with_ticker(mut self, ticker: &str) -> Self {
        self.ticker = ticker.to_string();
        self
    }

    /// Set bias
    pub fn with_bias(mut self, bias: &str) -> Self {
        self.bias = bias.to_string();
        self
    }

    /// Set week_of date
    pub fn with_week_of(mut self, week_of: &str) -> Self {
        self.week_of = Some(week_of.to_string());
        self
    }

    /// Set entry level
    pub fn with_entry(mut self, entry: &str) -> Self {
        self.entry = Some(entry.to_string());
        self
    }

    /// Set targets
    pub fn with_targets(mut self, t1: &str, t2: Option<&str>, t3: Option<&str>) -> Self {
        self.target1 = Some(t1.to_string());
        self.target2 = t2.map(|s| s.to_string());
        self.target3 = t3.map(|s| s.to_string());
        self
    }

    /// Set stop
    pub fn with_stop(mut self, stop: &str) -> Self {
        self.stop = Some(stop.to_string());
        self
    }

    /// Set runner and runner stop
    pub fn with_runner(mut self, runner: &str, runner_stop: &str) -> Self {
        self.runner = Some(runner.to_string());
        self.runner_stop = Some(runner_stop.to_string());
        self
    }

    /// Set options details
    pub fn with_options(mut self, strike: &str, exp: &str) -> Self {
        self.options_strike = Some(strike.to_string());
        self.options_exp = Some(exp.to_string());
        self
    }

    /// Set notes
    pub fn with_notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    /// Set sort order
    pub fn with_sort_order(mut self, sort_order: i32) -> Self {
        self.sort_order = Some(sort_order);
        self
    }

    /// Build JSON payload for API request
    pub fn build_json(&self) -> Value {
        json!({
            "room_slug": self.room_slug,
            "week_of": self.week_of,
            "ticker": self.ticker,
            "bias": self.bias,
            "entry": self.entry,
            "target1": self.target1,
            "target2": self.target2,
            "target3": self.target3,
            "runner": self.runner,
            "runner_stop": self.runner_stop,
            "stop": self.stop,
            "options_strike": self.options_strike,
            "options_exp": self.options_exp,
            "notes": self.notes,
            "sort_order": self.sort_order
        })
    }

    /// Get current week's Monday date
    fn current_week_monday() -> String {
        let today = Utc::now().date_naive();
        let days_from_monday = today.weekday().num_days_from_monday() as i64;
        let monday = today - Duration::days(days_from_monday);
        monday.format("%Y-%m-%d").to_string()
    }

    /// Get default expiration date (next Friday)
    fn default_expiration() -> String {
        let today = Utc::now().date_naive();
        let days_until_friday = (5 - today.weekday().num_days_from_monday() as i64 + 7) % 7;
        let expiration = today
            + Duration::days(if days_until_friday == 0 {
                7
            } else {
                days_until_friday
            });
        expiration.format("%Y-%m-%d").to_string()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════
// TOS FORMAT STRING UTILITIES
// ═══════════════════════════════════════════════════════════════════════════════════

/// Generate a ThinkOrSwim (TOS) format string
pub struct TosFormatter;

impl TosFormatter {
    /// Generate TOS format string for options
    /// Example: "BUY +1 SPY 100 (Weeklys) 20 JAN 26 500 CALL @MKT"
    #[allow(clippy::too_many_arguments)]
    pub fn options(
        action: &str,
        quantity: i32,
        ticker: &str,
        shares_per_contract: i32,
        contract_type: &str,
        expiration: &NaiveDate,
        strike: f64,
        option_type: &str,
        order_type: &str,
        limit_price: Option<f64>,
    ) -> String {
        let qty_prefix = if action == "BUY" { "+" } else { "-" };
        let exp_str = expiration.format("%d %b %y").to_string().to_uppercase();

        let price_str = match (order_type, limit_price) {
            ("LMT", Some(price)) => format!("@{:.2} LMT", price),
            _ => "@MKT".to_string(),
        };

        format!(
            "{} {}{} {} {} ({}) {} {} {} {}",
            action,
            qty_prefix,
            quantity,
            ticker,
            shares_per_contract,
            contract_type,
            exp_str,
            strike,
            option_type,
            price_str
        )
    }

    /// Generate TOS format string for shares
    /// Example: "BUY +100 SPY @MKT"
    pub fn shares(
        action: &str,
        quantity: i32,
        ticker: &str,
        order_type: &str,
        limit_price: Option<f64>,
    ) -> String {
        let qty_prefix = if action == "BUY" { "+" } else { "-" };

        let price_str = match (order_type, limit_price) {
            ("LMT", Some(price)) => format!("@{:.2} LMT", price),
            _ => "@MKT".to_string(),
        };

        format!(
            "{} {}{} {} {}",
            action, qty_prefix, quantity, ticker, price_str
        )
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════
// HTTP RESPONSE HELPERS
// ═══════════════════════════════════════════════════════════════════════════════════

/// Convert response body to JSON Value
pub async fn body_to_json(response: axum::http::Response<Body>) -> Value {
    let body = response.into_body();
    let bytes = body
        .collect()
        .await
        .expect("Failed to read body")
        .to_bytes();
    serde_json::from_slice(&bytes).unwrap_or_else(|_| {
        let text = String::from_utf8_lossy(&bytes);
        json!({"raw_response": text.to_string()})
    })
}

/// Assert response status and return body as JSON
pub async fn assert_status_and_json(
    response: axum::http::Response<Body>,
    expected_status: axum::http::StatusCode,
) -> Value {
    let status = response.status();
    let body = body_to_json(response).await;
    assert_eq!(
        status, expected_status,
        "Expected status {} but got {}. Body: {}",
        expected_status, status, body
    );
    body
}

// ═══════════════════════════════════════════════════════════════════════════════════
// TEST DATA CLEANUP
// ═══════════════════════════════════════════════════════════════════════════════════

/// Clean up test data for a specific room
pub async fn cleanup_room_data(pool: &PgPool, room_slug: &str) {
    // Delete in order respecting foreign keys
    sqlx::query("DELETE FROM room_alerts WHERE room_slug = $1")
        .bind(room_slug)
        .execute(pool)
        .await
        .ok();

    sqlx::query("DELETE FROM room_trades WHERE room_slug = $1")
        .bind(room_slug)
        .execute(pool)
        .await
        .ok();

    sqlx::query("DELETE FROM room_trade_plans WHERE room_slug = $1")
        .bind(room_slug)
        .execute(pool)
        .await
        .ok();

    sqlx::query("DELETE FROM room_weekly_videos WHERE room_slug = $1")
        .bind(room_slug)
        .execute(pool)
        .await
        .ok();

    sqlx::query("DELETE FROM room_stats_cache WHERE room_slug = $1")
        .bind(room_slug)
        .execute(pool)
        .await
        .ok();
}

/// Clean up test users
pub async fn cleanup_test_users(pool: &PgPool) {
    sqlx::query("DELETE FROM users WHERE email LIKE '%@explosiveswings.test'")
        .execute(pool)
        .await
        .ok();
}

// ═══════════════════════════════════════════════════════════════════════════════════
// STATS CALCULATION HELPERS
// ═══════════════════════════════════════════════════════════════════════════════════

/// Calculate expected win rate from wins and losses
pub fn calculate_win_rate(wins: i32, losses: i32) -> f64 {
    let total = wins + losses;
    if total == 0 {
        0.0
    } else {
        (wins as f64 / total as f64) * 100.0
    }
}

/// Calculate expected profit factor from wins and losses
pub fn calculate_profit_factor(total_wins: f64, total_losses: f64) -> f64 {
    if total_losses == 0.0 {
        if total_wins > 0.0 {
            f64::INFINITY
        } else {
            0.0
        }
    } else {
        total_wins / total_losses.abs()
    }
}
