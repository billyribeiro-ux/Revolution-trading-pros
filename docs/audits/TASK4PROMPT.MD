Go. Begin Task 4 — Postmark Email.

Per EXECUTION.md Task 4:

4.1 Setup
- Confirm POSTMARK_TOKEN in api/.env.local (test server token)
- Confirm POSTMARK_FROM_EMAIL set
- Add postmark crate to api/Cargo.toml OR use reqwest directly with Postmark HTTP API (pick whichever is simpler, pin the version)

4.2 Build api/src/services/email.rs
- struct EmailService { from_email, http client / postmark client }
- async fn send_transactional(to: &str, template_alias: &str, model: serde_json::Value) -> Result<()>
- Insert row into email_logs table on every attempt (status, to, template, sent_at, error)
- On failure: log error, write log row with status='failed', RETURN error (no .ok())

4.3 Create email templates in Postmark dashboard with these aliases:
- welcome
- email-verification
- password-reset
- subscription-confirmation
- subscription-canceled
- subscription-renewal-reminder
- trial-ending
- failed-payment
- receipt
- refund-confirmation
- dispute-created (admin notification)

Templates can be basic HTML for now; can be polished later. Use Postmark variables like {{name}}, {{plan_name}}, {{period_end}}.

4.4 Wire the triggers
- api/src/routes/auth.rs register handler → "welcome" + "email-verification"
- api/src/routes/auth.rs forgot_password → "password-reset"
- api/src/routes/payments.rs handle_checkout_completed → if subscription "subscription-confirmation"; if one-time "receipt"
- handle_subscription_deleted → "subscription-canceled"
- handle_payment_failed → "failed-payment"
- handle_refund → "refund-confirmation"
- handle_dispute_created → "dispute-created" to admin email
- Daily scheduled job api/src/jobs/email_reminders.rs → "subscription-renewal-reminder" (7 days before renewal) + "trial-ending"

Test at end of task — real Postmark test token, real inbox:
- Register a test user → welcome + verification arrive
- Trigger password reset → email arrives
- Test mode purchase a subscription → confirmation arrives
- Test mode purchase a course → receipt arrives
- Cancel via Customer Portal → cancellation email arrives
- stripe trigger invoice.payment_failed → failed-payment email arrives
- stripe trigger charge.refunded → refund email arrives
- stripe trigger charge.dispute.created → dispute admin email arrives

Document each trigger PASS/FAIL with Postmark message ID + inbox screenshot or message body. Write TASK4_RESULT.md.

Gates: cargo check + pnpm check 0/0. Commit on payments-fix-2026-04. Stop after Task 4. Wait for go before Task 5.