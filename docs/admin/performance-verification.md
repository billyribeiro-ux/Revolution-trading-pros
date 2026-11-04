# Performance workflow

The source now stores first-party performance observations and reports measured data. This is one completed workflow within the ongoing admin/backend task; the source-wide completion gate remains open.

## Collection and reporting

The existing consent initializer now registers collection automatically. Observers and listeners stop on analytics withdrawal, and pending retries are aborted. The root layout releases the consent lifecycle on destruction. A separate 6.3s native browser proof covers initial denial, grant, withdrawal and restored consent across navigation. The wider consent banner/admin workflow audit remains open.

- `POST /api/analytics/web-vitals` and the legacy `POST /api/analytics/performance` accept JSON, including legacy `text/plain` beacon bodies. The SvelteKit proxy awaits the backend receipt. Neither layer acknowledges failed storage.
- Bodies are bounded to 16 KiB. Metric names, finite values, identities, dates and metadata are validated. URL query strings/fragments are discarded. Raw user-agent strings are classified into a device category rather than stored.
- The measurement ID and metric form one observation. Duplicate delivery creates no extra row; newer changed values update that observation, and stale replays cannot overwrite it. Browser retries retain the same body and identity, with at most three attempts. This is bounded retry, not a durable offline browser queue.
- Reports use p75 of recorded observations, with actual sample counts and URL/device/window filters. Missing values stay unavailable. These are first-party observations, not CrUX measurements or an invented external score. FID remains available as a legacy metric.
- LCP, CLS and INP keep stable identities across hide events. A measured zero CLS is retained; no-interaction INP stays unavailable. The native browser proof finalizes LCP through actual navigation and verifies an exact increase in committed observations before checking the dashboard.

## Operational data

Host CPU, memory and root filesystem usage come from `sysinfo 0.39.6`, using the [versioned upstream API](https://github.com/GuillaumeGomez/sysinfo/blob/v0.39.6/src/common/system.rs). Sampling runs in the background; the dashboard never waits for OS probes. Initial, unavailable and stale samples have explicit states. Unsupported measurements remain null.

Cache statistics describe the API process's live L1 data. Serialized key/value byte counts exclude allocator overhead. Capacity evictions exclude TTL expiry. Main shares the initialized service cache with AppState. Cache warming uses the existing room statistics/trade and membership-plan handlers; it returns actual counts and any partial failures. Static configuration is not falsely counted as cached work.

PostgreSQL statement statistics are optional. Without them, connection-pool data remains available and the dashboard explains the missing dependency. To enable them in a **local database service you administer**, start PostgreSQL with `shared_preload_libraries=pg_stat_statements`, then run:

```sql
CREATE EXTENSION IF NOT EXISTS pg_stat_statements;
```

Statement calls and mean execution time come from that extension for the current database. The slow-call tile counts calls in statements whose mean execution time is at least 100 ms. No SQL text is exposed. The ordinary migration path does not install an extension or change database server settings.

## Local evidence

- Current API proof with statement statistics enabled: `perf_r22`, 13.59s. The expanded proof includes populated room/plan caches, mutation and profitable-close invalidation, real host measurements, nonblocking snapshot reads, p75/deduplication/filtering, authorization, malformed/oversized input, legacy content type and storage failure.
- Same current proof without the extension: `perf_r23`, 13.26s. Optional statistics remain unavailable without breaking other workflows.
- Native browser workflow: `performance-browser5`, 5.0s. Real PerformanceObserver LCP, pagehide finalization, exact committed count, dashboard/filter/reload, warming and no-data state; no fabricated browser metric was substituted.
- Focused client/proxy/delivery/event regressions pass. Current full frontend suite: 79 files, 2,826 passed, 32 existing skips. Svelte check is 0 errors/0 warnings. Current Rust library: 152 passed, one scanner-environment ignore; the actual isolated ClamAV regression was separately run and passed.
- Broad current fresh-install/admin proof `r31`: 46.45s. The source still has unfinished modules and a root warning gate; these results do not authorize extraction.

The dedicated statistics fixture uses `/private/tmp/admin-move-performance-pg-20261104`, PostgreSQL port 55433 and log `/tmp/admin-move-performance-pg.log`. Its test databases begin `admin_move_verify_`. The existing service on port 55432 was not reconfigured or restarted. Stop only this fixture using its exact data directory:

```bash
/Users/billyribeiro/.local/share/mise/installs/postgres/18.6/bin/pg_ctl \
  -D /private/tmp/admin-move-performance-pg-20261104 stop
```

Earlier failed browser runs timed out on DevTools page-exit network events despite database persistence. The final proof uses the existing SDK callback, actual navigation and committed counts; no timeout/test threshold was relaxed. The expanded API proof's initial 57.5% expectation was corrected to the real trigger-derived 0% for an open trade, then explicitly verifies 100% after closing that fixture profitably.

Latest automatic-consent integration checks: frontend 80 files / 2828 passed (32 existing skips); root 81 files / 2836 passed (32 existing skips); production-preview SEO 114 passed / 17.9s. Root check has 0 errors and 31 excluded editor-integration warnings pending approval. These supersede earlier suite totals above.
