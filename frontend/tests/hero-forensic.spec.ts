import { test, expect } from '@playwright/test';

const BASE = process.env.FRONTEND_URL || 'http://localhost:5173';

test('forensic: hero animation timeline', async ({ page }) => {
	const events: { t: number; opacities: number[]; ys: number[] }[] = [];

	await page.goto(`${BASE}/our-mission`, { waitUntil: 'domcontentloaded' });

	const start = Date.now();
	// Sample for 2 seconds at 30 ms cadence
	for (let i = 0; i < 70; i++) {
		const snap = await page
			.evaluate(() => {
				const els = Array.from(document.querySelectorAll('section [data-gsap]'));
				return els.slice(0, 4).map((el) => {
					const cs = getComputedStyle(el as HTMLElement);
					const m = cs.transform.match(/matrix\([^)]*,\s*([\d.-]+)\)$/);
					return {
						op: parseFloat(cs.opacity),
						y: m ? parseFloat(m[1]) : 0
					};
				});
			})
			.catch(() => null);
		if (snap) {
			events.push({
				t: Date.now() - start,
				opacities: snap.map((s) => Math.round(s.op * 100) / 100),
				ys: snap.map((s) => Math.round(s.y))
			});
		}
		await page.waitForTimeout(30);
	}

	console.log('\n========= HERO ANIMATION TIMELINE (4 above-fold elements) =========');
	for (const ev of events) {
		const opStr = ev.opacities.map((o) => o.toFixed(2)).join(' ');
		const yStr = ev.ys.map((y) => String(y).padStart(3)).join(' ');
		console.log(`t=${String(ev.t).padStart(4)}ms  op=[${opStr}]  y=[${yStr}]`);
	}

	// Verify animation actually happened: at least one frame in the middle
	// must show a partially-faded state (opacity between 0 and 1) for the hero
	// elements. If they pop straight from 0→1 in one frame, the animation broke.
	const heroFrames = events.filter((e) => e.t < 1500);
	const sawPartial = heroFrames.some((e) =>
		e.opacities.slice(0, 2).some((o) => o > 0.05 && o < 0.95)
	);
	expect(sawPartial, 'Hero elements must show a partial-opacity frame during animation').toBe(true);
});
