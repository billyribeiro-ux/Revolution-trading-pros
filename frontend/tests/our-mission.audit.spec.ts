/**
 * Forensic audit — /our-mission animations & interactivity
 * Verifies the May-2026 component-extraction refactor preserved behavior:
 *   - All [data-gsap] markers reach opacity:1 (in viewport or via scroll)
 *   - MathOfEdgeCalculator updates derived values on input change
 *   - SyllabusAccordion opens/closes on click
 *   - MissionFaqAccordion opens/closes on click
 *   - No console errors during page lifecycle
 */
import { test, expect } from '@playwright/test';

const BASE = process.env.FRONTEND_URL || 'http://localhost:5173';

test.describe('our-mission forensic audit', () => {
	test('GSAP animates all [data-gsap] markers visible after full scroll', async ({ page }) => {
		const consoleErrors: string[] = [];
		page.on('pageerror', (err) => consoleErrors.push(`pageerror: ${err.message}`));
		page.on('console', (msg) => {
			if (msg.type() === 'error') consoleErrors.push(`console.error: ${msg.text()}`);
		});

		await page.goto(`${BASE}/our-mission`, { waitUntil: 'networkidle' });

		// Slow scroll to bottom to trigger every ScrollTrigger.batch onEnter
		await page.evaluate(async () => {
			const distance = document.body.scrollHeight;
			const step = 300;
			for (let y = 0; y < distance; y += step) {
				window.scrollTo(0, y);
				await new Promise((r) => setTimeout(r, 80));
			}
			window.scrollTo(0, distance);
			await new Promise((r) => setTimeout(r, 1500));
		});

		// Every [data-gsap] should now be opacity:1 (animated in)
		const opacities = await page.$$eval('[data-gsap]', (els) =>
			els.map((el) => parseFloat(getComputedStyle(el as HTMLElement).opacity))
		);
		expect(opacities.length).toBeGreaterThan(0);
		const stillHidden = opacities.filter((o) => o < 0.99);
		expect(stillHidden, `Hidden [data-gsap] count: ${stillHidden.length}/${opacities.length}`).toHaveLength(0);

		// No console errors (excluding the expected anonymous-user 401 from /api/auth/me)
		const realErrors = consoleErrors.filter(
			(e) => !/401.*Unauthorized/.test(e) && !/auth\/me/.test(e)
		);
		expect(realErrors).toEqual([]);
	});

	test('MathOfEdgeCalculator updates derived risk amount on input change', async ({ page }) => {
		await page.goto(`${BASE}/our-mission`, { waitUntil: 'networkidle' });

		// Calculator default: $25,000 * 2% = $500
		await page.locator('#sim-account').scrollIntoViewIfNeeded();
		const initialRisk = await page.getByText('Dollar Risk (1R)').locator('..').locator('div').nth(1).textContent();
		expect(initialRisk?.trim()).toBe('$500');

		// Change account size to $50,000
		await page.locator('#sim-account').fill('50000');
		await page.locator('#sim-account').blur();

		// New risk: $50,000 * 2% = $1,000
		const newRisk = await page.getByText('Dollar Risk (1R)').locator('..').locator('div').nth(1).textContent();
		expect(newRisk?.trim()).toBe('$1000');
	});

	test('SyllabusAccordion toggles on click', async ({ page }) => {
		await page.goto(`${BASE}/our-mission`, { waitUntil: 'networkidle' });

		// Module 1 is open by default (openIndex = 0)
		const m1Body = page.getByText('Understanding the Auction Process', { exact: false });
		await m1Body.scrollIntoViewIfNeeded();
		await expect(m1Body).toBeVisible();

		// Click Module 2 — opens it, closes Module 1
		const m2Button = page.getByRole('button', { name: /Module 2: Volume Profiling/ });
		await m2Button.click();

		const m2Body = page.getByText('Identifying value areas, POC', { exact: false });
		await expect(m2Body).toBeVisible();
		await expect(m1Body).not.toBeVisible();
	});

	test('MissionFaqAccordion toggles on click', async ({ page }) => {
		await page.goto(`${BASE}/our-mission`, { waitUntil: 'networkidle' });

		// All FAQ items closed by default (openIndex = null)
		const ans1 = page.getByText('Absolutely not. If you are looking', { exact: false });
		await expect(ans1).not.toBeVisible();

		// Click first question
		const q1 = page.getByRole('button', { name: /Get Rich Quick/ });
		await q1.scrollIntoViewIfNeeded();
		await q1.click();

		await expect(ans1).toBeVisible();

		// Click again — closes
		await q1.click();
		await expect(ans1).not.toBeVisible();
	});
});
