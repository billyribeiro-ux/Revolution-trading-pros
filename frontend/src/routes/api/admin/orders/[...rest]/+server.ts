/**
 * FIX-2026-04-26: shim to break the SvelteKit POST=405 cliff for
 * `/api/admin/orders/*`.
 *
 * The parent `+server.ts` in this folder claims all sub-paths for routing
 * purposes, blocking GET/POST/PUT/PATCH/DELETE on:
 *   - /admin/orders/:id              (detail)
 *   - /admin/orders/:id/status       (status update)
 *   - /admin/orders/:id/refund       (refund)
 *   - /admin/orders/:id/cancel       (cancel)
 *   - /admin/orders/:id/fulfill      (fulfill)
 *   - /admin/orders/:id/resend-confirmation
 *   - /admin/orders/export           (CSV download)
 *   - /admin/orders/stats            (aggregate stats)
 *
 * This `[...rest]/+server.ts` explicitly handles the sub-paths and forwards
 * to the Rust backend with the canonical `rtp_access_token` Bearer token.
 */
import { createProxyShim } from '$lib/utils/createProxyShim';

export const { GET, POST, PUT, PATCH, DELETE } = createProxyShim('/api/admin/orders');
