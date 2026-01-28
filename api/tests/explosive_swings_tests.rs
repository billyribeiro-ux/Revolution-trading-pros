//! Explosive Swings Integration Tests - Entry Point
//! ═══════════════════════════════════════════════════════════════════════════════════
//! Apple Principal Engineer ICT Level 7 Grade - January 2026
//!
//! This file serves as the entry point for all Explosive Swings integration tests.
//! It imports the test modules which contain the actual test implementations.
//!
//! Test Modules:
//! - alerts_test: Alert CRUD operations
//! - trades_test: Trade lifecycle management
//! - trade_plans_test: Weekly trade plans
//! - stats_test: Performance metrics
//! - fixtures: Test utilities and helpers

mod explosive_swings;

// Re-export for convenience
pub use explosive_swings::*;
