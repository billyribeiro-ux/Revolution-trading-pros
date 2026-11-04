import { expect, it } from 'vite-plus/test';
import { planCloudflareRoutes } from '../cloudflare-routes.js';
import { isAuthenticatedPath } from '../private-routes.js';
const base = {
	appPath: '_app',
	appDir: '_app',
	base: '',
	clientAssets: ['favicon.png', '_app/immutable/start.js'],
	redirectPaths: ['/signup']
};
it('keeps every public HTML/asset exclusion within budget while data/unknown/private requests retain worker routing', () => {
	const pages = Array.from({ length: 60 }, (_, n) => `/public-${n}`);
	const plan = planCloudflareRoutes({
		...base,
		prerenderedPaths: pages.flatMap((path) => [path, `${path}/__data.json`])
	});
	expect(plan.include).toEqual(['/*']);
	expect(plan.include.length + plan.exclude.length).toBeLessThanOrEqual(100);
	for (const page of pages) {
		expect(plan.exclude).toContain(page);
		expect(plan.exclude).not.toContain(`${page}/__data.json`);
		expect(plan.exclude).not.toContain(`${page}*`);
	}
	expect(plan.exclude).toContain('/favicon.png');
	expect(plan.exclude).toContain('/_app/immutable/*');
	expect(plan.exclude).toContain('/signup');
	for (const path of [
		'/api/admin/users',
		'/admin',
		'/dashboard',
		'/my/courses',
		'/checkout/plan',
		'/public-1/missing',
		'/blog/unknown'
	]) {
		expect(
			plan.exclude.some((rule) =>
				rule.endsWith('*') ? path.startsWith(rule.slice(0, -1)) : path === rule
			),
			path
		).toBe(false);
	}
});
it('rejects private prerendered pages, static files and redirects instead of bypassing runtime authentication', () => {
	for (const path of ['/admin', '/dashboard/account', '/my/courses', '/checkout/plan']) {
		expect(() => planCloudflareRoutes({ ...base, prerenderedPaths: [path] })).toThrow(
			'Authenticated path'
		);
		expect(() =>
			planCloudflareRoutes({
				...base,
				prerenderedPaths: [],
				clientAssets: [path.slice(1) + '/private.json']
			})
		).toThrow('Authenticated path');
		expect(() =>
			planCloudflareRoutes({ ...base, prerenderedPaths: [], redirectPaths: [path] })
		).toThrow('Authenticated path');
	}
});
it('fails an overflowing rule set and overlong rules rather than truncating them', () => {
	expect(() =>
		planCloudflareRoutes({
			...base,
			prerenderedPaths: Array.from({ length: 100 }, (_, n) => `/page-${n}`)
		})
	).toThrow('budget exceeded');
	expect(() =>
		planCloudflareRoutes({ ...base, prerenderedPaths: ['/' + 'a'.repeat(101)] })
	).toThrow('100 characters');
});
it('supports a configured base path and uses path-segment boundaries', () => {
	expect(() =>
		planCloudflareRoutes({
			...base,
			base: '/portal',
			appPath: 'portal/_app',
			prerenderedPaths: ['/portal/my/courses']
		})
	).toThrow('Authenticated path');
	expect(isAuthenticatedPath('/my/courses')).toBe(true);
	expect(isAuthenticatedPath('/mythology')).toBe(false);
	expect(isAuthenticatedPath('/administrator')).toBe(false);
});
