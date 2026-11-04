import { expect, test } from '@playwright/test';

test('configured email elevation agrees across API, SvelteKit and the live auth store', async ({
	page,
	request: outsider,
	baseURL
}) => {
	expect(baseURL).toBe('http://127.0.0.1:5274');
	const failures: string[] = [];
	let authModuleUrl: string | undefined;
	page.on('pageerror', (e) => failures.push(e.message));
	page.on('response', (r) => {
		if (/\/lib\/stores\/auth\.svelte\.(js|ts)$/.test(new URL(r.url()).pathname))
			authModuleUrl ??= r.url();
		if (new URL(r.url()).pathname.startsWith('/api/') && r.status() >= 400)
			failures.push(`${r.status()} ${r.url()}`);
	});
	page.on('requestfailed', (r) => {
		if (
			new URL(r.url()).pathname.startsWith('/api/') &&
			!r.failure()?.errorText.includes('ERR_ABORTED')
		)
			failures.push(`${r.failure()?.errorText} ${r.url()}`);
	});
	page.on('console', (m) => {
		if (m.type() !== 'error') return;
		const u = m.location().url;
		if (u && !['127.0.0.1', 'localhost'].includes(new URL(u).hostname)) return;
		failures.push(m.text());
	});
	await page.route('**/*', (r) => {
		const u = new URL(r.request().url());
		return ['127.0.0.1', 'localhost'].includes(u.hostname) ||
			!['http:', 'https:'].includes(u.protocol)
			? r.continue()
			: r.abort();
	});
	const password = 'OnlyForDisposableTest123!';
	const ordinary = await outsider.post('/api/auth/login', {
		data: { email: 'user@admin-completion.test', password }
	});
	expect(ordinary.ok()).toBe(true);
	const ordinaryBody = await ordinary.json();
	expect(ordinaryBody.user.role).toBe('user');
	expect(ordinaryBody.user.is_superadmin).toBe(false);
	expect(ordinaryBody.user.is_developer).toBe(false);
	expect(
		(
			await outsider.post('/api/admin/connections/unknown_auth_fixture/test', {
				data: { is_superadmin: true }
			})
		).status()
	).toBe(403);
	const login = await page.request.post('/api/auth/login', {
		data: { email: 'admin@admin-completion.test', password }
	});
	expect(login.ok()).toBe(true);
	expect(login.headers()['cache-control']).toBe('no-store');
	const logged = await login.json();
	expect(logged.user.role).toBe('admin');
	expect(
		logged.user.is_superadmin,
		'Run only with task fixture SUPERADMIN_EMAILS configured as documented'
	).toBe(true);
	expect(logged.user.is_developer).toBe(true);
	const me = await page.request.get('/api/auth/me');
	expect(me.ok()).toBe(true);
	expect(me.headers()['cache-control']).toBe('no-store');
	const body = await me.json();
	expect(body.role).toBe('admin');
	expect(body.is_superadmin).toBe(true);
	// Unknown key uses a mounted backend operation after the real SvelteKit gate.
	// It cannot contact a provider or modify a connection.
	const invalid = await page.request.post('/api/admin/connections/unknown_auth_fixture/test', {
		data: {}
	});
	expect(invalid.status()).toBe(400);
	expect((await invalid.json()).error).toBe('No credentials provided');
	await page.goto('/admin', { waitUntil: 'networkidle' });
	expect(authModuleUrl, 'Inspect the exact module loaded by the application').toBeTruthy();
	const loadedAuthModuleUrl = authModuleUrl;
	if (!loadedAuthModuleUrl) throw new Error('The application auth module was not loaded');
	await expect
		.poll(() =>
			page.evaluate(async (path) => {
				const module = (await import(path)) as {
					authStore: {
						user: { role?: string; is_superadmin?: boolean } | null;
						isSuperAdmin: boolean;
						userRole: string | null;
					};
				};
				return {
					role: module.authStore.user?.role,
					flag: module.authStore.user?.is_superadmin,
					superadmin: module.authStore.isSuperAdmin,
					highestRole: module.authStore.userRole
				};
			}, loadedAuthModuleUrl)
		)
		.toEqual({ role: 'admin', flag: true, superadmin: true, highestRole: 'super-admin' });
	await page.reload({ waitUntil: 'networkidle' });
	const after = await page.request.get('/api/auth/me');
	expect((await after.json()).is_superadmin).toBe(true);
	expect(failures).toEqual([]);
});
