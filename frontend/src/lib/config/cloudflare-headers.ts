import { isNoindexPath, STATIC_NOINDEX_HOST_PATTERNS } from '../seo/config.js';

/** Keep the adapter/user cache headers and apply SEO policy to static responses too. */
export function appendStaticNoindexHeaders(
	existing: string,
	prerenderedPaths: readonly string[],
	base = ''
): string {
	const paths = prerenderedPaths.filter((path) => {
		const unbased = base && path.startsWith(base + '/') ? path.slice(base.length) : path;
		return isNoindexPath(unbased);
	});
	const rules = new Set<string>([...STATIC_NOINDEX_HOST_PATTERNS, ...paths]);
	for (const path of rules) {
		if (/[\r\n]/.test(path)) throw new Error('Invalid static header path');
	}
	const result =
		existing.trimEnd() +
		'\n\n' +
		[...rules].map((path) => `${path}\n  X-Robots-Tag: noindex, nofollow`).join('\n\n') +
		'\n';
	const lines = result.split('\n');
	const count = lines.filter((line) => line && !/^\s|#/.test(line)).length;
	if (count > 100) throw new Error(`Cloudflare header budget exceeded (${count}/100)`);
	if (lines.some((line) => line.length > 2000))
		throw new Error('Cloudflare header lines must be at most 2000 characters');
	return result;
}
