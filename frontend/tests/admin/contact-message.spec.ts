import { expect, test } from '@playwright/test';
import { fixtureRelay, mimeSubject, mimeText } from './fixtures/smtp-relay.js';
function record(value: unknown): Record<string, unknown> {
	if (!value || typeof value !== 'object' || Array.isArray(value))
		throw new Error('Invalid fixture response');
	return value as Record<string, unknown>;
}
function id(value: unknown): string {
	const result = String(record(value).id);
	if (!/^[1-9]\d*$/.test(result)) throw new Error('Unconfirmed fixture identity');
	return result;
}

test('contact message uses the selected native template, records actual SMTP outcomes and replays frozen UUIDs without another send', async ({
	page,
	baseURL
}) => {
	test.setTimeout(120000);
	expect(baseURL).toBe('http://127.0.0.1:5274');
	expect(
		(
			await page.request.post('/api/auth/login', {
				data: { email: 'admin@admin-completion.test', password: 'OnlyForDisposableTest123!' }
			})
		).status()
	).toBe(200);
	const stamp = Date.now(),
		email = `contact-personal-browser-${stamp}@example.test`,
		allowed = new Set([email]),
		relay = await fixtureRelay(19027, allowed),
		errors: string[] = [],
		blocked = new Set<string>(),
		expectedFailures = new Set<string>();
	page.on('pageerror', (cause) => errors.push(cause.message));
	page.on('console', (message) => {
		if (message.type() !== 'error') return;
		const source = message.location().url;
		if (blocked.has(source) && message.text() === 'Failed to load resource: net::ERR_FAILED')
			return;
		if (
			source &&
			expectedFailures.has(new URL(source).pathname) &&
			/^Failed to load resource: the server responded with a status of (409|502)/.test(
				message.text()
			)
		)
			return;
		errors.push(`${message.text()} ${source}`);
	});
	page.on('requestfailed', (request) => {
		if (!blocked.has(request.url()) && !request.failure()?.errorText.includes('ERR_ABORTED'))
			errors.push(`${request.failure()?.errorText} ${request.url()}`);
	});
	await page.route('**/*', (route) => {
		const url = new URL(route.request().url());
		if (
			['127.0.0.1', 'localhost'].includes(url.hostname) ||
			!['http:', 'https:'].includes(url.protocol)
		)
			return route.continue();
		blocked.add(route.request().url());
		return route.abort();
	});
	let contact: string | undefined, template: string | undefined;
	try {
		expect(
			record(await (await page.request.get('/api/admin/email/status')).json()).postmark_token_set
		).toBe(false);
		await page.goto('/admin/email/smtp', { waitUntil: 'networkidle' });
		for (const [label, value] of [
			['SMTP Host', '127.0.0.1'],
			['Port', '19027'],
			['Username', 'fixture-user'],
			['From Email', 'contact-personal-sender@example.test'],
			['From Name', 'Contact message fixture']
		] as const)
			await page.getByLabel(label, { exact: true }).fill(value);
		await page.getByLabel('Encryption', { exact: true }).selectOption('null');
		await page.getByLabel(/Password \/ App Password/).fill('fixture-password');
		await page.getByRole('button', { name: 'Save Settings', exact: true }).click();
		await expect(
			page.getByText('SMTP settings saved. Test the connection to verify relay access.', {
				exact: true
			})
		).toBeVisible();
		await page.getByRole('button', { name: 'Test Connection', exact: true }).click();
		await expect(
			page.getByText('Authenticated SMTP connection and NOOP passed; no email was sent', {
				exact: true
			})
		).toBeVisible();
		expect(relay.messages).toHaveLength(0);
		const created = await page.request.post('/api/admin/crm/contacts', {
			data: {
				email,
				first_name: 'Jordan',
				last_name: 'Personal',
				status: 'lead',
				lifecycle_stage: 'lead'
			}
		});
		expect(created.status(), await created.text()).toBe(200);
		contact = id(await created.json());
		const name = `Contact native template ${stamp}`,
			sourceSubject = `Personal {{contact.first_name}} ${stamp}`,
			html = `<p>Frozen contact creative ${stamp} for {{email}}.</p>`;
		const saved = await page.request.post('/api/admin/email/templates', {
			data: {
				name,
				slug: `contact-personal-${stamp}`,
				subject: sourceSubject,
				body_html: html,
				body_text: `Template alternative ${stamp}`
			}
		});
		expect(saved.status(), await saved.text()).toBe(200);
		template = id(record(await saved.json()).data);
		await page.goto(`/admin/crm/contacts/${contact}`, { waitUntil: 'networkidle' });
		await page.getByRole('button', { name: 'Send Email', exact: true }).click();
		const dialog = page.getByRole('dialog', { name: /^(Send Email|Retry same message)$/ });
		await expect(dialog.getByRole('heading', { name: 'Send Email', exact: true })).toBeVisible();
		await dialog.getByLabel('Search templates', { exact: true }).fill(name);
		await dialog.getByRole('button', { name: 'Search templates', exact: true }).click();
		await expect(dialog.getByRole('option', { name, exact: true })).toHaveCount(1);
		await dialog.getByLabel('Email Template', { exact: false }).selectOption(template);
		await expect(dialog.getByLabel('Subject Line', { exact: false })).toHaveValue(sourceSubject);
		await expect(dialog.getByLabel('Email Body', { exact: false })).toHaveValue(html);
		const endpoint = `/api/admin/crm/contacts/${contact}/send-email`,
			responseReady = page.waitForResponse(
				(response) =>
					new URL(response.url()).pathname === endpoint && response.request().method() === 'POST'
			);
		await dialog.getByRole('button', { name: 'Send Email', exact: true }).click();
		const response = await responseReady;
		expect(response.status(), await response.text()).toBe(200);
		const submitted = record(response.request().postDataJSON()),
			first = record(await response.json()),
			accepted = record(first.data);
		expect(submitted).toMatchObject({
			expected_email: email,
			subject: sourceSubject,
			body_html: html,
			template_id: Number(template)
		});
		expect(String(submitted.request_id)).toMatch(/^[0-9a-f-]{36}$/);
		expect(first.success).toBe(true);
		expect(accepted).toMatchObject({
			contact_id: contact,
			request_id: submitted.request_id,
			source_subject: sourceSubject,
			template_id: template,
			metrics_scope: 'contact_message'
		});
		expect(accepted.source_hash).toMatch(/^[0-9a-f]{64}$/);
		await expect(
			dialog.getByRole('heading', { name: 'Retry same message', exact: true })
		).toBeVisible();
		await expect(dialog.getByLabel('Message receipt')).toContainText('accepted');
		await expect(dialog.getByLabel('Subject Line', { exact: false })).toBeDisabled();
		await expect(
			dialog.getByRole('button', { name: 'Retry same message', exact: true })
		).toBeDisabled();
		expect(relay.messages).toHaveLength(1);
		const message = relay.messages[0];
		if (!message) throw new Error('No actual SMTP message');
		expect(mimeSubject(message.raw)).toBe(`Personal Jordan ${stamp}`);
		expect(mimeSubject(message.raw)).not.toContain('[Test]');
		expect(mimeText(message.raw)).toContain(`Frozen contact creative ${stamp} for ${email}.`);
		const connections = relay.connections;
		expect(
			(
				await page.request.put(`/api/admin/email/templates/${template}`, {
					data: {
						subject: 'Changed after acceptance',
						body_html: '<p>Changed source must not resend</p>'
					}
				})
			).status()
		).toBe(200);
		let replay = await page.request.post(endpoint, { data: submitted });
		expect(replay.status()).toBe(200);
		expect(await replay.json()).toEqual(first);
		expect(relay.connections).toBe(connections);
		expect((await page.request.delete(`/api/admin/email/templates/${template}`)).status()).toBe(
			200
		);
		template = undefined;
		replay = await page.request.post(endpoint, { data: submitted });
		expect(replay.status()).toBe(200);
		expect(await replay.json()).toEqual(first);
		expect(relay.connections).toBe(connections);
		expect(relay.messages).toHaveLength(1);
		expect(
			(
				await page.request.post(endpoint, {
					data: { ...submitted, body_html: '<p>Different intent under the same UUID</p>' }
				})
			).status()
		).toBe(409);
		expect(relay.connections).toBe(connections);
		await dialog.getByRole('button', { name: 'Refresh message outcome', exact: true }).click();
		await expect(dialog.getByLabel('Message receipt')).toContainText('Recorded outcome: accepted');
		expect(relay.connections).toBe(connections);
		allowed.delete(email);
		await dialog.getByRole('button', { name: 'Compose new message', exact: true }).click();
		await expect(dialog.getByRole('heading', { name: 'Send Email', exact: true })).toBeVisible();
		await dialog.getByLabel('Subject Line', { exact: false }).fill(`Rejected personal ${stamp}`);
		await dialog
			.getByLabel('Email Body', { exact: false })
			.fill('<p>Relay rejects this recipient before DATA</p>');
		await expect(dialog.getByRole('button', { name: 'Send Email', exact: true })).toBeInViewport({
			ratio: 1
		});
		expectedFailures.add(endpoint);
		const rejecting = page.waitForResponse(
			(item) => new URL(item.url()).pathname === endpoint && item.request().method() === 'POST'
		);
		await dialog.getByRole('button', { name: 'Send Email', exact: true }).click();
		const rejection = await rejecting;
		expect(rejection.status()).toBe(502);
		const rejected = record(record(await rejection.json()).data);
		expect(rejected.request_id).not.toBe(submitted.request_id);
		await expect(
			dialog.getByRole('heading', { name: 'Retry same message', exact: true })
		).toBeVisible();
		await expect(dialog.getByLabel('Message receipt')).toContainText('rejected');
		expect(relay.messages).toHaveLength(1);
		const rejectedConnections = relay.connections;
		const retrying = page.waitForResponse(
			(item) => new URL(item.url()).pathname === endpoint && item.request().method() === 'POST'
		);
		await dialog.getByRole('button', { name: 'Retry same message', exact: true }).click();
		expect((await retrying).status()).toBe(502);
		await expect(
			dialog.getByRole('button', { name: 'Retry same message', exact: true })
		).toBeEnabled();
		expect(relay.connections).toBe(rejectedConnections);
		expect(relay.messages).toHaveLength(1);
		await dialog.getByRole('button', { name: 'Close modal', exact: true }).click();
		await page.reload({ waitUntil: 'networkidle' });
		await page.getByRole('button', { name: 'Emails', exact: true }).click();
		const history = page.locator('.emails-section');
		await expect(history).toContainText(`Personal Jordan ${stamp}`);
		await expect(history).toContainText(`Rejected personal ${stamp}`);
		await expect(history).toContainText('accepted');
		await expect(history).toContainText('rejected');
		expect(relay.connections).toBe(rejectedConnections);
		expect(errors).toEqual([]);
	} finally {
		try {
			if (contact)
				expect((await page.request.delete(`/api/admin/crm/contacts/${contact}`)).status()).toBe(
					200
				);
			if (template)
				expect((await page.request.delete(`/api/admin/email/templates/${template}`)).status()).toBe(
					200
				);
		} finally {
			await relay.close();
		}
	}
});
