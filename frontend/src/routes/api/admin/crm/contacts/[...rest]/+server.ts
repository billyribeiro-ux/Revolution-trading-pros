/**
 * FIX-2026-04-26: shim for the SvelteKit POST=405 cliff (/api/admin/crm/contacts/*).
 * See createProxyShim.ts for the full explanation.
 */
import { createProxyShim } from '$lib/utils/createProxyShim';

export const { GET, POST, PUT, PATCH, DELETE } = createProxyShim('/api/admin/crm/contacts');
