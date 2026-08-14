import { defineEnvVars } from '@sveltejs/kit/env';

/**
 * Environment variables, declared explicitly for SvelteKit 3.
 *
 * This replaces the `$env/dynamic/*` + `$env/static/*` modules (and the old
 * hand-written `src/env.d.ts` ambient declarations, now deleted). Variables
 * declared here are importable from `$app/env/private`, or `$app/env/public`
 * when marked `public`.
 *
 * The `?? ''` validators are deliberate: under `$env/dynamic/*` an unset
 * variable read as `undefined`, and every consumer here funnels that through a
 * `||` fallback chain (`API_BASE_URL || BACKEND_URL || 'http://localhost:8080'`).
 * An empty string is falsy, so those chains behave exactly as before. Without a
 * validator SvelteKit would instead *require* a value and fail the build.
 */
export const variables = defineEnvVars({
	API_BASE_URL: { schema: (input) => input ?? '' },
	BACKEND_URL: { schema: (input) => input ?? '' },
	VITE_ERROR_TRACKING_URL: { schema: (input) => input ?? '' },
	FRED_API_KEY: { schema: (input) => input ?? '' },
	POLYGON_API_KEY: { schema: (input) => input ?? '' },
	API_URL: { schema: (input) => input ?? '' },
	BUNNY_STREAM_LIBRARY_ID: { schema: (input) => input ?? '' },
	FMP_API_KEY: { schema: (input) => input ?? '' },
	FINNHUB_API_KEY: { schema: (input) => input ?? '' },
	ANTHROPIC_API_KEY: { schema: (input) => input ?? '' },
	BUNNY_VIDEO_LIBRARY_ID: { schema: (input) => input ?? '' },
	// Optional on purpose — lib/config/stripe.ts guards on it
	// (`isStripeConfigured()`, plus an explicit "not configured" error log) so
	// the app degrades gracefully when checkout is not wired up. The identity
	// validator returns `undefined` when unset, which keeps the previous
	// `string | undefined` type; omitting it would make the value required and
	// break `pnpm build` in any environment without a Stripe key.
	PUBLIC_STRIPE_PUBLISHABLE_KEY: { public: true, static: true, schema: (value) => value }
});
