import { isAuthenticatedPath } from './private-routes.js';
export interface PagesRoutePlan {
	include: string[];
	exclude: string[];
}
interface RouteInput {
	appPath: string;
	appDir: string;
	base: string;
	clientAssets: readonly string[];
	prerenderedPaths: readonly string[];
	redirectPaths: readonly string[];
}
/** Fit Cloudflare's combined 100-rule budget without broad page wildcards or silent drops. */
export function planCloudflareRoutes(input: RouteInput): PagesRoutePlan {
	const unbase = (path: string) =>
		input.base && path.startsWith(input.base + '/') ? path.slice(input.base.length) : path;
	const assertPublic = (path: string) => {
		if (isAuthenticatedPath(unbase(path).replace(/\/\*$/, '')))
			throw new Error(`Authenticated path cannot bypass the worker: ${path}`);
	};
	const exclude = new Set([`/${input.appPath}/version.json`, `/${input.appPath}/immutable/*`]);
	for (const file of input.clientAssets) {
		if (file.startsWith(input.appDir + '/')) continue;
		const path = input.base + '/' + file;
		assertPublic(path);
		exclude.add(path);
	}
	for (const path of input.prerenderedPaths) {
		assertPublic(path);
		// Public HTML gets direct asset serving. Data requests retain worker routing,
		// preserving exact page boundaries and room for all HTML/assets in the budget.
		if (!path.endsWith('/__data.json')) exclude.add(path);
	}
	for (const path of input.redirectPaths) {
		assertPublic(path);
		exclude.add(path);
	}
	const plan = { include: ['/*'], exclude: [...exclude] };
	if (plan.include.length + plan.exclude.length > 100)
		throw new Error(
			`Cloudflare route budget exceeded (${1 + plan.exclude.length}/100); review exact public exclusions`
		);
	if ([...plan.include, ...plan.exclude].some((rule) => rule.length > 100))
		throw new Error('Cloudflare route rules must be at most 100 characters');
	return plan;
}
