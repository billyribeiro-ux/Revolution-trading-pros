/**
 * Favorites — Remote Functions (query + command)
 * ─────────────────────────────────────────────────────────────────────────────
 * Replaces the per-page `onMount` → `fetch('/api/favorites')` → `isLoading`
 * client waterfall with a SvelteKit `query`. The query is awaited in each
 * route's `+page.server.ts` `load` and the result is handed to the component as
 * `data.favorites` — the SAME server-prefetch pattern proven on the dashboard
 * sidebar (`memberships.remote.ts`), so the list is server-rendered on first
 * paint (verified: it appears in the SSR HTML) with no loading skeleton.
 *
 * Why prefetch-in-load rather than reactive `query.current` in the component:
 * this repo runs with `compilerOptions.experimental.async` OFF, so `await` in
 * markup / SSR-resolved `query.current` is unavailable — a component-level
 * `query.current` consumer would SSR its loading branch and only fill in after
 * hydration. Awaiting in `load` is the idiomatic way to SSR remote-query data
 * under that constraint. (If `experimental.async` is enabled later, this can
 * move to `{#await getFavorites(slug)}` + a `command().updates(...)` single-
 * flight mutation; the query/command split here is already shaped for it.)
 *
 * Auth: same-origin `event.fetch('/api/favorites…')` forwards the request
 * cookies to the `/api` proxy, which re-signs them for the Axum backend — the
 * identical path the old client `fetch` took, so no auth behavior changes.
 */
import * as v from 'valibot';
import { error } from '@sveltejs/kit';
import { query, command, getRequestEvent } from '$app/server';
import type { Favorite } from './favorites.types';

/** Room slugs are short kebab-case identifiers (e.g. `explosive-swings`). */
const RoomSlugSchema = v.pipe(v.string(), v.nonEmpty());

/**
 * The current user's favorites for a given room. Non-fatal on failure: returns
 * `[]` so the page renders its empty-state rather than erroring (the proxy
 * itself already collapses an unauthenticated/unavailable backend to an empty
 * success payload).
 */
export const getFavorites = query(RoomSlugSchema, async (roomSlug): Promise<Favorite[]> => {
	const { fetch } = getRequestEvent();

	const res = await fetch(`/api/favorites?room_slug=${encodeURIComponent(roomSlug)}`);
	if (!res.ok) return [];

	const body = await res.json();
	return Array.isArray(body?.data) ? (body.data as Favorite[]) : [];
});

/**
 * Remove a favorite. `roomSlug` is validated but not otherwise needed server-
 * side; it's part of the signature so the call site stays explicit and so a
 * future single-flight migration (`getFavorites(roomSlug).refresh()`) is a
 * one-line change. The page applies the removal optimistically, so no server-
 * driven refresh is wired here (an unconsumed `.refresh()` would be dead code).
 */
export const removeFavorite = command(
	v.object({
		id: v.pipe(v.number(), v.integer(), v.minValue(1)),
		roomSlug: RoomSlugSchema
	}),
	async ({ id }) => {
		const { fetch } = getRequestEvent();

		const res = await fetch(`/api/favorites/${id}`, { method: 'DELETE' });
		if (!res.ok) {
			error(res.status === 401 ? 401 : 500, 'Failed to remove favorite');
		}
	}
);
