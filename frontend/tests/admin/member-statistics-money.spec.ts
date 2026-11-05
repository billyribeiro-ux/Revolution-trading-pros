import { expect, request as playwrightRequest, test } from '@playwright/test';
import { execFileSync } from 'node:child_process';
import { createHash, randomUUID } from 'node:crypto';
import { readFileSync, writeFileSync, unlinkSync, existsSync } from 'node:fs';
import { writeFile } from 'node:fs/promises';
import { parseMemberStats, type RecordedOrderValue } from '../../src/lib/api/member-statistics';
const database = 'admin_move_verify_completion115_browser',
	directory = '/private/tmp/revolution-backend-resume-pg';
const binary = '/private/tmp/revolution-api-completion115-verified',
	binarySha = 'bf454cceb1403cc48ed003789dbb4717b572245c16bd3993e73e9cbc3a2bfbdb';
const prefix = `member-money-${randomUUID().replaceAll('-', '').slice(0, 24)}`,
	email = `${prefix}@example.test`,
	password = 'OnlyForDisposableTest123!';
const passfile = `/private/tmp/revolution-member-money-${randomUUID()}.pgpass`;
const childEnv = {
	PATH: process.env.PATH || '/usr/bin:/bin',
	PGPASSFILE: passfile,
	PGPASSWORD: '',
	PGCONNECT_TIMEOUT: '3',
	PGCLIENTENCODING: 'UTF8',
	PGAPPNAME: 'revolution-members-money-proof'
};
let cookies: Awaited<
	ReturnType<Awaited<ReturnType<typeof playwrightRequest.newContext>>['storageState']>
>['cookies'];
let userId: number | undefined;
let source: Record<string, unknown>[] = [];
let baseline: RecordedOrderValue;
let cleanupPath: string | undefined;
let identity: Record<string, unknown>;
function object(value: unknown): Record<string, unknown> {
	if (!value || typeof value !== 'object' || Array.isArray(value))
		throw new Error('Invalid disposable fixture identity');
	return value as Record<string, unknown>;
}
function literal(value: string): string {
	if (value.includes('\0')) throw new Error('Invalid fixture literal');
	return `'${value.replaceAll("'", "''")}'`;
}
function sql(query: string): unknown {
	const raw = execFileSync(
		'/Users/billyribeiro/.local/share/mise/installs/postgres/18.6/.mise-bins/psql',
		[
			'-X',
			'-w',
			'-A',
			'-t',
			'-q',
			'--set=ON_ERROR_STOP=1',
			'--host=127.0.0.1',
			'--port=55438',
			'--username=billyribeiro',
			`--dbname=${database}`,
			'--command',
			query
		],
		{ encoding: 'utf8', env: childEnv, timeout: 10000, maxBuffer: 1024 * 1024 }
	).trim();
	return JSON.parse(raw);
}
function guard(): string {
	return `DO $guard$ BEGIN IF current_database()<>${literal(database)} OR current_setting('data_directory')<>${literal(directory)} THEN RAISE EXCEPTION 'Wrong disposable fixture database'; END IF; END $guard$;`;
}
test.beforeAll(async ({ baseURL }) => {
	expect(baseURL).toBe('http://127.0.0.1:5274');
	writeFileSync(passfile, '', { mode: 0o600, flag: 'wx' });
	const pid = execFileSync('/usr/sbin/lsof', ['-nP', '-t', '-iTCP:8281', '-sTCP:LISTEN'], {
		encoding: 'utf8',
		env: childEnv
	}).trim();
	expect(pid).toMatch(/^\d+$/);
	const command = execFileSync('/bin/ps', ['-p', pid, '-o', 'command='], {
		encoding: 'utf8',
		env: childEnv
	}).trim();
	expect(command.split(/\s+/)[0]).toBe(binary);
	expect(createHash('sha256').update(readFileSync(binary)).digest('hex')).toBe(binarySha);
	const db = object(
		sql(
			"SELECT json_build_object('database',current_database(),'directory',current_setting('data_directory'),'timezone',current_setting('TimeZone'))"
		)
	);
	expect(db.database).toBe(database);
	expect(db.directory).toBe(directory);
	const api = await playwrightRequest.newContext({ baseURL });
	try {
		const login = await api.post('/api/auth/login', {
			data: { email: 'admin@admin-completion.test', password }
		});
		expect(login.status(), await login.text()).toBe(200);
		cookies = (await api.storageState()).cookies;
		const me = await api.get('/api/auth/me');
		expect(me.status()).toBe(200);
		const who = object(await me.json()),
			actor = who.user ? object(who.user) : who.data ? object(who.data) : who;
		expect(actor.name).toBe('Disposable completion115 browser admin');
		expect(actor.role).toBe('super_admin');
		const stats = await api.get('/api/admin/members/stats');
		expect(stats.status()).toBe(200);
		baseline = parseMemberStats(await stats.json()).revenue.recorded_net_usd_order_value;
		const created = await api.post('/api/admin/member-management', {
			data: {
				name: 'Recorded order value fixture',
				email,
				password,
				role: 'user',
				send_welcome_email: false
			}
		});
		expect(created.status(), await created.text()).toBe(200);
		const value = object(await created.json()),
			member = object(value.member || value.data || value);
		userId = Number(member.id);
		expect(Number.isSafeInteger(userId) && userId > 0).toBe(true);
		const matched = object(
			sql(
				`SELECT json_build_object('id',id,'email',email) FROM users WHERE id=${userId} AND email=${literal(email)} AND deleted_at IS NULL`
			)
		);
		expect(matched.id).toBe(userId);
		expect(matched.email).toBe(email);
		identity = {
			...db,
			api: actor.name,
			role: actor.role,
			pid,
			command,
			binarySha,
			fixtureMember: userId
		};
		const result = sql(
			`BEGIN; ${guard()} DO $member$ BEGIN IF NOT EXISTS(SELECT 1 FROM users WHERE id=${userId} AND email=${literal(email)} AND deleted_at IS NULL) THEN RAISE EXCEPTION 'API member does not match owned database'; END IF; END $member$; WITH saved AS(INSERT INTO orders(user_id,order_number,status,currency,total,refund_amount,completed_at,metadata) VALUES (${userId},${literal(prefix + '-usd-a')},'partial_refund','USD',120.00,20.00,(NOW() AT TIME ZONE 'UTC'),jsonb_build_object('fixture',${literal(prefix)})),(${userId},${literal(prefix + '-usd-b')},'partially_refunded','USD',25.45,2.00,(NOW() AT TIME ZONE 'UTC'),jsonb_build_object('fixture',${literal(prefix)})),(${userId},${literal(prefix + '-eur')},'paid','EUR',999.00,0,(NOW() AT TIME ZONE 'UTC'),jsonb_build_object('fixture',${literal(prefix)})),(${userId},${literal(prefix + '-unknown')},'paid',NULL,777.00,0,(NOW() AT TIME ZONE 'UTC'),jsonb_build_object('fixture',${literal(prefix)})) RETURNING id,user_id,order_number,status,currency,total::text,refund_amount::text,ROUND(GREATEST(total-COALESCE(refund_amount,0),0)*100)::bigint AS net_cents) SELECT json_agg(saved ORDER BY id) FROM saved; COMMIT;`
		);
		expect(Array.isArray(result)).toBe(true);
		source = (result as unknown[]).map(object);
		expect(source).toHaveLength(4);
		expect(source.slice(0, 2).map((row) => row.net_cents)).toEqual([10000, 2345]);
	} finally {
		await api.dispose();
	}
});
test.afterAll(async ({ baseURL }) => {
	if (!cookies) {
		if (existsSync(passfile)) unlinkSync(passfile);
		return;
	}
	const api = await playwrightRequest.newContext({
		baseURL,
		storageState: { cookies, origins: [] }
	});
	try {
		if (source.length && userId) {
			const ids = source.map((row) => {
				const id = Number(row.id);
				if (!Number.isSafeInteger(id) || id <= 0) throw new Error('Invalid owned order ID');
				return id;
			});
			const removed = object(
				sql(
					`BEGIN; ${guard()} WITH removed AS(DELETE FROM orders WHERE id=ANY(ARRAY[${ids.join(',')}]::bigint[]) AND user_id=${userId} AND metadata->>'fixture'=${literal(prefix)} RETURNING id) SELECT json_build_object('deleted_order_rows',COUNT(*)) FROM removed; COMMIT;`
				)
			);
			expect(removed.deleted_order_rows).toBe(source.length);
			const restored = await api.get('/api/admin/members/stats');
			expect(restored.status()).toBe(200);
			expect(parseMemberStats(await restored.json()).revenue.recorded_net_usd_order_value).toEqual(
				baseline
			);
		}
		if (userId) {
			expect((await api.delete(`/api/admin/member-management/${userId}`)).status()).toBe(200);
			const retained = object(
				sql(
					`SELECT json_build_object('id',id,'is_active',is_active,'deleted_at',deleted_at,'email_is_original',email=${literal(email)}) FROM users WHERE id=${userId}`
				)
			);
			expect(retained.id).toBe(userId);
			expect(retained.is_active).toBe(false);
			expect(retained.deleted_at).toBeNull();
			expect(retained.email_is_original).toBe(false);
			if (cleanupPath)
				await writeFile(
					cleanupPath,
					JSON.stringify(
						{
							deletedOwnedOrderIds: source.map((row) => row.id),
							restoredRecordedMoney: baseline,
							retainedAnonymizedAccount: retained
						},
						null,
						2
					)
				);
		} // Existing CRUD anonymizes/disables; it retains the user row.
	} finally {
		await api.dispose();
		if (existsSync(passfile)) unlinkSync(passfile);
	}
});
test('nonzero recorded USD after two refunds renders cents correctly and excludes other/unknown currencies', async ({
	page
}, info) => {
	cleanupPath = info.outputPath('fixture-cleanup.json');
	await page.context().addCookies(cookies);
	const errors: string[] = [],
		warnings: string[] = [],
		failed: string[] = [],
		blocked = new Set<string>();
	page.on('pageerror', (cause) => errors.push(cause.message));
	page.on('console', (message) => {
		if (message.type() === 'warning') warnings.push(message.text());
		if (
			message.type() === 'error' &&
			!(
				blocked.has(message.location().url) &&
				message.text() === 'Failed to load resource: net::ERR_FAILED'
			)
		)
			errors.push(message.text());
	});
	page.on('requestfailed', (request) => {
		if (!blocked.has(request.url()) && !request.failure()?.errorText.includes('ERR_ABORTED'))
			failed.push(request.failure()?.errorText || request.url());
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
	const responseReady = page.waitForResponse(
		(r) => new URL(r.url()).pathname === '/api/admin/members/stats'
	);
	await page.goto('/admin/members');
	const response = await responseReady;
	expect(response.status()).toBe(200);
	const stats = parseMemberStats(await response.json()),
		value = stats.revenue.recorded_net_usd_order_value;
	expect(value.net_cents).toBe(baseline.net_cents + 12345);
	expect(value.order_count).toBe(baseline.order_count + 2);
	expect(value.excluded_non_usd_orders).toBe(baseline.excluded_non_usd_orders + 1);
	expect(value.excluded_unknown_currency_orders).toBe(
		baseline.excluded_unknown_currency_orders + 1
	);
	const expected = new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD',
			minimumFractionDigits: 2,
			maximumFractionDigits: 2
		}).format((baseline.net_cents + 12345) / 100),
		field = page
			.locator('.stats-grid .stat-card')
			.filter({ hasText: 'Recorded net USD order value' })
			.locator('.stat-value');
	await expect(field).toHaveText(expected);
	await page.getByText('Measurement coverage and catalogue estimate', { exact: true }).click();
	await expect(
		page.getByText(
			`${value.excluded_non_usd_orders} non-USD and ${value.excluded_unknown_currency_orders} unknown-currency qualifying orders excluded.`,
			{ exact: true }
		)
	).toBeVisible();
	await expect(
		page.getByText('Monthly recurring revenue: unmeasured.', { exact: false })
	).toBeVisible();
	await page.screenshot({ path: info.outputPath('nonzero-recorded-usd.png') });
	await writeFile(
		info.outputPath('recorded-money-proof.json'),
		JSON.stringify(
			{
				identity,
				source,
				baseline,
				actual: value,
				expectedDom: expected,
				actualDom: await field.textContent(),
				errors,
				warnings,
				failed
			},
			null,
			2
		)
	);
	expect(errors).toEqual([]);
	expect(warnings).toEqual([]);
	expect(failed).toEqual([]);
});
