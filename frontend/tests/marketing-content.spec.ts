import { expect, test } from '@playwright/test';
const mode = process.env.MARKETING_PUBLIC_FIXTURE;
test.skip(
	!['empty', 'published'].includes(mode ?? ''),
	'Requires the owned marketing fixture; see audit report.'
);

test('tutorials exposes the approved public library rather than foreign placeholder videos', async ({
	page
}) => {
	await page.goto('/tutorials');
	await expect(page.locator('h1')).toHaveText('Trading Tutorials');
	await expect(page.locator('a[href*="time-frames-in-tos"]')).toHaveCount(0);
	await expect(page.locator('img[src*="simplertrading"]')).toHaveCount(0);
	if (mode === 'empty')
		await expect(page.locator('main')).toContainText(
			'No public learning videos are currently published.'
		);
	else {
		const video = page.getByRole('link', { name: /Marketing fixture educational video/ }).first();
		await video.click();
		await expect(page.locator('h1')).toHaveText('Marketing fixture educational video');
		await expect(page.locator('video source')).toHaveAttribute(
			'src',
			'https://marketing-fixture.b-cdn.net/educational.mp4'
		);
		await page.goBack();
		await expect(page.locator('h1')).toHaveText('Trading Tutorials');
	}
});

test('blog uses the real publication service and pagination follows browser history', async ({
	page
}) => {
	await page.goto('/blog');
	if (mode === 'empty') {
		await expect(page.locator('main')).toContainText('Featured Trading Guides');
		await expect(page.locator('main')).not.toContainText('temporarily unavailable');
	} else {
		await expect(page.locator('.post-card')).toHaveCount(20);
		await page.getByRole('link', { name: 'Next page', exact: true }).click();
		await expect(page).toHaveURL(/page=2/);
		await expect(page.locator('.post-card')).toHaveCount(1);
		await page.goBack();
		await expect(page.locator('.post-card')).toHaveCount(20);
		await page.goForward();
		await expect(page.locator('.post-card')).toHaveCount(1);
		await page.locator('.post-card').first().click();
		await expect(page.locator('h1')).toContainText('Marketing fixture article');
	}
});

test('invalid public catalog pagination is rejected', async ({ request }) => {
	for (const route of ['/tutorials?page=-1', '/learning-center?page=0', '/blog?page=abc']) {
		expect((await request.get(route)).status()).toBe(400);
	}
});
