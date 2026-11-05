# Backend verification

Run the complete guarded email/CRM integration proof from the repository root:

```bash
api/scripts/verify-email-backend.sh
```

The runner creates a new PostgreSQL cluster and empty database, applies the
tracked baseline and migrations through the existing test, and executes its
local provider adapters. It uses `env -i` with explicit fixture credentials;
it does not source `.env`. PostgreSQL listens only on `127.0.0.1` and stops
when the runner exits. The fixture directory and logs remain available after
success or failure; the printed path identifies that run's data.

Install the pinned Rust toolchain and dependencies before running: Cargo uses
offline mode. PostgreSQL must include `pg_stat_statements`. Its default port
is `55439`; the test also requires free ports `19020`, `19021`, `19025` and
`19026`. Those provider ports are fixed by the integration fixtures.

Choose another PostgreSQL port or an installation outside `PATH` with:

```bash
api/scripts/verify-email-backend.sh --port 55440 --pg-bin /absolute/postgresql/bin
```

The proof verifies local API, database and provider-adapter behavior. It does
not establish live provider delivery, external inbox receipt or browser
workflow completion.
