/**
 * FIX-2026-04-26: shim to break the SvelteKit POST=405 cliff for /api/admin/posts/*.
 *
 * The parent `+server.ts` in this folder claims all sub-paths for routing
 * purposes, blocking POST/PUT/PATCH on /admin/posts/<id>, bulk-delete,
 * bulk-status, duplicate, status, featured, import, export, analytics, etc.
 * (9 broken blog-admin actions per docs/audits/ADMIN_FAILURE_DATA.md §3a.)
 *
 * This `[...rest]/+server.ts` explicitly handles the sub-paths and forwards
 * to the Rust backend with the canonical `rtp_access_token` Bearer token.
 */
import { createProxyShim } from '$lib/utils/createProxyShim';

export const { GET, POST, PUT, PATCH, DELETE } = createProxyShim('/api/admin/posts');
