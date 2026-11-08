# @revolution/behavior-tracker

Browser-side behavior events with bounded buffering, path-only URLs, optional interaction tracking, and host-controlled construction and destruction. This package is the client SDK; the host application supplies consent decisions and a compatible ingest endpoint. It does not include the Rust admin API or database migrations.

## Install

Build and pack from this folder, then install the tarball in another project:

```sh
pnpm build
pnpm pack
pnpm add /path/to/revolution-behavior-tracker-0.1.0.tgz
```

Import the browser class only after the host's analytics consent is granted:

```ts
const { BehaviorTracker } = await import('@revolution/behavior-tracker');
const tracker = new BehaviorTracker({ apiEndpoint: '/api/behavior/events' });
// On consent withdrawal or route teardown:
tracker.destroy();
```

The endpoint must accept a `BehaviorEventBatch` JSON POST and return success only after durable storage. The current Revolution Trading Pros adapter is `frontend/src/routes/api/behavior/events/+server.ts` with `api/src/routes/behavior.rs` and its migrations. The host must enforce same-origin requests, rate and payload limits, retention, and authenticated access to reports. The package's browser controls do not replace server-side policy.

The tracker omits URL query strings, fragments, form values, CSS IDs and classes, and protected account paths. It honors browser Do Not Track and Global Privacy Control. The host must bind creation and destruction to its own consent system; this package does not set cookies or make a legal consent determination.

`pnpm check` and `pnpm test` verify the package. The repository's `frontend/src/lib/behavior/tracker.test.ts` and `frontend/tests/admin/behavior.spec.ts` exercise the website integration against a disposable API/database.
