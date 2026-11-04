# Connections verification and remaining scope

Connections is **in progress**, and source-wide extraction remains forbidden. The core provider/credential workflows below have local evidence. They do not establish live provider access or complete the other connection integrations.

## Implemented core

The seven external definitions use actual read-only requests:

| Service          | Check                                        | What success establishes                                                                                   |
| ---------------- | -------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| Stripe           | Authenticated balance retrieval              | Valid response and selected live/test environment                                                          |
| SendGrid         | Authenticated scopes retrieval               | The key has mail.send permission; no email is submitted                                                    |
| OpenAI           | Authenticated model listing                  | A valid model-list response; no inference is requested                                                     |
| Bunny            | Authenticated storage-zone directory listing | Storage access on an official regional endpoint, plus matching optional Stream library with a separate key |
| Meilisearch      | Authenticated version retrieval              | Valid version response from the configured host                                                            |
| Google Analytics | Authenticated Admin API web-stream lookup    | Matching measurement ID/property/stream; measurement configuration alone is insufficient                   |
| R2               | AWS SDK signed HeadBucket                    | Authenticated bucket access; object read/write permissions remain separate                                 |

HTTP200 alone is insufficient: payload shape, required resource/permission and Stripe environment are checked. Latency is measured. Provider rejection/unavailability stays explicit. Saving unverified credentials never marks them connected. Provider error bodies and credentials are not echoed into receipts.

Save/test receipts are transactional, with encrypted credentials and superadmin authorization in both SvelteKit and Rust. A test with different caller-supplied credentials cannot overwrite the saved configuration's result. Concurrent replacement/disconnect prevents a stale test update. Disconnect/delete lock the row and atomically revoke credentials, disable/delete dependent webhooks and persist their audit record; failed audits roll back every change.

Migration090 stores revocation separately from the deletable configuration. The resolver respects that barrier across cold API state. Only an explicit verified reconnect clears its selected scope; legacy unscoped removal blocks both scopes. Development/staging/test normalize to sandbox. Legacy/unverified rows require verification, and unavailable storage cannot reactivate environment keys. Environment keys must match their scope; public development placeholders are disabled. Cache generations prevent an old lookup from repopulating rotated credentials. All28async Stripe operations reject unavailable credentials before provider HTTP. Encrypted webhook secrets reach the runtime resolver.

## Evidence

- `connections_r34`: **passed36.45s**, `/tmp/admin-move-connections-r34.log`, new empty disposable database,82migration versions/latest090. Actual loopback acknowledgments for all seven definitions, resource/permission/environment failures, encryption, audit receipts, rollback, rotation, actual API-state cache invalidation, cold-state disconnect/deletion, failed/successful reconnect, legacy data, and storage outage. Digests are inspected through a test-only route; it is never mounted by production and never returns a secret.
- Earlier atomic-revocation `r32` passed14.57s; the expanded `r33` applied090 but failed its own incorrect root-vs-data envelope assertion. Corrected the established response shape and added explicit HTTP200 validation; no assertion was removed. Onlyr34 is current for the expanded proof.
- Browser5: **2passed9.3s**, `/tmp/admin-move-connections-browser5.log`, after actual API process restart and migration090. Real save/test/reload, rejected credentials, scoped views, and a clearly identified controlled listing503 followed by real Retry recovery. Strict positive console/API assertions. The actual Passed last-check label, scoped states and failure recovery are included.
- Proxy/error regressions, full restored frontend2855/root2863tests and production SEO114passed18.7s are recorded in `move-progress.md`; they predate this backend-only follow-up. Library157passed/1scanner-environment ignore before the final test-fixture transport adjustment; the strengthened final no-network guard also passed1test/0.03s. No removed tests, lowered thresholds, waived lint rules or deployment.

Deterministic expanded **r35 passed34.31s**, `/tmp/admin-move-connections-r35.log`, supersedesr34: delayed provider acknowledgments overlap credential replacement and disconnect; stale updates return409 and actual runtime state preserves the newer configuration or revocation. All prior resolver/legacy/storage/rollback cases also pass. All-target Clippy and main build pass.

## Local adapter and external setup

`CONNECTION_TEST_BASE_URL` is operator-controlled and optional. It is accepted only in development with a literal loopback HTTP address; production/remote overrides are rejected. Fixed real provider origins are otherwise used. Redirect following is disabled; HTTP bodies/timeouts are bounded and user-configured HTTP targets are checked and DNS-pinned. The email proof requires `http://127.0.0.1:19021`, starts/owns that listener and must run on an explicitly guarded empty disposable database with the existing local email fixture. Never supply live credentials or run against production.

Live verification needs authorized provider credentials/resources and the documented read permissions. GA additionally needs an analytics.readonly OAuth access token and property/stream IDs. Its [Measurement Protocol validation endpoint](https://developers.google.com/analytics/devguides/collection/protocol/ga4/validating-events) does not validate credentials; the [Admin API stream lookup](https://developers.google.com/analytics/devguides/config/admin/v1/rest/v1beta/properties.dataStreams/get) establishes stream access. Other primary contracts: [OpenAI model listing](https://developers.openai.com/api/reference/resources/models/methods/list), [SendGrid key scopes](https://www.twilio.com/docs/sendgrid/api-reference/api-key-permissions/retrieve-a-list-of-scopes-for-which-this-user-has-access), [Meilisearch version](https://www.meilisearch.com/docs/reference/api/version/get-version), [Bunny storage authentication](https://raw.githubusercontent.com/BunnyWay/documentation/main/api-reference/storage/index.mdx).

## Still open

The standalone compatibility status endpoint now uses canonical backend receipts, preserves its broader service catalog and reports integrations without a verification result as pending; it does not manufacture built-in success. Native SMTP verification/configuration, configuration and execution for the broader catalog, OAuth token refresh, runtime consumption beyond Stripe and webhook execution remain open. The default Meilisearch production policy also needs verification against intended private service deployments. Adapter acknowledgment does not establish real provider account access, sending/delivery, inference capability, object permissions or end-to-end workflows for these remaining modules.

## Regional storage and compatibility status follow-up

Fresh **connections_r36 passed34.28s**, `/tmp/admin-move-connections-r36.log`,82migration versions/latest090. This supersedesr35 and additionally verifies supported regional Bunny hostnames, separately authenticated optional Stream library checks, rejected/wrong-library Stream replies, paired field validation and valid model/directory arrays above64KiB. Streaming response parsing retains bounded chunks and entry summaries; provider checks have an overall20s deadline. No live provider was contacted.

**Browser9 passed3tests14.5s**, `/tmp/admin-move-connections-browser9.log`: actual SendGrid receipts, save/reload and scope isolation; controlled listing503/retry; compatibility status follows the saved rejection and preserves catalog entries without fake healthy native services; Bunny regional storage and optional Stream pass/reject/storage-only recovery. Browser6 exposed a real modal overflow that made Connect unreachable; the form now scrolls within the viewport while its actions remain visible. Browser7 could not start under the restricted local-listener sandbox; browser8 passed14.7s after the modal fix, and browser9 includes the status follow-up. Strict workflow assertions were preserved.

Bunny contracts: [storage HTTP API](https://raw.githubusercontent.com/BunnyWay/documentation/main/storage/http.mdx), [storage schema](https://raw.githubusercontent.com/BunnyWay/documentation/main/api-reference/storage/openapi.json), [Stream schema](https://video.bunnycdn.com/openapi/bunnynet-video-api.public.json). Storage uses its zone password; a configured Stream library requires its separate API key.

## Runtime scope and current checks

Fresh **r37 passed34.66s**, `/tmp/admin-move-connections-r37.log`, includes the canonical runtime-scope query and proves the actual development resolver and connection view both select Sandbox while explicit Production stays isolated. `environment=runtime` is a read-only alias; existing API defaults remain compatible. The UI initially selects the API runtime scope and preserves an explicit view in its URL across reload. The shared connection store queries the runtime scope.

**Browser11 passed3tests14.5s**, `/tmp/admin-move-connections-browser11.log`, after rebuilding/restarting the isolated API. Browser10 hit existing fixture login throttling after its first successful workflow; only the task Redis database on port16380 was reset, with rate limits unchanged. Frontend89files/2857passed/32existing skips; root all-project90files/2865passed/32existing skips. Svelte0errors/0warnings, ESLint0, root0warnings2080files. Production build4 exits0 with88Cloudflare rules; SEO13 **114passed17.7s**.

Streaming verification additionally enforces a64MiB total budget, including chunked bodies without Content-Length, while retaining the existing20s deadline and bounded chunks. The focused actual loopback body-budget test passed1test/0.01s. This avoids unbounded individual field allocation; valid large-array provider proof is being rerun after that final guard.
