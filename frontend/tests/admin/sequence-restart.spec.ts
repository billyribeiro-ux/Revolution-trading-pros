import { expect, test, type APIRequestContext } from '@playwright/test';
import { spawn, type ChildProcess } from 'node:child_process';
import { createConnection } from 'node:net';
import { lstat, realpath } from 'node:fs/promises';
import { openSync, closeSync, writeSync } from 'node:fs';
import { fixtureRelay, mimeSubject, mimeText } from './fixtures/smtp-relay.js';
const root = '/api/admin/crm/sequences',
	allowedLauncher = '/private/tmp/revolution-resume-run.py';
function row(value: unknown): Record<string, unknown> {
	if (!value || typeof value !== 'object' || Array.isArray(value))
		throw new Error('Expected actual persisted restart fixture');
	return value as Record<string, unknown>;
}
function key(value: unknown): string {
	const result =
		typeof value === 'number' && Number.isSafeInteger(value) && value > 0
			? value.toString()
			: typeof value === 'string'
				? value
				: '';
	if (!/^[1-9]\d*$/.test(result)) throw new Error('Restart fixture identity is missing');
	return result;
}
async function listening() {
	return new Promise<boolean>((resolve, reject) => {
		const socket = createConnection({ host: '127.0.0.1', port: 8281 });
		socket.once('connect', () => {
			socket.destroy();
			resolve(true);
		});
		socket.once('error', (cause: Error) => {
			socket.destroy();
			if ('code' in cause && cause.code === 'ECONNREFUSED') resolve(false);
			else reject(cause);
		});
		socket.setTimeout(1000, () => {
			socket.destroy();
			reject(new Error('Cannot establish exclusive ownership of API8281'));
		});
	});
}
async function tracker(client: APIRequestContext, id: string) {
	const response = await client.get(`${root}/${id}/subscribers`);
	expect(response.status()).toBe(200);
	const data = row(await response.json()).data;
	if (!Array.isArray(data) || data.length !== 1)
		throw new Error('Expected the one actual saved enrollment');
	return row(data[0]);
}

test('owned API process restarts preserve delayed sequence progress, real SMTP receipt and completed no-resend state', async ({
	page,
	baseURL
}, testInfo) => {
	test.skip(
		process.env.ALLOW_OWNED_SEQUENCE_RESTART !== '1',
		'Explicit opt-in required; this test starts and stops only its own API child, after root stops8281.'
	);
	test.setTimeout(210000);
	expect(baseURL).toBe('http://127.0.0.1:5274');
	expect(
		process.env.OWNED_SEQUENCE_API_LAUNCHER,
		'Supply the reviewed owned fixture launcher explicitly'
	).toBe(allowedLauncher);
	expect((await lstat(allowedLauncher)).isFile()).toBe(true);
	expect(await realpath(allowedLauncher)).toBe(allowedLauncher);
	expect(
		await listening(),
		'Refusing to stop or replace an API process this test did not start'
	).toBe(false);
	const logFd = openSync(testInfo.outputPath('owned-api-processes.log'), 'a'),
		pids: number[] = [],
		recipient = 'sequence-browser@example.test';
	let child: ChildProcess | undefined,
		startError: Error | undefined,
		sequence: string | undefined,
		relay: Awaited<ReturnType<typeof fixtureRelay>> | undefined;
	const running = () => !!child && child.exitCode === null && child.signalCode === null;
	async function start() {
		expect(await listening(), 'API8281 must be free before each owned spawn').toBe(false);
		startError = undefined;
		child = spawn('python3', [allowedLauncher, 'api'], {
			stdio: ['ignore', logFd, logFd],
			shell: false
		});
		child.once('error', (cause) => {
			startError = cause;
		});
		if (!child.pid) throw new Error('Owned API did not receive a process identity');
		pids.push(child.pid);
		writeSync(
			logFd,
			`\nOwned generation ${pids.length}, PID ${child.pid}, started ${new Date().toISOString()}\n`
		);
		await expect
			.poll(
				async () => {
					if (startError) throw startError;
					if (!running())
						throw new Error(`Owned API generation exited: ${child?.exitCode ?? child?.signalCode}`);
					try {
						return (
							(
								await page.request.get('http://127.0.0.1:8281/health', { timeout: 1000 })
							).status() === 200
						);
					} catch {
						return false;
					}
				},
				{ timeout: 30000, intervals: [250, 500, 1000] }
			)
			.toBe(true);
		expect(running()).toBe(true);
	}
	async function stop() {
		const owned = child;
		child = undefined;
		if (!owned || !owned.pid) return;
		if (owned.exitCode === null && owned.signalCode === null) {
			await new Promise<void>((resolve, reject) => {
				let force: ReturnType<typeof setTimeout> | undefined;
				const deadline = setTimeout(() => {
					if (owned.exitCode === null && owned.signalCode === null) owned.kill('SIGKILL');
					force = setTimeout(() => reject(new Error('Owned API child did not stop')), 5000);
				}, 10000);
				owned.once('exit', () => {
					clearTimeout(deadline);
					if (force) clearTimeout(force);
					resolve();
				});
				owned.kill('SIGTERM');
			});
		}
		expect(
			await listening(),
			'A different process claimed8281 after the owned child stopped; it will not be killed'
		).toBe(false);
	}
	async function stats() {
		const response = await page.request.get(`${root}/${sequence}/stats`);
		expect(response.status()).toBe(200);
		return row(await response.json());
	}
	try {
		relay = await fixtureRelay(19027, new Set([recipient]));
		const smtp = relay;
		await start();
		expect(
			(
				await page.request.post('/api/auth/login', {
					data: { email: 'admin@admin-completion.test', password: 'OnlyForDisposableTest123!' }
				})
			).status()
		).toBe(200);
		expect(
			row(await (await page.request.get('/api/admin/email/status')).json()).postmark_token_set
		).toBe(false);
		const contacts = row(
			await (
				await page.request.get(`/api/admin/crm/contacts?search=${encodeURIComponent(recipient)}`)
			).json()
		).data;
		if (!Array.isArray(contacts)) throw new Error('Seed the owned confirmed restart contact');
		const matches = contacts.map(row).filter((contact) => contact.email === recipient);
		expect(matches).toHaveLength(1);
		const person = key(matches[0]?.id);
		const errors: string[] = [],
			blocked = new Set<string>();
		page.on('pageerror', (cause) => errors.push(cause.message));
		page.on('console', (message) => {
			if (
				message.type() === 'error' &&
				!(
					blocked.has(message.location().url) &&
					message.text() === 'Failed to load resource: net::ERR_FAILED'
				)
			)
				errors.push(message.text());
		});
		await page.route('**/*', (route) => {
			if (['127.0.0.1', 'localhost'].includes(new URL(route.request().url()).hostname))
				return route.continue();
			blocked.add(route.request().url());
			return route.abort();
		});
		await page.goto('/admin/email/smtp', { waitUntil: 'networkidle' });
		for (const [label, value] of [
			['SMTP Host', '127.0.0.1'],
			['Port', '19027'],
			['Username', 'fixture-user'],
			['From Email', 'sequence-restart-sender@example.test'],
			['From Name', 'Owned sequence restart fixture']
		])
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
		expect(smtp.messages).toHaveLength(0);
		const title = `Owned restart sequence ${Date.now()}`;
		await page.goto('/admin/crm/sequences/new', { waitUntil: 'networkidle' });
		await page.getByLabel('Sequence title', { exact: true }).fill(title);
		await page.getByRole('button', { name: 'Add email', exact: true }).click();
		const mail = page.locator('[data-sequence-step="1"]');
		await mail.getByLabel('Email title', { exact: true }).fill('Durable delayed restart email');
		await mail.getByLabel('Subject', { exact: true }).fill('Restart receipt for {{email}}');
		await mail
			.getByLabel('Email HTML', { exact: true })
			.fill('<p>Actual durable restart body for {{email}}</p>');
		await mail
			.getByLabel('Plain text alternative', { exact: true })
			.fill('Actual durable restart alternative');
		await mail
			.getByLabel('Delay after the previous accepted email (first email: after enrollment)', {
				exact: true
			})
			.fill('1');
		await mail.getByLabel('Delay unit for email 1', { exact: true }).selectOption('minutes');
		const creating = page.waitForResponse(
			(response) =>
				new URL(response.url()).pathname === root && response.request().method() === 'POST'
		);
		await page.getByRole('button', { name: 'Create draft', exact: true }).click();
		const created = await creating;
		expect(created.status()).toBe(200);
		sequence = key(row(await created.json()).id);
		await expect(page).toHaveURL(new RegExp(`/admin/crm/sequences/${sequence}/edit$`));
		await page.getByRole('button', { name: 'Activate saved sequence', exact: true }).click();
		await expect(page.getByText('Saved sequence activated.', { exact: true })).toBeVisible();
		await page
			.locator('.sequence-form')
			.getByRole('link', { name: 'Subscribers', exact: true })
			.click();
		await page
			.getByLabel('Contact IDs (separated by commas or spaces)', { exact: true })
			.fill(person);
		await page.getByRole('button', { name: 'Enroll contacts', exact: true }).click();
		await expect(
			page
				.locator('.sequence-detail')
				.getByRole('status')
				.filter({ hasText: '1 contacts enrolled' })
		).toBeVisible();
		const before = await tracker(page.request, sequence),
			due = Date.parse(String(before.next_execution_at));
		expect(before.status).toBe('active');
		expect(Number.isFinite(due)).toBe(true);
		expect(due - Date.parse(String(before.started_at))).toBeGreaterThanOrEqual(60000);
		const enrolledAt = Date.now();
		await expect
			.poll(
				() => {
					expect(smtp.messages).toHaveLength(0);
					return Date.now() - enrolledAt;
				},
				{ timeout: 20000, intervals: [250, 500] }
			)
			.toBeGreaterThanOrEqual(15000);
		expect(smtp.messages).toHaveLength(0);
		expect(Date.now()).toBeLessThan(due);
		await stop();
		expect(smtp.messages).toHaveLength(0);
		await start();
		await page.reload({ waitUntil: 'networkidle' });
		const resumed = await tracker(page.request, sequence);
		expect(resumed.id).toBe(before.id);
		expect(resumed.revision_id).toBe(before.revision_id);
		expect(resumed.next_execution_at).toBe(before.next_execution_at);
		await expect
			.poll(() => smtp.messages.length, { timeout: 95000, intervals: [250, 500, 1000] })
			.toBe(1);
		const actual = smtp.messages[0];
		if (!actual) throw new Error('No actual SMTP acceptance after restart');
		expect(actual.receivedAt).toBeGreaterThanOrEqual(due);
		expect(mimeSubject(actual.raw)).toBe(`Restart receipt for ${recipient}`);
		expect(mimeText(actual.raw)).toContain(`Actual durable restart body for ${recipient}`);
		expect(mimeText(actual.raw)).toContain('Actual durable restart alternative');
		await expect
			.poll(async () => row(await stats()).completed, { timeout: 20000, intervals: [250, 500] })
			.toBe(1);
		const accepted = await tracker(page.request, sequence),
			recorded = await stats();
		expect(accepted.id).toBe(before.id);
		expect(accepted.emails_sent).toBe(1);
		expect(accepted.status).toBe('completed');
		expect(recorded.total_sent).toBe(1);
		expect(recorded.total_delivered).toBe(0);
		const acceptedConnections = smtp.connections;
		await stop();
		await start();
		expect(smtp.connections).toBe(acceptedConnections);
		await page.reload({ waitUntil: 'networkidle' });
		const after = await tracker(page.request, sequence);
		expect(after.id).toBe(accepted.id);
		expect(after.revision_id).toBe(accepted.revision_id);
		expect(after.completed_at).toBe(accepted.completed_at);
		expect(after.emails_sent).toBe(1);
		const restartedAt = Date.now(),
			connections = acceptedConnections;
		await expect
			.poll(
				() => {
					expect(smtp.messages).toHaveLength(1);
					expect(smtp.connections).toBe(connections);
					return Date.now() - restartedAt;
				},
				{ timeout: 20000, intervals: [250, 500] }
			)
			.toBeGreaterThanOrEqual(12000);
		expect((await stats()).total_sent).toBe(1);
		expect((await tracker(page.request, sequence)).status).toBe('completed');
		expect(new Set(pids).size).toBe(3);
		expect(errors).toEqual([]);
		await testInfo.attach('actual-restart-evidence.json', {
			body: Buffer.from(
				JSON.stringify(
					{
						sequence_id: sequence,
						tracker_id: before.id,
						revision_id: before.revision_id,
						process_ids: pids,
						due_at: before.next_execution_at,
						accepted_at_observed: new Date(actual.receivedAt).toISOString(),
						smtp_message_id: actual.raw.match(/^Message-ID:\s*(.*)$/im)?.[1] ?? null,
						completed_at: accepted.completed_at,
						accepted_count: recorded.total_sent,
						delivered_count: recorded.total_delivered,
						messages_after_final_restart: smtp.messages.length
					},
					null,
					2
				)
			),
			contentType: 'application/json'
		});
	} finally {
		try {
			if (sequence) {
				if (!running()) {
					await stop();
					await start();
				}
				const response = await page.request.get(`${root}/${sequence}`);
				expect(response.status()).toBe(200);
				const saved = row(row(await response.json()).sequence);
				if (saved.status === 'active')
					expect(
						(
							await page.request.put(`${root}/${sequence}`, {
								data: { status: 'paused', expected_version: saved.authoring_version }
							})
						).status()
					).toBe(200);
				await expect.poll(async () => row(await stats()).processing, { timeout: 20000 }).toBe(0);
				expect((await page.request.delete(`${root}/${sequence}`)).status()).toBe(200);
			}
		} finally {
			try {
				await stop();
			} finally {
				try {
					await relay?.close();
				} finally {
					closeSync(logFd);
				}
			}
		}
	}
});
