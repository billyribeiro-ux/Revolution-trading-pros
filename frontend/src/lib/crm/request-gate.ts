export function isCrmRequestUrl(input: string): boolean {
	const path = new URL(
		input,
		typeof window === 'undefined' ? 'http://localhost' : window.location.href
	).pathname;
	return path === '/api/admin/crm' || path.startsWith('/api/admin/crm/');
}
export interface CrmClientRequest {
	method: string;
	path: string;
	query: URLSearchParams;
	checked: boolean;
	reason: string;
}
/** Synchronous browser preflight, with no stored identity/grants. Server checks remain authoritative. */
export function assertCrmClientRequest(input: string, method = 'GET'): void {
	if (typeof window === 'undefined') return;
	const url = new URL(input, window.location.href);
	if (!url.pathname.startsWith('/api/admin/crm/') || url.pathname === '/api/admin/crm/permissions')
		return;
	const detail: CrmClientRequest = {
		method,
		path: url.pathname,
		query: url.searchParams,
		checked: false,
		reason: 'CRM access context is unavailable'
	};
	const accepted = window.dispatchEvent(
		new CustomEvent<CrmClientRequest>('crm-request', { detail, cancelable: true })
	);
	if (!accepted || !detail.checked) throw new Error(detail.reason);
}
export function isCrmClientRequest(value: unknown): value is CrmClientRequest {
	if (!value || typeof value !== 'object') return false;
	const row = value as Record<string, unknown>;
	return (
		typeof row.method === 'string' &&
		typeof row.path === 'string' &&
		row.query instanceof URLSearchParams &&
		typeof row.checked === 'boolean' &&
		typeof row.reason === 'string'
	);
}
