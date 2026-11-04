# SMTP settings verification — implementation in progress

The settings workflow now has real backend handlers and local protocol evidence. Application mail and campaign delivery still use the earlier Postmark path; SMTP runtime delivery is not complete. Source-wide phase one remains open and copying is forbidden.

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

Proxy regressions **3passed296ms**; frontend full suite **90files/2860passed/32existing skips**. Svelte check and all-target Rust check/Clippy pass for this stated scope. Full ESLint initially failed on a missing config-directory TS parser match and two assertions in the earlier authorization test; those were corrected, with authorization browser1passing5.3s. Await the current full lint result before claiming that gate passes. **Browser3 passed2tests10.0s**, `/tmp/admin-move-smtp-browser3.log`, also proves strict save acknowledgment, schema validation and retry through503 → malformed200 → real saved configuration. Current Svelte check0/0, root0warnings2083files, and root production build1 exit0/88Cloudflare rules.

Lettre0.11.23 uses only SMTP transport and Tokio/Rustls features. The unused MIME-builder feature introduced a license outside the existing policy, so it was omitted. No license/advisory policy exemption was added. Dependency deny exits0 with advisories, bans, licenses and sources accepted; the previously recorded duplicate/unmatched-license warnings remain open for the final gate.

Primary transport contract: [Lettre asynchronous SMTP transport](https://docs.rs/lettre/0.11.23/lettre/transport/smtp/struct.AsyncSmtpTransport.html). Its connection test uses NOOP; it does not send an email.

## Remaining work and external requirements

Wire saved SMTP configuration into raw transactional email, template rendering/defaults, queued email and campaign provider selection, with acknowledged acceptance and uncertain outcomes recorded accurately. Complete native SMTP connection status and actual delivery/tracking workflows. Verify process restart and representative workers after that wiring.

External verification needs an authorized SMTP relay, validated TLS identity, credentials/relay permissions, sender identity, and controlled test recipient. The local development protocol fixture establishes implementation behavior only. Real TLS/account access, message acceptance and inbox delivery have not been verified.
