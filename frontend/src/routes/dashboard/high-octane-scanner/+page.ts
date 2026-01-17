/**
 * High Octane Scanner Dashboard Page - Load Configuration
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Svelte 5 / SvelteKit (Dec 2025):
 * - SSR enabled for authenticated content
 * - No prerendering (dynamic user content)
 * - Type-safe configuration
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */

// ═══════════════════════════════════════════════════════════════════════════
// SSR/CSR CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Disable prerendering - this is authenticated user content
 */
export const prerender = false;

/**
 * Enable SSR for proper authentication handling
 */
export const ssr = true;

/**
 * Enable CSR for interactive features
 */
export const csr = true;
