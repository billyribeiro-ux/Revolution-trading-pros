#!/usr/bin/env node
/**
 * Generates tests/fixtures/sveltekit-routes.ts from src/routes/+page.svelte files.
 * Run: node scripts/generate-sveltekit-e2e-routes.mjs
 */
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.join(__dirname, '..');
const routesRoot = path.join(root, 'src/routes');
const outFile = path.join(root, 'tests/fixtures/sveltekit-routes.ts');

function walk(dir) {
	const out = [];
	for (const name of fs.readdirSync(dir)) {
		if (name.startsWith('.')) continue;
		const p = path.join(dir, name);
		const st = fs.statSync(p);
		if (st.isDirectory()) {
			out.push(...walk(p));
		} else if (name === '+page.svelte') {
			out.push(dir);
		}
	}
	return out;
}

function toUrlPath(routeDir) {
	let rel = path.relative(routesRoot, routeDir);
	if (!rel || rel === '.') return '/';
	const parts = rel.split(path.sep).filter((seg) => !(seg.startsWith('(') && seg.endsWith(')')));
	const urlParts = [];
	for (const seg of parts) {
		if (seg.startsWith('[...')) {
			urlParts.push('catch-all-segment');
		} else if (seg.startsWith('[') && seg.endsWith(']')) {
			const inner = seg.slice(1, -1);
			if (inner === 'hash') urlParts.push('test-verification-hash');
			else if (inner === 'id' || inner.endsWith('_id')) urlParts.push('00000000-0000-0000-0000-000000000001');
			else if (inner.includes('slug')) urlParts.push('sample-slug');
			else if (inner.includes('date')) urlParts.push('2024-01-01');
			else urlParts.push('param');
		} else {
			urlParts.push(seg);
		}
	}
	return '/' + urlParts.join('/');
}

const dirs = walk(routesRoot);
const urls = [...new Set(dirs.map(toUrlPath))].sort();

const header = `/**
 * Auto-generated list of SvelteKit page routes for E2E smoke coverage.
 * Regenerate: node scripts/generate-sveltekit-e2e-routes.mjs
 */
export const SVELTEKIT_PAGE_ROUTES = `;

fs.writeFileSync(outFile, `${header}${JSON.stringify(urls, null, '\t')} as const;\n`, 'utf8');
console.log(`Wrote ${urls.length} routes to ${path.relative(root, outFile)}`);
