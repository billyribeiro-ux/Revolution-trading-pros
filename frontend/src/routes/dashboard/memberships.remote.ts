/**
 * Dashboard membership remote functions.
 * ─────────────────────────────────────────────────────────────────────────────
 * A type-safe SvelteKit remote `query` for the current user's membership list.
 * Runs on the server, reads the auth cookie via `getRequestEvent()`, and forwards
 * it through the same-origin `/api` proxy. Because it's a remote function it can
 * be awaited from anywhere — server `load`, components, event handlers — and
 * SvelteKit dedupes concurrent/identical calls across SSR and the client
 * (kit ≥ 2.61: `await query()` directly, no `.run()`).
 */
import { query, getRequestEvent } from '$app/server';

/** Minimal flat membership shape the sidebar needs to render its dynamic nav. */
export interface SidebarMembership {
	id: string;
	name: string;
	type: string;
	slug: string;
	status: string;
	icon?: string;
}

/** Category order the client derives — keep SSR + client identical to avoid a
 *  sidebar re-order (and the ~0.01 CLS that comes with it) after hydration. */
const TYPE_ORDER: Record<string, number> = {
	'trading-room': 0,
	'alert-service': 1,
	course: 2,
	indicator: 3,
	scanner: 4,
	'weekly-watchlist': 5,
	'premium-report': 6
};

interface RawMembership {
	id?: string | number;
	name?: string;
	type?: string;
	slug?: string;
	status?: string;
	icon?: string;
}

/**
 * Returns the current user's active (or expiring) memberships, sorted into the
 * canonical category order. Non-fatal on failure — returns `[]` so the caller
 * (e.g. the dashboard sidebar) degrades gracefully rather than erroring.
 */
export const getMemberships = query(async (): Promise<SidebarMembership[]> => {
	const { fetch, locals } = getRequestEvent();

	// hooks.server.ts populates locals.user for authenticated dashboard routes.
	if (!(locals as { user?: unknown }).user) return [];

	try {
		const res = await fetch('/api/user/memberships');
		if (!res.ok) return [];

		const body = await res.json();
		const raw: RawMembership[] = body?.data?.memberships ?? body?.memberships ?? body?.data ?? [];
		if (!Array.isArray(raw)) return [];

		return raw
			.filter((m) => m && (m.status === 'active' || m.status === 'expiring'))
			.map((m) => ({
				id: String(m.id ?? m.slug ?? ''),
				name: m.name ?? '',
				type: m.type ?? '',
				slug: m.slug ?? '',
				status: m.status ?? 'active',
				icon: m.icon ?? undefined
			}))
			.map((m, i) => ({ m, i })) // stable tiebreak within a category
			.sort((a, b) => {
				const d = (TYPE_ORDER[a.m.type] ?? 99) - (TYPE_ORDER[b.m.type] ?? 99);
				return d !== 0 ? d : a.i - b.i;
			})
			.map(({ m }) => m);
	} catch {
		return [];
	}
});
