import { expect, test, type Page, type APIRequestContext } from '@playwright/test';
import { authors } from '../../src/lib/authors/authors.js';
import { PRODUCTION_ORIGIN } from '../../src/lib/seo/config.js';
import { publicMediaFixture } from './fixtures/public-media.js';
const author = authors[0];
if (!author) throw new Error('Actual author registry is empty');
const password = 'OnlyForDisposableTest123!';
function row(value: unknown): Record<string, unknown> {
	if (!value || typeof value !== 'object' || Array.isArray(value))
		throw new Error('Expected actual saved fixture record');
	return value as Record<string, unknown>;
}
async function login(
	page: Page,
	baseURL: string | undefined,
	email = 'admin@admin-completion.test'
) {
	expect(baseURL).toBe('http://127.0.0.1:5274');
	expect((await page.request.post('/api/auth/login', { data: { email, password } })).status()).toBe(
		200
	);
}
async function saved(client: APIRequestContext, kind: 'news' | 'video') {
	const response = await client.get(`/api/admin/seo/sitemaps/${kind}`);
	expect(response.status()).toBe(200);
	return row(await response.json());
}
async function restore(client: APIRequestContext, kind: 'news' | 'video', settings: unknown) {
	const latest = await saved(client, kind);
	expect(
		(
			await client.put(`/api/admin/seo/sitemaps/${kind}`, {
				data: { expected_version: latest.settings_version, config: settings }
			})
		).status()
	).toBe(200);
}
async function diagnostics(page: Page, expected = new Set<string>()) {
	const errors: string[] = [],
		blocked = new Set<string>();
	page.on('pageerror', (cause) => errors.push(cause.message));
	page.on('console', (message) => {
		const status = message
				.text()
				.match(/^Failed to load resource: the server responded with a status of (\d+)/)?.[1],
			source = message.location().url;
		if (status && source && expected.has(`${status} ${new URL(source).pathname}`)) return;
		if (
			message.type() === 'error' &&
			!(
				blocked.has(message.location().url) &&
				message.text() === 'Failed to load resource: net::ERR_FAILED'
			)
		)
			errors.push(message.text());
	});
	page.on('response', (response) => {
		if (
			new URL(response.url()).pathname.startsWith('/api/') &&
			response.status() >= 400 &&
			!expected.has(`${response.status()} ${new URL(response.url()).pathname}`)
		)
			errors.push(`${response.status()} ${response.url()}`);
	});
	await page.route('**/*', (route) => {
		if (['127.0.0.1', 'localhost'].includes(new URL(route.request().url()).hostname))
			return route.continue();
		blocked.add(route.request().url());
		return route.abort();
	});
	return errors;
}
async function xml(page: Page, kind: 'news' | 'video') {
	const response = await page.request.get(`/${kind}-sitemap.xml`);
	expect(response.status()).toBe(200);
	expect(response.headers()['content-type']).toContain('application/xml');
	return response.text();
}
function graph(value: unknown): Array<Record<string, unknown>> {
	const root = row(value);
	if (!Array.isArray(root['@graph'])) throw new Error('Central JSON-LD graph is missing');
	return root['@graph'].map(row);
}

test('news sitemap saves actual settings, uses original source facts, expires old or noindex articles and preserves SSR byline/date parity', async ({
	page,
	baseURL
}) => {
	await login(page, baseURL, 'seo-author-browser@example.test');
	const ids: number[] = [],
		stamp = Date.now(),
		title = `Actual news browser ${stamp}`,
		description =
			'This actual published fixture description is shared by the article page and news publication workflow.',
		published = new Date(Date.now() - 60000).toISOString().slice(0, -1);
	let initial: unknown;
	try {
		const make = async (title: string, published_at: string, indexable = true) => {
			const response = await page.request.post('/api/admin/posts', {
				data: {
					title,
					excerpt: description,
					content_blocks: [
						{ type: 'paragraph', content: { text: 'Actual public fixture article body.' } }
					],
					featured_image: '/revolution-trading-pros.png',
					status: 'published',
					published_at,
					indexable
				}
			});
			expect(response.status()).toBe(200);
			const post = row(await response.json());
			if (typeof post.id !== 'number' || typeof post.slug !== 'string')
				throw new Error('Saved article identity is missing');
			ids.push(post.id);
			return post;
		};
		const actual = await make(title, published),
			old = await make(
				`Old news browser ${stamp}`,
				new Date(Date.now() - 49 * 3600000).toISOString().slice(0, -1)
			),
			noindex = await make(`Noindex news browser ${stamp}`, published, false);
		await login(page, baseURL);
		const unknown = await make(`Unknown author news browser ${stamp}`, published);
		initial = (await saved(page.request, 'news')).settings;
		const errors = await diagnostics(page);
		await page.goto('/admin/seo/news-sitemap', { waitUntil: 'networkidle' });
		await expect(page.getByRole('heading', { name: 'News Sitemap', exact: true })).toBeVisible();
		await page.getByLabel('Enable News Sitemap', { exact: true }).check();
		await page
			.getByLabel('Publication Name', { exact: true })
			.fill(`Actual browser publication ${stamp}`);
		await page.getByRole('button', { name: 'Save Settings', exact: true }).click();
		await expect(
			page
				.locator('.sitemap-manager')
				.getByRole('status')
				.filter({ hasText: 'Sitemap settings saved.' })
		).toContainText('Sitemap settings saved.');
		await page.reload({ waitUntil: 'networkidle' });
		await expect(page.getByLabel('Publication Name', { exact: true })).toHaveValue(
			`Actual browser publication ${stamp}`
		);
		const output = await xml(page, 'news');
		expect(output).toContain(`${PRODUCTION_ORIGIN}/blog/${actual.slug}`);
		expect(output).toContain(`<news:title>${title}</news:title>`);
		expect(output).not.toContain(`/blog/${old.slug}`);
		expect(output).not.toContain(`/blog/${noindex.slug}`);
		expect(output).not.toContain(`/blog/${unknown.slug}`);
		const source = row(await (await page.request.get(`/api/posts/${actual.slug}`)).json());
		expect(row(source.author).name).toBe(author.name);
		const sourceDate = String(source.published_at);
		expect(output).toContain(sourceDate);
		const generating = page.waitForResponse(
			(response) => new URL(response.url()).pathname === '/api/admin/seo/sitemaps/news/generate'
		);
		await page.getByRole('button', { name: 'Regenerate', exact: true }).click();
		const generated = await generating;
		expect(generated.status()).toBe(200);
		const receipt = row(row(await generated.json()).generation),
			feed = row(await (await page.request.get('/api/seo/sitemaps/news')).json());
		expect(receipt.entry_count).toBe(row(row(feed.entries).meta).total);
		const html = await page.request.get(`/blog/${actual.slug}`);
		expect(html.status()).toBe(200);
		const markup = await html.text();
		expect(markup).toContain(title);
		expect(markup).toContain(`/authors/${author.slug}`);
		expect(markup).toContain('Actual public fixture article body.');
		expect(markup).not.toMatch(/<img\b[^>]*\son(?:load|error)=/i);
		await page.goto(`/blog/${actual.slug}`, { waitUntil: 'networkidle' });
		await expect(page.getByRole('heading', { name: title, exact: true })).toBeVisible();
		await expect(page.locator('.featured-image img')).toHaveClass(/\bloaded\b/);
		await expect(page.locator('.author-details a')).toHaveAttribute(
			'href',
			`/authors/${author.slug}`
		);
		await expect(page.locator('head title')).toHaveCount(1);
		await expect(page.locator('link[rel="canonical"]')).toHaveCount(1);
		const article = graph(
			JSON.parse(await page.locator('head script[type="application/ld+json"]').innerText())
		).find((node) => node['@type'] === 'BlogPosting');
		expect(article?.headline).toBe(title);
		expect(row(article?.author).name).toBe(author.name);
		expect(Date.parse(String(article?.datePublished))).toBe(Date.parse(`${sourceDate}Z`));
		expect(errors).toEqual([]);
	} finally {
		if (initial) await restore(page.request, 'news', initial);
		for (const id of ids)
			expect((await page.request.delete(`/api/admin/posts/${id}`)).status()).toBe(200);
	}
});

test('video source authoring requires explicit approval and public watch metadata matches actual local media before audited sitemap inclusion and removal', async ({
	page,
	baseURL
}) => {
	await login(page, baseURL);
	const initial = (await saved(page.request, 'video')).settings,
		media = await publicMediaFixture(),
		title = `Actual public video ${Date.now()}`,
		description =
			'This actual two-second color clip is a local fixture for public video metadata and playback verification.',
		date = new Date(Date.now() - 60000).toISOString().slice(0, 16);
	let id: string | undefined;
	try {
		const errors = await diagnostics(page),
			options = row(await (await page.request.get('/api/admin/seo/video-sources/options')).json()),
			traders = row(options.data).traders;
		if (!Array.isArray(traders)) throw new Error('Saved video authors are missing');
		const matches = traders.map(row).filter((trader) => trader.name === author.name);
		expect(matches).toHaveLength(1);
		const trader = matches[0];
		if (!trader) throw new Error('Dedicated actual author fixture is missing');
		await page.goto('/admin/seo/video-sitemap', { waitUntil: 'networkidle' });
		await page.getByRole('button', { name: 'Add Video', exact: true }).click();
		const dialog = page.getByRole('dialog', { name: 'Add public video source', exact: true });
		for (const [label, value] of [
			['Video title', title],
			['Video description', description],
			['Public video or player URL', `${media.origin}/actual.mp4`],
			['Actual thumbnail URL', `${media.origin}/actual.png`],
			['Actual duration (seconds)', '2'],
			['Original publication time (UTC)', date]
		])
			await dialog.getByLabel(label, { exact: true }).fill(value);
		await dialog.getByLabel('Saved video author', { exact: true }).selectOption(String(trader.id));
		const creating = page.waitForResponse(
			(response) =>
				new URL(response.url()).pathname === '/api/admin/seo/video-sources' &&
				response.request().method() === 'POST'
		);
		await dialog.getByRole('button', { name: 'Save source video', exact: true }).click();
		const created = await creating;
		expect(created.status()).toBe(200);
		id = String(row(row(await created.json()).data).id);
		const candidate = row(
				row(await (await page.request.get(`/api/admin/seo/video-publications/${id}`)).json()).record
			),
			path = String(candidate.path);
		expect((await page.request.get(path)).status()).toBe(404);
		const approval = page.getByRole('dialog', { name: 'Approve public watch page', exact: true });
		await expect(approval).toBeVisible();
		await approval
			.getByLabel('Actual public author profile', { exact: true })
			.selectOption(author.slug);
		await approval.getByRole('button', { name: 'Approve public watch page', exact: true }).click();
		await expect(approval).toHaveCount(0);
		const sourceRow = page.getByRole('row').filter({ hasText: title });
		await expect(sourceRow).toHaveCount(1);
		await expect(sourceRow.getByText('Eligible', { exact: true })).toBeVisible();
		await page.getByLabel('Enable Video Sitemap', { exact: true }).check();
		await page.getByRole('button', { name: 'Save Settings', exact: true }).click();
		await expect(
			page
				.locator('.sitemap-manager')
				.getByRole('status')
				.filter({ hasText: 'Sitemap settings saved.' })
		).toContainText('Sitemap settings saved.');
		const generating = page.waitForResponse(
			(response) => new URL(response.url()).pathname === '/api/admin/seo/sitemaps/video/generate'
		);
		await page.getByRole('button', { name: 'Regenerate', exact: true }).click();
		expect((await generating).status()).toBe(200);
		const output = await xml(page, 'video');
		expect(output).toContain(`${PRODUCTION_ORIGIN}${path}`);
		expect(output).toContain('<video:duration>2</video:duration>');
		expect(output).toContain(`<video:content_loc>${media.origin}/actual.mp4</video:content_loc>`);
		expect(output).toContain(description);
		const html = await page.request.get(path);
		expect(html.status()).toBe(200);
		const markup = await html.text();
		expect(markup).toContain(title);
		expect(markup).toContain(description);
		expect(markup).toContain(`/authors/${author.slug}`);
		expect(markup).toContain(String(candidate.published_at));
		await page.goto(path, { waitUntil: 'networkidle' });
		await expect(page.getByRole('heading', { name: title, exact: true })).toBeVisible();
		await expect(page.locator('video')).toHaveAttribute('poster', `${media.origin}/actual.png`);
		await expect
			.poll(() => page.locator('video').evaluate((video) => video.duration))
			.toBeCloseTo(2, 1);
		const video = graph(
			JSON.parse(await page.locator('head script[type="application/ld+json"]').innerText())
		).find((node) => node['@type'] === 'VideoObject');
		expect(video?.name).toBe(title);
		expect(video?.description).toBe(description);
		expect(video?.duration).toBe('PT2S');
		expect(video?.contentUrl).toBe(`${media.origin}/actual.mp4`);
		expect(video?.uploadDate).toBe(candidate.published_at);
		expect(row(video?.author).name).toBe(author.name);
		expect(await (await page.request.get('/learning-center')).text()).toContain(`href="${path}"`);
		expect(await (await page.request.get('/sitemap.xml')).text()).toContain(
			`${PRODUCTION_ORIGIN}${path}`
		);
		await page.goto('/admin/seo/video-sitemap', { waitUntil: 'networkidle' });
		await sourceRow.getByRole('button', { name: 'Edit metadata', exact: true }).click();
		const editing = page.getByRole('dialog', { name: 'Edit saved video metadata', exact: true });
		await editing
			.getByLabel('Video description', { exact: true })
			.fill(`${description} This description has now been explicitly revised.`);
		await editing.getByRole('button', { name: 'Save source video', exact: true }).click();
		await expect(approval).toBeVisible();
		expect((await page.request.get(path)).status()).toBe(404);
		expect(
			(
				await page.request.post('/api/admin/seo/video-publications', {
					data: {
						video_id: id,
						author_slug: author.slug,
						expected_source_hash: candidate.source_hash
					}
				})
			).status()
		).toBe(409);
		await approval.getByRole('button', { name: 'Approve public watch page', exact: true }).click();
		await expect(approval).toHaveCount(0);
		expect((await page.request.get(path)).status()).toBe(200);
		await sourceRow.getByRole('button', { name: 'Remove public listing', exact: true }).click();
		await page
			.getByRole('dialog', { name: 'Remove Public Video Listing', exact: true })
			.getByRole('button', { name: 'Remove listing', exact: true })
			.click();
		await expect(
			sourceRow.getByText('Public watch page has not been approved', { exact: true })
		).toBeVisible();
		expect((await page.request.get(path)).status()).toBe(404);
		expect(await xml(page, 'video')).not.toContain(`${PRODUCTION_ORIGIN}${path}`);
		expect(errors).toEqual([]);
	} finally {
		try {
			if (id) {
				const current = await page.request.get(`/api/admin/seo/video-publications/${id}`);
				if (current.ok() && row(row(await current.json()).record).mapping_id)
					expect(
						(await page.request.delete(`/api/admin/seo/video-publications/${id}`)).status()
					).toBe(200);
				expect((await page.request.delete(`/api/admin/videos/${id}`)).status()).toBe(200);
			}
			await restore(page.request, 'video', initial);
		} finally {
			await media.close();
		}
	}
});

test('sitemap admin reads fail visibly without writes, recover from malformed data, reject stale settings and protect both reads and mutations', async ({
	page,
	baseURL,
	browser
}) => {
	await login(page, baseURL);
	const expected = new Set(['503 /api/admin/seo/sitemaps/news']),
		errors = await diagnostics(page, expected);
	let mode: 'unavailable' | 'malformed' | 'live' = 'unavailable',
		writes = 0;
	const initial = await saved(page.request, 'news');
	await page.route('**/api/admin/seo/sitemaps/news*', (route) => {
		if (route.request().method() !== 'GET') {
			writes += 1;
			return route.continue();
		}
		if (mode === 'live') return route.continue();
		return route.fulfill({
			status: mode === 'unavailable' ? 503 : 200,
			contentType: 'application/json',
			body: JSON.stringify(
				mode === 'unavailable'
					? { error: 'Fixture source read unavailable' }
					: { kind: 'news', preview: [] }
			)
		});
	});
	try {
		await page.goto('/admin/seo/news-sitemap', { waitUntil: 'networkidle' });
		await expect(page.getByRole('alert')).toContainText('Fixture source read unavailable');
		await expect(page.getByRole('button', { name: 'Save Settings', exact: true })).toHaveCount(0);
		expect(writes).toBe(0);
		mode = 'malformed';
		expected.clear();
		await page.getByRole('button', { name: 'Refresh saved state', exact: true }).click();
		await expect(page.getByRole('alert')).toBeVisible();
		expect(writes).toBe(0);
		mode = 'live';
		await page.getByRole('button', { name: 'Refresh saved state', exact: true }).click();
		await expect(page.getByRole('button', { name: 'Save Settings', exact: true })).toBeVisible();
		expect(writes).toBe(0);
		const current = await saved(page.request, 'news');
		expect(
			(
				await page.request.put('/api/admin/seo/sitemaps/news', {
					data: { expected_version: current.settings_version, config: current.settings }
				})
			).status()
		).toBe(200);
		writes = 0;
		expected.add('409 /api/admin/seo/sitemaps/news');
		await page.getByRole('button', { name: 'Save Settings', exact: true }).click();
		await expect(page.getByRole('alert')).toContainText('reload before saving');
		expected.clear();
		expect(writes).toBe(1);
		const anon = await browser.newContext({ baseURL });
		try {
			for (const method of ['get', 'put'] as const)
				expect(
					(
						await anon.request[method]('/api/admin/seo/sitemaps/news', {
							data: method === 'put' ? {} : undefined
						})
					).status()
				).toBe(401);
		} finally {
			await anon.close();
		}
		const member = await browser.newContext({ baseURL });
		try {
			expect(
				(
					await member.request.post('/api/auth/login', {
						data: { email: 'user@admin-completion.test', password }
					})
				).status()
			).toBe(200);
			for (const method of ['get', 'put'] as const)
				expect(
					(
						await member.request[method]('/api/admin/seo/sitemaps/news', {
							data: method === 'put' ? {} : undefined
						})
					).status()
				).toBe(403);
		} finally {
			await member.close();
		}
		expect(errors).toEqual([]);
	} finally {
		await page.unroute('**/api/admin/seo/sitemaps/news*');
		await restore(page.request, 'news', initial.settings);
	}
});
