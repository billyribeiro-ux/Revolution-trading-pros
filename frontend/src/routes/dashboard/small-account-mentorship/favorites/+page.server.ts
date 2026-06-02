/**
 * Small Account Mentorship Favorites — server load.
 * ─────────────────────────────────────────────────────────────────────────────
 * Server-prefetches the favorites via the `getFavorites` remote query so the
 * list is server-rendered on first paint (no client fetch, no loading
 * skeleton). Mirrors the proven `dashboard/+layout.server.ts` membership
 * prefetch. Non-fatal: the query returns `[]` on failure, so the page shows its
 * empty-state rather than erroring.
 */
import type { PageServerLoad } from './$types';
import { getFavorites } from '../../favorites.remote';

export const load: PageServerLoad = async () => {
	return { favorites: await getFavorites('small-account-mentorship') };
};
