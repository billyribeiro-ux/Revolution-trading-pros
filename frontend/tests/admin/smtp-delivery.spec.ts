import { expect, test } from '@playwright/test';
import { createServer, type Server, type Socket } from 'node:net';

// Only the disposable source-verification stack; seed smtp_campaign_browser.sql.
let relay: Server;
const sockets = new Set<Socket>();
const messages: string[] = [];
test.beforeAll(async () => {
	relay = createServer((socket) => {
		sockets.add(socket);
		socket.on('close', () => sockets.delete(socket));
		socket.on('error', () => socket.destroy());
		let buffer = '',
			content = '',
			authenticated = false,
			data = false;
		socket.write('220 fixture.example.test ESMTP\r\n');
		socket.on('data', (chunk) => {
			buffer += chunk.toString();
			if (buffer.length + content.length > 8_000_000) return socket.destroy();
			while (buffer.includes('\r\n')) {
				const position = buffer.indexOf('\r\n');
				const line = buffer.slice(0, position);
				buffer = buffer.slice(position + 2);
				if (data) {
					if (line === '.') {
						messages.push(content);
						data = false;
						socket.end('250 2.0.0 fixture accepted\r\n');
					} else content += `${line}\r\n`;
				} else if (line.startsWith('EHLO '))
					socket.write('250-fixture.example.test\r\n250-AUTH PLAIN\r\n250 SMTPUTF8\r\n');
				else if (line.startsWith('AUTH PLAIN ')) {
					authenticated =
						Buffer.from(line.slice(11), 'base64').toString() === '\0fixture-user\0fixture-password';
					socket.write(authenticated ? '235 2.7.0 Accepted\r\n' : '535 5.7.8 Rejected\r\n');
				} else if (!authenticated && line !== 'QUIT') socket.end('530 Authentication required\r\n');
				else if (line === 'NOOP' || line.startsWith('MAIL FROM:')) socket.write('250 OK\r\n');
				else if (line.startsWith('RCPT TO:'))
					socket.write(
						line.includes('<smtp-campaign-browser@example.test>')
							? '250 OK\r\n'
							: '550 Fixture recipient only\r\n'
					);
				else if (line === 'DATA') {
					data = true;
					socket.write('354 Continue\r\n');
				} else if (line === 'QUIT') socket.end('221 Bye\r\n');
				else socket.end('500 Unsupported fixture command\r\n');
			}
		});
	});
	await new Promise<void>((resolve, reject) => {
		relay.once('error', reject);
		relay.listen(19027, '127.0.0.1', resolve);
	});
});
test.afterAll(async () => {
	for (const socket of sockets) socket.destroy();
	if (relay)
		await new Promise<void>((resolve, reject) =>
			relay.close((error) => (error ? reject(error) : resolve()))
		);
});

test('verified native SMTP dispatches the campaign worker and persists actual tracking in the report', async ({
	page,
	baseURL
}) => {
	expect(baseURL).toBe('http://127.0.0.1:5274');
	expect(
		(
			await page.request.post('/api/auth/login', {
				data: { email: 'admin@admin-completion.test', password: 'OnlyForDisposableTest123!' }
			})
		).status()
	).toBe(200);
	const diagnostics = await page.request.get('/api/admin/email/status');
	expect((await diagnostics.json()).postmark_token_set).toBe(false);
	const failures: string[] = [];
	const blockedExternal = new Set<string>();
	page.on('pageerror', (error) => failures.push(error.message));
	page.on('console', (message) => {
		if (message.type() === 'error') {
			const source = message.location().url;
			// Account only for requests this fixture deliberately aborted. Every
			// application error and unexpected resource failure still fails the test.
			if (
				blockedExternal.has(source) &&
				message.text() === 'Failed to load resource: net::ERR_FAILED'
			)
				return;
			failures.push(`${message.text()} ${source}`);
		}
	});
	page.on('response', (response) => {
		if (new URL(response.url()).pathname.startsWith('/api/') && response.status() >= 400)
			failures.push(`${response.status()} ${response.url()}`);
	});
	page.on('requestfailed', (request) => {
		if (
			new URL(request.url()).pathname.startsWith('/api/') &&
			!request.failure()?.errorText.includes('ERR_ABORTED')
		)
			failures.push(request.url());
	});
	await page.route('**/*', (route) => {
		if (['localhost', '127.0.0.1'].includes(new URL(route.request().url()).hostname))
			return route.continue();
		blockedExternal.add(route.request().url());
		return route.abort();
	});
	await page.goto('/admin/email/smtp', { waitUntil: 'networkidle' });
	for (const [label, value] of [
		['SMTP Host', '127.0.0.1'],
		['Port', '19027'],
		['Username', 'fixture-user'],
		['From Email', 'smtp-browser@example.test'],
		['From Name', 'SMTP browser delivery fixture']
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
	expect(messages).toHaveLength(0);
	const created = await page.request.post('/api/admin/crm/campaigns', {
		data: {
			title: `SMTP browser ${Date.now()}`,
			email_subject: 'SMTP browser actual campaign',
			email_body:
				'<p><a href="https://target.example.test/offer?a=1&amp;b=2#details">Offer</a></p>',
			subscriber_settings: {
				advanced_filters: [
					{ field: 'email', operator: 'equals', value: 'smtp-campaign-browser@example.test' }
				]
			}
		}
	});
	expect(created.status()).toBe(200);
	const campaign = (await created.json()).id;
	await page.goto(`/admin/crm/campaigns/${campaign}`, { waitUntil: 'networkidle' });
	await expect(page.locator('.campaign-audience tbody tr')).toHaveCount(1);
	await page.getByRole('button', { name: 'Send Now', exact: true }).click();
	const sent = page.waitForResponse(
		(response) => new URL(response.url()).pathname === `/api/admin/crm/campaigns/${campaign}/send`
	);
	await page.getByRole('dialog').getByRole('button', { name: 'Send', exact: true }).click();
	expect((await sent).ok()).toBe(true);
	await expect.poll(() => messages.length, { timeout: 20000 }).toBe(1);
	await expect(page.locator('.campaign-report header strong')).toHaveText('sent', {
		timeout: 20000
	});
	await page.reload({ waitUntil: 'networkidle' });
	const metric = (label: string) =>
		page
			.locator('.campaign-report .metrics > div')
			.filter({ has: page.getByText(label, { exact: true }) })
			.locator('dd');
	await expect(metric('Provider accepted')).toHaveText('1');
	await expect(metric('Recorded opens')).toHaveText('0');
	await expect(metric('Recorded clicks')).toHaveText('0');
	const raw = messages[0];
	if (!raw) throw new Error('No SMTP message captured');
	const html = raw
		.split('Content-Transfer-Encoding: base64\r\n\r\n')
		.slice(1)
		.map((part) =>
			Buffer.from(
				(part.split('\r\n\r\n')[0] ?? '')
					.split('\r\n')
					.filter((line) => line && !line.startsWith('--'))
					.join(''),
				'base64'
			).toString()
		)
		.find((part) => part.includes('/tracking/open/'));
	if (!html) throw new Error('SMTP message has no tracking HTML');
	const open = html.match(/src="(http[^" ]+\/tracking\/open\/[^" ]+)"/)?.[1];
	const click = html.match(/href="(http[^" ]+\/tracking\/click\/[^" ]+)"/)?.[1];
	if (!open || !click) throw new Error('Missing actual SMTP capability URLs');
	expect((await page.request.head(open)).status()).toBe(200);
	await page.getByRole('button', { name: 'Refresh', exact: true }).click();
	await expect(metric('Recorded opens')).toHaveText('0');
	const image = await page.request.get(open);
	expect(image.status()).toBe(200);
	expect(image.headers()['referrer-policy']).toBe('no-referrer');
	expect(image.headers()['cache-control']).toContain('no-store');
	const redirect = await page.request.get(click, { maxRedirects: 0 });
	expect(redirect.status()).toBe(302);
	expect(redirect.headers()['location']).toBe('https://target.example.test/offer?a=1&b=2#details');
	expect(redirect.headers()['referrer-policy']).toBe('no-referrer');
	await page.getByRole('button', { name: 'Refresh', exact: true }).click();
	await expect(metric('Recorded opens')).toHaveText('1');
	await expect(metric('Recorded clicks')).toHaveText('1');
	await page.reload({ waitUntil: 'networkidle' });
	await expect(metric('Recorded opens')).toHaveText('1');
	await expect(metric('Recorded clicks')).toHaveText('1');
	const receipts = await page.request.get(`/api/admin/crm/campaigns/${campaign}/deliveries`);
	const receipt = (await receipts.json()).data[0];
	expect(receipt.provider).toBe('smtp');
	expect(receipt.status).toBe('accepted');
	expect(receipt.delivered_at).toBeNull();
	expect(raw).toContain(`Message-ID: <${receipt.provider_message_id}>`);
	expect(messages).toHaveLength(1);
	expect(failures).toEqual([]);
});
