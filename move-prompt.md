Complete the unfinished admin/backend functionality in my CURRENT project first, excluding the blog editor. Only after that work is completed and verified should you copy the admin system into a separate folder and make the copy independently runnable.

SOURCE PROJECT:
/Users/billyribeiro/dev/revolution-trading-pros

DESTINATION:
[I WILL PROVIDE THE EMPTY FOLDER PATH]

REQUIRED ORDER

1. Finish and verify the unfinished admin/backend functionality IN THE SOURCE PROJECT.
2. Then copy the completed admin system and all required dependencies into my destination folder.
3. Make that copy run independently so I can tweak and polish it.
4. Installation into other websites and web apps is a later phase.

Do not reverse this order. Do not begin extraction while source functionality remains unfinished, except for the explicitly excluded blog editor.

This is an implementation task. Perform the work, not merely an audit or plan.

GENERAL RULES

Read AGENTS.md and applicable project instructions first. Preserve existing user changes.

Follow the required Svelte MCP documentation and autofixer workflow.

Preserve the existing stack, design, features, and working behavior. Avoid unrelated rewrites.

Do not deploy, publish, modify production databases, charge payments, or send real campaigns. Use isolated local databases, fixtures, and provider sandboxes for verification.

Do not weaken security, lower test thresholds, suppress errors, remove failing tests, or hide unfinished features to claim completion.

PHASE 1 — FINISH THE CURRENT PROJECT

First establish the actual implementation status of the entire admin/backend.

Inventory every admin page, navigation item, action, API client, server endpoint, backend handler, database dependency, integration, and background job.

Trace each workflow through:
UI → API client/server function → backend handler → authorization → persistence/provider → response → updated UI.

Previous completion reports are not proof. Re-check the actual code and running behavior.

Create a completion matrix in the source project containing:

- feature and route;
- expected behavior;
- actual gap;
- implementation files;
- verification evidence;
- current status.

Use the following earlier audit findings as investigation leads. Verify whether they still apply:

- CRM pages rendering ComingSoonStub.
- Boards interfaces without matching mounted backend routes.
- Behavior endpoints acknowledging events without saving them.
- CRM settings updates returning success without persistence.
- Malware scanning returning a clean result without actually scanning.
- Heatmap and session replay placeholders.
- SEO interfaces with empty loaders or unmatched endpoints.
- Member-segment actions marked coming soon.
- Inconsistent role policies between SvelteKit and Rust.
- Fresh-database setup requiring a special schema baseline.
- API startup continuing after required migration failures.

These examples are not the complete scope. Inspect all existing admin modules and finish the missing functionality you find.

Complete the actual intended workflows, including as applicable:

- durable persistence;
- validation and useful errors;
- server-side authorization;
- pagination, filtering, search, and bulk actions;
- imports and exports;
- uploads and media processing;
- integrations and connection states;
- background jobs, retries, and scheduling;
- loading, empty, disconnected, and failure states.

Match frontend routes, payloads, and response contracts to real backend implementations.

Do not treat a rendered page, button, HTTP 200 response, or success notification as proof that a feature works.

Do not replace unfinished functionality with “coming soon,” disabled controls, hidden navigation, or deleted features.

Do not invent adjacent features beyond the functionality established by the current interfaces and contracts.

Specific requirements:

- Settings must survive reloads and process restarts.
- Behavior and analytics must ingest, persist, retrieve, aggregate, and display actual test activity.
- Scheduled actions must execute through a working worker.
- Malware scanning must use a real scanner and truthful states such as pending, clean, infected, failed, or unavailable. Unscanned files must never be reported as clean.
- Fresh-database installation must be reproducible and include required structural defaults.
- Required migration failures must not produce a misleading healthy startup.
- Authorization must be consistent across the UI, SvelteKit server, and Rust API.

BLOG EDITOR EXCLUSION

Identify the exact blog-editor file boundary before editing.

Do not repair, redesign, replace, or extend the blog editor, including its BlockEditor implementation and editor-specific integrations.

The rest of the CMS, blog listing, categories, media library, and unrelated content functionality remain in scope.

Preserve compatibility when changing shared infrastructure. If something cannot be completed without modifying excluded editor code, document that dependency explicitly.

EXTERNAL INTEGRATIONS

When live credentials are unavailable:

- finish the implementation and configuration flow;
- verify against a provider sandbox, local service, or controlled adapter where possible;
- distinguish adapter tests from real provider verification;
- record the exact remaining external requirement.

Never fabricate provider success. Continue work that does not depend on the missing requirement.

PHASE 1 VERIFICATION

Add meaningful tests for completed workflows and regression risks.

Run all applicable AGENTS.md gate commands, including:

- vp check
- vp test using the appropriate non-watch invocation
- vp build
- vp pack
- pnpm --filter revolution-svelte check
- pnpm --filter revolution-svelte lint
- pnpm --filter revolution-svelte test:unit
- pnpm --filter revolution-svelte build
- pnpm test:seo

Also run the relevant Rust checks and integration tests.

Let checks finish. A stopped, skipped, blocked, or unrun check is not a passing check.

Use a fresh disposable database to verify installation.

Perform authenticated browser verification:

- login and logout;
- navigation through every in-scope admin page;
- dynamic pages using test fixtures;
- the actual workflows for every module;
- persistence after reload;
- unauthenticated and non-admin access rejection;
- failed integrations and unavailable-service behavior.

Check console errors and failed network requests.

Update the completion matrix with actual evidence.

PHASE 1 COMPLETION GATE

Do not begin copying or extracting until all discovered in-scope implementation gaps are resolved and relevant verification passes.

The blog editor remains excluded.

If an external requirement prevents verification, clearly report it and whether it blocks phase completion. Do not silently redefine incomplete work as complete.

Once Phase 1 passes, continue directly to Phase 2 if the destination path is available. If it is missing, request it at that point. Do not let a missing destination delay Phase 1.

PHASE 2 — COPY THE COMPLETED SYSTEM

Verify that my destination folder is empty. Never overwrite unrelated files.

Copy the completed admin system and its required dependencies. Leave the completed source project intact.

Include everything required to run:

- admin pages, layouts, navigation, components, styles, and assets;
- authentication, login/logout, sessions, and permissions;
- shared stores and utilities;
- SvelteKit hooks, endpoints, proxies, and server functions;
- Rust API, services, models, and business logic;
- database schema, migrations, structural defaults, and bootstrap scripts;
- background workers and schedules;
- cache, search, storage, and integration configuration;
- build configuration, lockfiles, and relevant tests.

Preserve excluded blog-editor files unchanged if they are included.

Do not copy live secrets, customer data, production database contents, existing sessions, dependency folders, generated builds, or Git history.

PHASE 3 — RUN THE COPY INDEPENDENTLY

Make the destination application work without the original project’s running frontend, API, database, or login session.

Use isolated local ports, service names, database names, and volumes. Starting or resetting the copy must not affect the source project.

Provide a reproducible setup for:

- dependency installation;
- required local services;
- empty-database initialization;
- administrator creation;
- API and worker startup;
- admin interface startup;
- shutdown and safe local reset.

Make branding, application URLs, cookie names, email identity, storage, and website-specific assumptions configurable where appropriate.

Keep existing functionality available. Separate business-specific features from shared infrastructure where needed for independence.

Optional integrations must not prevent unrelated functionality from starting. Preserve strict validation for enabled integrations.

Verify the destination application from a fresh setup, including login, navigation, persistence, permissions, and representative workflows for every module.

PHASE 4 — HAND OFF FOR MY POLISHING

Deliver:

- the repaired and verified source project;
- the independently runnable copy;
- exact setup and startup commands;
- environment examples explaining required and optional variables;
- a secure administrator bootstrap process;
- the completion matrix and verification results;
- a concise guide to changing branding, navigation, styles, and modules;
- any genuine remaining external requirements.

Future installation into other websites and web apps is NOT part of this implementation. Document useful integration boundaries, but do not build a plugin marketplace, universal installer, or multi-tenant platform now.

COMPLETION STANDARD

Do not claim “everything is complete end to end” without supporting evidence.

Clearly distinguish:

- implemented and verified;
- implemented but awaiting external verification;
- blocked;
- excluded blog-editor work.

Keep a progress document so the task can resume across context limits.

The required result is: finish the current admin/backend first, then provide a separate working copy that I can run and polish.
