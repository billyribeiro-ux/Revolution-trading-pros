// @vitest-environment node
import { expect, it, vi } from 'vite-plus/test';
vi.mock('$app/env', () => ({ version: 'fixture-version' }));
import { authenticatedUser, privateAuthRedirect, secureAuthCookie } from '../auth-response.js';
it('accepts only validated identity-service fields and boolean authorization flags', () => {
	expect(
		authenticatedUser({
			data: { id: 12, email: 'fixture@example.test', role: 'user', is_superadmin: 'true' }
		})
	).toMatchObject({ id: 12, role: 'user', is_superadmin: false, is_developer: false });
	for (const input of [
		null,
		{},
		{ data: {} },
		{ id: '12', email: 'e', role: 'admin' },
		{ id: 0, email: 'e', role: 'admin' },
		{ id: 12, email: 'e' }
	])
		expect(() => authenticatedUser(input)).toThrow('Invalid identity response');
});
function event(method = 'GET', isDataRequest = false, headers?: Record<string, string>) {
	return { request: new Request('http://127.0.0.1/admin', { method, headers }), isDataRequest };
}
it('returns an actual private HTML redirect with no protected body', async () => {
	const response = privateAuthRedirect(event(), '/login?redirect=%2Fadmin');
	expect(response.status).toBe(303);
	expect(response.headers.get('Location')).toBe('/login?redirect=%2Fadmin');
	expect(response.headers.get('Cache-Control')).toBe('no-store');
	expect(response.headers.get('X-Robots-Tag')).toContain('noindex');
	expect(await response.text()).toBe('');
});
it('preserves the Kit3 data/action redirect envelope, version and private headers', async () => {
	for (const request of [
		event('GET', true),
		event('POST', false, { Accept: 'application/json', 'x-sveltekit-action': 'true' })
	]) {
		const response = privateAuthRedirect(
			request,
			'/login',
			'rtp_access_token=new-fixture; Path=/; HttpOnly'
		);
		expect(response.status).toBe(200);
		expect(await response.json()).toEqual({ type: 'redirect', status: 303, location: '/login' });
		expect(response.headers.get('Cache-Control')).toBe('no-store');
		expect(response.headers.get('X-Robots-Tag')).toContain('noindex');
		expect(response.headers.get('x-sveltekit-version')).toBe('fixture-version');
		expect(response.headers.get('Set-Cookie')).toContain('new-fixture');
	}
});
it('supports loopback HTTP consistently without weakening remote/HTTPS cookie security', () => {
	for (const host of ['localhost', '127.0.0.1', '[::1]']) {
		expect(secureAuthCookie(`http://${host}:5275`)).toBe(false);
		expect(secureAuthCookie(`https://${host}:5275`)).toBe(true);
	}
	expect(secureAuthCookie('https://example.test')).toBe(true);
	expect(secureAuthCookie('http://example.test')).toBe(true);
	expect(secureAuthCookie('http://localhost.example.test')).toBe(true);
});
