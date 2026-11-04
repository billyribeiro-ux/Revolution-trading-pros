# Email template verification

The primary `/admin/email/templates` authoring workflow and application delivery policy are verified locally. The broader email module remains incomplete; source extraction is still gated.

## Implemented

- Create, read, edit, search, paginate and delete stored templates. HTML, text, variables, type and explicit slug survive reloads. Name-only updates preserve the slug.
- Validation rejects malformed syntax, invalid fields and duplicate identities with useful errors. Referenced templates return409. Mutations and their operational logs commit together.
- Preview uses the same strict rendering rules as local delivery, with explicit JSON values, HTML escaping and a separate plain text alternative. Preview does not send email.
- Explicit admin tests submit the saved rendered content through the selected provider and persist its acknowledgment, rejection or uncertain outcome. Capability values are redacted in log models.
- Saved or newly created content becomes authoritative for application messages. Unchanged existing hosted aliases retain their previous delivery source. Inactive, renamed and deleted aliases cannot silently revive hosted content. Explicit recreation clears the durable barrier only after its audited transaction succeeds.
- The shared admin client preserves validation messages, keeps envelopes serializable, retries reads only, and does not mark a service unavailable merely because validation rejected an input.

## Evidence

Fresh **r46 passed94.97s**, `/tmp/admin-move-email-templates-r46.log`, with86 migration versions/latest094. It includes all preceding email, SMTP and connection regressions plus local application content, cold-state revocation, explicit recreation, rollback and policy-storage failure. Migrations093/094 are applied and immutable.

**Template browser3 passed2tests18.2s**, `/tmp/admin-move-email-template-browser3.log`: malformed JSON recovery without a write, authoring, edit/reload, text clearing, slug/type retention, strict preview, pagination across21 fixtures, server search, deletion and503/malformed-response recovery.

Original DTO tests9passed; proxy tests3passed; admin-client regressions4passed. Rust all-target check/Clippy and library164passed/1existing scanner-environment ignore. Frontend93files/2870passed with32existing skips. Root check reported zero warnings in2095 files; current components/modules pass Svelte MCP. The corrected isolated production build and SEO suite pass, using API8281 explicitly.

## Remaining scope

The secondary catalog client still needs matching metadata, envelopes, duplication and observed usage counts. The older `/email` dashboard now enforces admin access but its data/actions and comparative metrics remain unfinished. Legacy campaigns, sequences, automations and reports remain tracked in the completion matrix.

Provider tests use controlled local adapters and synthetic recipients. Authorized provider credentials, sender permissions, real TLS/account access and inbox delivery remain external verification requirements. No real campaign was sent.
