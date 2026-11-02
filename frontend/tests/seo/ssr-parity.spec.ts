import { test, expect } from '@playwright/test';
import { REPRESENTATIVE_URLS } from './test-helpers.js';

test.describe('SSR Parity & No Competing Head Tags (REND-01..06)', () => {
	for (const sample of REPRESENTATIVE_URLS) {
		test(`SSR parity on ${sample.path} (${sample.template})`, async ({ page, request }) => {
			const res = await request.get(sample.path);
			expect(res.status()).toBe(200);
			const ssrHtml = await res.text();

			const ssrTitleMatch = ssrHtml.match(/<title>([^<]+)<\/title>/i);
			expect(ssrTitleMatch?.[1]?.trim().length).toBeGreaterThan(0);

			const ssrH1Match = ssrHtml.match(/<h1[^>]*>([\s\S]*?)<\/h1>/i);
			const ssrH1Text = ssrH1Match?.[1]?.replace(/<[^>]+>/g, ' ').replace(/\s+/g, ' ').trim() ?? '';
			expect(ssrH1Text.length).toBeGreaterThan(0);

			const ssrCanonicals = ssrHtml.match(/<link[^>]+rel=["']canonical["'][^>]*>/gi) ?? [];
			expect(ssrCanonicals.length).toBe(1);

			const ssrMainMatch = ssrHtml.match(/<main[^>]*>([\s\S]*?)<\/main>/i);
			const ssrMainText = (ssrMainMatch?.[1] ?? ssrHtml)
				.replace(/<script[\s\S]*?<\/script>/gi, ' ')
				.replace(/<style[\s\S]*?<\/style>/gi, ' ')
				.replace(/<[^>]+>/g, ' ')
				.replace(/\s+/g, ' ')
				.trim();
			const ssrWordCount = ssrMainText.split(/\s+/).filter(Boolean).length;
			const minWords = sample.template === 'article' ? 300 : 100;
			expect(ssrWordCount).toBeGreaterThanOrEqual(minWords);

			const consoleErrors: string[] = [];
			page.on('console', (msg) => {
				if (msg.type() === 'error') consoleErrors.push(msg.text());
			});
			page.on('pageerror', (err) => {
				consoleErrors.push(err.message);
			});

			await page.goto(sample.path, { waitUntil: 'domcontentloaded' });

			const domTitle = await page.title();
			expect(domTitle.trim().length).toBeGreaterThan(0);

			const domH1Count = await page.locator('h1').count();
			expect(domH1Count).toBe(1);

			const domCanonicalCount = await page.locator('head link[rel="canonical"]').count();
			expect(domCanonicalCount).toBe(1);

			const domDescCount = await page.locator('head meta[name="description"]').count();
			expect(domDescCount).toBe(1);

			expect(consoleErrors).toEqual([]);
		});
	}
});

