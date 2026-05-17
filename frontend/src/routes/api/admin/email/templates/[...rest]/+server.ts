/**
 * FIX-2026-04-26: shim for the SvelteKit POST=405 cliff (/api/admin/email/templates/*).
 * See createProxyShim.ts for the full explanation.
 *
 * PRINCIPAL-2026-05-17 (audit FULL_REPO_AUDIT_2026-05-17 §P2-F): the shim only
 * checks token PRESENCE. The canonical admin RBAC gate (`requireAdmin`) now
 * runs here BEFORE delegating to the shim, matching every other admin proxy.
 * The shim's upstream forwarding (URL, body, headers, streaming) is preserved.
 */
import type { RequestHandler } from '@sveltejs/kit';
import { createProxyShim } from '$lib/utils/createProxyShim';
import { requireAdmin } from '$lib/server/auth';

const shim = createProxyShim('/api/admin/email/templates');

export const GET: RequestHandler = (event) => {
	requireAdmin(event);
	return shim.GET(event);
};

export const POST: RequestHandler = (event) => {
	requireAdmin(event);
	return shim.POST(event);
};

export const PUT: RequestHandler = (event) => {
	requireAdmin(event);
	return shim.PUT(event);
};

export const PATCH: RequestHandler = (event) => {
	requireAdmin(event);
	return shim.PATCH(event);
};

export const DELETE: RequestHandler = (event) => {
	requireAdmin(event);
	return shim.DELETE(event);
};
