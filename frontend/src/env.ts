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

/** Optional free-form secret: absent reads as `''`, which is falsy for the `||` chains. */
const optionalSecret = v.optional(v.string(), '');

/**
 * Optional URL: absent reads as `''`, but a value that *is* set must be a
 * well-formed absolute URL. A typo'd API host would otherwise surface as a
 * confusing fetch failure on first request; this fails fast at startup
 * instead, which is the behaviour the SvelteKit docs recommend validators for.
 */
const optionalUrl = v.optional(v.union([v.literal(''), v.pipe(v.string(), v.url())]), '');

export const variables = defineEnvVars({
	API_BASE_URL: {
		description: 'Base URL of the Rust/Axum API. Primary source for every +server.ts proxy.',
		schema: optionalUrl
	},
	BACKEND_URL: {
		description: 'Fallback API base URL, used when API_BASE_URL is unset.',
		schema: optionalUrl
	},
	API_URL: {
		description: 'Legacy API base URL still read by a few server load functions.',
		schema: optionalUrl
	},
	VITE_ERROR_TRACKING_URL: {
		description: 'Endpoint that hooks.server.ts POSTs handled errors to in production.',
		schema: optionalUrl
	},
	FRED_API_KEY: {
		description: 'Federal Reserve (FRED) API key — options-calculator treasury rates.',
		schema: optionalSecret
	},
	POLYGON_API_KEY: {
		description: 'Polygon.io API key — options chain and quote data.',
		schema: optionalSecret
	},
	FMP_API_KEY: {
		description: 'Financial Modeling Prep API key — historical volatility.',
		schema: optionalSecret
	},
	FINNHUB_API_KEY: {
		description: 'Finnhub API key — symbol search and quotes.',
		schema: optionalSecret
	},
	ANTHROPIC_API_KEY: {
		description: 'Anthropic API key — CMS AI content generation and media alt-text.',
		schema: optionalSecret
	},
	BUNNY_STREAM_LIBRARY_ID: {
		description: 'Bunny.net Stream library id for video playback.',
		schema: optionalSecret
	},
	BUNNY_VIDEO_LIBRARY_ID: {
		description: 'Bunny.net video library id used by the admin upload flow.',
		schema: optionalSecret
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
