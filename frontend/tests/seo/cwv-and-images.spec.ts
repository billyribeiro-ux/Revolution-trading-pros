import { test, expect } from '@playwright/test';
import { REPRESENTATIVE_URLS } from './test-helpers.js';

test.describe('Core Web Vitals (CLS) & Image Hygiene (PERF-01..07)', () => {
	for (const sample of REPRESENTATIVE_URLS) {
		test(`Image dimensions, alt text, above-fold eager loading & CLS <= 0.1 on ${sample.path}`, async ({
			page
		}) => {
			await page.goto(sample.path, { waitUntil: 'networkidle' });

			const imgAudit = await page.evaluate(() => {
				const vh = window.innerHeight || 900;
				return Array.from(document.querySelectorAll('img')).map((img) => {
					const rect = img.getBoundingClientRect();
					const style = window.getComputedStyle(img);
					const inMain = Boolean(img.closest('main'));
					const aboveFold =
						inMain && rect.top < vh && rect.bottom > 0 && rect.width > 0 && rect.height > 0;
					const hasDimensions =
						(img.hasAttribute('width') && img.hasAttribute('height')) ||
						Boolean(style.aspectRatio && style.aspectRatio !== 'auto');
					return {
						src: img.getAttribute('src') || '',
						hasAlt: img.hasAttribute('alt'),
						loading: img.getAttribute('loading'),
						aboveFold,
						hasDimensions
					};
				});
			});

			for (const img of imgAudit) {
				expect(img.hasAlt, `Missing alt attribute on ${img.src} at ${sample.path}`).toBe(true);
				expect(img.hasDimensions, `Missing dimensions on ${img.src} at ${sample.path}`).toBe(true);
				if (img.aboveFold) {
					expect(
						img.loading,
						`Above-the-fold image ${img.src} has loading="lazy" on ${sample.path}`
					).not.toBe('lazy');
				}
			}

			const cls = await page.evaluate(async () => {
				return await new Promise<number>((resolve) => {
					let total = 0;
					const obs = new PerformanceObserver((list) => {
						for (const entry of list.getEntries()) {
							const ls = entry as PerformanceEntry & { hadRecentInput?: boolean; value?: number };
							if (!ls.hadRecentInput && typeof ls.value === 'number') {
								total += ls.value;
							}
						}
					});
					try {
						obs.observe({ type: 'layout-shift', buffered: true });
					} catch {
						// ignore
					}
					setTimeout(() => {
						obs.disconnect();
						resolve(Number(total.toFixed(4)));
					}, 300);
				});
			});

			expect(cls, `CLS ${cls} exceeded 0.1 on ${sample.path}`).toBeLessThanOrEqual(0.1);
		});
	}
});
