import { test, expect } from '@playwright/test';
import { REPRESENTATIVE_URLS } from './test-helpers.js';

const RETIRED_SCHEMA_TYPES = new Set([
	'BookActions',
	'CourseInfo',
	'ClaimReview',
	'EstimatedSalary',
	'LearningVideo',
	'SpecialAnnouncement',
	'VehicleListing',
	'PracticeProblems',
	'Dataset',
	'HowTo',
	'FAQPage'
]);

test.describe('Structured Data / JSON-LD Validity & Visible Parity (SD-01..15)', () => {
	for (const sample of REPRESENTATIVE_URLS) {
		test(`JSON-LD validity & visible parity on ${sample.path} (${sample.template})`, async ({
			page
		}) => {
			await page.goto(sample.path, { waitUntil: 'domcontentloaded' });

			const rawBlocks = await page
				.locator('script[type="application/ld+json"]')
				.evaluateAll((els) => els.map((e) => e.textContent || ''));
			expect(rawBlocks.length).toBeGreaterThanOrEqual(1);

			const allNodes: Record<string, unknown>[] = [];
			for (const raw of rawBlocks) {
				expect(raw).not.toContain('</script');
				const parsed = JSON.parse(raw) as Record<string, unknown>;
				if (Array.isArray(parsed['@graph'])) {
					for (const item of parsed['@graph']) {
						if (item && typeof item === 'object') {
							allNodes.push(item as Record<string, unknown>);
						}
					}
				} else {
					allNodes.push(parsed);
				}
			}

			const types = allNodes.map((n) => String(n['@type'] || ''));
			for (const t of types) {
				expect(RETIRED_SCHEMA_TYPES.has(t), `Retired schema type ${t} on ${sample.path}`).toBe(
					false
				);
			}

			for (const n of allNodes) {
				expect(n['aggregateRating'], `Fabricated aggregateRating on ${sample.path}`).toBeUndefined();
			}

			if (sample.path === '/') {
				expect(types).toContain('Organization');
				expect(types).toContain('WebSite');
			} else {
				expect(types).toContain('BreadcrumbList');
				const breadcrumbNav = page.locator('nav[aria-label="Breadcrumb"]');
				await expect(breadcrumbNav).toHaveCount(1);
			}

			if (sample.template === 'article') {
				const hasArticle = types.some((t) =>
					['Article', 'BlogPosting', 'NewsArticle'].includes(t)
				);
				expect(hasArticle).toBe(true);
				await expect(page.locator('time[datetime]').first()).toBeVisible();
			}

			if (sample.template === 'product') {
				expect(types).toContain('Product');
				const productNode = allNodes.find((n) => n['@type'] === 'Product');
				expect(productNode?.['offers']).toBeDefined();
			}

			if (sample.template === 'profile') {
				expect(types).toContain('ProfilePage');
			}

			if (sample.template === 'course') {
				expect(types).toContain('Course');
			}

			if (sample.template === 'software') {
				expect(types).toContain('SoftwareApplication');
			}
		});
	}
});

