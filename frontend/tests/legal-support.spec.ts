import { expect, test } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';
import { readFile } from 'node:fs/promises';
const routes = ['/privacy', '/terms', '/refund', '/disclaimer', '/cookie-policy', '/contact'];

test('legal and support pages share the marketing heading scale and readable body type', async ({
	page
}) => {
	await page.setViewportSize({ width: 1280, height: 800 });
	const headings = [];
	for (const route of routes) {
		await page.goto(route);
		headings.push(
			await page.locator('main h1').evaluate((e) => {
				const s = getComputedStyle(e);
				return {
					family: s.fontFamily,
					size: s.fontSize,
					weight: s.fontWeight,
					lineHeight: s.lineHeight
				};
			})
		);
		const body = await page.locator('.information-prose').evaluate((element) => {
			const style = getComputedStyle(element);
			return {
				size: parseFloat(style.fontSize),
				lineHeight: parseFloat(style.lineHeight) / parseFloat(style.fontSize)
			};
		});
		expect(body.size).toBeGreaterThanOrEqual(16);
		expect(body.lineHeight).toBeGreaterThanOrEqual(1.6);
		await expect(page.locator('main')).toHaveCount(1);
		await expect(page.locator('[aria-label="Breadcrumb"]')).toHaveCount(1);
	}
	expect(new Set(headings.map((h) => JSON.stringify(h))).size).toBe(1);
	expect(Number(headings[0].weight)).toBeGreaterThanOrEqual(700);
});
for (const route of routes) {
	test(`${route} exposes legal/support navigation and reflows enlarged text`, async ({ page }) => {
		await page.setViewportSize({ width: 390, height: 560 });
		await page.goto(route);
		const navigation = page.getByRole('navigation', { name: 'Legal and support' });
		await expect(navigation).toBeVisible();
		await expect(navigation.locator('a[aria-current="page"]')).toHaveAttribute('href', route);
		await expect(navigation.getByRole('link')).toHaveCount(6);
		await page.evaluate(() => {
			document.documentElement.style.fontSize = '32px';
		});
		const clipped = await page.locator('main h1').evaluate((e) => {
			const range = document.createRange();
			range.selectNodeContents(e);
			return [...range.getClientRects()].filter(
				(r) => r.left < 0 || r.right > document.documentElement.clientWidth
			).length;
		});
		expect(clipped).toBe(0);
		await page.reload();
		const destination = route === '/contact' ? '/privacy' : '/contact';
		await navigation.locator(`a[href="${destination}"]`).click();
		await expect(page).toHaveURL(new RegExp(`${destination}$`));
		await page.goBack();
		await expect(page).toHaveURL(new RegExp(`${route}$`));
	});
}

test('cookie preferences still open from the standardized policy page', async ({ page }) => {
	await page.goto('/cookie-policy');
	await page.getByRole('button', { name: 'Cookie Settings', exact: true }).click();
	const dialog = page.getByRole('dialog', { name: 'Cookie Preferences' });
	await expect(dialog).toBeVisible();
	await dialog.getByRole('button', { name: 'Close preferences' }).click();
	await expect(dialog).toHaveCount(0);
});

test('cookie language, keyboard focus and consent receipt remain functional', async ({ page }) => {
	await page.goto('/cookie-policy');
	await page.getByRole('combobox', { name: 'Select policy language' }).selectOption('de');
	await expect(page.locator('main h1')).toHaveText('Cookie-Richtlinie');
	await page.getByRole('combobox', { name: 'Select policy language' }).selectOption('en');
	const settings = page.getByRole('button', { name: 'Cookie Settings', exact: true });
	await settings.click();
	const dialog = page.getByRole('dialog', { name: 'Cookie Preferences' });
	await expect(dialog).toBeFocused();
	await page.keyboard.press('Shift+Tab');
	await expect(dialog.getByRole('button', { name: 'Save Preferences', exact: true })).toBeFocused();
	await page.keyboard.press('Tab');
	await expect(
		dialog.getByRole('button', { name: 'Close preferences', exact: true })
	).toBeFocused();
	await dialog.getByRole('button', { name: 'Reject All', exact: true }).click();
	await expect(dialog).toHaveCount(0);
	await expect(settings).toBeFocused();
	const downloadPromise = page.waitForEvent('download');
	await page.getByRole('button', { name: 'Download Receipt', exact: true }).click();
	const download = await downloadPromise;
	expect(await download.failure()).toBeNull();
	expect(download.suggestedFilename()).toMatch(/^consent-receipt-.*\.json$/);
	const path = await download.path();
	if (!path) throw new Error('Receipt was not saved');
	const receipt: unknown = JSON.parse(await readFile(path, 'utf8'));
	expect(receipt).toMatchObject({
		categories: { necessary: true, analytics: false, marketing: false, preferences: false }
	});
});

test('cookie dialog stays readable and accessible on a narrow enlarged-text screen', async ({
	page
}) => {
	await page.emulateMedia({ reducedMotion: 'reduce' });
	await page.setViewportSize({ width: 320, height: 560 });
	await page.goto('/cookie-policy');
	await page.getByRole('button', { name: 'Cookie Settings', exact: true }).click();
	const dialog = page.getByRole('dialog', { name: 'Cookie Preferences' });
	await expect(dialog).toBeVisible();
	const axe = await new AxeBuilder({ page }).include('[role="dialog"]').analyze();
	expect(axe.violations).toEqual([]);
	await page.evaluate(() => (document.documentElement.style.fontSize = '32px'));
	const clipped = await dialog.locator('button,h2,p').evaluateAll((elements) =>
		elements.flatMap((e) => {
			const range = document.createRange();
			range.selectNodeContents(e);
			return [...range.getClientRects()]
				.filter((r) => r.left < 0 || r.right > document.documentElement.clientWidth)
				.map((r) => r.right);
		})
	);
	expect(clipped).toEqual([]);
	await page.keyboard.press('Escape');
	await expect(dialog).toHaveCount(0);
});

test('cookie declaration refreshes after a visitor changes preferences', async ({
	page,
	context
}) => {
	await page.goto('/cookie-policy');
	await context.addCookies([
		{ name: 'rtp_legal_support_fixture', value: 'isolated', url: page.url() }
	]);
	await page.getByRole('button', { name: 'Cookie Settings', exact: true }).click();
	await page
		.getByRole('dialog', { name: 'Cookie Preferences' })
		.getByRole('button', { name: 'Reject All', exact: true })
		.click();
	await expect(
		page.locator('.cookie-name code').filter({ hasText: 'rtp_legal_support_fixture' })
	).toBeVisible();
});

test('leaving cookie policy closes its dialog and restores page scrolling', async ({ page }) => {
	await page.goto('/contact');
	await page
		.getByRole('navigation', { name: 'Legal and support' })
		.locator('a[href="/cookie-policy"]')
		.click();
	await expect(page).toHaveURL(/\/cookie-policy$/);
	await page.getByRole('button', { name: 'Cookie Settings', exact: true }).click();
	await expect(page.getByRole('dialog', { name: 'Cookie Preferences' })).toBeVisible();
	await page.goBack();
	await expect(page).toHaveURL(/\/contact$/);
	await expect(page.locator('body')).not.toHaveCSS('overflow', 'hidden');
	await page.goForward();
	await expect(page).toHaveURL(/\/cookie-policy$/);
	await expect(page.getByRole('dialog', { name: 'Cookie Preferences' })).toHaveCount(0);
});
