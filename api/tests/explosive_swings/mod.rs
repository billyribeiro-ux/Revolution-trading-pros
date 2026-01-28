//! Explosive Swings Integration Tests - Module Root
//! ═══════════════════════════════════════════════════════════════════════════════════
//! Apple Principal Engineer ICT Level 7 Grade - January 2026
//!
//! Comprehensive integration test suite for the Explosive Swings trading room feature.
//! Built for the next 10 years with modern Rust testing patterns.
//!
//! Test Modules:
//! - `alerts_test`: Alert CRUD operations, TOS format, authentication
//! - `trades_test`: Trade lifecycle management, P&L calculations, filtering
//! - `trade_plans_test`: Weekly trade plans, bias validation, date handling
//! - `stats_test`: Performance metrics calculation and caching
//! - `fixtures`: Test utilities, factories, and helpers
//!
//! Design Principles:
//! - Isolated: Each test runs independently with no shared mutable state
//! - Deterministic: No flaky tests, predictable data generation
//! - Fast: Optimized setup with connection pooling
//! - Comprehensive: Covers happy paths, edge cases, and error scenarios
//! - Well-documented: Clear test names and inline documentation

pub mod fixtures;
pub mod alerts_test;
pub mod trades_test;
pub mod trade_plans_test;
pub mod stats_test;

// Re-export common fixtures for easy access
pub use fixtures::{
    AlertBuilder,
    TestContext,
    TestTradingRoom,
    TestUser,
    TosFormatter,
    TradeBuilder,
    TradePlanBuilder,
    assert_status_and_json,
    body_to_json,
    calculate_profit_factor,
    calculate_win_rate,
    cleanup_room_data,
    cleanup_test_users,
    init_test_env,
};
