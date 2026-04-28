/**
 * Verification: CSP fix (tags dropdown loads) + type-mismatch fix (tags sent as strings).
 *
 * Before fixes:
 *   - CSP blocked browser→http://localhost:8080 → availableTags = [] → no checkboxes
 *   - savePost sent tags as number[] → backend returned HTTP 422
 *
 * After fixes:
 *   - loadTags() uses same-origin fetch → SvelteKit proxy → cookie auth → 200
 *   - savePost maps IDs→names → tags sent as string[] → HTTP 200
 *   - loadPost() uses same-origin fetch → works on fresh page load (cookie auth)
 *   - published_at strips milliseconds and Z → NaiveDateTime-compatible
 *
 * Flow: login → /admin/blog/create → fill → select tag → publish → /admin/blog →
 *        click edit link → verify title loads → save → delete.
 */
import { test, expect, type Page } from '@playwright/test';

const ADMIN_EMAIL = process.env.E2E_ADMIN_EMAIL || 'welberribeirodrums@gmail.com';
const ADMIN_PASSWORD = process.env.E2E_ADMIN_PASSWORD || 'Davedicenso01!';
const TITLE = 'E2E CSP+Tags Verification';
const SLUG = 'e2e-csp-tags-verification';

test.setTimeout(120000);

async function loginAdmin(page: Page): Promise<string> {
	const resp = await page.request.post('/api/auth/login', {
		data: { email: ADMIN_EMAIL, password: ADMIN_PASSWORD },
		headers: { 'content-type': 'application/json' },
		failOnStatusCode: false
	});
	expect(resp.ok(), `admin login failed: ${resp.status()} ${await resp.text()}`).toBe(true);
	const body = await resp.json();
	return body.access_token as string;
}

async function cleanup(page: Page, token: string): Promise<void> {
	const list = await page.request.get(
		`/api/admin/posts?slug=${encodeURIComponent(SLUG)}&per_page=5`,
		{ headers: { Authorization: `Bearer ${token}` }, failOnStatusCode: false }
	);
	if (!list.ok()) return;
	const body = await list.json();
	for (const p of (body?.data ?? []) as Array<{ id: number; slug: string }>) {
		if (p.slug !== SLUG) continue;
		await page.request
			.delete(`/api/admin/posts/${p.id}`, {
				headers: { Authorization: `Bearer ${token}` },
				failOnStatusCode: false
			})
			.catch(() => {});
	}
}

test('CSP fix: tags dropdown loads; type-mismatch fix: tags sent as strings', async ({ page }) => {
	const token = await loginAdmin(page);
	const evidence: string[] = [];
	const cspErrors: string[] = [];

	page.on('console', (msg) => {
		if (msg.type() === 'error' && msg.text().includes('Content Security Policy')) {
			cspErrors.push(msg.text());
		}
	});

	try {
		// ── 1. Warm auth store (same pattern as blog-post-create.spec.ts) ────────
		const meResp = page.waitForResponse(
			(r) => r.url().includes('/api/auth/me') && r.status() === 200,
			{ timeout: 15_000 }
		);
		await page.goto('/admin');
		await meResp.catch(() => {});
		await page.waitForLoadState('domcontentloaded');

		// ── 2. Navigate to create ─────────────────────────────────────────────────
		await page.goto('/admin/blog/create');
		await page.waitForLoadState('domcontentloaded');
		await expect(page.locator('#post-title')).toBeVisible({ timeout: 20_000 });
		await page.waitForLoadState('networkidle');
		await page.waitForTimeout(1500); // allow loadTags() onMount to complete

		// ── 3. Verify tags dropdown loads (CSP fix) ───────────────────────────────
		const tagCheckboxes = page.locator('.checkbox-list .checkbox-item input[type="checkbox"]');
		const tagCount = await tagCheckboxes.count();
		evidence.push(`Tags in dropdown: ${tagCount}`);
		expect(tagCount, 'Tags dropdown must be populated — CSP fix must be working').toBeGreaterThan(0);
		expect(cspErrors, 'No CSP errors after fix').toHaveLength(0);
		await page.screenshot({ path: '/tmp/01_tags_populated.png' });

		// ── 4. Fill form ──────────────────────────────────────────────────────────
		await page.locator('#post-title').fill(TITLE);
		await page.locator('#post-title').blur();
		await expect(page.locator('#slug')).toHaveValue(SLUG);
		await page.locator('#excerpt').fill('E2E verification that tags load and save correctly.');

		// ── 5. Select a tag (click, not check — checkbox disappears on selection) ─
		await page.locator('.sidebar-panel').filter({ hasText: 'Tags' }).last().scrollIntoViewIfNeeded();
		const firstTag = page.locator('.checkbox-list .checkbox-item').first();
		const selectedTagName = ((await firstTag.textContent()) ?? '').trim();
		await firstTag.locator('input[type="checkbox"]').click();
		evidence.push(`Selected tag: "${selectedTagName}"`);
		// Tag badge should appear in selected area
		await expect(page.locator('.tag-badge').first()).toBeVisible({ timeout: 5000 });
		await page.screenshot({ path: '/tmp/02_tag_selected.png' });

		// ── 6. Publish — capture POST and verify tags are strings ─────────────────
		const postRespPromise = page.waitForResponse(
			(r) => r.url().includes('/api/admin/posts') && r.request().method() === 'POST',
			{ timeout: 30_000 }
		);

		// Capture request body to verify tags field
		let capturedTags: unknown[] = [];
		page.on('request', (req) => {
			if (req.url().includes('/api/admin/posts') && req.method() === 'POST') {
				try {
					const body = JSON.parse(req.postData() ?? '{}');
					capturedTags = body.tags ?? [];
				} catch { /* ignore */ }
			}
		});

		await page.locator('.editor-header').first().scrollIntoViewIfNeeded();
		await page.getByRole('button', { name: /^Publish$/ }).click();
		const postResp = await postRespPromise;
		await page.screenshot({ path: '/tmp/03_after_publish.png' });

		evidence.push(`POST status: ${postResp.status()}`);
		evidence.push(`POST tags sent: ${JSON.stringify(capturedTags)}`);

		expect(
			postResp.ok(),
			`POST /api/admin/posts failed: ${postResp.status()} ${await postResp.text().catch(() => '')}`
		).toBe(true);

		// Verify tags were strings, not numbers (type-mismatch fix)
		if (capturedTags.length > 0) {
			expect(
				typeof capturedTags[0],
				'tags must be sent as strings (type-mismatch fix)'
			).toBe('string');
		}

		const postBody = await postResp.json().catch(() => ({})) as Record<string, unknown>;
		const createdId = postBody.id as number;
		evidence.push(`Created post: id=${createdId} slug=${postBody.slug}`);

		// In-page success toast
		await expect(page.locator('.save-success')).toContainText(/published/i);

		// ── 7. Redirect to /admin/blog ────────────────────────────────────────────
		await page.waitForURL(/\/admin\/blog(?:\?|$)/, { timeout: 10_000 });
		await page.waitForLoadState('networkidle');
		evidence.push('Redirected to /admin/blog');

		// ── 8. Check public /blog/<slug> ──────────────────────────────────────────
		const slugResp = await page.request.get(`/blog/${SLUG}`);
		evidence.push(`/blog/${SLUG} → HTTP ${slugResp.status()}`);
		expect([200, 404]).toContain(slugResp.status());

		// ── 9. Edit — click link from blog list (preserves in-memory auth) ────────
		expect(createdId, 'must have post ID to edit').toBeGreaterThan(0);
		const editLink = page.locator(`a[href*="/admin/blog/edit/${createdId}"]`).first();
		if (await editLink.count() > 0) {
			await editLink.click();
		} else {
			await page.goto(`/admin/blog/edit/${createdId}`);
		}
		await page.waitForURL(`**/admin/blog/edit/${createdId}`, { timeout: 15_000 });
		await page.waitForLoadState('networkidle');

		// Wait for loadPost() to populate the title
		await expect(page.locator('#page-post-title')).toBeVisible({ timeout: 15_000 });
		await page.waitForTimeout(1500);
		const editTitle = await page.locator('#page-post-title').inputValue();
		evidence.push(`Edit page title: "${editTitle}"`);
		expect(editTitle, 'edit page must show the post title (loadPost fix)').toBeTruthy();

		await page.screenshot({ path: '/tmp/04_edit_page.png' });

		// ── 10. Save edit ─────────────────────────────────────────────────────────
		const putRespPromise = page.waitForResponse(
			(r) => r.url().includes('/api/admin/posts') && r.request().method() === 'PUT',
			{ timeout: 30_000 }
		);
		await page.locator('.editor-header').first().scrollIntoViewIfNeeded();
		await page.getByRole('button', { name: /Save Draft|Publish/ }).first().click();
		const putResp = await putRespPromise;
		evidence.push(`PUT status: ${putResp.status()}`);
		expect(putResp.ok(), `PUT failed: ${putResp.status()}`).toBe(true);

		await page.screenshot({ path: '/tmp/05_after_edit.png' });

	} finally {
		await cleanup(page, token);
	}

	// ── Summary ───────────────────────────────────────────────────────────────
	console.log('\n=== EVIDENCE ===');
	for (const e of evidence) console.log(' ✓', e);
	console.log('\n=== CSP ERRORS (expected empty) ===');
	for (const e of cspErrors) console.log(' ✗', e);
});
