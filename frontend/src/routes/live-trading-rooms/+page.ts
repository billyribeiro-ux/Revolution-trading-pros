/**
 * Live Trading Rooms - Page Configuration
 *
 * ARCHITECTURE NOTE: This parent page uses SSR (not prerender) because:
 * 1. Child routes (/day-trading, /swing-trading, /small-accounts) are prerendered
 * 2. SvelteKit cannot create both /live-trading-rooms.html AND /live-trading-rooms/ directory
 * 3. SSR with caching headers (set in +page.server.ts) provides equivalent performance
 * 4. The +page.server.ts already sets Cache-Control for edge caching
 *
 * @see https://kit.svelte.dev/docs/page-options
 * @module routes/live-trading-rooms/+page
 * @version 1.0.1
 */

// SSR parent page - child routes prerender into /live-trading-rooms/ directory
export const prerender = false;
export const ssr = true;
export const csr = true;
