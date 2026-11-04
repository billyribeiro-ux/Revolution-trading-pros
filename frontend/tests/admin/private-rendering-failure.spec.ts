import { expect, test } from '@playwright/test';
import { createServer } from 'node:http';
// Explicit operational precondition: run while ONLY the task API8281 is stopped.
// The caller restores that task process in finally; never stop an unrelated API.
test('unavailable identity service denies private rendering without decoding claims or erasing cookies', async ({
	request,
	baseURL
}) => {
	expect(baseURL).toBe('http://127.0.0.1:5275');
	const headers = { Cookie: 'rtp_access_token=unverified-outage-fixture' };
	for (const path of ['/admin', '/my/courses']) {
		const response = await request.get(path, { headers, maxRedirects: 0 });
		expect(response.status(), 'Requires the isolated verification API to be offline').toBe(503);
		expect(response.headers()['cache-control']).toBe('no-store');
		expect(response.headers()['x-robots-tag']).toContain('noindex');
		expect(await response.text()).toContain('Authentication service is temporarily unavailable');
		expect(response.headers()['set-cookie']).toBeUndefined();
	}
});

test('identity and refresh adapter failures fail closed and preserve retryable sessions', async ({
	request,
	baseURL
}) => {
	expect(baseURL).toBe('http://127.0.0.1:5275');
	const fixture = createServer((incoming, outgoing) => {
		void (async () => {
			outgoing.setHeader('Content-Type', 'application/json');
			const token = incoming.headers.authorization?.replace('Bearer ', '');
			if (incoming.url === '/api/auth/me') {
				if (token === 'invalid-json') return outgoing.end('invalid JSON');
				if (token === 'missing-user') return outgoing.end('{"data":{}}');
				if (token === 'initial-service-503' || token === 'refreshed-user503') {
					outgoing.statusCode = 503;
					return outgoing.end('{}');
				}
				if (token === 'ordinary-with-flags')
					return outgoing.end(
						JSON.stringify({
							data: {
								id: 17,
								email: 'ordinary@example.test',
								role: 'user',
								is_superadmin: true,
								is_developer: true
							}
						})
					);
				outgoing.statusCode = 401;
				return outgoing.end('{}');
			}
			if (incoming.url === '/api/auth/refresh') {
				let body = '';
				for await (const chunk of incoming) body += String(chunk);
				const payload: { refresh_token?: string } = JSON.parse(body);
				if (payload.refresh_token === 'timeout') return;
				if (payload.refresh_token === 'refresh503') outgoing.statusCode = 503;
				else if (payload.refresh_token === 'denied') outgoing.statusCode = 401;
				else if (payload.refresh_token === 'malformed')
					return outgoing.end('{"data":{"access_token":12}}');
				else
					return outgoing.end(
						JSON.stringify({
							data: {
								access_token:
									payload.refresh_token === 'user503' ? 'refreshed-user503' : 'refreshed-denied',
								expires_in: 60
							}
						})
					);
				return outgoing.end('{}');
			}
			outgoing.statusCode = 404;
			outgoing.end('{}');
		})().catch(() => outgoing.destroy());
	});
	await new Promise<void>((resolve, reject) => {
		fixture.once('error', reject);
		fixture.listen(8281, '127.0.0.1', resolve);
	});
	try {
		for (const [access, refresh, status] of [
			['initial-service-503', '', 503],
			['invalid-json', '', 503],
			['missing-user', '', 503],
			['expired', 'refresh503', 503],
			['expired', 'malformed', 503],
			['expired', 'user503', 503],
			['expired', 'timeout', 503],
			['expired', 'denied', 303],
			['expired', 'user-denied', 303],
			['ordinary-with-flags', '', 303]
		] as const) {
			const response = await request.get('/admin', {
				headers: {
					Cookie: `rtp_access_token=${access}${refresh ? `; rtp_refresh_token=${refresh}` : ''}`
				},
				maxRedirects: 0
			});
			expect(response.status(), `${access}/${refresh}`).toBe(status);
			expect(response.headers()['cache-control']).toBe('no-store');
			expect(response.headers()['x-robots-tag']).toContain('noindex');
			if (status === 503) expect(response.headers()['retry-after']).toBe('5');
			if (refresh === 'user503' || refresh === 'user-denied') {
				expect(response.headers()['set-cookie']).toContain('rtp_access_token=refreshed-');
				expect(response.headers()['set-cookie']).not.toContain('Max-Age=0');
			} else expect(response.headers()['set-cookie']).toBeUndefined();
			if (access === 'ordinary-with-flags')
				expect(response.headers().location).toBe('/?error=admin_required');
		}
	} finally {
		await new Promise<void>((resolve, reject) =>
			fixture.close((error) => (error ? reject(error) : resolve()))
		);
	}
});
