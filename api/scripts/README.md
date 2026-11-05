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

## API container inputs

Build from the repository root:

```bash
docker build -f api/Dockerfile -t revolution-admin-api:local .
```

The Dockerfile-specific context allowlist includes the API sources, locked
dependencies, migrations, fonts/licenses and the two canonical frontend author
and public-media JSON files. The API embeds these inputs; it does not need
frontend files or installed host fonts at runtime. The `sources` target checks
the exact packaged inputs without starting a service or compiling Rust:

```bash
docker build -f api/Dockerfile --target sources -t revolution-admin-api-sources:local .
```

## Actual malware scanner

The protocol-peer tests do not establish real malware detection. With an actual
ClamAV daemon listening on a dedicated loopback port, run the existing test:

```bash
CLAMD_ADDRESS=127.0.0.1:13310 cargo test --manifest-path api/Cargo.toml --lib \
  services::malware::tests::real_clamav_clean_and_eicar_verdicts -- \
  --exact --ignored --nocapture --test-threads=1
```

This sends a clean document and the harmless EICAR antivirus test string to
the real engine. Record its version/signature database and readiness; an
unavailable scanner or unscanned file must never be counted as clean. See the
[official ClamAV container instructions](https://docs.clamav.net/manual/Installing/Docker.html)
for an isolated daemon. The ordinary library suite intentionally requires this
test to be invoked explicitly when that service is available.
