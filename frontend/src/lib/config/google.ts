/**
 * Google API browser-exposed configuration
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * SECURITY NOTE — read before touching these values.
 *
 * The Google Calendar API key and the OAuth client ID below are loaded into
 * the *browser* bundle by design (the Google Calendar JS client runs in the
 * page). They are therefore inherently public — they CANNOT be made secret by
 * any amount of bundler config, env plumbing, or obfuscation. Anyone can read
 * them from the shipped JS. This is true of every browser-side Google API key.
 *
 * The real, and only, mitigations are owned OUTSIDE this codebase:
 *   1. Google Cloud Console → API key → Application restrictions →
 *      "HTTP referrers" locked to the production domain(s).
 *   2. Google Cloud Console → API key restrictions → restrict to the
 *      Calendar API only.
 *   3. Periodic key / client-ID rotation.
 *
 * What this module DOES do: collapse five duplicated literals into one
 * env-overridable definition so a rotation is a single-line change (or, in
 * deployed environments, just a `VITE_*` env var) instead of a find-replace
 * across the codebase.
 *
 * The fallback values below are the CURRENT LIVE production key/client-ID.
 * They are kept as the default so that an environment with the `VITE_*` vars
 * unset (e.g. local dev, the current production deploy) builds and behaves
 * EXACTLY as before this change. Do not remove the fallback without first
 * confirming every deploy target sets the env vars — that would break the
 * live calendar.
 *
 * To rotate: set a new key in Google Cloud, then either update the fallback
 * here OR set VITE_GOOGLE_CALENDAR_API_KEY / VITE_GOOGLE_OAUTH_CLIENT_ID in
 * the deploy environment (preferred — no code change, no redeploy of source).
 */

export const GOOGLE_CALENDAR_API_KEY: string =
	import.meta.env.VITE_GOOGLE_CALENDAR_API_KEY ?? 'AIzaSyBTC-zYg65B6xD8ezr4gMWCeUNk7y2Hlrw';

export const GOOGLE_OAUTH_CLIENT_ID: string =
	import.meta.env.VITE_GOOGLE_OAUTH_CLIENT_ID ??
	'656301048421-g2s2jvb2pe772mnj8j8it67eirh4jq1f.apps.googleusercontent.com';
