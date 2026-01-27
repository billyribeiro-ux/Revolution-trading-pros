//! Domain Layer - Apple ICT 7+ Principal Engineer Grade
//!
//! Clean architecture domain layer implementing:
//! - Entities with business logic
//! - Repository patterns for data access
//! - Workflow state machine for content lifecycle
//! - Value objects for type safety
//!
//! @version 2.0.0 - January 2026

pub mod content;
pub mod workflow;

pub use content::entity::*;
pub use content::repository::*;
pub use workflow::state_machine::*;
