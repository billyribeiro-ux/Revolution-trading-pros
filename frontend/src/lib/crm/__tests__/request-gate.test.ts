import { afterEach, expect, it, vi } from 'vite-plus/test';
import { assertCrmClientRequest, isCrmClientRequest } from '../request-gate.js';
import { authorizedCrmRequestFixture } from './request-fixture.js';
afterEach(() => vi.unstubAllGlobals());
it('closes browser requests without a capability provider while preserving bootstrap and native guards', () => {
	expect(() => assertCrmClientRequest('/api/admin/crm/contacts')).toThrow('context is unavailable');
	expect(() => assertCrmClientRequest('/api/admin/crm/permissions')).not.toThrow();
	expect(() => assertCrmClientRequest('/api/admin/email/templates')).not.toThrow();
});
it('accepts only a checked canonical operation and rejects unknown paths and duplicate queries', () => {
	const release = authorizedCrmRequestFixture();
	try {
		expect(() => assertCrmClientRequest('/api/admin/crm/contacts?page=1')).not.toThrow();
		expect(() => assertCrmClientRequest('/api/admin/crm/contacts?page=1&page=2')).toThrow();
		expect(() => assertCrmClientRequest('/api/admin/crm/contacts/0')).toThrow();
		expect(() => assertCrmClientRequest('/api/admin/crm/managers/17', 'PATCH')).toThrow();
	} finally {
		release();
	}
	expect(() => assertCrmClientRequest('/api/admin/crm/contacts')).toThrow();
});
it('does not trust a noncancelable or unchecked event payload', () => {
	expect(
		isCrmClientRequest({ method: 'GET', path: '/contacts', query: {}, checked: true, reason: '' })
	).toBe(false);
	const deny = (event: Event) => {
		if (event instanceof CustomEvent && isCrmClientRequest(event.detail)) {
			event.detail.checked = true;
			event.detail.reason = 'Revoked current grant';
			event.preventDefault();
		}
	};
	window.addEventListener('crm-request', deny);
	try {
		expect(() => assertCrmClientRequest('/api/admin/crm/contacts')).toThrow(
			'Revoked current grant'
		);
	} finally {
		window.removeEventListener('crm-request', deny);
	}
});
