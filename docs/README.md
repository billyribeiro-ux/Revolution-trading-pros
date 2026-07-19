# Docs

Documentation for the Revolution Trading Pros monorepo.

| Folder | Purpose | Read this when |
|--------|---------|----------------|
| [`development/`](development/) | Local dev runbook | You're picking the project up from a fresh clone or after time away |
| [`architecture/`](architecture/) | System design, auth flow, public service URLs | You need a map of how the pieces fit |
| [`setup/`](setup/) | Production deployment + Stripe + R2 setup | You're deploying or wiring up a 3rd-party service |
| [`audits/`](audits/) | 2026-04-25 audit series | You want to know what's broken / what's been done / what's next |
| [`frontend/`](frontend/) | Frontend-specific guides (responsive admin, footer audit, remote-functions migration) | You're working on the SvelteKit side |
| [`audits/`](audits/) (incident/forensic reports) | Older incident and forensic reports live in `audits/` — there is no separate `forensics/` folder | You're investigating "why was this done this way?" |

## Where to start

- **First-time contributor:** [`development/LOCAL_DEV.md`](development/LOCAL_DEV.md)
- **Returning to the project:** [`audits/REPO_STATE_2026-04-25.md`](audits/REPO_STATE_2026-04-25.md)
- **Need a system map:** [`architecture/SYSTEM_ARCHITECTURE_AND_AUTH.md`](architecture/SYSTEM_ARCHITECTURE_AND_AUTH.md)
- **Planning the next sprint:** [`audits/DISTINGUISHED_ENGINEER_AUDIT_2026-04-25.md`](audits/DISTINGUISHED_ENGINEER_AUDIT_2026-04-25.md) §9 (the prioritized backlog)
