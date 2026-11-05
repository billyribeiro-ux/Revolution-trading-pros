import {
	expect,
	request as playwrightRequest,
	test,
	type APIRequestContext,
	type Page,
	type Locator
} from '@playwright/test';
import { execFileSync } from 'node:child_process';
import { createHash, randomUUID } from 'node:crypto';
import {
	readFileSync,
	writeFileSync,
	existsSync,
	unlinkSync,
	realpathSync,
	mkdirSync
} from 'node:fs';
import { parseMemberProfile, type MemberProfile } from '../../src/lib/api/member-profile';
const database = 'admin_move_verify_completion115_browser',
	directory = '/private/tmp/revolution-backend-resume-pg';
const binary = '/tmp/revolution-api-completion116e-verified',
	binarySha = 'ccb6519f788e91ca3393b670e376e442cb39b46e0cd4634bc3559db83675dea0';
const password = 'OnlyForDisposableTest123!',
	stamp = `profile-${randomUUID().replaceAll('-', '').slice(0, 20)}`,
	primaryEmail = `${stamp}@example.test`,
	secondaryEmail = `${stamp}-second@example.test`;
const primaryName = `Factual profile ${stamp}`,
	secondaryName = `Second profile ${stamp}`,
	passfile = `/private/tmp/revolution-${stamp}.pgpass`;
const childEnv = {
	PATH: process.env.PATH || '/usr/bin:/bin',
	PGPASSFILE: passfile,
	PGPASSWORD: '',
	PGCONNECT_TIMEOUT: '3',
	PGCLIENTENCODING: 'UTF8',
	PGAPPNAME: 'revolution-member-profile-proof'
};
test.describe.configure({ mode: 'default' });
let admin: APIRequestContext,
	actorId: string,
	primary: string,
	secondary: string,
	plan: string | undefined,
	seed: MemberProfile;
let cookies: Awaited<ReturnType<APIRequestContext['storageState']>>['cookies'];
const qaDirectory = `/private/tmp/revolution-member116e-qa-${stamp}`;
let orderIds: string[] = [],
	membershipIds: string[] = [],
	emailIds: string[] = [],
	noteIds: string[] = [];
function object(value: unknown): Record<string, unknown> {
	if (!value || typeof value !== 'object' || Array.isArray(value))
		throw new Error('Invalid owned fixture response');
	return value as Record<string, unknown>;
}
function id(value: unknown): string {
	const result =
		typeof value === 'string'
			? value
			: typeof value === 'number' && Number.isSafeInteger(value)
				? value.toString()
				: '';
	if (!/^[1-9]\d*$/.test(result)) throw new Error('Invalid actual fixture identity');
	return result;
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
function guard() {
	return `DO $guard$ BEGIN IF current_database()<>${literal(database)} OR current_user<>'billyribeiro' OR current_setting('data_directory')<>${literal(directory)} THEN RAISE EXCEPTION 'Wrong disposable profile database'; END IF; END $guard$;`;
}
function memberGuard() {
	return `${guard()} DO $member$ BEGIN IF NOT EXISTS(SELECT 1 FROM users WHERE id=${primary} AND email=${literal(primaryEmail)} AND deleted_at IS NULL) THEN RAISE EXCEPTION 'API fixture is not in the owned database'; END IF; END $member$;`;
}
async function readProfile(
	client: APIRequestContext,
	member: string,
	family = 'member-management'
) {
	const response = await client.get(`/api/admin/${family}/${member}`);
	expect(response.status(), await response.text()).toBe(200);
	expect(response.headers()['cache-control']).toContain('no-store');
	expect(response.headers().vary).toContain('Cookie');
	return parseMemberProfile(await response.json(), member);
}
async function createMember(email: string, name: string) {
	const response = await admin.post('/api/admin/member-management', {
		data: { name, email, password, role: 'user', send_welcome_email: false }
	});
	expect(response.status(), await response.text()).toBe(200);
	return id(object(object(await response.json()).member).id);
}
function captured(value: unknown): string[] {
	if (!Array.isArray(value)) throw new Error('Missing captured fixture rows');
	return value.map((row) => id(object(row).id));
}
async function history(
	client: APIRequestContext,
	member: string,
	kind: string,
	page = 1,
	size = 25
) {
	const response = await client.get(
		`/api/admin/member-management/${member}/${kind}?page=${page}&per_page=${size}`
	);
	expect(response.status(), await response.text()).toBe(200);
	const body = object(await response.json());
	expect(id(body.member_id)).toBe(member);
	return body;
}
async function diagnostics(page: Page, expected = new Set<string>()) {
	const errors: string[] = [],
		blocked = new Set<string>();
	page.on('pageerror', (error) => errors.push(error.message));
	page.on('console', (message) => {
		if (!['error', 'warning'].includes(message.type())) return;
		const source = message.location().url,
			status = message
				.text()
				.match(/^Failed to load resource: the server responded with a status of (\d+)/)?.[1];
		if (status && source && expected.has(`${status} ${new URL(source).pathname}`)) return;
		if (blocked.has(source) && message.text() === 'Failed to load resource: net::ERR_FAILED')
			return;
		errors.push(`${message.text()} ${source}`);
	});
	page.on('requestfailed', (request) => {
		if (!blocked.has(request.url()) && !request.failure()?.errorText.includes('ERR_ABORTED'))
			errors.push(`${request.failure()?.errorText} ${request.url()}`);
	});
	page.on('response', (response) => {
		const path = new URL(response.url()).pathname;
		if (
			path.startsWith('/api/') &&
			response.status() >= 400 &&
			!expected.has(`${response.status()} ${path}`)
		)
			errors.push(`${response.status()} ${path}`);
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
	return errors;
}
async function authenticate(page: Page) {
	await page.context().addCookies(cookies);
}
async function tab(page: Page, name: string) {
	await page
		.getByRole('navigation', { name: 'Member profile sections', exact: true })
		.getByRole('button', { name, exact: true })
		.click();
}

async function nextPage(page: Page, panel: Locator, kind: string, label: string) {
	const response = page.waitForResponse((response) => {
		const url = new URL(response.url());
		return url.pathname.endsWith(`/${primary}/${kind}`) && url.searchParams.get('page') === '2';
	});
	await panel.getByRole('button', { name: `Next ${label}`, exact: true }).click();
	const wire = await response;
	expect(wire.status()).toBe(200);
	const body = object(await wire.json());
	expect(id(body.member_id)).toBe(primary);
	expect(object(body.pagination).current_page).toBe(2);
	await expect(panel.getByText(/Page 2 of/)).toBeVisible();
	return body;
}

test.beforeAll(async ({ baseURL }) => {
	test.setTimeout(120000);
	expect(baseURL).toBe('http://127.0.0.1:5274');
	expect(process.env.MEMBER_PROFILE_DISPOSABLE_PROOF).toBe('1');
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
	const launched = command.split(/\s+/)[0];
	if (!launched) throw new Error('Missing owned API executable');
	expect(realpathSync(launched)).toBe(realpathSync(binary));
	expect(createHash('sha256').update(readFileSync(binary)).digest('hex')).toBe(binarySha);
	const db = object(
		sql(
			"SELECT json_build_object('database',current_database(),'directory',current_setting('data_directory'),'user',current_user)"
		)
	);
	expect(db).toEqual({ database, directory, user: 'billyribeiro' });
	admin = await playwrightRequest.newContext({ baseURL });
	const login = await admin.post('/api/auth/login', {
		data: { email: 'admin@admin-completion.test', password }
	});
	expect(login.status(), await login.text()).toBe(200);
	cookies = (await admin.storageState()).cookies;
	const actor = object(await (await admin.get('/api/auth/me')).json());
	expect(actor.name).toBe('Disposable completion115 browser admin');
	expect(actor.role).toBe('super_admin');
	actorId = id(actor.id);
	primary = await createMember(primaryEmail, primaryName);
	secondary = await createMember(secondaryEmail, secondaryName);
	for (let index = 1; index <= 26; index++) {
		const content = `${stamp} seeded note ${index}`,
			response = await admin.post(`/api/admin/members/${primary}/notes`, { data: { content } });
		expect(response.status(), await response.text()).toBe(200);
		const row = object(object(await response.json()).note);
		expect(row.content).toBe(content);
		expect(id(row.created_by)).toBe(actorId);
		noteIds.push(id(row.id));
	}
	plan = id(
		object(
			sql(
				`BEGIN; ${memberGuard()} WITH saved AS(INSERT INTO membership_plans(name,slug,price,billing_cycle,is_active,metadata) VALUES(${literal(stamp + ' recorded plan')},${literal(stamp)},123.45,'monthly',false,jsonb_build_object('fixture',${literal(stamp)})) RETURNING id) SELECT row_to_json(saved) FROM saved; COMMIT;`
			)
		).id
	);
	membershipIds = captured(
		sql(
			`BEGIN; ${memberGuard()} WITH saved AS(INSERT INTO user_memberships(user_id,plan_id,status,starts_at,expires_at,created_at,updated_at) SELECT ${primary},${plan},CASE WHEN g=27 THEN 'active' ELSE 'cancelled' END,TIMESTAMP '2026-01-02 03:04:05',CASE WHEN g=27 THEN (NOW() AT TIME ZONE 'UTC')+INTERVAL '7 days' WHEN g=26 THEN 'infinity'::timestamp ELSE TIMESTAMP '2026-02-01 03:04:05' END,TIMESTAMP '2026-01-02 03:04:05'+g*INTERVAL '1 second',TIMESTAMP '2026-01-02 03:04:05' FROM generate_series(1,27) g RETURNING id) SELECT json_agg(saved) FROM saved; COMMIT;`
		)
	);
	orderIds = captured(
		sql(
			`BEGIN; ${memberGuard()} WITH saved AS(INSERT INTO orders(user_id,order_number,status,currency,total,refund_amount,created_at,completed_at,metadata) SELECT ${primary},CASE WHEN g=1 THEN ${literal('=owned-' + stamp)} ELSE ${literal(stamp + '-order-')}||g END,CASE WHEN g=27 THEN 'partial_refund' WHEN g IN(25,26) THEN 'paid' ELSE 'pending' END,CASE WHEN g=26 THEN 'EUR' WHEN g=25 THEN NULL ELSE 'USD' END,CASE WHEN g=27 THEN 120.00 WHEN g=26 THEN 25.45 WHEN g=25 THEN 3.14 ELSE 1.00 END,CASE WHEN g=27 THEN 20.00 ELSE 0 END,CASE WHEN g=1 THEN TIMESTAMP '270000-01-02 03:04:05' WHEN g=2 THEN NULL ELSE TIMESTAMP '2026-01-02 03:04:05'+g*INTERVAL '1 second' END,CASE WHEN g>=25 THEN TIMESTAMP '2026-01-02 03:04:05' ELSE NULL END,jsonb_build_object('fixture',${literal(stamp)}) FROM generate_series(1,27) g RETURNING id) SELECT json_agg(saved) FROM saved; COMMIT;`
		)
	);
	emailIds = captured(
		sql(
			`BEGIN; ${memberGuard()} WITH saved AS(INSERT INTO email_logs(user_id,email,subject,status,provider,provider_message_id,sent_at,opened_at,created_at,metadata) SELECT ${primary},${literal(primaryEmail)},${literal(stamp + ' historical email ')}||g,'sent','owned-historical-fixture',NULL,NULL,CASE WHEN g=27 THEN TIMESTAMP '2026-01-03 04:05:06' ELSE NULL END,TIMESTAMP '2026-01-02 03:04:05'+g*INTERVAL '1 second',jsonb_build_object('fixture',${literal(stamp)}) FROM generate_series(1,27) g RETURNING id) SELECT json_agg(saved) FROM saved; COMMIT;`
		)
	);
	sql(
		`BEGIN; ${memberGuard()} UPDATE users SET is_active=false,mfa_enabled=NULL,created_at=TIMESTAMP '2026-01-02 03:04:05' WHERE id=${primary} AND email=${literal(primaryEmail)}; INSERT INTO user_status(user_id,status,status_reason,ban_reason) VALUES(${primary},'banned',${literal(stamp + ' recorded restriction')},${literal('Owned fixture restriction')}); SELECT json_build_object('updated',true); COMMIT;`
	);
	mkdirSync(qaDirectory, { mode: 0o700 });
	const source = sql(
		`BEGIN; ${memberGuard()} SELECT json_agg(json_build_object('id',id::text,'member_id',user_id::text,'order_number',order_number,'recorded_status',status,'currency',currency,'total_units',total::text,'refund_units',refund_amount::text,'net_recorded_units',GREATEST(total-COALESCE(refund_amount,0),0)::text,'created_at_recorded',created_at,'completed_at_recorded',completed_at,'refunded_at_recorded',refunded_at) ORDER BY created_at DESC NULLS LAST,id DESC) FROM orders WHERE id=ANY(ARRAY[${orderIds.join(',')}]::bigint[]) AND user_id=${primary} AND metadata->>'fixture'=${literal(stamp)}; COMMIT;`
	);
	if (!Array.isArray(source) || source.length !== 27)
		throw new Error('Captured source order snapshot is incomplete');
	writeFileSync(
		qaDirectory + '/orders-source.json',
		JSON.stringify(
			{
				fixture: stamp,
				member_id: primary,
				binary: realpathSync(binary),
				binary_sha256: binarySha,
				pid,
				command,
				database_identity: db,
				actor_id: actorId,
				rows: source
			},
			null,
			2
		)
	);
	seed = await readProfile(admin, primary);
	expect(seed.member.status).toBe('active');
	expect(seed.member.account).toMatchObject({
		is_active: false,
		mfa_enabled: null,
		recorded_restriction: 'banned'
	});
	expect(seed.recorded_order_value).toMatchObject({
		net_cents: 10000,
		excluded_non_usd_orders: 1,
		excluded_unknown_currency_orders: 1
	});
	expect(seed.engagement.score).toBeNull();
	for (const kind of ['memberships', 'orders', 'notes', 'activity', 'emails', 'timeline'] as const)
		expect(seed[kind].pagination.total, kind).toBeGreaterThan(25);
	for (const kind of ['memberships', 'orders', 'notes', 'activity', 'emails', 'timeline']) {
		const next = await history(admin, primary, kind, 2);
		expect(object(next.pagination).current_page).toBe(2);
	}
	expect(seed.emails.emails[0]?.outcome).toBe('unknown');
	expect((await readProfile(admin, primary, 'members')).member_id).toBe(primary);
});

test.afterAll(async () => {
	if (!admin) {
		if (existsSync(passfile)) unlinkSync(passfile);
		return;
	}
	try {
		if (primary) {
			for (const note of noteIds) {
				const response = await admin.delete(`/api/admin/members/${primary}/notes/${note}`);
				expect(response.status(), await response.text()).toBe(200);
			}
			if (orderIds.length || membershipIds.length || emailIds.length) {
				const removed = object(
					sql(
						`BEGIN; ${memberGuard()} WITH a AS(DELETE FROM orders WHERE id=ANY(ARRAY[${orderIds.join(',')}]::bigint[]) AND user_id=${primary} AND metadata->>'fixture'=${literal(stamp)} RETURNING id),b AS(DELETE FROM user_memberships WHERE id=ANY(ARRAY[${membershipIds.join(',')}]::bigint[]) AND user_id=${primary} AND plan_id=${plan} RETURNING id),c AS(DELETE FROM email_logs WHERE id=ANY(ARRAY[${emailIds.join(',')}]::bigint[]) AND user_id=${primary} AND metadata->>'fixture'=${literal(stamp)} RETURNING id),d AS(DELETE FROM user_status WHERE user_id=${primary} AND status_reason=${literal(stamp + ' recorded restriction')} RETURNING id) SELECT json_build_object('orders',(SELECT COUNT(*) FROM a),'memberships',(SELECT COUNT(*) FROM b),'emails',(SELECT COUNT(*) FROM c)) ; COMMIT;`
					)
				);
				expect(removed).toEqual({
					orders: orderIds.length,
					memberships: membershipIds.length,
					emails: emailIds.length
				});
			}
			if (plan) {
				const removed = object(
					sql(
						`BEGIN; ${guard()} WITH removed AS(DELETE FROM membership_plans WHERE id=${plan} AND slug=${literal(stamp)} AND metadata->>'fixture'=${literal(stamp)} RETURNING id) SELECT json_build_object('deleted',COUNT(*)) FROM removed; COMMIT;`
					)
				);
				expect(removed.deleted).toBe(1);
			}
			expect((await readProfile(admin, primary)).recorded_order_value.net_cents).toBe(0);
		}
		for (const member of [primary, secondary].filter(Boolean)) {
			const response = await admin.delete('/api/admin/member-management/' + member);
			expect(response.status(), await response.text()).toBe(200);
		}
	} finally {
		await admin.dispose();
		if (existsSync(passfile)) unlinkSync(passfile);
	}
});

test('full member profile preserves factual cohorts, unknowns, currencies and every independent history page', async ({
	page
}) => {
	await authenticate(page);
	const errors = await diagnostics(page);
	await page.goto(`/admin/members/${primary}`, { waitUntil: 'networkidle' });
	await expect(page.getByRole('heading', { name: primaryName, exact: true })).toBeVisible();
	await expect(page.getByText('Membership cohort: Active', { exact: true })).toBeVisible();
	await expect(page.getByText('Inactive account', { exact: true })).toBeVisible();
	await expect(page.getByText('Recorded account status: banned', { exact: true })).toBeVisible();
	await expect(page.locator('.stats-grid').getByText('USD 100.00', { exact: true })).toBeVisible();
	await expect(
		page.locator('.stats-grid').getByText('Not measured', { exact: true })
	).toBeVisible();
	await expect(
		page
			.locator('.stats-grid')
			.getByText('2026-01-02T03:04:05 (timezone not recorded)', { exact: true })
	).toBeVisible();
	const details = page.getByRole('region', { name: 'Recorded member details', exact: true });
	await expect(
		details
			.getByText('Two-factor setting', { exact: true })
			.locator('xpath=following-sibling::dd[1]')
	).toHaveText('Not recorded');
	await nextPage(
		page,
		page.getByRole('region', { name: 'Recorded member timeline', exact: true }),
		'timeline',
		'timeline'
	);
	await tab(page, 'Subscriptions');
	const subscriptions = page.getByRole('region', { name: 'Membership history', exact: true });
	await expect(subscriptions.getByText(/123.45 units \(currency not recorded\)/)).toHaveCount(25);
	await nextPage(page, subscriptions, 'memberships', 'memberships');
	await tab(page, 'Orders');
	const orders = page.getByRole('region', { name: 'Recorded order history', exact: true });
	await expect(orders.getByRole('cell', { name: 'USD 120.00', exact: true })).toBeVisible();
	await expect(orders.getByRole('cell', { name: 'USD 20.00', exact: true })).toBeVisible();
	await expect(orders.getByRole('cell', { name: 'EUR 25.45', exact: true })).toHaveCount(2);
	await expect(
		orders.getByRole('cell', { name: 'Currency not recorded 3.14', exact: true })
	).toHaveCount(2);

	await orders.getByRole('button', { name: `View order ${stamp}-order-27`, exact: true }).click();
	const detail = page.getByRole('dialog', {
		name: `Recorded Order ${stamp}-order-27`,
		exact: true
	});
	await expect(detail.getByText('USD 20.00', { exact: true })).toBeVisible();
	await expect(
		detail.getByText('Member identity', { exact: true }).locator('xpath=following-sibling::dd[1]')
	).toHaveText(primary);
	await detail
		.getByRole('button', { name: `Close Recorded Order ${stamp}-order-27`, exact: true })
		.click();
	const sized = page.waitForResponse((response) => {
		const url = new URL(response.url());
		return (
			url.pathname.endsWith(`/${primary}/orders`) &&
			url.searchParams.get('per_page') === '10' &&
			url.searchParams.get('page') === '1'
		);
	});
	await orders.getByRole('combobox', { name: 'Rows per page', exact: true }).selectOption('10');
	expect((await sized).status()).toBe(200);
	await expect(orders.locator('tbody tr')).toHaveCount(10);
	const downloaded = page.waitForEvent('download'),
		exported = page.waitForResponse((response) =>
			new URL(response.url()).pathname.endsWith(`/${primary}/orders/export`)
		);
	await orders.getByRole('button', { name: 'Export', exact: true }).click();
	const [download, response] = await Promise.all([downloaded, exported]);
	expect(response.status()).toBe(200);
	expect(download.suggestedFilename()).toBe(`member-${primary}-orders.csv`);
	const path = qaDirectory + '/member-orders.csv';
	await download.saveAs(path);
	const bytes = readFileSync(path),
		headers = response.headers();
	writeFileSync(
		qaDirectory + '/receipt.json',
		JSON.stringify(
			{
				download_path: path,
				source_path: qaDirectory + '/orders-source.json',
				suggested_filename: download.suggestedFilename(),
				headers,
				actual_sha256: createHash('sha256').update(bytes).digest('hex'),
				actual_bytes: bytes.length,
				member_id: primary,
				visible_page_size: 10
			},
			null,
			2
		)
	);
	console.log('Member order QA artifacts: ' + qaDirectory);

	expect(headers['x-export-member-id']).toBe(primary);
	expect(headers['x-export-row-count']).toBe('27');
	expect(headers['x-export-scope']).toBe('all_recorded_member_orders');
	expect(headers['x-export-sha256']).toBe(createHash('sha256').update(bytes).digest('hex'));
	await expect(
		page.getByRole('status').filter({ hasText: 'Exported 27 recorded orders' })
	).toContainText(headers['x-export-sha256'] ?? '');
	expect(headers['content-type']).toMatch(/^text\/csv\s*;\s*charset=utf-8$/i);
	expect(headers['cache-control']).toContain('no-store');
	expect([...bytes.subarray(0, 3)]).toEqual([239, 187, 191]);
	expect(bytes.includes(Buffer.from('\r\n'))).toBe(true);
	const decoded: unknown = JSON.parse(
		execFileSync(
			'/usr/bin/python3',
			[
				'-c',
				'import sys,io,csv,json; print(json.dumps(list(csv.reader(io.StringIO(sys.stdin.buffer.read().decode("utf-8-sig"),newline="")))))'
			],
			{ input: bytes, encoding: 'utf8', env: childEnv, timeout: 10000 }
		)
	);
	if (
		!Array.isArray(decoded) ||
		!decoded.every((row) => Array.isArray(row) && row.every((cell) => typeof cell === 'string'))
	)
		throw new Error('Independent CSV parser did not return actual cells');
	const cells = decoded as string[][];
	const names = cells.shift();
	expect(names).toEqual([
		'id',
		'member_id',
		'order_number',
		'recorded_status',
		'currency',
		'total_units',
		'refund_units',
		'net_recorded_units',
		'created_at_recorded',
		'completed_at_recorded',
		'refunded_at_recorded',
		'timestamp_basis'
	]);
	expect(cells).toHaveLength(27);
	const rows = cells.map((row) => {
		expect(row).toHaveLength(12);
		return Object.fromEntries((names ?? []).map((name, index) => [name, row[index]]));
	});
	expect(rows.map((row) => id(row.id)).sort((a, b) => (a < b ? -1 : a > b ? 1 : 0))).toEqual(
		[...orderIds].sort((a, b) => (a < b ? -1 : a > b ? 1 : 0))
	);
	expect(rows.every((row) => row.member_id === primary)).toBe(true);
	const refunded = rows.find((row) => row.order_number === stamp + '-order-27');
	expect(refunded).toMatchObject({
		currency: 'USD',
		total_units: '120.00',
		refund_units: '20.00',
		net_recorded_units: '100.00'
	});
	expect(rows.find((row) => row.order_number === stamp + '-order-26')?.currency).toBe('EUR');
	expect(rows.find((row) => row.order_number === stamp + '-order-25')?.currency).toBe('');
	expect(rows.find((row) => row.order_number === stamp + '-order-2')?.created_at_recorded).toBe('');
	expect(
		rows.find((row) => row.order_number === "'=owned-" + stamp)?.created_at_recorded
	).toContain('270000-01-02');
	expect(
		rows.every(
			(row) =>
				row.timestamp_basis ===
				'Recorded timestamps without timezone; historical timezone unverified'
		)
	).toBe(true);
	await nextPage(page, orders, 'orders', 'orders');
	await tab(page, 'Emails');
	const emails = page.getByRole('region', { name: 'Linked email receipts', exact: true });
	await expect(emails.getByText('unknown', { exact: true })).toHaveCount(25);
	await expect(emails.getByText(/No Emails Sent/)).toHaveCount(0);
	await nextPage(page, emails, 'emails', 'email receipts');
	await expect(emails.getByText('unknown', { exact: true })).toHaveCount(2);
	await tab(page, 'Notes');
	const notes = page.getByRole('region', { name: 'Recorded internal notes', exact: true });
	await nextPage(page, notes, 'notes', 'notes');
	await expect(notes.locator('.note-item')).toHaveCount(1);
	await tab(page, 'Activity');
	await nextPage(
		page,
		page.getByRole('region', { name: 'Recorded member activity', exact: true }),
		'activity',
		'activity'
	);
	expect(errors).toEqual([]);
});

test('saved note create/delete has the actual creator and remains reachable in a narrow viewport', async ({
	page
}) => {
	await authenticate(page);
	const errors = await diagnostics(page);
	await page.setViewportSize({ width: 390, height: 740 });
	await page.goto(`/admin/members/${primary}`, { waitUntil: 'networkidle' });
	await tab(page, 'Notes');
	const notes = page.getByRole('region', { name: 'Recorded internal notes', exact: true });
	await notes.getByRole('button', { name: 'Add Note', exact: true }).click();
	const dialog = page.getByRole('dialog', { name: 'Add Note', exact: true }),
		content = `${stamp} browser note\nSecond recorded line`;
	await dialog.getByRole('textbox', { name: 'Note', exact: true }).fill(content);
	const submit = dialog.getByRole('button', { name: 'Save Note', exact: true });
	await submit.scrollIntoViewIfNeeded();
	const box = await submit.boundingBox();
	expect(box).not.toBeNull();
	expect(box?.y).toBeGreaterThanOrEqual(0);
	expect((box?.y ?? 0) + (box?.height ?? 0)).toBeLessThanOrEqual(740);
	const created = page.waitForResponse(
		(response) =>
			new URL(response.url()).pathname.endsWith(`/${primary}/notes`) &&
			response.request().method() === 'POST'
	);
	await submit.click();
	const receipt = await created;
	expect(receipt.status()).toBe(200);
	const note = object(object(await receipt.json()).note),
		key = id(note.id);
	expect(note.content).toBe(content);
	expect(id(note.created_by)).toBe(actorId);
	noteIds.push(key);
	await expect(dialog).toHaveCount(0);
	await page.reload({ waitUntil: 'networkidle' });
	await tab(page, 'Notes');
	await expect(notes.getByText(content, { exact: true })).toBeVisible();
	await notes.getByRole('button', { name: `Delete note ${key}`, exact: true }).click();
	const confirmation = page.getByRole('dialog', { name: 'Delete Note', exact: true });
	await confirmation.getByRole('button', { name: 'Delete Note', exact: true }).click();
	await expect(notes.getByText(content, { exact: true })).toHaveCount(0);
	noteIds = noteIds.filter((value) => value !== key);
	await page.reload({ waitUntil: 'networkidle' });
	await tab(page, 'Notes');
	await expect(notes.getByText(content, { exact: true })).toHaveCount(0);
	await notes.getByRole('button', { name: 'Add Note', exact: true }).click();
	const uncertainDialog = page.getByRole('dialog', { name: 'Add Note', exact: true }),
		uncertainContent = `${stamp} actual unknown-write note`;
	await uncertainDialog.getByRole('textbox', { name: 'Note', exact: true }).fill(uncertainContent);
	let posts = 0,
		persistedKey: string | undefined;
	const notePath = new RegExp(`/api/admin/(?:members|member-management)/${primary}/notes$`);
	await page.route(notePath, async (route) => {
		if (route.request().method() !== 'POST') return route.continue();
		posts++;
		const actual = await route.fetch();
		expect(actual.status()).toBe(200);
		const persisted = object(object(await actual.json()).note);
		expect(persisted.content).toBe(uncertainContent);
		expect(id(persisted.created_by)).toBe(actorId);
		persistedKey = id(persisted.id);
		noteIds.push(persistedKey);
		return route.fulfill({
			status: 200,
			contentType: 'application/json',
			body: JSON.stringify({ saved: true })
		});
	});
	try {
		await uncertainDialog.getByRole('button', { name: 'Save Note', exact: true }).click();
		await expect(uncertainDialog.getByRole('alert')).toContainText(/unconfirmed|unreadable/i);
		await expect(uncertainDialog.getByRole('textbox', { name: 'Note', exact: true })).toHaveValue(
			uncertainContent
		);
		await expect(
			uncertainDialog.getByRole('button', { name: 'Save Note', exact: true })
		).toBeDisabled();
		expect(posts).toBe(1);
		if (!persistedKey) throw new Error('Actual persisted unknown-write identity was not captured');
		await expect(
			page.getByText(`Note #${persistedKey} saved with its recorded author.`, { exact: true })
		).toHaveCount(0);
		await uncertainDialog
			.getByRole('button', { name: 'Refresh notes before retry', exact: true })
			.click();
		await expect(uncertainDialog.getByRole('alert')).toContainText('History refreshed');
		await uncertainDialog.getByRole('button', { name: 'Cancel', exact: true }).click();
		const journal = await history(page.request, primary, 'notes');
		if (!Array.isArray(journal.notes)) throw new Error('Saved notes are unavailable');
		const confirmed = journal.notes.map(object).find((row) => id(row.id) === persistedKey);
		expect(confirmed).toBeDefined();
		expect(confirmed?.content).toBe(uncertainContent);
		expect(id(confirmed?.created_by)).toBe(actorId);
		await expect(
			notes.getByRole('button', { name: `Delete note ${persistedKey}`, exact: true })
		).toBeVisible();
		await page.reload({ waitUntil: 'networkidle' });
		await tab(page, 'Notes');
		await expect(notes.getByText(uncertainContent, { exact: true })).toBeVisible();
		expect(posts).toBe(1);
		await notes.getByRole('button', { name: `Delete note ${persistedKey}`, exact: true }).click();
		await page
			.getByRole('dialog', { name: 'Delete Note', exact: true })
			.getByRole('button', { name: 'Delete Note', exact: true })
			.click();
		await expect(notes.getByText(uncertainContent, { exact: true })).toHaveCount(0);
		noteIds = noteIds.filter((value) => value !== persistedKey);
		expect(posts).toBe(1);
	} finally {
		await page.unroute(notePath);
	}

	expect(errors).toEqual([]);
});

test('history503 and malformed200 recover without false empty states or stale member identities in the drawer', async ({
	page
}) => {
	await authenticate(page);
	const expected = new Set([
			`503 /api/admin/member-management/${primary}`,
			`503 /api/admin/members/${primary}`,
			`503 /api/admin/member-management/${primary}/orders`,
			`503 /api/admin/members/${primary}/orders`
		]),
		errors = await diagnostics(page, expected);
	await page.goto(`/admin/members/${primary}`, { waitUntil: 'networkidle' });
	await tab(page, 'Orders');
	const panel = page.getByRole('region', { name: 'Recorded order history', exact: true });
	let mode: 'outage' | 'malformed' = 'outage';
	const pattern = `**/api/admin/*/${primary}/orders?*`;
	await page.route(pattern, (route) =>
		route.fulfill({
			status: mode === 'outage' ? 503 : 200,
			contentType: 'application/json',
			body: JSON.stringify(
				mode === 'outage'
					? { message: 'Owned order history outage' }
					: {
							schema_version: 1,
							metrics_scope: 'member_orders',
							member_id: secondary,
							orders: [],
							pagination: { total: 0, per_page: 25, current_page: 1, last_page: 1 },
							timestamp_basis: 'fixture mismatch'
						}
			)
		})
	);
	await panel.getByRole('button', { name: 'Refresh orders', exact: true }).click();
	await expect(panel.getByRole('alert')).toContainText('Owned order history outage');
	await expect(panel.getByText('No recorded orders on this page', { exact: true })).toHaveCount(0);
	mode = 'malformed';
	await panel.getByRole('button', { name: 'Retry orders', exact: true }).click();
	await expect(panel.getByRole('alert')).toContainText(/invalid|unconfirmed/i);
	await expect(panel.getByText('No recorded orders on this page', { exact: true })).toHaveCount(0);
	await page.unroute(pattern);
	await panel.getByRole('button', { name: 'Retry orders', exact: true }).click();
	await expect(panel.getByRole('cell', { name: 'USD 120.00', exact: true })).toBeVisible();

	const foreign = await readProfile(page.request, secondary);
	const badProfilePath = new RegExp(
		`/api/admin/(?:members|member-management)/${primary}(?:\\?.*)?$`
	);
	let profileMode: 'outage' | 'foreign' = 'outage';
	await page.route(badProfilePath, (route) =>
		route.fulfill({
			status: profileMode === 'outage' ? 503 : 200,
			contentType: 'application/json',
			body: JSON.stringify(
				profileMode === 'outage' ? { message: 'Owned member profile outage' } : foreign
			)
		})
	);
	await page.getByRole('button', { name: 'Refresh profile', exact: true }).click();
	const profile = page.getByRole('region', { name: 'Member profile', exact: true });
	await expect(profile.getByRole('alert')).toContainText('Owned member profile outage');
	await expect(profile.getByRole('heading', { name: primaryName, exact: true })).toHaveCount(0);
	profileMode = 'foreign';
	await profile.getByRole('button', { name: 'Retry member profile', exact: true }).click();
	await expect(profile.getByRole('alert')).toContainText(/invalid|unconfirmed/i);
	await expect(profile.getByRole('heading', { name: secondaryName, exact: true })).toHaveCount(0);
	await page.unroute(badProfilePath);
	await profile.getByRole('button', { name: 'Retry member profile', exact: true }).click();
	await expect(profile.getByRole('heading', { name: primaryName, exact: true })).toBeVisible();
	await page.goto('/admin/members', { waitUntil: 'networkidle' });
	await page.getByPlaceholder('Search members by name or email...').fill(stamp);
	await page.getByPlaceholder('Search members by name or email...').press('Enter');
	const first = page.getByRole('row').filter({ hasText: primaryEmail }),
		second = page.getByRole('row').filter({ hasText: secondaryEmail });
	await first.locator('.member-info-btn').click();
	const loadedDrawer = page.getByRole('dialog', { name: 'Member details', exact: true });
	await expect(loadedDrawer.getByRole('heading', { name: primaryName, exact: true })).toBeVisible();
	await expect(loadedDrawer.getByText('Membership cohort: Active', { exact: true })).toBeVisible();
	await expect(
		loadedDrawer.locator('.stats-grid').getByText('USD 100.00', { exact: true })
	).toBeVisible();
	await loadedDrawer
		.getByRole('navigation', { name: 'Member profile sections', exact: true })
		.getByRole('button', { name: 'Notes', exact: true })
		.click();
	const drawerNotes = loadedDrawer.getByRole('region', {
		name: 'Recorded internal notes',
		exact: true
	});
	await nextPage(page, drawerNotes, 'notes', 'notes');
	await expect(drawerNotes.locator('.note-item')).toHaveCount(1);
	await loadedDrawer.getByRole('button', { name: 'Close member details', exact: true }).click();
	let release: () => void = () => undefined,
		heldReady: () => void = () => undefined,
		settle: () => void = () => undefined;
	const held = new Promise<void>((resolve) => {
			release = resolve;
		}),
		ready = new Promise<void>((resolve) => {
			heldReady = resolve;
		}),
		settled = new Promise<void>((resolve) => {
			settle = resolve;
		});
	let delayedFailure: unknown;
	const profilePath = new RegExp(`/api/admin/(?:members|member-management)/${primary}(?:\\?.*)?$`);
	await page.route(profilePath, async (route) => {
		try {
			const actual = await route.fetch();
			expect(actual.status()).toBe(200);
			expect(parseMemberProfile(await actual.json(), primary).member_id).toBe(primary);
			heldReady();
			await held;
			if (!route.request().failure()?.errorText.includes('ERR_ABORTED'))
				await route.fulfill({ response: actual });
		} catch (cause) {
			if (!route.request().failure()?.errorText.includes('ERR_ABORTED')) delayedFailure = cause;
		} finally {
			heldReady();
			settle();
		}
	});
	try {
		await first.locator('.member-info-btn').click();
		await ready;
		const drawer = page.getByRole('dialog', { name: 'Member details', exact: true });
		await drawer.getByRole('button', { name: 'Close member details', exact: true }).click();
		await second.locator('.member-info-btn').click();
		await expect(drawer.getByRole('heading', { name: secondaryName, exact: true })).toBeVisible();
		release();
		await settled;
		await page.evaluate(
			() =>
				new Promise<void>((resolve) =>
					requestAnimationFrame(() => requestAnimationFrame(() => resolve()))
				)
		);
		expect(delayedFailure).toBeUndefined();
		await expect(drawer.getByRole('heading', { name: secondaryName, exact: true })).toBeVisible();
		await expect(drawer.getByText(primaryEmail, { exact: true })).toHaveCount(0);
	} finally {
		release();
		await page.unroute(profilePath);
	}

	expect(errors).toEqual([]);
});

test('both profile families and notes preserve anonymous401 and ordinary-user403 without CRM delegation expansion', async ({
	page,
	context
}) => {
	const errors = await diagnostics(page);
	for (const family of ['members', 'member-management'])
		for (const suffix of [
			'',
			'/notes',
			'/orders',
			'/orders/export',
			'/emails',
			'/memberships',
			'/activity',
			'/timeline'
		])
			expect((await context.request.get(`/api/admin/${family}/${primary}${suffix}`)).status()).toBe(
				401
			);
	const login = await context.request.post('/api/auth/login', {
		data: { email: 'user@admin-completion.test', password }
	});
	expect(login.status()).toBe(200);
	const caps = object(await (await context.request.get('/api/admin/crm/permissions')).json());
	expect(caps).toMatchObject({ global_admin: false, manager_id: null, permissions: [] });
	for (const family of ['members', 'member-management'])
		for (const suffix of [
			'',
			'/notes',
			'/orders',
			'/orders/export',
			'/emails',
			'/memberships',
			'/activity',
			'/timeline'
		])
			expect((await context.request.get(`/api/admin/${family}/${primary}${suffix}`)).status()).toBe(
				403
			);
	await page.goto(`/admin/members/${primary}`);
	await expect(page).toHaveURL(/admin_required/);
	expect(errors).toEqual([]);
});
