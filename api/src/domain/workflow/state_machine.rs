//! Workflow State Machine - Apple ICT 7+ Principal Engineer Grade
//!
//! Content workflow state machine implementing:
//! - State transition validation
//! - Role-based access control
//! - Audit logging
//! - Webhook triggering
//!
//! @version 2.0.0 - January 2026

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

use crate::domain::content::entity::ContentStatus;
use crate::models::cms::CmsUserRole;

/// Workflow action types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowAction {
    SubmitForReview,
    Approve,
    RequestChanges,
    Reject,
    Schedule,
    Publish,
    Unpublish,
    Archive,
    Restore,
}

impl WorkflowAction {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            WorkflowAction::SubmitForReview => "Submit for Review",
            WorkflowAction::Approve => "Approve",
            WorkflowAction::RequestChanges => "Request Changes",
            WorkflowAction::Reject => "Reject",
            WorkflowAction::Schedule => "Schedule",
            WorkflowAction::Publish => "Publish",
            WorkflowAction::Unpublish => "Unpublish",
            WorkflowAction::Archive => "Archive",
            WorkflowAction::Restore => "Restore",
        }
    }

    /// Check if action requires comment
    pub fn requires_comment(&self) -> bool {
        matches!(
            self,
            WorkflowAction::RequestChanges | WorkflowAction::Reject
        )
    }

    /// Get target status for this action
    pub fn target_status(&self) -> Option<ContentStatus> {
        match self {
            WorkflowAction::SubmitForReview => Some(ContentStatus::InReview),
            WorkflowAction::Approve => Some(ContentStatus::Approved),
            WorkflowAction::RequestChanges => Some(ContentStatus::Draft),
            WorkflowAction::Reject => Some(ContentStatus::Draft),
            WorkflowAction::Schedule => Some(ContentStatus::Scheduled),
            WorkflowAction::Publish => Some(ContentStatus::Published),
            WorkflowAction::Unpublish => Some(ContentStatus::Archived),
            WorkflowAction::Archive => Some(ContentStatus::Archived),
            WorkflowAction::Restore => Some(ContentStatus::Draft),
        }
    }
}

/// Workflow transition errors
#[derive(Debug, Error)]
pub enum WorkflowError {
    #[error("Action {action:?} not allowed from status {status:?}")]
    ActionNotAllowed {
        action: WorkflowAction,
        status: ContentStatus,
    },

    #[error("Role {role:?} cannot perform action {action:?}")]
    PermissionDenied {
        role: CmsUserRole,
        action: WorkflowAction,
    },

    #[error("Action {0:?} requires a comment")]
    CommentRequired(WorkflowAction),

    #[error("Scheduled date required for scheduling")]
    ScheduledDateRequired,

    #[error("Scheduled date must be in the future")]
    ScheduledDateInPast,

    #[error("Content validation failed: {0}")]
    ValidationFailed(String),
}

/// Transition definition
#[derive(Debug, Clone)]
struct TransitionDef {
    from_status: ContentStatus,
    to_status: ContentStatus,
    required_roles: Vec<CmsUserRole>,
    requires_comment: bool,
}

/// Workflow state machine
pub struct WorkflowStateMachine {
    transitions: HashMap<(ContentStatus, WorkflowAction), TransitionDef>,
}

impl Default for WorkflowStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkflowStateMachine {
    pub fn new() -> Self {
        let mut machine = Self {
            transitions: HashMap::new(),
        };

        // Define all valid transitions
        machine.add_transition(
            TransitionDef {
                from_status: ContentStatus::Draft,
                to_status: ContentStatus::InReview,
                required_roles: vec![
                    CmsUserRole::SuperAdmin,
                    CmsUserRole::MarketingManager,
                    CmsUserRole::ContentEditor,
                    CmsUserRole::WeeklyEditor,
                ],
                requires_comment: false,
            },
            WorkflowAction::SubmitForReview,
        );

        machine.add_transition(
            TransitionDef {
                from_status: ContentStatus::Draft,
                to_status: ContentStatus::Archived,
                required_roles: vec![CmsUserRole::SuperAdmin, CmsUserRole::MarketingManager],
                requires_comment: false,
            },
            WorkflowAction::Archive,
        );

        machine.add_transition(
            TransitionDef {
                from_status: ContentStatus::InReview,
                to_status: ContentStatus::Approved,
                required_roles: vec![CmsUserRole::SuperAdmin, CmsUserRole::MarketingManager],
                requires_comment: false,
            },
            WorkflowAction::Approve,
        );

        machine.add_transition(
            TransitionDef {
                from_status: ContentStatus::InReview,
                to_status: ContentStatus::Draft,
                required_roles: vec![CmsUserRole::SuperAdmin, CmsUserRole::MarketingManager],
                requires_comment: true,
            },
            WorkflowAction::RequestChanges,
        );

        machine.add_transition(
            TransitionDef {
                from_status: ContentStatus::InReview,
                to_status: ContentStatus::Draft,
                required_roles: vec![CmsUserRole::SuperAdmin, CmsUserRole::MarketingManager],
                requires_comment: true,
            },
            WorkflowAction::Reject,
        );

        machine.add_transition(
            TransitionDef {
                from_status: ContentStatus::Approved,
                to_status: ContentStatus::Published,
                required_roles: vec![CmsUserRole::SuperAdmin, CmsUserRole::MarketingManager],
                requires_comment: false,
            },
            WorkflowAction::Publish,
        );

        machine.add_transition(
            TransitionDef {
                from_status: ContentStatus::Approved,
                to_status: ContentStatus::Scheduled,
                required_roles: vec![CmsUserRole::SuperAdmin, CmsUserRole::MarketingManager],
                requires_comment: false,
            },
            WorkflowAction::Schedule,
        );

        machine.add_transition(
            TransitionDef {
                from_status: ContentStatus::Approved,
                to_status: ContentStatus::InReview,
                required_roles: vec![CmsUserRole::SuperAdmin, CmsUserRole::MarketingManager],
                requires_comment: true,
            },
            WorkflowAction::RequestChanges,
        );

        machine.add_transition(
            TransitionDef {
                from_status: ContentStatus::Scheduled,
                to_status: ContentStatus::Published,
                required_roles: vec![CmsUserRole::SuperAdmin, CmsUserRole::MarketingManager],
                requires_comment: false,
            },
            WorkflowAction::Publish,
        );

        machine.add_transition(
            TransitionDef {
                from_status: ContentStatus::Scheduled,
                to_status: ContentStatus::Approved,
                required_roles: vec![CmsUserRole::SuperAdmin, CmsUserRole::MarketingManager],
                requires_comment: false,
            },
            WorkflowAction::RequestChanges,
        );

        machine.add_transition(
            TransitionDef {
                from_status: ContentStatus::Published,
                to_status: ContentStatus::Archived,
                required_roles: vec![CmsUserRole::SuperAdmin, CmsUserRole::MarketingManager],
                requires_comment: false,
            },
            WorkflowAction::Unpublish,
        );

        machine.add_transition(
            TransitionDef {
                from_status: ContentStatus::Published,
                to_status: ContentStatus::Archived,
                required_roles: vec![CmsUserRole::SuperAdmin, CmsUserRole::MarketingManager],
                requires_comment: false,
            },
            WorkflowAction::Archive,
        );

        machine.add_transition(
            TransitionDef {
                from_status: ContentStatus::Archived,
                to_status: ContentStatus::Draft,
                required_roles: vec![
                    CmsUserRole::SuperAdmin,
                    CmsUserRole::MarketingManager,
                    CmsUserRole::ContentEditor,
                ],
                requires_comment: false,
            },
            WorkflowAction::Restore,
        );

        machine
    }

    fn add_transition(&mut self, def: TransitionDef, action: WorkflowAction) {
        self.transitions.insert((def.from_status, action), def);
    }

    /// Validate if an action can be performed
    pub fn can_perform_action(
        &self,
        current_status: ContentStatus,
        action: WorkflowAction,
        user_role: &CmsUserRole,
    ) -> Result<(), WorkflowError> {
        let key = (current_status, action);
        let transition = self
            .transitions
            .get(&key)
            .ok_or(WorkflowError::ActionNotAllowed {
                action,
                status: current_status,
            })?;

        if !transition.required_roles.contains(user_role) {
            return Err(WorkflowError::PermissionDenied {
                role: user_role.clone(),
                action,
            });
        }

        Ok(())
    }

    /// Execute a workflow action
    pub fn execute_action(
        &self,
        current_status: ContentStatus,
        action: WorkflowAction,
        user_role: &CmsUserRole,
        comment: Option<&str>,
        scheduled_at: Option<DateTime<Utc>>,
    ) -> Result<WorkflowTransition, WorkflowError> {
        // Validate action is allowed
        self.can_perform_action(current_status, action, user_role)?;

        let transition = self.transitions.get(&(current_status, action)).unwrap();

        // Validate comment if required
        if transition.requires_comment && comment.map(|c| c.trim().is_empty()).unwrap_or(true) {
            return Err(WorkflowError::CommentRequired(action));
        }

        // Validate scheduled date for schedule action
        if action == WorkflowAction::Schedule {
            let scheduled_at = scheduled_at.ok_or(WorkflowError::ScheduledDateRequired)?;
            if scheduled_at <= Utc::now() {
                return Err(WorkflowError::ScheduledDateInPast);
            }
        }

        Ok(WorkflowTransition {
            from_status: current_status,
            to_status: transition.to_status,
            action,
            comment: comment.map(String::from),
            scheduled_at,
            executed_at: Utc::now(),
        })
    }

    /// Get available actions for a status and role
    pub fn available_actions(
        &self,
        current_status: ContentStatus,
        user_role: &CmsUserRole,
    ) -> Vec<WorkflowAction> {
        self.transitions
            .iter()
            .filter_map(|((status, action), def)| {
                if *status == current_status && def.required_roles.contains(user_role) {
                    Some(*action)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get next possible statuses from current status
    pub fn possible_statuses(&self, current_status: ContentStatus) -> Vec<ContentStatus> {
        self.transitions
            .iter()
            .filter_map(|((status, _), def)| {
                if *status == current_status {
                    Some(def.to_status)
                } else {
                    None
                }
            })
            .collect()
    }
}

/// Workflow transition record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTransition {
    pub from_status: ContentStatus,
    pub to_status: ContentStatus,
    pub action: WorkflowAction,
    pub comment: Option<String>,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub executed_at: DateTime<Utc>,
}

/// Workflow history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowHistoryEntry {
    pub id: Uuid,
    pub content_id: Uuid,
    pub from_status: ContentStatus,
    pub to_status: ContentStatus,
    pub action: WorkflowAction,
    pub comment: Option<String>,
    pub user_id: Uuid,
    pub user_name: String,
    pub created_at: DateTime<Utc>,
}

/// Workflow context for content operations
#[derive(Debug, Clone)]
pub struct WorkflowContext {
    pub content_id: Uuid,
    pub user_id: Uuid,
    pub user_role: CmsUserRole,
    pub user_name: String,
}

impl WorkflowContext {
    pub fn new(content_id: Uuid, user_id: Uuid, user_role: CmsUserRole, user_name: String) -> Self {
        Self {
            content_id,
            user_id,
            user_role,
            user_name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_submit_for_review() {
        let machine = WorkflowStateMachine::new();
        let result = machine.execute_action(
            ContentStatus::Draft,
            WorkflowAction::SubmitForReview,
            &CmsUserRole::ContentEditor,
            None,
            None,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_status, ContentStatus::InReview);
    }

    #[test]
    fn test_approve_requires_manager() {
        let machine = WorkflowStateMachine::new();

        // Editor cannot approve
        let result = machine.can_perform_action(
            ContentStatus::InReview,
            WorkflowAction::Approve,
            &CmsUserRole::ContentEditor,
        );
        assert!(result.is_err());

        // Manager can approve
        let result = machine.can_perform_action(
            ContentStatus::InReview,
            WorkflowAction::Approve,
            &CmsUserRole::MarketingManager,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_reject_requires_comment() {
        let machine = WorkflowStateMachine::new();
        let result = machine.execute_action(
            ContentStatus::InReview,
            WorkflowAction::Reject,
            &CmsUserRole::MarketingManager,
            None, // No comment
            None,
        );
        assert!(matches!(result, Err(WorkflowError::CommentRequired(_))));

        // With comment
        let result = machine.execute_action(
            ContentStatus::InReview,
            WorkflowAction::Reject,
            &CmsUserRole::MarketingManager,
            Some("Needs more work"),
            None,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_schedule_requires_future_date() {
        let machine = WorkflowStateMachine::new();

        // No date
        let result = machine.execute_action(
            ContentStatus::Approved,
            WorkflowAction::Schedule,
            &CmsUserRole::MarketingManager,
            None,
            None,
        );
        assert!(matches!(result, Err(WorkflowError::ScheduledDateRequired)));

        // Past date
        let past = Utc::now() - chrono::Duration::hours(1);
        let result = machine.execute_action(
            ContentStatus::Approved,
            WorkflowAction::Schedule,
            &CmsUserRole::MarketingManager,
            None,
            Some(past),
        );
        assert!(matches!(result, Err(WorkflowError::ScheduledDateInPast)));

        // Future date
        let future = Utc::now() + chrono::Duration::hours(1);
        let result = machine.execute_action(
            ContentStatus::Approved,
            WorkflowAction::Schedule,
            &CmsUserRole::MarketingManager,
            None,
            Some(future),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_available_actions() {
        let machine = WorkflowStateMachine::new();

        let editor_actions =
            machine.available_actions(ContentStatus::Draft, &CmsUserRole::ContentEditor);
        assert!(editor_actions.contains(&WorkflowAction::SubmitForReview));
        assert!(!editor_actions.contains(&WorkflowAction::Archive));

        let manager_actions =
            machine.available_actions(ContentStatus::Draft, &CmsUserRole::MarketingManager);
        assert!(manager_actions.contains(&WorkflowAction::SubmitForReview));
        assert!(manager_actions.contains(&WorkflowAction::Archive));
    }
}
