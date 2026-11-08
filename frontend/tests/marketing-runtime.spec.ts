import { expect, test } from '@playwright/test';

test('consent gates a public popup and records its impression', async ({ page }) => {
	let lookups = 0;
	let impressions = 0;
	await page.route('**/api/popups/active?**', async (route) => {
		lookups += 1;
		await route.fulfill({
			status: 200,
			contentType: 'application/json',
			body: JSON.stringify({
				popups: [
					{
						id: 7001,
						type: 'timed',
						title: 'Test campaign',
						content: 'Welcome',
						cta_text: null,
						cta_url: null,
						cta_new_tab: false,
						close_on_escape: true,
						close_on_overlay_click: true,
						show_close_button: true,
						auto_close_after: null,
						has_form: false,
						form_id: null,
						trigger_rules: { delay: 0 },
						frequency_rules: { frequency: 'once_per_session' }
					}
				]
			})
		});
	});
	await page.route('**/api/popups/7001/impression', async (route) => {
		impressions += 1;
		await route.fulfill({ status: 200, contentType: 'application/json', body: '{"success":true}' });
	});
	await page.goto('/about');
	await expect(page.getByRole('dialog', { name: 'Cookie consent' })).toBeVisible();
	expect(lookups).toBe(0);
	expect(await page.evaluate(() => localStorage.getItem('rtp_ab_variant'))).toBeNull();
	await page.getByRole('dialog', { name: 'Cookie consent' }).locator('.btn-accept').click();
	await expect(page.getByRole('dialog', { name: 'Test campaign' })).toBeVisible();
	await expect.poll(() => impressions).toBe(1);
	await page
		.getByRole('dialog', { name: 'Test campaign' })
		.getByRole('button', { name: 'Close popup' })
		.click();
	await expect(page.getByRole('dialog', { name: 'Test campaign' })).toHaveCount(0);
	await page.reload();
	await expect(page.getByRole('dialog', { name: 'Test campaign' })).toHaveCount(0);
	expect(impressions).toBe(1);
});

test('rejecting consent suppresses popup lookup and settings remain available', async ({
	page
}) => {
	let lookups = 0;
	await page.route('**/api/popups/active?**', async (route) => {
		lookups += 1;
		await route.fulfill({ status: 200, contentType: 'application/json', body: '{"popups":[]}' });
	});
	await page.goto('/about');
	await page.getByRole('dialog', { name: 'Cookie consent' }).locator('.btn-reject').click();
	await expect(page.getByRole('dialog', { name: 'Cookie consent' })).toHaveCount(0);
	expect(lookups).toBe(0);
	expect(
		await page.evaluate(() => ({
			variant: localStorage.getItem('rtp_ab_variant'),
			results: localStorage.getItem('rtp_ab_results'),
			session: sessionStorage.getItem('rtp_ab_session')
		}))
	).toEqual({ variant: null, results: null, session: null });
	await page.getByRole('button', { name: 'Cookie settings' }).click();
	await expect(page.getByRole('dialog', { name: /cookie preferences/i })).toBeVisible();
	expect(lookups).toBe(0);
});

test('a published form can submit from a consented popup', async ({ page }) => {
	let submissions = 0;
	let impressions = 0;
	let conversions = 0;
	await page.route('**/api/popups/active?**', async (route) => {
		await route.fulfill({
			status: 200,
			contentType: 'application/json',
			body: JSON.stringify({
				popups: [
					{
						id: 7002,
						type: 'timed',
						title: 'Contact us',
						content: null,
						cta_text: null,
						cta_url: null,
						cta_new_tab: false,
						close_on_escape: true,
						close_on_overlay_click: true,
						show_close_button: true,
						auto_close_after: null,
						has_form: true,
						form_id: 901,
						trigger_rules: { delay: 0 },
						frequency_rules: { frequency: 'once_per_session' }
					}
				]
			})
		});
	});
	await page.route('**/api/forms/901', async (route) => {
		await route.fulfill({
			status: 200,
			contentType: 'application/json',
			body: JSON.stringify({
				id: 901,
				name: 'Contact',
				slug: 'contact',
				fields: [{ name: 'email', label: 'Email', field_type: 'email', required: true }],
				settings: { submit_text: 'Send' },
				is_published: true
			})
		});
	});
	await page.route('**/api/forms/contact/submit', async (route) => {
		submissions += 1;
		const submitted: unknown = route.request().postDataJSON();
		expect(submitted).toMatchObject({ email: 'reader@example.test' });
		await route.fulfill({
			status: 200,
			contentType: 'application/json',
			body: '{"success":true,"submission_id":"55"}'
		});
	});
	await page.route('**/api/forms/contact/view', async (route) => {
		await route.fulfill({ status: 200, contentType: 'application/json', body: '{"success":true}' });
	});
	await page.route('**/api/popups/7002/impression', async (route) => {
		impressions += 1;
		await route.fulfill({ status: 200, contentType: 'application/json', body: '{"success":true}' });
	});
	await page.route('**/api/popups/7002/conversion', async (route) => {
		conversions += 1;
		await route.fulfill({ status: 200, contentType: 'application/json', body: '{"success":true}' });
	});
	await page.goto('/about');
	await page.getByRole('dialog', { name: 'Cookie consent' }).locator('.btn-accept').click();
	const popup = page.getByRole('dialog', { name: 'Contact us' });
	await expect(popup).toBeVisible();
	await popup.getByRole('textbox', { name: 'Email' }).fill('reader@example.test');
	await popup.getByRole('button', { name: 'Send' }).click();
	await expect(popup.getByText('Form submitted successfully!')).toBeVisible();
	await expect.poll(() => submissions).toBe(1);
	await expect.poll(() => impressions).toBe(1);
	await expect.poll(() => conversions).toBe(1);
});
