import { expect, test, type Page } from '@playwright/test';
import { readFile } from 'node:fs/promises';

// Run only against the isolated verification stack described in docs/admin.
// These are disposable fixture accounts, never production credentials.
const fixturePassword = 'OnlyForDisposableTest123!';
const browserFailures = new WeakMap<Page, string[]>();

test('CMS datasource create, edit, entries and CSV round-trip persist through its proxy', async ({
	page,
	request: outsider
}) => {
	expect((await outsider.get('/api/admin/cms/datasources')).status()).toBe(401);
	expect(
		(
			await outsider.post('/api/auth/login', {
				data: { email: 'user@admin-completion.test', password: fixturePassword }
			})
		).ok()
	).toBe(true);
	expect((await outsider.get('/api/admin/cms/datasources')).status()).toBe(403);

	expect(
		(
			await page.request.post('/api/auth/login', {
				data: { email: 'admin@admin-completion.test', password: fixturePassword }
			})
		).ok()
	).toBe(true);
	const title = `Browser datasource ${Date.now()}`;
	await page.goto('/admin/cms/datasources', { waitUntil: 'networkidle' });
	await page.getByRole('button', { name: 'New Datasource', exact: true }).click();
	await page.getByLabel('Name *', { exact: true }).fill(title);
	await page.getByLabel('Slug *', { exact: true }).fill(`browser-datasource-${Date.now()}`);
	const createdResponse = page.waitForResponse(
		(response) =>
			new URL(response.url()).pathname === '/api/admin/cms/datasources' &&
			response.request().method() === 'POST'
	);
	await page
		.getByRole('dialog')
		.getByRole('button', { name: 'Create Datasource', exact: true })
		.click();
	const created = await createdResponse;
	expect(created.status()).toBe(201);
	const saved = await page.request.get(
		`/api/admin/cms/datasources?search=${encodeURIComponent(title)}`
	);
	expect(saved.ok()).toBe(true);
	const id = (await saved.json()).data.find((row: { name: string }) => row.name === title)?.id;
	expect(id).toBeTruthy();
	const card = page
		.locator('.datasource-card')
		.filter({ has: page.getByRole('heading', { name: title, exact: true }) });
	await expect(card).toBeVisible();
	await card.getByTitle('Edit', { exact: true }).click();
	await page.getByLabel('Description', { exact: true }).fill('Persisted browser fixture');
	await page
		.getByRole('dialog')
		.getByRole('button', { name: 'Update Datasource', exact: true })
		.click();
	await expect(card).toContainText('Persisted browser fixture');
	await page.reload();
	await expect(card).toContainText('Persisted browser fixture');
	await card.getByTitle('Edit', { exact: true }).click();
	await page.getByLabel('Description', { exact: true }).fill('');
	await page
		.getByRole('dialog')
		.getByRole('button', { name: 'Update Datasource', exact: true })
		.click();
	await expect(card).not.toContainText('Persisted browser fixture');
	await page.reload();
	await expect(card).not.toContainText('Persisted browser fixture');
	await card.getByRole('button', { name: 'Manage Entries', exact: true }).click();
	await page.getByRole('button', { name: 'Add Entry', exact: true }).first().click();
	await page.getByLabel('Name (Label) *', { exact: true }).fill('Alpha "quoted"');
	await page.getByLabel('Value *', { exact: true }).fill('alpha,one');
	await page.getByRole('dialog').getByRole('button', { name: 'Add Entry', exact: true }).click();
	await expect(page.getByRole('cell', { name: 'alpha,one', exact: true })).toBeVisible();
	const csv = await page.request.get(`/api/admin/cms/datasources/${id}/export`);
	expect(csv.ok()).toBe(true);
	expect(csv.headers()['content-type']).toContain('text/csv');
	expect(await csv.text()).toContain('"Alpha ""quoted""","alpha,one","default"');
	const downloadReady = page.waitForEvent('download');
	await page.getByRole('button', { name: 'Export CSV', exact: true }).click();
	const download = await downloadReady;
	expect(download.suggestedFilename()).toMatch(/-entries\.csv$/);
	const downloadPath = await download.path();
	if (!downloadPath) throw new Error('The CSV download was not saved');
	expect(await readFile(downloadPath, 'utf8')).toContain(
		'"Alpha ""quoted""","alpha,one","default"'
	);
	await page.getByRole('button', { name: 'Import CSV', exact: true }).click();
	await page.getByLabel('CSV File', { exact: true }).setInputFiles({
		name: 'fixture.csv',
		mimeType: 'text/csv',
		buffer: Buffer.from('name,value,dimension\nBeta,two,default\n')
	});
	await page.getByRole('dialog').getByRole('button', { name: 'Import', exact: true }).click();
	await expect(page.getByRole('cell', { name: 'Beta', exact: true })).toBeVisible();
	await page.getByRole('button', { name: 'Import CSV', exact: true }).click();
	await page.getByLabel('CSV File', { exact: true }).setInputFiles({
		name: 'update.csv',
		mimeType: 'text/csv',
		buffer: Buffer.from('name,value,dimension\nBeta updated,two,default\n')
	});
	await page.getByRole('dialog').getByRole('button', { name: 'Import', exact: true }).click();
	await expect(
		page.getByText('Import completed: 0 created, 1 updated, 0 skipped, 0 errors', { exact: true })
	).toBeVisible();
	await expect(page.getByRole('cell', { name: 'Beta updated', exact: true })).toBeVisible();
	await page.getByRole('button', { name: 'Import CSV', exact: true }).click();
	await page.getByLabel('CSV File', { exact: true }).setInputFiles({
		name: 'partial.csv',
		mimeType: 'text/csv',
		buffer: Buffer.from(
			`name,value,dimension\n${'x'.repeat(256)},bad,default\nGamma,three,default\n`
		)
	});
	await page.getByRole('dialog').getByRole('button', { name: 'Import', exact: true }).click();
	await expect(
		page.getByText('Import completed: 1 created, 0 updated, 0 skipped, 1 errors', { exact: true })
	).toBeVisible();
	await expect(page.getByRole('cell', { name: 'Gamma', exact: true })).toBeVisible();
	await page.getByRole('button', { name: 'Import CSV', exact: true }).click();
	await page.getByLabel('CSV File', { exact: true }).setInputFiles({
		name: 'translation.csv',
		mimeType: 'text/csv',
		buffer: Buffer.from('name,value,dimension\nFrançais,bonjour,fr\n')
	});
	await page.getByRole('dialog').getByRole('button', { name: 'Import', exact: true }).click();
	const dimension = page.locator('select').filter({ has: page.locator('option[value="fr"]') });
	await expect(dimension).toHaveCount(1);
	await dimension.selectOption('fr');
	await expect(page.getByRole('cell', { name: 'bonjour', exact: true })).toBeVisible();
	await dimension.selectOption('default');
	await expect(page.getByRole('cell', { name: 'Gamma', exact: true })).toBeVisible();
	const rows = page.locator('tbody tr');
	const alpha = rows.filter({ hasText: 'alpha,one' });
	const reorderReady = page.waitForResponse((response) =>
		new URL(response.url()).pathname.endsWith('/entries/reorder')
	);
	await rows.filter({ hasText: 'Gamma' }).dragTo(alpha);
	expect((await reorderReady).ok()).toBe(true);
	await expect(rows.first()).toContainText('Gamma');
	await page.reload();
	await card.getByRole('button', { name: 'Manage Entries', exact: true }).click();
	await expect(page.getByRole('cell', { name: 'Beta updated', exact: true })).toBeVisible();
	await expect(page.getByRole('cell', { name: 'Gamma', exact: true })).toBeVisible();
	await expect(page.getByRole('cell', { name: 'alpha,one', exact: true })).toBeVisible();
	await expect(rows.first()).toContainText('Gamma');
	await rows.filter({ hasText: 'Beta updated' }).getByTitle('Edit', { exact: true }).click();
	await page.getByLabel('Name (Label) *', { exact: true }).fill('Beta edited');
	await page.getByRole('dialog').getByRole('button', { name: 'Update Entry', exact: true }).click();
	await expect(page.getByRole('cell', { name: 'Beta edited', exact: true })).toBeVisible();
	await alpha.getByTitle('Delete', { exact: true }).click();
	await page.getByRole('dialog').getByRole('button', { name: 'Delete', exact: true }).click();
	await expect(alpha).toHaveCount(0);
	await page.getByRole('button', { name: 'Back to Datasources', exact: true }).click();
	await card.getByTitle('Duplicate', { exact: true }).click();
	const copy = page
		.locator('.datasource-card')
		.filter({ has: page.getByRole('heading', { name: `${title} (Copy)`, exact: true }) });
	await expect(copy).toBeVisible();
	await copy.getByRole('button', { name: 'Manage Entries', exact: true }).click();
	await expect(rows).toHaveCount(2);
	await expect(page.getByRole('cell', { name: 'Beta edited', exact: true })).toBeVisible();
	await expect(page.getByRole('cell', { name: 'Gamma', exact: true })).toBeVisible();
	await dimension.selectOption('fr');
	await expect(page.getByRole('cell', { name: 'bonjour', exact: true })).toBeVisible();
	await page.getByRole('button', { name: 'Back to Datasources', exact: true }).click();
	for (const item of [copy, card]) {
		await item.getByTitle('Delete', { exact: true }).click();
		await page.getByRole('dialog').getByRole('button', { name: 'Delete', exact: true }).click();
		await expect(item).toHaveCount(0);
	}
	await page.reload();
	await expect(card).toHaveCount(0);
	await expect(copy).toHaveCount(0);
});

test('canonical admin aliases redirect on the server and open their existing creator dialogs', async ({
	page
}) => {
	const login = await page.request.post('/api/auth/login', {
		data: { email: 'admin@admin-completion.test', password: fixturePassword }
	});
	expect(login.ok()).toBe(true);
	for (const [from, to, dialog] of [
		['/admin/dashboard', '/admin', null],
		['/admin/analytics/cohorts/create', '/admin/analytics/cohorts?create=1', 'Create Cohort'],
		['/admin/analytics/funnels/create', '/admin/analytics/funnels?create=1', 'Create Funnel']
	] as const) {
		const redirect = await page.request.get(from, { maxRedirects: 0 });
		expect(redirect.status()).toBe(307);
		expect(redirect.headers()['location']).toBe(to);
		await page.goto(from, { waitUntil: 'networkidle' });
		expect(new URL(page.url()).pathname + new URL(page.url()).search).toBe(to);
		if (dialog) await expect(page.getByRole('dialog', { name: dialog, exact: true })).toBeVisible();
	}
});

test('newsletter bulk snapshots appear in campaign preview, edited resume and duplication', async ({
	page
}) => {
	const login = await page.request.post('/api/auth/login', {
		data: { email: 'admin@admin-completion.test', password: fixturePassword }
	});
	expect(login.ok()).toBe(true);
	const diagnostics = await page.request.get('/api/admin/email/status');
	expect(diagnostics.ok()).toBe(true);
	expect(
		(await diagnostics.json()).postmark_token_set,
		'Never run this browser workflow with real provider credentials'
	).toBe(false);
	const access = (await page.context().cookies()).find(
		(cookie) => cookie.name === 'rtp_access_token'
	)?.value;
	expect(access).toBeTruthy();
	const queued = await page.request.post('http://127.0.0.1:8281/api/newsletter/bulk-send', {
		headers: { Authorization: `Bearer ${access}` },
		data: {
			subject: `Browser newsletter ${Date.now()}`,
			html_content: '<p>Browser newsletter HTML</p>',
			text_content: 'Browser newsletter text',
			segment: 'browser-bulk',
			rate_limit: 1
		}
	});
	expect(queued.status()).toBe(202);
	const job = (await queued.json()).job;
	expect(job.recipient_count).toBe(2);
	const path = `/admin/crm/campaigns/${job.campaign_id}`;
	await page.goto(path, { waitUntil: 'networkidle' });
	await expect(page.locator('.campaign-report header strong')).toHaveText('paused', {
		timeout: 20000
	});
	await expect(page.locator('.campaign-audience tbody tr')).toHaveCount(2);
	await expect(page.locator('.campaign-audience')).toContainText('browser-bulk-a@example.test');
	await expect(page.locator('.campaign-audience')).not.toContainText(
		'browser-bulk-pending@example.test'
	);
	await expect(page.getByLabel('Newsletter segment (all, confirmed, or tag)')).toHaveValue(
		'browser-bulk'
	);
	await page.getByRole('link', { name: 'Edit Campaign', exact: true }).click();
	await expect(page.getByLabel('Plain-text alternative (optional)')).toHaveValue(
		'Browser newsletter text'
	);
	await page.getByLabel('Plain-text alternative (optional)').fill('Edited browser newsletter text');
	await page.getByRole('button', { name: 'Save Campaign', exact: true }).click();
	await expect(page).toHaveURL(new RegExp(`${path}$`));
	await page.getByRole('button', { name: 'Resume Campaign', exact: true }).click();
	const resumeResponse = page.waitForResponse(
		(response) =>
			new URL(response.url()).pathname === `/api/admin/crm/campaigns/${job.campaign_id}/resume`
	);
	await page.getByRole('dialog').getByRole('button', { name: 'Resume', exact: true }).click();
	expect((await resumeResponse).ok()).toBe(true);
	await expect
		.poll(
			async () =>
				(await (await page.request.get(`/api/admin/crm/campaigns/${job.campaign_id}`)).json())
					.status,
			{ timeout: 20000 }
		)
		.toBe('paused');
	await expect(page.locator('.campaign-report header strong')).toHaveText('paused', {
		timeout: 20000
	});
	await page.reload();
	await expect(page.locator('.campaign-audience tbody tr')).toHaveCount(2);
	await page.getByRole('button', { name: 'Duplicate Campaign', exact: true }).click();
	await expect(page).not.toHaveURL(new RegExp(`${path}$`));
	await expect(page).toHaveURL(/\/admin\/crm\/campaigns\/\d+\/edit$/);
	const duplicate = page.url().split('/').at(-2);
	expect(duplicate).not.toBe(job.campaign_id);
	await expect(page.getByLabel('Plain-text alternative (optional)')).toHaveValue(
		'Edited browser newsletter text'
	);
	const detail = await page.request.get(`/api/admin/crm/campaigns/${job.campaign_id}/deliveries`);
	expect(detail.ok()).toBe(true);
	for (const row of (await detail.json()).data) {
		expect(row.attempt_count).toBe(0);
		expect(row.provider_message_id).toBeNull();
	}
	for (const id of [duplicate, job.campaign_id]) {
		const deleted = await page.request.delete(`/api/admin/crm/campaigns/${id}`);
		expect(deleted.ok(), `${id}: ${deleted.status()} ${await deleted.text()}`).toBe(true);
	}
	await page.goto('/admin/crm/logs', { waitUntil: 'networkidle' });
	const search = page.getByRole('textbox', { name: 'Search logs', exact: true });
	await search.fill('Newsletter broadcast queued');
	await search.press('Tab');
	const persistedLogs = await page.request.get(
		'/api/admin/crm/logs?search=Newsletter%20broadcast%20queued&per_page=200'
	);
	expect(persistedLogs.ok()).toBe(true);
	const savedLog = (await persistedLogs.json()).data.find(
		(row: { context: { campaign_id: number } }) =>
			row.context.campaign_id === Number(job.campaign_id)
	);
	expect(savedLog).toBeTruthy();
	const entry = page.locator(`[data-log-id="${savedLog.id}"]`);
	await expect(entry).toContainText('Newsletter broadcast queued');
	await entry.getByText('Context', { exact: true }).click();
	await expect(entry.locator('pre')).toContainText(`"recipient_count": 2`);
	await page.reload();
	await expect(entry).toContainText('Newsletter broadcast queued');
	await search.fill('Newsletter broadcast queued');
	await search.press('Tab');
	await page.getByRole('button', { name: 'Clear Logs', exact: true }).click();
	await page.getByRole('dialog').getByRole('button', { name: 'Clear Logs', exact: true }).click();
	await expect(page.getByRole('heading', { name: 'No logs found', exact: true })).toBeVisible();
});

test('campaign scheduling, pause, edited resume and unavailable-provider delivery persist', async ({
	page
}) => {
	const login = await page.request.post('/api/auth/login', {
		data: { email: 'admin@admin-completion.test', password: fixturePassword }
	});
	expect(login.ok()).toBe(true);
	const provider = await page.request.get('/api/admin/email/status');
	expect(provider.ok()).toBe(true);
	expect(
		(await provider.json()).postmark_token_set,
		'Never run the browser send workflow with a real provider token'
	).toBe(false);
	const suffix = Date.now().toString();
	const make = async (name: string) => {
		const created = await page.request.post('/api/admin/crm/campaigns', {
			data: {
				title: name,
				email_subject: name,
				email_body: '<p>Local queue fixture</p>',
				subscriber_settings: {
					advanced_filters: [
						{
							field: 'email',
							operator: 'equals',
							value: 'browser-campaign-confirmed-a@example.test'
						}
					]
				}
			}
		});
		expect(created.ok()).toBe(true);
		return (await created.json()).id as string;
	};
	const scheduledId = await make(`Browser scheduled ${suffix}`);
	await page.goto(`/admin/crm/campaigns/${scheduledId}`, { waitUntil: 'networkidle' });
	await page.getByLabel('Schedule in your local time').fill('2030-01-01T10:00');
	await page.getByRole('button', { name: 'Schedule Campaign', exact: true }).click();
	await page.getByRole('dialog').getByRole('button', { name: 'Schedule', exact: true }).click();
	await expect(page.locator('.campaign-report header strong')).toHaveText('scheduled');
	await page.reload();
	await expect(page.locator('.campaign-report header strong')).toHaveText('scheduled');
	await expect(page.locator('.campaign-delivery tbody tr')).toHaveCount(1);
	await expect(page.locator('.campaign-delivery tbody')).toContainText('queued');
	await page.getByRole('button', { name: 'Pause Campaign', exact: true }).click();
	await expect(page.locator('.campaign-report header strong')).toHaveText('paused');
	await page.getByRole('link', { name: 'Edit Campaign', exact: true }).click();
	await page.getByLabel('Email HTML', { exact: true }).fill('<p>Edited while paused</p>');
	await page.getByRole('button', { name: 'Save Campaign', exact: true }).click();
	await expect(page).toHaveURL(new RegExp(`/admin/crm/campaigns/${scheduledId}$`));
	await page.getByRole('button', { name: 'Resume Campaign', exact: true }).click();
	await page.getByRole('dialog').getByRole('button', { name: 'Resume', exact: true }).click();
	await expect(page.locator('.campaign-report header strong')).toHaveText('scheduled');
	await page.getByRole('button', { name: 'Pause Campaign', exact: true }).click();
	await expect(page.locator('.campaign-report header strong')).toHaveText('paused');
	const dueId = await make(`Browser unavailable ${suffix}`);
	await page.goto(`/admin/crm/campaigns/${dueId}`, { waitUntil: 'networkidle' });
	await page.getByRole('button', { name: 'Send Now', exact: true }).click();
	await page.getByRole('dialog').getByRole('button', { name: 'Send', exact: true }).click();
	await expect(page.locator('.campaign-report header strong')).toHaveText('paused', {
		timeout: 20000
	});
	await expect(
		page.getByText('The email provider is unavailable.', { exact: false })
	).toBeVisible();
	await page.reload();
	await expect(page.locator('.campaign-report header strong')).toHaveText('paused');
	await expect(page.locator('.campaign-delivery tbody')).toContainText('queued');
	const receipt = await page.request.get(`/api/admin/crm/campaigns/${dueId}/deliveries`);
	expect(receipt.ok()).toBe(true);
	const row = (await receipt.json()).data[0];
	expect(row.attempt_count).toBe(0);
	expect(row.provider_message_id).toBeNull();
	for (const id of [scheduledId, dueId])
		expect((await page.request.delete(`/api/admin/crm/campaigns/${id}`)).ok()).toBe(true);
});

test.beforeEach(async ({ page, baseURL }) => {
	expect(baseURL, 'Use the isolated verification frontend on port 5274').toBe(
		'http://127.0.0.1:5274'
	);
	const failures: string[] = [];
	browserFailures.set(page, failures);
	page.on('pageerror', (error) => failures.push(error.message));
	page.on('response', (response) => {
		if (new URL(response.url()).pathname.startsWith('/api/') && response.status() >= 400)
			failures.push(`${response.status()} ${new URL(response.url()).pathname}`);
	});
	page.on('requestfailed', (request) => {
		if (
			new URL(request.url()).pathname.startsWith('/api/') &&
			!request.failure()?.errorText.includes('ERR_ABORTED')
		)
			failures.push(`${request.failure()?.errorText} ${new URL(request.url()).pathname}`);
	});
	page.on('console', (message) => {
		if (message.type() !== 'error') return;
		const origin = message.location().url;
		// Third-party requests are intentionally blocked by this local-only harness.
		if (origin && !['127.0.0.1', 'localhost'].includes(new URL(origin).hostname)) return;
		if (!origin && message.text().startsWith('Failed to load resource')) return;
		failures.push(message.text());
	});
	// Keep this verification local, including third-party scripts/assets.
	await page.route('**/*', (route) => {
		const url = new URL(route.request().url());
		return ['127.0.0.1', 'localhost'].includes(url.hostname) ||
			!['http:', 'https:'].includes(url.protocol)
			? route.continue()
			: route.abort();
	});
});

test.afterEach(async ({ page }) => {
	expect(
		browserFailures.get(page) ?? [],
		'No unexpected application console or API failures'
	).toEqual([]);
});

test('deal create, linked records, cents, edit, notes, outcomes and delete persist through the UI', async ({
	page
}) => {
	const login = await page.request.post('/api/auth/login', {
		data: { email: 'admin@admin-completion.test', password: fixturePassword }
	});
	expect(login.ok()).toBe(true);
	const suffix = Date.now().toString();
	const createdPipeline = await page.request.post('/api/admin/crm/pipelines', {
		data: { name: `Browser deals ${suffix}` }
	});
	expect(createdPipeline.ok()).toBe(true);
	const pipeline = await createdPipeline.json();
	for (const [position, name, probability, won, lost] of [
		[0, 'Open', 20, false, false],
		[1, 'Qualified', 60, false, false],
		[2, 'Won', 100, true, false],
		[3, 'Lost', 0, false, true]
	] as const) {
		const stage = await page.request.post(`/api/admin/crm/pipelines/${pipeline.id}/stages`, {
			data: { name, position, probability, is_closed_won: won, is_closed_lost: lost }
		});
		expect(stage.ok()).toBe(true);
	}
	const contactName = `Deal contact ${suffix}`,
		companyName = `Deal company ${suffix}`,
		dealName = `Browser deal ${suffix}`;
	const createdContact = await page.request.post('/api/admin/crm/contacts', {
		data: { first_name: contactName, email: `deal-${suffix}@example.test` }
	});
	expect(createdContact.ok()).toBe(true);
	const contact = (await createdContact.json()).data;
	const createdCompany = await page.request.post('/api/admin/crm/companies', {
		data: { name: companyName }
	});
	expect(createdCompany.ok()).toBe(true);
	const company = await createdCompany.json();
	await page.goto('/admin/crm/deals/new', { waitUntil: 'networkidle' });
	await page.getByLabel('Deal name', { exact: true }).fill(dealName);
	await page.getByLabel('Amount', { exact: true }).fill('123.45');
	await page.getByLabel('Currency', { exact: true }).fill('EUR');
	await page.getByLabel('Pipeline', { exact: true }).selectOption(pipeline.id);
	await page.getByLabel('Expected close date').fill('2026-12-12');
	await page.getByLabel('Find contact').fill(contactName);
	await page.getByRole('button', { name: 'Search contacts', exact: true }).click();
	await page.getByRole('button', { name: new RegExp(contactName) }).click();
	await page.getByLabel('Find company').fill(companyName);
	await page.getByRole('button', { name: 'Search companies', exact: true }).click();
	await page.getByRole('button', { name: companyName, exact: true }).click();
	await page.getByLabel('Tags, separated by commas').fill('browser, exact-cents');
	await page.getByLabel('Custom fields (JSON object)').fill('{"source":"browser fixture"}');
	expect(
		await page.locator('.deal-editor form').evaluate((form) =>
			Array.from(form.querySelectorAll('input,select,textarea'))
				.filter((input) => !(input as HTMLInputElement).checkValidity())
				.map((input) => ({ id: input.id, message: (input as HTMLInputElement).validationMessage }))
		)
	).toEqual([]);
	await page.getByRole('button', { name: 'Save Deal', exact: true }).click();
	await expect(page).toHaveURL(/\/admin\/crm\/deals\/\d+$/);
	const detailPath = new URL(page.url()).pathname;
	await expect(page.getByRole('heading', { name: dealName, exact: true })).toBeVisible();
	await expect(page.locator('.value-amount')).toHaveText('€123.45');
	await page.reload();
	await expect(page.locator('.value-amount')).toHaveText('€123.45');
	await page.getByRole('button', { name: /^Notes \(/ }).click();
	await page.getByRole('button', { name: 'Add Note', exact: true }).click();
	await page.getByLabel('Note Content').fill('Persisted browser note <script>plain text</script>');
	await page.getByRole('button', { name: 'Save Note', exact: true }).click();
	await expect(
		page
			.locator('.notes-section')
			.getByText('Persisted browser note <script>plain text</script>', { exact: true })
	).toBeVisible();
	await page.reload();
	await page.getByRole('button', { name: /^Notes \(/ }).click();
	await expect(
		page
			.locator('.notes-section')
			.getByText('Persisted browser note <script>plain text</script>', { exact: true })
	).toBeVisible();
	await page.getByRole('link', { name: 'Edit Deal', exact: true }).click();
	await expect(page.getByLabel('Amount', { exact: true })).toHaveValue('123.45');
	await page.getByLabel('Amount', { exact: true }).fill('100.01');
	await page.getByLabel('Expected close date').fill('');
	await page.getByRole('button', { name: 'Clear contact', exact: true }).click();
	await page.getByRole('button', { name: 'Clear company', exact: true }).click();
	await page.getByRole('button', { name: 'Save Deal', exact: true }).click();
	await expect(page.locator('.value-amount')).toHaveText('€100.01');
	await page.getByRole('button', { name: 'Mark Won', exact: true }).click();
	await page.getByLabel('Win Details (optional)').fill('Browser accepted offer');
	await page.getByRole('button', { name: 'Mark as Won', exact: true }).click();
	await expect(page.locator('.status-badge')).toHaveText('Won');
	await page.getByRole('link', { name: 'Edit Deal', exact: true }).click();
	await expect(page.getByLabel('Expected close date')).toHaveValue('');
	await expect(page.getByText('No linked contact', { exact: true })).toBeVisible();
	await expect(page.getByText('No linked company', { exact: true })).toBeVisible();
	await page.getByLabel('Stage', { exact: true }).selectOption({ label: 'Open' });
	await page.getByRole('button', { name: 'Save Deal', exact: true }).click();
	await expect(page.locator('.status-badge')).toHaveText('Open');
	await page.getByRole('button', { name: 'Mark Lost', exact: true }).click();
	await page.getByLabel('Lost Reason').fill('Browser budget unavailable');
	await page.getByRole('button', { name: 'Mark as Lost', exact: true }).click();
	await expect(page.locator('.status-badge')).toHaveText('Lost');
	await page.getByRole('button', { name: /^Timeline \(/ }).click();
	await expect(page.getByText('Stage changed', { exact: true }).first()).toBeVisible();
	await page.goto('/admin/crm/deals', { waitUntil: 'networkidle' });
	await page.locator('#deals-selectedpipeline').selectOption({ label: `Browser deals ${suffix}` });
	await page.getByLabel('Currency filter').fill('EUR');
	await page.getByLabel('Currency filter').press('Tab');
	await page.locator('#deals-selectedstatus').selectOption('lost');
	await page.locator('#search-deals').fill(dealName);
	await page.getByRole('button', { name: 'Search', exact: true }).click();
	await expect(page.getByRole('link', { name: dealName, exact: true })).toBeVisible();
	await page.reload();
	await page.goto(detailPath, { waitUntil: 'networkidle' });
	await page.getByRole('button', { name: 'Delete', exact: true }).click();
	await page.getByRole('dialog').getByRole('button', { name: 'Delete', exact: true }).click();
	await expect(page).toHaveURL(/\/admin\/crm\/deals$/);
	expect((await page.request.delete(`/api/admin/crm/contacts/${contact.id}`)).ok()).toBe(true);
	expect((await page.request.delete(`/api/admin/crm/companies/${company.id}`)).ok()).toBe(true);
	expect((await page.request.delete(`/api/admin/crm/pipelines/${pipeline.id}`)).ok()).toBe(true);
});

test('campaign authoring, preview, full duplication and deletion persist through the UI', async ({
	page
}) => {
	const login = await page.request.post('/api/auth/login', {
		data: { email: 'admin@admin-completion.test', password: fixturePassword }
	});
	expect(login.ok()).toBe(true);
	const title = `Browser campaign ${Date.now()}`;
	const contactsResponse = await page.request.get(
		'/api/admin/crm/contacts?search=browser-campaign-'
	);
	expect(contactsResponse.ok()).toBe(true);
	const recipients = (await contactsResponse.json()).data as Array<{
		id: string | number;
		source: string;
	}>;
	const fixtureIds = recipients
		.filter((row) => row.source === 'admin-move-campaign-browser')
		.map((row) => row.id);
	expect(
		fixtureIds,
		'Seed api/tests/fixtures/campaign_browser.sql in the isolated task DB'
	).toHaveLength(3);
	const audienceTitle = `Browser campaign audience ${Date.now()}`;
	const listResponse = await page.request.post('/api/admin/crm/lists', {
		data: { title: audienceTitle }
	});
	expect(listResponse.ok()).toBe(true);
	const audienceList = await listResponse.json();
	expect(
		(
			await page.request.post(`/api/admin/crm/lists/${audienceList.id}/contacts`, {
				data: { contact_ids: fixtureIds }
			})
		).ok()
	).toBe(true);

	await page.goto('/admin/crm/campaigns/new', { waitUntil: 'networkidle' });
	await page.getByLabel('Campaign title', { exact: true }).fill(title);
	await page.getByLabel('Email subject', { exact: true }).fill('Browser campaign subject');
	await page.getByLabel('Preheader', { exact: true }).fill('Browser campaign preheader');
	await page
		.getByLabel('Email HTML', { exact: true })
		.fill('<h1>Actual email content</h1><p>Durable browser body</p>');
	await page.getByLabel('Use a custom sender').check();
	await page.getByLabel('From name', { exact: true }).fill('Browser Sender');
	await page.getByLabel('From email', { exact: true }).fill('fixture-sender@example.test');
	await page.getByLabel('Reply-to email', { exact: true }).fill('fixture-replies@example.test');
	await page.getByText('Campaign link tracking', { exact: true }).click();
	await page.getByLabel('UTM source', { exact: true }).fill('browser-fixture');
	await page.getByRole('button', { name: 'Save Campaign', exact: true }).click();
	await expect(page).toHaveURL(/\/admin\/crm\/campaigns\/\d+$/);
	const reportPath = new URL(page.url()).pathname;
	await expect(page.getByRole('heading', { name: title, exact: true })).toBeVisible();
	await expect(
		page
			.frameLocator('iframe[title="Campaign email preview"]')
			.getByRole('heading', { name: 'Actual email content' })
	).toBeVisible();
	await page.reload();
	await expect(
		page
			.frameLocator('iframe[title="Campaign email preview"]')
			.getByText('Durable browser body', { exact: true })
	).toBeVisible();
	await page.getByLabel('Search lists', { exact: true }).fill(audienceTitle);
	await page.getByRole('button', { name: 'Find lists', exact: true }).click();
	await page.getByRole('button', { name: 'Add include group', exact: true }).click();
	await page
		.getByRole('group', { name: 'Include group 1', exact: true })
		.getByLabel('List', { exact: true })
		.selectOption({ label: audienceTitle });
	await page.getByRole('button', { name: 'Save Audience', exact: true }).click();
	await expect(
		page
			.locator('.campaign-audience')
			.getByText(
				'2 eligible recipients from 3 matching contacts. 1 excluded without an active confirmation.',
				{ exact: true }
			)
	).toBeVisible();
	await expect(page.locator('.campaign-audience table')).toContainText(
		'browser-campaign-confirmed-a@example.test'
	);
	await expect(page.locator('.campaign-audience table')).not.toContainText(
		'browser-campaign-pending@example.test'
	);
	await page.reload();
	await expect(
		page
			.getByRole('group', { name: 'Include group 1', exact: true })
			.getByLabel('List', { exact: true })
	).toHaveValue(String(audienceList.id));

	await page.getByRole('link', { name: 'Edit Campaign', exact: true }).click();
	await expect(page.getByLabel('Email HTML', { exact: true })).toHaveValue(
		'<h1>Actual email content</h1><p>Durable browser body</p>'
	);
	await expect(page.getByLabel('From email', { exact: true })).toHaveValue(
		'fixture-sender@example.test'
	);
	await page.getByText('Campaign link tracking', { exact: true }).click();
	await expect(page.getByLabel('UTM source', { exact: true })).toHaveValue('browser-fixture');
	await page.getByLabel('Email subject', { exact: true }).fill('Revised browser subject');
	await page.getByRole('button', { name: 'Save Campaign', exact: true }).click();
	await expect(page.getByText('Revised browser subject', { exact: true })).toBeVisible();
	await page.getByRole('button', { name: 'Duplicate Campaign', exact: true }).click();
	await expect(page).toHaveURL(/\/admin\/crm\/campaigns\/\d+\/edit$/);
	await expect(page.getByLabel('Campaign title', { exact: true })).toHaveValue(`${title} (Copy)`);
	await expect(page.getByLabel('Email HTML', { exact: true })).toHaveValue(
		'<h1>Actual email content</h1><p>Durable browser body</p>'
	);
	await expect(page.getByLabel('From email', { exact: true })).toHaveValue(
		'fixture-sender@example.test'
	);
	await page.getByRole('button', { name: 'Save Campaign', exact: true }).click();
	await expect(page.getByRole('heading', { name: `${title} (Copy)`, exact: true })).toBeVisible();
	await page.getByRole('button', { name: 'Delete Campaign', exact: true }).click();
	await page.getByRole('dialog').getByRole('button', { name: 'Delete', exact: true }).click();
	await expect(page).toHaveURL(/\/admin\/crm\/campaigns$/);
	await page.getByPlaceholder('Search campaigns...').fill(title);
	await page.getByRole('button', { name: 'Search campaigns', exact: true }).click();
	await expect(page.getByText(title, { exact: true })).toBeVisible();
	await page.goto(reportPath, { waitUntil: 'networkidle' });
	await page.getByRole('button', { name: 'Delete Campaign', exact: true }).click();
	await page.getByRole('dialog').getByRole('button', { name: 'Delete', exact: true }).click();
	await expect(page).toHaveURL(/\/admin\/crm\/campaigns$/);
	expect((await page.request.delete(`/api/admin/crm/lists/${audienceList.id}`)).ok()).toBe(true);
});

test('login, CRM save/reload, durable behavior dashboard, and logout', async ({ page }) => {
	const errors: string[] = [];
	const failedApi: string[] = [];
	page.on('pageerror', (error) => errors.push(error.message));
	page.on('response', (response) => {
		if (response.url().includes('/api/') && response.status() >= 400) {
			failedApi.push(`${response.status()} ${new URL(response.url()).pathname}`);
		}
	});
	await page.goto('/login?redirect=/admin/crm/settings', { waitUntil: 'networkidle' });
	await page.locator('input[name="email"]').fill('admin@admin-completion.test');
	await page.locator('input[name="password"]').fill(fixturePassword);
	await page.getByRole('button', { name: 'Sign In to Trade' }).click();
	await expect(page).not.toHaveURL(/\/login/);
	await page.goto('/admin/crm/settings');
	const subject = page.getByLabel('Confirmation Email Subject');
	await expect(subject).toBeVisible();
	await subject.fill('Browser persistence fixture');
	await page.getByRole('button', { name: 'Save Settings' }).click();
	await expect(page.getByText('Double opt-in settings saved successfully')).toBeVisible();
	await page.reload();
	await expect(subject).toHaveValue('Browser persistence fixture');
	const selector = `#browser-sdk-${Date.now()}`;
	const ingestion = page.waitForResponse(
		(response) => new URL(response.url()).pathname === '/api/behavior/events'
	);
	await page.evaluate(async (elementSelector) => {
		const modulePath = '/src/lib/behavior/tracker.ts';
		const { BehaviorTracker } = await import(modulePath);
		const tracker = new BehaviorTracker({
			trackScrollDepth: false,
			trackHoverIntent: false,
			trackFormBehavior: false,
			trackIdleTime: false,
			bufferSize: 100
		});
		for (let i = 0; i < 20; i++)
			tracker.trackEvent({
				event_type: 'click',
				timestamp: Date.now() + i,
				page_url: location.href,
				element_selector: elementSelector
			});
		await tracker.flush();
		tracker.destroy({ flush: false });
	}, selector);
	const receipt = await ingestion;
	expect(receipt.ok()).toBe(true);
	expect(await receipt.json()).toMatchObject({ success: true, received: 21, persisted: 21 });
	await page.goto('/admin/behavior');
	await expect(
		page.locator('.admin-behavior').getByRole('heading', { name: 'Behavior Tracking', exact: true })
	).toBeVisible();
	await expect(page.getByText(selector, { exact: true })).toBeVisible();
	await page.reload();
	await expect(page.getByText(selector, { exact: true })).toBeVisible();
	await expect(page.getByText('#subscribe', { exact: true })).toBeVisible();
	await expect(page.getByText('75%', { exact: true }).first()).toBeVisible();
	await page.screenshot({ path: 'test-results/admin-behavior.png', fullPage: true });
	const cookies = await page.context().cookies();
	const accessToken = cookies.find((cookie) => cookie.name === 'rtp_access_token')?.value;
	const refreshToken = cookies.find((cookie) => cookie.name === 'rtp_refresh_token')?.value;
	expect(accessToken).toBeTruthy();
	expect(refreshToken).toBeTruthy();
	const logout = await page.request.post('/api/logout');
	expect(logout.ok()).toBe(true);
	expect((await logout.json()).server_revoked).toBe(true);
	const revoked = await page.request.get('http://127.0.0.1:8281/api/auth/me', {
		headers: { Authorization: `Bearer ${accessToken}` }
	});
	expect(revoked.status()).toBe(401);
	const replay = await page.request.post('http://127.0.0.1:8281/api/auth/refresh', {
		data: { refresh_token: refreshToken }
	});
	expect(replay.status()).toBe(401);
	await page.goto('/admin/crm/settings');
	await expect(page).toHaveURL(/\/login/);
	expect(errors).toEqual([]);
	expect(failedApi).toEqual([]);
});

test('anonymous and authenticated non-admin users cannot open admin pages', async ({ page }) => {
	await page.goto('/admin/crm/settings');
	await expect(page).toHaveURL(/\/login/);
	const login = await page.request.post('/api/auth/login', {
		data: { email: 'user@admin-completion.test', password: fixturePassword }
	});
	expect(login.ok()).toBe(true);
	await page.goto('/admin/crm/settings');
	await expect(page).toHaveURL(/admin_required/);
});

test('company create, edit, reload, view, duplicate and delete persist through the CRM UI', async ({
	page
}) => {
	const login = await page.request.post('/api/auth/login', {
		data: { email: 'admin@admin-completion.test', password: fixturePassword }
	});
	expect(login.ok()).toBe(true);
	await page.goto('/admin/crm/companies/new', { waitUntil: 'networkidle' });
	const companyName = `Browser Company ${Date.now()}`;
	await page.getByLabel('Company name', { exact: true }).fill(companyName);
	await page.getByLabel('Website', { exact: true }).fill('https://example.test');
	await page.getByLabel('Industry', { exact: true }).selectOption('finance');
	await page.getByLabel('Company size', { exact: true }).selectOption('11-50');
	await page
		.getByLabel('Description', { exact: true })
		.fill('Created by the isolated browser fixture.');
	await page.getByRole('button', { name: 'Save Company', exact: true }).click();
	await expect(page).toHaveURL(/\/admin\/crm\/companies$/);
	let row = page.getByRole('row').filter({ hasText: companyName });
	await expect(row).toBeVisible();
	await row.getByRole('link', { name: 'Edit', exact: true }).click();
	await expect(page.getByLabel('Company name', { exact: true })).toHaveValue(companyName);
	await page
		.getByLabel('Description', { exact: true })
		.fill('Edited and persisted browser fixture.');
	await page.getByRole('button', { name: 'Save Company', exact: true }).click();
	await expect(page).toHaveURL(/\/admin\/crm\/companies$/);
	row = page.getByRole('row').filter({ hasText: companyName });
	await row.getByRole('link', { name: 'View', exact: true }).click();
	await expect(
		page.getByText('Edited and persisted browser fixture.', { exact: true })
	).toBeVisible();
	await page.reload();
	await expect(
		page.getByText('Edited and persisted browser fixture.', { exact: true })
	).toBeVisible();
	await page.getByRole('link', { name: '← Companies', exact: true }).click();
	row = page.getByRole('row').filter({ hasText: companyName });
	await row.getByRole('button', { name: 'Duplicate', exact: true }).click();
	const copy = page.getByRole('row').filter({ hasText: `${companyName} (copy)` });
	await expect(copy).toBeVisible();
	await copy.getByRole('button', { name: 'Delete', exact: true }).click();
	await page.getByRole('button', { name: 'Delete', exact: true }).last().click();
	await expect(copy).toHaveCount(0);
});

test('contact lists save edits and persist membership without deleting contacts', async ({
	page
}) => {
	const login = await page.request.post('/api/auth/login', {
		data: { email: 'admin@admin-completion.test', password: fixturePassword }
	});
	expect(login.ok()).toBe(true);
	const email = `list-browser-${Date.now()}@completion.test`;
	const created = await page.request.post('/api/admin/crm/contacts', {
		data: { email, first_name: 'List browser fixture' }
	});
	expect(created.ok()).toBe(true);
	const contactResponse = (await created.json()) as {
		id?: string | number;
		data?: { id: string | number };
	};
	const contactId = contactResponse.data?.id ?? contactResponse.id;
	expect(contactId).toBeDefined();
	const title = `Browser List ${Date.now()}`;
	await page.goto('/admin/crm/lists/new', { waitUntil: 'networkidle' });
	await page.getByLabel('List title', { exact: true }).fill(title);
	await page.getByLabel('Description', { exact: true }).fill('List description fixture');
	await page.getByLabel('Public list', { exact: true }).check();
	await page.getByRole('button', { name: 'Save List', exact: true }).click();
	await expect(page).toHaveURL(/\/admin\/crm\/lists$/);
	let row = page.getByRole('row').filter({ hasText: title });
	await expect(row).toBeVisible();
	await row.getByRole('link', { name: 'Edit', exact: true }).click();
	await expect(page.getByLabel('Public list', { exact: true })).toBeChecked();
	await page.getByLabel('List title', { exact: true }).fill(`${title} Renamed`);
	await page.getByRole('button', { name: 'Save List', exact: true }).click();
	await expect(page).toHaveURL(/\/admin\/crm\/lists$/);
	row = page.getByRole('row').filter({ hasText: `${title} Renamed` });
	await row.getByRole('link', { name: 'View Contacts', exact: true }).click();
	await page.getByLabel('Contact IDs', { exact: true }).fill(String(contactId));
	await page.getByRole('button', { name: 'Add Contacts', exact: true }).click();
	await expect(page.getByRole('link', { name: email, exact: true })).toBeVisible();
	await page.reload();
	await expect(page.getByRole('link', { name: email, exact: true })).toBeVisible();
	await page.getByRole('checkbox', { name: `Select ${email}`, exact: true }).check();
	await page.getByRole('button', { name: 'Remove Selected From List', exact: true }).click();
	await expect(page.getByRole('link', { name: email, exact: true })).toHaveCount(0);
	const preserved = await page.request.get(`/api/admin/crm/contacts/${contactId}`);
	expect(preserved.ok()).toBe(true);
});

test('email templates save HTML and configuration, reload, preview, duplicate and delete', async ({
	page
}) => {
	const errors: string[] = [];
	page.on('pageerror', (error) => errors.push(error.message));
	const login = await page.request.post('/api/auth/login', {
		data: { email: 'admin@admin-completion.test', password: fixturePassword }
	});
	expect(login.ok()).toBe(true);
	const title = `Browser Template ${Date.now()}`;
	await page.goto('/admin/crm/templates/new', { waitUntil: 'networkidle' });
	await page.getByLabel('Template title', { exact: true }).fill(title);
	await page.getByLabel('Email subject', { exact: true }).fill('Browser subject');
	await page.getByLabel('Category', { exact: true }).fill('Browser category');
	await page.getByLabel('Email HTML', { exact: true }).fill('<h2>Original fixture HTML</h2>');
	await page
		.getByLabel('Template configuration (JSON)', { exact: true })
		.fill('{"theme":"fixture"}');
	await page.getByRole('checkbox', { name: 'Default template', exact: true }).check();
	await page.getByRole('button', { name: 'Save Template', exact: true }).click();
	await expect(page).toHaveURL(/\/admin\/crm\/templates$/);
	let card = page.getByRole('listitem', { name: title, exact: true });
	await expect(card).toBeVisible();
	await card.getByRole('link', { name: `Edit ${title}`, exact: true }).click();
	await expect(page.getByLabel('Email HTML', { exact: true })).toHaveValue(
		'<h2>Original fixture HTML</h2>'
	);
	await expect(page.getByRole('checkbox', { name: 'Default template', exact: true })).toBeChecked();
	await page.getByLabel('Email HTML', { exact: true }).fill('<h2>Persisted browser HTML</h2>');
	await page.getByRole('button', { name: 'Save Template', exact: true }).click();
	await expect(page).toHaveURL(/\/admin\/crm\/templates$/);
	await page.reload();
	card = page.getByRole('listitem', { name: title, exact: true });
	await card.getByRole('button', { name: `Preview ${title}`, exact: true }).click();
	await expect(
		page.getByRole('heading', { name: 'Persisted browser HTML', exact: true })
	).toBeVisible();
	await page.keyboard.press('Escape');
	await card.getByRole('button', { name: `Duplicate ${title}`, exact: true }).click();
	const copy = page.getByRole('listitem', { name: `${title} (copy)`, exact: true });
	await expect(copy).toBeVisible();
	await copy.getByRole('button', { name: `Delete ${title} (copy)`, exact: true }).click();
	await page.getByRole('button', { name: 'Delete', exact: true }).last().click();
	await expect(copy).toHaveCount(0);
	expect(errors).toEqual([]);
});

test('contact create/edit and lead edit persist without an external CRM connection', async ({
	page
}) => {
	const errors: string[] = [];
	page.on('pageerror', (error) => errors.push(error.message));
	const login = await page.request.post('/api/auth/login', {
		data: { email: 'admin@admin-completion.test', password: fixturePassword }
	});
	expect(login.ok()).toBe(true);
	const email = `contact-browser-${Date.now()}@completion.test`;
	await page.goto('/admin/crm/contacts/new', { waitUntil: 'networkidle' });
	await page.getByLabel('Email address', { exact: true }).fill(email);
	await page.getByLabel('First name', { exact: true }).fill('Browser contact');
	await page.getByLabel('Company', { exact: true }).fill('Original company');
	const saved = page.waitForResponse(
		(response) =>
			response.request().method() === 'POST' &&
			new URL(response.url()).pathname === '/api/admin/crm/contacts'
	);
	await page.getByRole('button', { name: 'Save Contact', exact: true }).click();
	const response = await saved;
	expect(response.ok()).toBe(true);
	const record = (await response.json()) as {
		id?: string | number;
		data?: { id: string | number };
	};
	const contactId = record.data?.id ?? record.id;
	expect(contactId).toBeDefined();
	await expect(page).toHaveURL(/\/admin\/crm$/);
	await expect(page.getByText(email, { exact: true })).toBeVisible();
	await page.goto(`/admin/crm/contacts/${contactId}/edit`, { waitUntil: 'networkidle' });
	await expect(page.getByLabel('Company', { exact: true })).toHaveValue('Original company');
	await page.getByLabel('Company', { exact: true }).fill('Persisted company');
	await page.getByLabel('Status', { exact: true }).selectOption('prospect');
	await page.getByRole('button', { name: 'Save Contact', exact: true }).click();
	await expect(page).toHaveURL(/\/admin\/crm$/);
	await page.goto(`/admin/crm/contacts/${contactId}/edit`);
	await page.reload();
	await expect(page.getByLabel('Company', { exact: true })).toHaveValue('Persisted company');
	await expect(page.getByLabel('Status', { exact: true })).toHaveValue('prospect');
	const lead = await page.request.post('/api/admin/crm/leads', {
		data: {
			email: `lead-browser-${Date.now()}@completion.test`,
			first_name: 'Browser lead',
			company_name: 'Lead company',
			status: 'new'
		}
	});
	expect(lead.ok()).toBe(true);
	const leadRecord = (await lead.json()) as { id: string };
	await page.goto(`/admin/crm/leads/${leadRecord.id}/edit`, { waitUntil: 'networkidle' });
	await expect(page.getByLabel('Company', { exact: true })).toHaveValue('Lead company');
	await page.getByLabel('Status', { exact: true }).selectOption('qualified');
	await page.getByRole('button', { name: 'Save Contact', exact: true }).click();
	await expect(page).toHaveURL(/\/admin\/crm\/leads$/);
	await page.goto(`/admin/crm/leads/${leadRecord.id}/edit`);
	await page.reload();
	await expect(page.getByLabel('Status', { exact: true })).toHaveValue('qualified');
	expect(errors).toEqual([]);
});

test('dynamic segments save rules, reload, and retrieve live matching contacts', async ({
	page
}) => {
	const login = await page.request.post('/api/auth/login', {
		data: { email: 'admin@admin-completion.test', password: fixturePassword }
	});
	expect(login.ok()).toBe(true);
	const fixture = async (suffix: string) => {
		const email = `segment-browser-${suffix}-${Date.now()}@completion.test`;
		const response = await page.request.post('/api/admin/crm/contacts', {
			data: { email, first_name: 'Segment fixture' }
		});
		expect(response.ok()).toBe(true);
		const record = (await response.json()) as {
			id?: string | number;
			data?: { id: string | number };
		};
		return { email, id: record.data?.id ?? record.id };
	};
	const first = await fixture('a');
	const second = await fixture('b');
	expect(second.id).toBeDefined();
	const title = `Browser Segment ${Date.now()}`;
	await page.goto('/admin/crm/segments/new', { waitUntil: 'networkidle' });
	await page.getByLabel('Segment title', { exact: true }).fill(title);
	const rule = page.getByRole('group', { name: 'Rule 1', exact: true });
	await rule.getByLabel('Contact field', { exact: true }).selectOption('email');
	await rule.getByLabel('Value', { exact: true }).fill(first.email);
	await page.getByRole('button', { name: 'Save Segment', exact: true }).click();
	await expect(page).toHaveURL(/\/admin\/crm\/segments$/);
	let card = page
		.locator('.segment-card')
		.filter({ has: page.getByRole('heading', { name: title, exact: true }) });
	await expect(card).toBeVisible();
	await card.getByTitle('View Contacts', { exact: true }).click();
	await expect(page.getByRole('link', { name: first.email, exact: true })).toBeVisible();
	await expect(page.getByText('1 matching contacts', { exact: true })).toBeVisible();
	await page.getByRole('link', { name: 'Edit Segment', exact: true }).click();
	await expect(page.getByLabel('Value', { exact: true })).toHaveValue(first.email);
	await page.getByLabel('Value', { exact: true }).fill(second.email);
	await page.getByRole('button', { name: 'Save Segment', exact: true }).click();
	await expect(page).toHaveURL(/\/admin\/crm\/segments$/);
	card = page
		.locator('.segment-card')
		.filter({ has: page.getByRole('heading', { name: title, exact: true }) });
	await card.getByTitle('View Contacts', { exact: true }).click();
	await expect(page.getByRole('link', { name: second.email, exact: true })).toBeVisible();
	await page.reload();
	await expect(page.getByRole('link', { name: second.email, exact: true })).toBeVisible();
	const changed = await page.request.put(`/api/admin/crm/contacts/${second.id}`, {
		data: { email: `changed-${second.email}` }
	});
	expect(changed.ok()).toBe(true);
	await page.getByRole('button', { name: 'Refresh Matches', exact: true }).click();
	await expect(page.getByText('0 matching contacts', { exact: true })).toBeVisible();
	const restored = await page.request.put(`/api/admin/crm/contacts/${second.id}`, {
		data: { email: second.email }
	});
	expect(restored.ok()).toBe(true);
	await page.getByRole('button', { name: 'Refresh Matches', exact: true }).click();
	await expect(page.getByRole('link', { name: second.email, exact: true })).toBeVisible();
});

test('media upload, durable metadata, queued processing and object deletion through the page', async ({
	page
}) => {
	await page.goto('/login?redirect=/admin/media', { waitUntil: 'networkidle' });
	await page.locator('input[name="email"]').fill('admin@admin-completion.test');
	await page.locator('input[name="password"]').fill(fixturePassword);
	await page.getByRole('button', { name: 'Sign In to Trade' }).click();
	await expect(page).not.toHaveURL(/\/login/);
	await page.goto('/admin/media');
	const filename = `browser-media-${Date.now()}.png`;
	const bytes = await readFile(new URL('../fixtures/media-processing.png', import.meta.url));
	await page.getByRole('button', { name: 'Upload', exact: true }).click();
	await page
		.locator('input[type="file"]')
		.first()
		.setInputFiles({ name: filename, mimeType: 'image/png', buffer: bytes });
	const item = page.locator('.media-item').filter({ hasText: filename });
	await expect(item).toBeVisible();
	await item.click();
	const details = page.locator('.details-panel');
	await expect(details.locator('img')).toHaveJSProperty('naturalWidth', 1000);
	await expect(details.getByText('1000 × 400', { exact: true })).toBeVisible();
	const alt = page.locator('#media-alt-text');
	await alt.fill('Accessible browser verification image');
	await alt.blur();
	await expect(page.getByText('Alt text saved', { exact: true })).toBeVisible();
	await page.reload();
	await expect(item).toBeVisible();
	await item.click();
	await expect(alt).toHaveValue('Accessible browser verification image');
	await details.getByRole('button', { name: 'Responsive Preview', exact: true }).click();
	const originalPreview = page.getByRole('dialog', { name: 'Responsive Preview', exact: true });
	await expect(originalPreview.getByAltText('Original preview', { exact: true })).toHaveJSProperty(
		'naturalWidth',
		1000
	);
	await originalPreview.getByRole('button', { name: 'Close preview', exact: true }).click();
	await details.getByRole('button', { name: 'Crop', exact: true }).click();
	const crop = page.getByRole('dialog', { name: 'Crop & Edit Image' });
	await expect(crop.getByText('800 x 320 px', { exact: true })).toBeVisible();
	await crop.getByRole('button', { name: 'Apply Crop', exact: true }).click();
	await expect(page.getByText('Image cropped and saved', { exact: true })).toBeVisible();
	await expect(details.getByText('800 × 320', { exact: true })).toBeVisible();
	await expect(details.locator('img')).toHaveJSProperty('naturalWidth', 800);
	const originalVersion = details
		.locator('.version-row')
		.filter({ has: page.getByText('Version 1', { exact: true }) });
	await originalVersion.getByRole('button', { name: 'Restore', exact: true }).click();
	await expect(page.getByText('Version restored', { exact: true })).toBeVisible();
	await expect(details.getByText('1000 × 400', { exact: true })).toBeVisible();
	await expect(details.locator('img')).toHaveJSProperty('naturalWidth', 1000);
	await details.getByRole('button', { name: 'Optimize', exact: true }).click();
	await expect(page.getByText('Image processing queued', { exact: true })).toBeVisible();
	await expect(details.getByText('Optimized', { exact: true })).toBeVisible({ timeout: 45000 });
	const statusFilter = page
		.locator('select')
		.filter({ has: page.locator('option[value="optimized"]') });
	await statusFilter.selectOption('optimized');
	await expect(item).toBeVisible();
	await details.getByRole('button', { name: 'Responsive Preview', exact: true }).click();
	const preview = page.getByRole('dialog', { name: 'Responsive Preview', exact: true });
	const images = preview.locator('.responsive-preview__thumbnail-image');
	await expect(images).toHaveCount(3);
	for (const [index, width] of [320, 768, 1000].entries())
		await expect(images.nth(index)).toHaveJSProperty('naturalWidth', width);
	await preview.getByRole('button', { name: 'Close preview', exact: true }).click();
	await page.reload();
	await item.click();
	await expect(details.getByText('Optimized', { exact: true })).toBeVisible();
	await details.getByRole('button', { name: 'Delete', exact: true }).click();
	await page.getByRole('dialog').getByRole('button', { name: 'Delete', exact: true }).click();
	await expect(item).toHaveCount(0);
	await page.reload();
	await expect(item).toHaveCount(0);
});

test('CRM webhook authoring, activation, actual event history and failed receiver states persist', async ({
	page,
	request: outsider
}) => {
	expect((await outsider.get('/api/admin/crm/webhooks/events')).status()).toBe(401);
	const member = await outsider.post('/api/auth/login', {
		data: { email: 'user@admin-completion.test', password: fixturePassword }
	});
	expect(member.ok()).toBe(true);
	expect((await outsider.get('/api/admin/crm/webhooks')).status()).toBe(403);
	const login = await page.request.post('/api/auth/login', {
		data: { email: 'admin@admin-completion.test', password: fixturePassword }
	});
	expect(login.ok()).toBe(true);
	const title = `Browser Webhook ${Date.now()}`;
	await page.goto('/admin/crm/webhooks/new', { waitUntil: 'networkidle' });
	await page.getByLabel('Webhook Name', { exact: false }).fill(title);
	await page.getByLabel('Webhook URL', { exact: false }).fill('https://receiver.example.test/hook');
	await page
		.getByLabel('Secret Key (Optional)', { exact: true })
		.fill('only-disposable-browser-secret');
	await page.getByRole('button', { name: 'Contact Created', exact: true }).click();
	await page.getByRole('button', { name: 'Add Header', exact: true }).click();
	await page.getByLabel('Header name 1', { exact: true }).fill('X-Browser-Fixture');
	await page.getByLabel('Header value 1', { exact: true }).fill('persisted-value');
	await page.getByRole('checkbox', { name: 'Active', exact: true }).uncheck();
	await page.getByRole('button', { name: 'Create Webhook', exact: true }).click();
	await expect(page).toHaveURL(/\/admin\/crm\/webhooks$/);
	let card = page
		.locator('.webhook-card')
		.filter({ has: page.getByRole('heading', { name: title, exact: true }) });
	await expect(card).toBeVisible();
	const id = await card.getAttribute('data-webhook-id');
	expect(id).toMatch(/^\d+$/);
	await page.reload();
	await expect(card.getByRole('button', { name: `Activate ${title}`, exact: true })).toBeVisible();
	await card.getByRole('link', { name: `Edit ${title}`, exact: true }).click();
	await expect(page.getByLabel('Secret Key (Optional)', { exact: true })).toHaveValue(
		'only-disposable-browser-secret'
	);
	await expect(page.getByLabel('Header value 1', { exact: true })).toHaveValue('persisted-value');
	await page.getByRole('checkbox', { name: 'Active', exact: true }).check();
	await page.getByLabel('Secret Key (Optional)', { exact: true }).fill('');
	await page.getByTitle('Remove Header', { exact: true }).click();
	await page.getByRole('button', { name: 'Save Changes', exact: true }).click();
	await expect(page).toHaveURL(/\/admin\/crm\/webhooks$/);
	await card.getByRole('link', { name: `Edit ${title}`, exact: true }).click();
	await expect(page.getByLabel('Secret Key (Optional)', { exact: true })).toHaveValue('');
	await expect(page.getByLabel('Header value 1', { exact: true })).toHaveCount(0);
	await page.getByRole('link', { name: 'Back to Webhooks', exact: true }).click();
	const contact = await page.request.post('/api/admin/crm/contacts', {
		data: { email: `webhook-browser-${Date.now()}@example.test`, first_name: 'Webhook Browser' }
	});
	expect(contact.ok()).toBe(true);
	await card.getByRole('button', { name: `Test ${title}`, exact: true }).click();
	await expect(page.getByText(/Test queued\. View delivery history/)).toBeVisible();
	await card.getByRole('link', { name: `View logs for ${title}`, exact: true }).click();
	await expect(
		page.getByRole('heading', { name: `${title} delivery history`, exact: true })
	).toBeVisible();
	await expect(page.getByRole('heading', { name: 'contact created', exact: true })).toBeVisible();
	await expect(page.getByRole('heading', { name: 'webhook test', exact: true })).toBeVisible();
	const testEntry = page
		.locator('article')
		.filter({ has: page.getByRole('heading', { name: 'webhook test', exact: true }) });
	await expect(testEntry).toContainText(/[1-3] of 3 attempts/, {
		timeout: 20000
	});
	await expect(testEntry).not.toContainText('delivered');
	await page.reload();
	await expect(page.getByRole('heading', { name: 'contact created', exact: true })).toBeVisible();
	await page.getByRole('link', { name: 'Back to Webhooks', exact: true }).click();
	await card.getByRole('button', { name: `Deactivate ${title}`, exact: true }).click();
	await expect(card.getByRole('button', { name: `Activate ${title}`, exact: true })).toBeVisible();
	await card.getByRole('button', { name: `Delete ${title}`, exact: true }).click();
	await page.getByRole('dialog').getByRole('button', { name: 'Delete', exact: true }).click();
	await expect(card).toHaveCount(0);
});

test('native browser vitals persist through the acknowledged proxy and display in performance reports', async ({
	page,
	request: outsider
}) => {
	expect((await outsider.get('/api/performance/dashboard')).status()).toBe(401);
	const login = await page.request.post('/api/auth/login', {
		data: { email: 'admin@admin-completion.test', password: fixturePassword }
	});
	expect(login.ok()).toBe(true);
	const reportPath = '/api/performance/core-web-vitals?url=http%3A%2F%2F127.0.0.1%3A5274%2F';
	const beforeResponse = await page.request.get(reportPath);
	expect(beforeResponse.ok()).toBe(true);
	const before = (await beforeResponse.json()) as {
		data: { sample_counts: Record<string, number> };
	};
	const beforeCount = before.data.sample_counts.LCP ?? 0;
	await page.goto(`/?performance-fixture=${Date.now()}`, { waitUntil: 'networkidle' });
	const observationKey = `native-lcp-${Date.now()}`;
	await page.evaluate(async (key) => {
		const path = '/src/lib/utils/webVitals.ts';
		const module = (await import(path)) as {
			initWebVitals: (options: {
				sampleRate: number;
				onMetric: (metric: { name: string; value: number; id: string }) => void;
			}) => void;
		};
		module.initWebVitals({
			sampleRate: 1,
			onMetric: (metric) => {
				if (metric.name === 'LCP') sessionStorage.setItem(key, JSON.stringify(metric));
			}
		});
		await new Promise<void>((resolve) => {
			const observer = new PerformanceObserver((entries) => {
				if (entries.getEntries().some((entry) => entry.startTime > 0)) {
					observer.disconnect();
					resolve();
				}
			});
			observer.observe({ type: 'largest-contentful-paint', buffered: true });
		});
	}, observationKey);
	// Read the actual SDK observation after native pagehide; the committed
	// count below verifies delivery independently of DevTools exit events.
	await page.goto('/admin/performance', { waitUntil: 'networkidle' });
	const observation = await page.evaluate((key) => {
		const stored = sessionStorage.getItem(key);
		sessionStorage.removeItem(key);
		if (!stored) throw new Error('Native LCP was not finalized on pagehide');
		return JSON.parse(stored) as { name: string; value: number; id: string };
	}, observationKey);
	expect(observation.name).toBe('LCP');
	expect(observation.value).toBeGreaterThan(0);
	await expect
		.poll(async () => {
			const response = await page.request.get(reportPath);
			expect(response.ok()).toBe(true);
			const result = (await response.json()) as { data: { sample_counts: Record<string, number> } };
			return result.data.sample_counts.LCP;
		})
		.toBe(beforeCount + 1);
	await expect(
		page.getByRole('heading', { name: 'Performance Dashboard', exact: true })
	).toBeVisible();
	await page.getByLabel('Page URL', { exact: true }).fill('http://127.0.0.1:5274/');
	await page.getByRole('button', { name: 'Apply filters', exact: true }).click();
	const lcp = page
		.locator('.vital-card')
		.filter({ has: page.locator('.vital-name').filter({ hasText: /^LCP$/ }) });
	await expect(lcp).toContainText(/\d+ samples/);
	await expect(lcp.locator('.vital-value')).not.toHaveText('Unavailable');
	const report = await page.request.get(
		'/api/performance/core-web-vitals?url=http%3A%2F%2F127.0.0.1%3A5274%2F'
	);
	expect(report.ok()).toBe(true);
	const data = (await report.json()) as {
		data: { sample_counts: Record<string, number>; lcp: number; source: string };
	};
	expect(data.data.sample_counts.LCP).toBeGreaterThan(0);
	expect(data.data.lcp).toBeGreaterThan(0);
	expect(data.data.source).toBe('first_party_observations');
	await page.reload();
	await expect(
		page.getByText('Enable PostgreSQL pg_stat_statements', { exact: false })
	).toBeVisible();
	await page.getByRole('button', { name: 'Warm Caches', exact: true }).click();
	await expect(page.getByText(/Warmed \d+ cached responses across/)).toBeVisible();
	await page
		.getByLabel('Page URL', { exact: true })
		.fill(`http://127.0.0.1:5274/unmeasured-${Date.now()}`);
	await page.getByRole('button', { name: 'Apply filters', exact: true }).click();
	await expect(lcp.locator('.vital-value')).toHaveText('Unavailable');
	await expect(
		page.getByRole('heading', { name: 'No suggestions from recorded observations', exact: true })
	).toBeVisible();
	await expect(page.getByRole('heading', { name: 'All Good!', exact: true })).toHaveCount(0);
});

test('consent lifecycle automatically collects native vitals and stops on withdrawal', async ({
	page
}) => {
	const login = await page.request.post('/api/auth/login', {
		data: { email: 'admin@admin-completion.test', password: fixturePassword }
	});
	expect(login.ok()).toBe(true);
	const reportPath =
		'/api/performance/core-web-vitals?url=http%3A%2F%2F127.0.0.1%3A5274%2Fcookie-policy';
	const counts = async () => {
		const response = await page.request.get(reportPath);
		expect(response.ok()).toBe(true);
		return (await response.json()) as {
			data: { sample_counts: Record<string, number>; lcp: number | null };
		};
	};
	const before = await counts();
	const beforeLCP = before.data.sample_counts.LCP ?? 0;
	const beforeFCP = before.data.sample_counts.FCP ?? 0;
	await page.goto('/cookie-policy', { waitUntil: 'networkidle' });
	expect((await counts()).data.sample_counts.FCP ?? 0).toBe(beforeFCP);
	const consent = async (granted: boolean) =>
		page.evaluate(async (value) => {
			const path = '/src/lib/consent/store.svelte.ts';
			const module = (await import(path)) as {
				consentStore: { setCategory: (category: 'analytics', granted: boolean) => void };
			};
			module.consentStore.setCategory('analytics', value);
		}, granted);
	await consent(true);
	await expect.poll(async () => (await counts()).data.sample_counts.FCP).toBe(beforeFCP + 1);
	await consent(false);
	await page.goto('/admin/performance', { waitUntil: 'networkidle' });
	expect((await counts()).data.sample_counts.LCP ?? 0).toBe(beforeLCP);
	await consent(true);
	await page.goto('/cookie-policy', { waitUntil: 'networkidle' });
	await expect.poll(async () => (await counts()).data.sample_counts.FCP).toBe(beforeFCP + 2);
	await page.goto('/admin/performance', { waitUntil: 'networkidle' });
	await expect.poll(async () => (await counts()).data.sample_counts.LCP).toBe(beforeLCP + 1);
	expect((await counts()).data.lcp).toBeGreaterThan(0);
	await consent(false);
});
