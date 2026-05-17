/**
 * Server-side admin gate (P0-3).
 *
 * Before this file existed, `/admin` was protected only by:
 *   - `hooks.server.ts` PROTECTED_ROUTES → blocks UNAUTHENTICATED users, but
 *     never checks `role`; and
 *   - a client-side `onMount` redirect in `admin/+layout.svelte` → bypassable
 *     (disable JS, kill the navigation, or race the in-flight data fetches).
 *
 * An authenticated non-admin (a plain `user`/`member`) was therefore served
 * the admin shell; only the Rust API's own RBAC stood between them and the
 * data. This restores defense-in-depth: the role decision is now made on the
 * server from `event.locals.user.role` (populated by `hooks.server.ts`).
 *
 * Server `load` runs on the server even though `+layout.ts` sets
 * `ssr = false` (SvelteKit still fetches `__data.json`), so the redirect is
 * authoritative and cannot be skipped by disabling JavaScript.
 *
 * Role policy is the single server-side source of truth in
 * `$lib/server/auth.ts` — the SAME predicate the admin API proxies use, so
 * the page gate and the API gate can never drift apart.
 */

import { redirect } from '@sveltejs/kit';
import { isAdminRole } from '$lib/server/auth';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ locals, url }) => {
	const user = locals.user;

	if (!user) {
		redirect(303, `/login?redirect=${encodeURIComponent(url.pathname)}`);
	}

	if (!isAdminRole(user.role)) {
		redirect(303, '/?error=admin_required');
	}

	// Hand the verified admin to the client for hydration (house pattern,
	// matches dashboard/+layout.server.ts).
	return { user };
};
