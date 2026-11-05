import { expect, test } from '@playwright/test';
import { fixtureRelay, mimeText, mimeSubject } from './fixtures/smtp-relay.js';
let relay: Awaited<ReturnType<typeof fixtureRelay>>;
const allowed = new Set<string>();
test.beforeAll(async () => {
	relay = await fixtureRelay(19027, allowed);
});
test.afterAll(async () => {
	if (relay) await relay.close();
});
function id(value: unknown): number {
	if (
		!value ||
		typeof value !== 'object' ||
		!('id' in value) ||
		typeof value.id !== 'number' ||
		!Number.isSafeInteger(value.id) ||
		value.id <= 0
	)
		throw new Error('Disposable record identity was not confirmed');
	return value.id;
}
test('native timed campaign uses actual opt-in, stays paused across its deadline and resumes through the SMTP worker', async ({
	page,
	baseURL
}) => {
	test.setTimeout(90000);
	expect(baseURL).toBe('http://127.0.0.1:5274');
	const stamp = Date.now(),
		email = `native-worker-browser-${stamp}@example.test`,
		name = `Native worker browser ${stamp}`,
		subject = `Native worker ${stamp}`;
	allowed.add(email);
	expect(
		(
			await page.request.post('/api/auth/login', {
				data: { email: 'admin@admin-completion.test', password: 'OnlyForDisposableTest123!' }
			})
		).status()
	).toBe(200);
	const settingResponse = await page.request.get('/api/admin/email/settings');
	expect(settingResponse.status()).toBe(200);
	const raw = await settingResponse.json(),
		settings = raw.data ?? raw.settings ?? raw;
	expect(settings.host).toBe('127.0.0.1');
	expect(Number(settings.port)).toBe(19027);
	expect(settings.from_address).toBe('smtp-browser@example.test');
	expect(settings.has_password).toBe(true);
	expect(settings.password).toBeUndefined();
	const checking = await page.request.post('/api/admin/email/settings/test', {
		data: {
			provider: 'smtp',
			host: settings.host,
			port: settings.port,
			username: settings.username,
			encryption: settings.encryption,
			from_address: settings.from_address,
			from_name: settings.from_name
		}
	});
	expect(checking.status(), await checking.text()).toBe(200);
	expect((await checking.json()).success).toBe(true);
	expect(relay.messages).toHaveLength(0);
	const errors: string[] = [],
		blocked = new Set<string>();
	page.on('pageerror', (e) => errors.push(e.message));
	page.on('console', (m) => {
		if (
			m.type() === 'error' &&
			!(blocked.has(m.location().url) && m.text() === 'Failed to load resource: net::ERR_FAILED')
		)
			errors.push(m.text());
	});
	page.on('response', (r) => {
		if (new URL(r.url()).pathname.startsWith('/api/') && r.status() >= 400)
			errors.push(`${r.status()} ${r.url()}`);
	});
	page.on('requestfailed', (r) => {
		if (
			new URL(r.url()).pathname.startsWith('/api/') &&
			!r.failure()?.errorText.includes('ERR_ABORTED')
		)
			errors.push(r.url());
	});
	await page.route('**/*', (route) => {
		if (['127.0.0.1', 'localhost'].includes(new URL(route.request().url()).hostname))
			return route.continue();
		blocked.add(route.request().url());
		return route.abort();
	});
	const user = await page.request.post('/api/admin/users', {
		data: { name, email, password: 'OnlyForDisposableTest123!', role: 'user' }
	});
	expect(user.status()).toBe(200);
	const userId = id(await user.json());
	const subscribing = await page.request.post('/api/newsletter/subscribe', {
		data: { name, email, gdpr_consent: true }
	});
	expect(subscribing.status()).toBe(200);
	await expect.poll(() => relay.messages.length, { timeout: 10000 }).toBeGreaterThan(0);
	const confirmation = mimeText(relay.messages.map((message) => message.raw).join('\r\n'));
	const links = await page.evaluate((html) => {
		const doc = new DOMParser().parseFromString(html, 'text/html');
		return Array.from(doc.querySelectorAll('a[href]'), (anchor) => anchor.getAttribute('href'));
	}, confirmation);
	const link = links.find(
		(value) => typeof value === 'string' && value.includes('/newsletter/confirm?')
	);
	if (typeof link !== 'string')
		throw new Error('The actual SMTP confirmation link was not received');
	const url = new URL(link);
	expect(url.origin).toBe(baseURL);
	const token = url.searchParams.get('token');
	expect(token).toBeTruthy();
	const confirmed = await page.request.get(
		`/api/newsletter/confirm?token=${encodeURIComponent(token ?? '')}`
	);
	expect(confirmed.status()).toBe(200);
	const tag = await page.request.post('/api/admin/members/tags', {
		data: { name: `Native worker tag ${stamp}`, color: '#3b82f6' }
	});
	expect(tag.status()).toBe(200);
	const tagId = id(await tag.json());
	expect(
		(
			await page.request.post('/api/admin/members/tags/assign', {
				data: { user_id: userId, tag_id: tagId }
			})
		).status()
	).toBe(200);
	const segment = await page.request.post('/api/admin/members/segments', {
		data: {
			name: `Native worker audience ${stamp}`,
			rules: [{ field: 'tag', operator: 'has', value: tagId }]
		}
	});
	expect(segment.status()).toBe(200);
	const segmentId = id(await segment.json());
	const template = await page.request.post('/api/admin/email/templates', {
		data: {
			name: `Native worker template ${stamp}`,
			slug: `native-worker-template-${stamp}`,
			subject: 'Native worker source',
			body_html: '<p>Hello {{name}}. Native scheduled fixture.</p>',
			body_text: 'Hello {{name}}. Native scheduled fixture.'
		}
	});
	expect(template.status()).toBe(200);
	const templateId = id((await template.json()).data);
	await page.goto('/admin/email/campaigns', { waitUntil: 'networkidle' });
	await page.getByRole('button', { name: 'New Campaign', exact: true }).click();
	await page.getByLabel('Campaign Name', { exact: true }).fill(name);
	await page.getByLabel('Subject Line', { exact: true }).fill(subject);
	await page.getByLabel('Target Segment', { exact: true }).selectOption(String(segmentId));
	await page.getByLabel('Email Template', { exact: true }).selectOption(String(templateId));
	const due = Math.ceil((Date.now() + 20000) / 1000) * 1000;
	const local = await page.evaluate((timestamp) => {
		const d = new Date(timestamp),
			pad = (n: number) => String(n).padStart(2, '0');
		return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}T${pad(d.getHours())}:${pad(d.getMinutes())}:${pad(d.getSeconds())}`;
	}, due);
	await page.getByLabel('Schedule (optional)', { exact: true }).fill(local);
	const creating = page.waitForResponse(
		(r) =>
			new URL(r.url()).pathname === '/api/admin/email/campaigns' && r.request().method() === 'POST'
	);
	const scheduling = page.waitForResponse(
		(r) =>
			/\/api\/admin\/email\/campaigns\/\d+\/schedule$/.test(new URL(r.url()).pathname) &&
			r.request().method() === 'POST'
	);
	await page
		.getByRole('dialog')
		.getByRole('button', { name: 'Schedule Campaign', exact: true })
		.click();
	const created = await creating;
	expect(created.status()).toBe(200);
	const campaign = id((await created.json()).data);
	const scheduled = await scheduling;
	expect(scheduled.status()).toBe(200);
	const saved = (await scheduled.json()).data;
	expect(saved.total_recipients).toBe(1);
	expect(new Date(saved.scheduled_at).getTime()).toBe(due);
	let card = page
		.locator('.campaign-card')
		.filter({ has: page.getByRole('heading', { name, exact: true }) });
	await expect(card).toContainText('scheduled');
	await card.getByRole('button', { name: 'Pause future sends', exact: true }).click();
	await page.getByRole('button', { name: /^All \(/ }).click();
	await expect(card).toContainText('paused');
	await page.reload({ waitUntil: 'networkidle' });
	await expect(card).toContainText('paused');
	await expect
		.poll(() => Date.now(), { timeout: 30000, intervals: [250] })
		.toBeGreaterThan(due + 1000);
	expect(relay.messages.filter((message) => mimeSubject(message.raw) === subject)).toHaveLength(0);
	await card.getByRole('button', { name: 'Resume', exact: true }).click();
	await expect
		.poll(() => relay.messages.filter((message) => mimeSubject(message.raw) === subject).length, {
			timeout: 20000,
			message: 'Actual SMTP subjects: ' + relay.messages.map((m) => mimeSubject(m.raw)).join(', ')
		})
		.toBe(1);
	await page.getByRole('button', { name: 'Refresh', exact: true }).click();
	await expect(card.locator('.status-badge')).toHaveText('sent', { timeout: 20000 });
	await page.reload({ waitUntil: 'networkidle' });
	await expect(card.locator('.status-badge')).toHaveText('sent');
	const receipt = await page.request.get(`/api/admin/email/campaigns/${campaign}`);
	expect(receipt.status()).toBe(200);
	expect((await receipt.json()).data.sent_count).toBe(1);
	const actual = relay.messages.find((message) => mimeSubject(message.raw) === subject);
	if (!actual) throw new Error('No actual native campaign SMTP message received');
	expect(actual.receivedAt).toBeGreaterThanOrEqual(due);
	expect(mimeText(actual.raw)).toContain(name);
	expect(errors).toEqual([]);
	// Retain this completed, uniquely named disposable fixture for inspection.
	// It contains only the local example.test address and no queued submissions.
});
