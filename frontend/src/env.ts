import { defineEnvVars } from '@sveltejs/kit/env';
import * as v from 'valibot';

/**
 * Environment variables, declared explicitly for SvelteKit 3.
 *
 * This replaces the `$env/dynamic/*` + `$env/static/*` modules (and the old
 * hand-written `src/env.d.ts` ambient declarations, now deleted). Variables
 * declared here are importable from `$app/env/private`, or `$app/env/public`
 * when marked `public`.
 *
 * Validators are Standard Schema (valibot), per the SvelteKit environment
 * variables docs. Every variable here is genuinely optional — each consumer
 * funnels it through a `||` fallback chain such as
 * `API_BASE_URL || BACKEND_URL || 'http://localhost:8080'` — so the schemas
 * default to `''` rather than requiring a value, which is what the previous
 * `$env/dynamic/*` reads did. Without a validator SvelteKit would *require*
 * a value and fail the build.
 */

/**
 * Optional free-form value: absent reads as `''`, which is falsy for the `||`
 * chains every consumer funnels through.
 *
 * Deliberately NOT narrowed further — in particular the URL-valued variables
 * below are not run through `v.url()`. On this stack a rejected value is not a
 * localised error, it is a total outage:
 *
 *   - a failed schema makes SvelteKit call `handle_issues()`, which throws
 *     (`@sveltejs/kit/src/exports/internal/env.js`);
 *   - adapter-cloudflare calls `server.init({ env })` at Worker module scope and
 *     `await`s it before routing *every* request (`.svelte-kit/cloudflare/
 *     _worker.js`), so the throw takes down 100% of dynamic routes — every
 *     `/api/*` proxy and all SSR pages — not just the feature that reads the
 *     variable.
 *
 * These values are set in the Cloudflare Pages dashboard, so CI cannot see them
 * and a stricter schema can only ever be disproved in production. Meanwhile the
 * consumers already degrade gracefully: the API bases sit behind
 * `API_BASE_URL || BACKEND_URL || 'http://localhost:8080'`, and
 * VITE_ERROR_TRACKING_URL is guarded by `if (errorEndpoint)` inside a
 * `try`/`catch`, so a malformed value costs one failed fetch. A relative path
 * is also a legitimate value there — `fetch()` accepts one, and the client half
 * of that feature reads the same name via `import.meta.env` — but `v.url()`
 * rejects it.
 *
 * Trading a silently-degraded error report for a site-wide 500 is a bad
 * exchange, so validation stops at "is a string".
 */
const optionalString = v.optional(v.string(), '');

export const variables = defineEnvVars({
	API_BASE_URL: {
		description: 'Base URL of the Rust/Axum API. Primary source for every +server.ts proxy.',
		schema: optionalString
	},
	BACKEND_URL: {
		description: 'Fallback API base URL, used when API_BASE_URL is unset.',
		schema: optionalString
	},
	API_URL: {
		description: 'Legacy API base URL still read by a few server load functions.',
		schema: optionalString
	},
	VITE_ERROR_TRACKING_URL: {
		description: 'Endpoint that hooks.server.ts POSTs handled errors to in production.',
		schema: optionalString
	},
	FRED_API_KEY: {
		description: 'Federal Reserve (FRED) API key — options-calculator treasury rates.',
		schema: optionalString
	},
	POLYGON_API_KEY: {
		description: 'Polygon.io API key — options chain and quote data.',
		schema: optionalString
	},
	FMP_API_KEY: {
		description: 'Financial Modeling Prep API key — historical volatility.',
		schema: optionalString
	},
	FINNHUB_API_KEY: {
		description: 'Finnhub API key — symbol search and quotes.',
		schema: optionalString
	},
	ANTHROPIC_API_KEY: {
		description: 'Anthropic API key — CMS AI content generation and media alt-text.',
		schema: optionalString
	},
	BUNNY_STREAM_LIBRARY_ID: {
		description: 'Bunny.net Stream library id for video playback.',
		schema: optionalString
	},
	BUNNY_VIDEO_LIBRARY_ID: {
		description: 'Bunny.net video library id used by the admin upload flow.',
		schema: optionalString
	},
	PUBLIC_STRIPE_PUBLISHABLE_KEY: {
		description:
			'Stripe publishable key for client-side checkout. Safe to expose; optional because lib/config/stripe.ts guards on it via isStripeConfigured().',
		public: true,
		static: true,
		// Identity validator: returns `undefined` when unset, preserving the
		// previous `string | undefined` type. Omitting a validator entirely would
		// make the value required and break `pnpm build` wherever Stripe is not
		// configured. `static` inlines it so unused checkout code can be dropped.
		schema: (value) => value
	}
});
