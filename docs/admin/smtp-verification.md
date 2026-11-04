# SMTP settings verification — implementation in progress

The settings workflow now has real backend handlers and local protocol evidence. Application transactional/raw mail and generic email jobs now select verified saved SMTP configuration, with actual runtime proof. Campaign/newsletter outbox provider selection and native SMTP tracking are implemented, with fresh installation and actual browser-worker verification. Source-wide phase one remains open and copying is forbidden.

## Implemented

- Both `/admin/email/settings` and `/admin/email/smtp` use the actual same-origin backend proxy, with admin reads and superadmin credential changes/tests.
- Settings are encrypted with the existing AES-GCM credentials key and stored per application runtime scope in the existing settings table. Omitted passwords retain saved credentials. Changing the hostname/username requires deliberate credentials; selecting anonymous relay access clears the password.
- Save and its operational log commit atomically. Database failures remain failures, and failed logging rolls back the configuration.
- The connection test resolves a destination once, uses the configured hostname for TLS identity validation, authenticates when credentials are configured, and issues SMTP NOOP. It never submits mail or establishes inbox delivery.
- TLS mode requires STARTTLS; SSL mode uses direct TLS. Unencrypted mode is restricted to a literal loopback address in development. Remote/production SMTP never downgrades to plaintext.
- A saved verification receipt updates only the configuration that was tested. An unsaved candidate cannot overwrite the saved configuration's receipt.
- Frontend load failures stay visible with Retry. Missing/malformed responses are not treated as empty settings or successful saves. Proxy errors preserve status, including plain-text schema rejection, without retrying a submitted save.

## Evidence

Fresh disposable proof **smtp_r40 passed39.40s**, `/tmp/admin-move-smtp-r40.log`, includes the preceding email/Connections regressions and actual local SMTP authentication/NOOP, omitted-password retention, cold API-state reload, required-STARTTLS rejection, rejected candidate credentials, encrypted persistence, injected log rollback and unavailable settings storage. The initial r39 failed35.43s because its new log category did not match the existing schema; corrected the category, without changing the constraint or historical migration.

**Browser2 passed2tests10.2s**, `/tmp/admin-move-smtp-browser2.log`: both forms save/reload, omit the saved password, authenticate through the real task API, report rejected credentials, and recover from a clearly identified injected503. No SMTP MAIL/DATA command is allowed by the connection-check fixture. Browser1's positive workflow passed; its failure-recovery test had an ambiguous alert locator, corrected to identify the specific load error.

Proxy regressions **3passed296ms**; frontend full suite **90files/2860passed/32existing skips**. Svelte check and all-target Rust check/Clippy pass for this stated scope. Full ESLint initially failed on a missing config-directory TS parser match and two assertions in the earlier authorization test; those were corrected, with authorization browser1passing5.3s. The subsequent full lint3 completed with exit0. **Browser3 passed2tests10.0s**, `/tmp/admin-move-smtp-browser3.log`, also proves strict save acknowledgment, schema validation and retry through503 → malformed200 → real saved configuration. Current Svelte check0/0, root0warnings2083files, and root production build1 exit0/88Cloudflare rules.

Lettre0.11.23 uses only SMTP transport and Tokio/Rustls features. The unused MIME-builder feature introduced a license outside the existing policy, so it was omitted. No license/advisory policy exemption was added. Dependency deny exits0 with advisories, bans, licenses and sources accepted; the previously recorded duplicate/unmatched-license warnings remain open for the final gate.

Primary transport contract: [Lettre asynchronous SMTP transport](https://docs.rs/lettre/0.11.23/lettre/transport/smtp/struct.AsyncSmtpTransport.html). Its connection test uses NOOP; it does not send an email.

## Remaining work and external requirements

Raw transactional email, local template rendering/defaults, queued email and native SMTP status have fresh local proof. Verify the new campaign provider selection and tracking, then verify process restart and representative browser/worker workflows. Broader template editing/client contracts remain open.

External verification needs an authorized SMTP relay, validated TLS identity, credentials/relay permissions, sender identity, and controlled test recipient. The local development protocol fixture establishes implementation behavior only. Real TLS/account access, message acceptance and inbox delivery have not been verified.

## Runtime delivery follow-up

**Fresh smtp_r41 passed44.74s**, `/tmp/admin-move-smtp-r41.log`, including migration091, actual local-template API submission, newsletter confirmation, queued sender configuration recovery, SMTP250/550/disconnection outcomes, no automatic resend of unknown work, current provider diagnostics, cold API-state reload and no environment fallback during storage failure. Migration091 seeds the12 existing application aliases without replacing operator rows and accommodates valid SMTP Message-ID/subject lengths.

SMTP MIME wire tests2passed; template renderer tests2passed; full Rust library163passed/1existing scanner ignore. These show actual local protocol behavior, not external account access or inbox delivery. Confirmation/reset capability values are redacted in stored log models and remain intact in the actual message. The next no-Postmark regression reached the correct unknown outcome, then failed on an incorrect test expectation (409 versus the existing429 resend cooldown). That assertion is corrected without changing rate limits; the next fresh proof must pass.

## Campaign tracking follow-up (verification pending)

The outbox now resolves the same runtime configuration as transactional email. Verified SMTP does not require Postmark credentials; pending or unavailable saved configuration blocks submission. Receipts persist the actual provider and stream, wire Message-ID after SMTP DATA acknowledgment, and distinct known rejection/uncertain outcomes. Native links are persisted before submission and use random capabilities, without destination URLs in the public callback. SMTP HTML preserves the email document, inserts a transparent image and rewrites HTTP(S) links; text alternatives get click links without an image. Unsubscribe links stay direct and carry the existing RFC8058 headers.

Public callbacks record unique image/link requests in the existing event store and refresh the existing campaign counters atomically. HEAD does not record an event. Tracking never implies human engagement, inbox delivery, or SMTP acceptance and cannot reconcile unknown submissions. The frontend proxy omits credentials, preserves HEAD/errors, and does not follow the stored redirect. Local proxy regressions3passed326ms; Svelte check0/0. Fresh **smtp_r44 passed74.93s**, with84migration versions/latest092. Actual browser-worker **delivery browser2 passed1test10.9s** after restarting the isolated API. It saves/verifies SMTP in the UI, sends through the real worker, matches the SMTP Message-ID to its receipt, follows only the local tracking callbacks, verifies the redirect without opening its external target and confirms totals after reload. New migration092 must remain immutable once applied. No live provider verification or source completion is claimed.

The initial r43 reached accepted SMTP/tracking and failed60.82s at an injected event-store outage (500 versus required503); the new callback error mapping was corrected. R44 includes accepted, explicitly rejected and uncertain SMTP campaigns, cold-state callback persistence, HEAD without observations, unique counters, actual unsubscribe and no different-provider reconciliation of SMTP acceptance. Browser1 completed workflow assertions but its console check counted deliberately blocked external assets; browser2 excludes only exact resource errors for requests recorded as aborted by that fixture, while retaining all application/API checks. Neither failed run is counted as a passing proof.

Current frontend91files/2863passed/32existing skips, full lint2 exit0, Svelte0/0, build1 exit0 with88Cloudflare rules. Current Rust all-target check/Clippy/main build pass. The editor boundary remains55 originals/85 integrations/15 approved exceptions/0 unexpected changes. A second actual API process restart preserved the saved host/port/sender, write-only password and native SMTP selection with no Postmark token. Production SEOtracking1 passed114tests17.6s against the restarted current-build preview. Current narrow browser-test lint2 also exits0. SMTP TLS/live-account verification remains external. Source phase one and broader email-template/campaign/report contracts remain incomplete; extraction is still forbidden.
