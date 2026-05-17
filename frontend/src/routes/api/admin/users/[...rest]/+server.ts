/**
 * FIX-2026-04-26: shim for the SvelteKit POST=405 cliff (/api/admin/users/*).
 * See createProxyShim.ts for the full explanation.
 *
 * PRINCIPAL-2026-05-17 (audit FULL_REPO_AUDIT_2026-05-17 §P2-F): the shim only
 * checks token PRESENCE. User sub-paths are privilege-bearing (role/credential
 * mutation lives under `/api/admin/users/*`), so the canonical RBAC gate runs
 * here BEFORE delegating to the shim — read via `requireAdmin`, mutation via
 * `requireSuperadmin` — matching `users/[id]/+server.ts`. The shim's upstream
 * forwarding (URL, body, headers, streaming) is preserved verbatim.
 */
import type { RequestHandler } from '@sveltejs/kit';
import { createProxyShim } from '$lib/utils/createProxyShim';
import { requireAdmin, requireSuperadmin } from '$lib/server/auth';

const shim = createProxyShim('/api/admin/users');

export const GET: RequestHandler = (event) => {
	requireAdmin(event);
	return shim.GET(event);
};

export const POST: RequestHandler = (event) => {
	requireSuperadmin(event);
	return shim.POST(event);
};

export const PUT: RequestHandler = (event) => {
	requireSuperadmin(event);
	return shim.PUT(event);
};

export const PATCH: RequestHandler = (event) => {
	requireSuperadmin(event);
	return shim.PATCH(event);
};

export const DELETE: RequestHandler = (event) => {
	requireSuperadmin(event);
	return shim.DELETE(event);
};
