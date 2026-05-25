# Retired audit docs (2026-05-24)

Twelve audit/result/plan markdowns retired here on 2026-05-24 because they
had zero inbound references anywhere in the repository — no source code,
no other docs, no CHANGELOG entries linked to them. Per repo policy
("never delete retired files; move them into the retired folder") they
are preserved here for forensic reference.

## Forensic verification

Each file was passed through two grep passes across the entire repo
**excluding the file itself**:

1. Word-boundary match — `git grep -lE "\\b<filename>\\b"`
2. Fixed-string basename match — `git grep -lFw "<basename-no-extension>"`

Both passes returned **0 hits** for each of the twelve files. Verification
performed at commit `30cda69^` (immediately before the move commit).

## Retired files

| File | Original location | Inbound references |
|---|---|---|
| `ADMIN_CMS_AUDIT_2026-05-10.md` | `docs/audits/` | 0 |
| `CSS_ISOLATION_PLAN_2026-04-25.md` | `docs/audits/` | 0 |
| `GSAP_ANIMATION_AUDIT_2026-05-23.md` | `docs/audits/` (was at repo root until `eb91d4c`) | 0 |
| `GSAP_COMPLETE_VERIFICATION_2026-05-23.md` | `docs/audits/` (was at repo root until `eb91d4c`) | 0 |
| `OVERSIZED_COMPONENTS_CHANGELOG_2026-05-16.md` | `docs/audits/` | 0 |
| `PORT_FLEXIBILITY_2026-04-27.md` | `docs/audits/` | 0 |
| `RESTORE_AND_MIGRATE_PLAN_2026-05-23.md` | `docs/audits/` (was at repo root until `eb91d4c`) | 0 |
| `SVELTE_REMEDIATION_PLAN_2026-05-16.md` | `docs/audits/` | 0 |
| `TASK3_RESULT.md` | `docs/audits/` | 0 |
| `TASK5_RESULT.md` | `docs/audits/` | 0 |
| `TASK6_RESULT.md` | `docs/audits/` | 0 |
| `TRADING_ROOMS_BACKEND_GAPS_2026-05-16.md` | `docs/audits/` | 0 |

## What was kept in `docs/audits/`

53 audit MDs remain in `docs/audits/` because the forensic sweep found
inbound references from:

- `CHANGELOG.md`, `CLAUDE.md`, `README.md`, `docs/README.md`
- Other audit docs that cross-link to them
- **Source code** — e.g. `api/src/routes/consent.rs`,
  `frontend/src/lib/api/abandoned-carts.ts`,
  `frontend/src/lib/stores/connections.svelte.ts`,
  `frontend/src/routes/+layout.svelte`,
  `api/migrations/schema.sql`. Moving them would break those links.

To re-run the verification on the current tree:

```bash
for f in docs/audits/*.md; do
  name=$(basename "$f")
  refs=$(git grep -lFw "${name%.md}" -- ":!docs/audits/$name" | wc -l)
  [ "$refs" = "0" ] && echo "ORPHAN: $name"
done
```
