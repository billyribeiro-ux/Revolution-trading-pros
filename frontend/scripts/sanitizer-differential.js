#!/usr/bin/env node
/**
 * Real-browser differential check for the isomorphic HTML sanitizer.
 *
 * The sanitizer parses with parse5 on the server and with the browser's own
 * `DOMParser` on the client, then runs identical code, so server-rendered
 * `{@html}` content should be byte-identical to what the client computes. The
 * Vitest suites can only compare against jsdom, which is itself parse5. This
 * script compares against real Chromium:
 *
 * 1. Bundles the shipped facades (`#lib/utils/sanitize.js`,
 *    `#lib/utils/sanitization.js`) twice with esbuild — once with the default
 *    conditions (server parser), once with `browser` (DOMParser) — exactly as
 *    the package.json `imports` mapping resolves them in the real builds.
 * 2. Runs the corpus (`differential-corpus.json` plus the Vitest fixtures)
 *    through every policy in Node and in Chromium and compares the strings.
 *    Inputs containing `<select>` are exempt from the equality check: Chromium
 *    implements relaxed select parsing that parse5 8 does not yet, which can
 *    move where a malformed select ends (both outputs are still sanitized).
 * 3. Inserts every browser output into a live document and fails on any event
 *    handler attribute, script-capable URL, forbidden element, or dialog.
 *
 * Usage (from frontend/): pnpm test:sanitizer:browser
 * Set CHROMIUM_PATH to use a specific Chromium (e.g. /opt/pw-browsers/chromium);
 * otherwise Playwright's own download is used (`pnpm exec playwright install chromium`).
 */

import { mkdtempSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { build } from 'esbuild';
import { chromium } from 'playwright';

const FRONTEND = fileURLToPath(new URL('..', import.meta.url));

const ENTRY = `
import { sanitizeHtml, stripHtml } from '#lib/utils/sanitize.js';
import { sanitizeHTML } from '#lib/utils/sanitization.js';
import { CASES, XSS_PAYLOADS } from '#lib/utils/html-sanitizer/__tests__/fixtures.js';
import corpus from '#lib/utils/html-sanitizer/__tests__/differential-corpus.json';

export const inputs = [...new Set([...corpus, ...CASES.map((c) => c.html), ...XSS_PAYLOADS])];

export function run(list) {
	return list.map((html) => ({
		strict: sanitizeHtml(html, 'strict'),
		standard: sanitizeHtml(html, 'standard'),
		rich: sanitizeHtml(html, 'rich'),
		minimal: sanitizeHtml(html, 'minimal'),
		modeDefault: sanitizeHTML(html),
		modeStrict: sanitizeHTML(html, { mode: 'strict' }),
		modeCustom: sanitizeHTML(html, { mode: 'custom' }),
		text: stripHtml(html)
	}));
}
`;

async function bundle(outfile, options) {
	await build({
		stdin: {
			contents: ENTRY,
			resolveDir: FRONTEND,
			sourcefile: 'differential-entry.ts',
			loader: 'ts'
		},
		bundle: true,
		outfile,
		logLevel: 'error',
		// `#lib/utils/logger.js` reads Vite's import.meta.env.
		define: { 'import.meta.env': '{"DEV":false}' },
		...options
	});
}

/** Evaluated in the page: audits the live DOM built from each output. */
function auditInPage(outputs) {
	const problems = [];
	const forbidden = new Set([
		'SCRIPT',
		'STYLE',
		'OBJECT',
		'EMBED',
		'BASE',
		'META',
		'LINK',
		'FORM',
		'INPUT',
		'TEMPLATE',
		'NOSCRIPT',
		'svg',
		'math'
	]);
	const urlAttributes = new Set([
		'href',
		'src',
		'xlink:href',
		'action',
		'formaction',
		'data',
		'poster',
		'cite',
		'srcdoc'
	]);
	const sink = document.createElement('div');
	document.body.append(sink);
	outputs.forEach((output, index) => {
		for (const [policy, html] of Object.entries(output)) {
			if (policy === 'text') continue;
			sink.innerHTML = html;
			for (const element of sink.querySelectorAll('*')) {
				if (forbidden.has(element.tagName))
					problems.push(`#${index} ${policy}: <${element.tagName}>`);
				for (const attribute of element.attributes) {
					// eslint-disable-next-line no-control-regex -- WHATWG URL whitespace/C0 stripping
					const value = attribute.value.replace(/[\u0000- ]/g, '').toLowerCase();
					if (/^on/i.test(attribute.name)) problems.push(`#${index} ${policy}: ${attribute.name}`);
					if (
						urlAttributes.has(attribute.name.toLowerCase()) &&
						/^(javascript|vbscript|data:text)/.test(value)
					) {
						problems.push(`#${index} ${policy}: ${attribute.name}=${attribute.value}`);
					}
				}
			}
		}
	});
	sink.remove();
	return problems;
}

const work = mkdtempSync(join(tmpdir(), 'sanitizer-differential-'));
try {
	const serverFile = join(work, 'server.mjs');
	const browserFile = join(work, 'browser.js');
	await bundle(serverFile, { platform: 'node', format: 'esm' });
	await bundle(browserFile, {
		platform: 'browser',
		format: 'iife',
		globalName: 'Differential',
		conditions: ['browser']
	});

	const browserSource = readFileSync(browserFile, 'utf8');
	if (browserSource.includes('scriptingEnabled')) throw new Error('browser bundle contains parse5');
	if (!browserSource.includes('parseFromString')) throw new Error('browser bundle lacks DOMParser');

	const server = await import(pathToFileURL(serverFile).href);
	const inputs = server.inputs;
	const serverOutputs = server.run(inputs);

	const executablePath = process.env.CHROMIUM_PATH;
	const browser = await chromium.launch(executablePath ? { executablePath } : {});
	const dialogs = [];
	let browserOutputs;
	let problems;
	let version;
	try {
		const page = await browser.newPage();
		page.on('dialog', async (dialog) => {
			dialogs.push(dialog.message());
			await dialog.dismiss();
		});
		await page.setContent('<!doctype html><html><head></head><body></body></html>');
		await page.addScriptTag({ content: browserSource });
		browserOutputs = await page.evaluate((list) => globalThis.Differential.run(list), inputs);
		problems = await page.evaluate(auditInPage, browserOutputs);
		await page.waitForTimeout(300);
		version = browser.version();
	} finally {
		await browser.close();
	}

	let compared = 0;
	let exempt = 0;
	const mismatches = [];
	inputs.forEach((input, index) => {
		const isSelect = /<select/i.test(input);
		for (const policy of Object.keys(serverOutputs[index])) {
			if (isSelect) {
				exempt++;
				continue;
			}
			compared++;
			const expected = serverOutputs[index][policy];
			const actual = browserOutputs[index][policy];
			if (expected !== actual) {
				let at = 0;
				while (at < expected.length && expected[at] === actual[at]) at++;
				mismatches.push({
					index,
					policy,
					input: input.slice(0, 60),
					lengths: [expected.length, actual.length],
					firstDifference: at,
					server: expected.slice(Math.max(0, at - 40), at + 40),
					browser: actual.slice(Math.max(0, at - 40), at + 40)
				});
			}
		}
	});

	console.log(`Chromium ${version}: ${inputs.length} inputs`);
	console.log(
		`  server vs browser: ${compared} compared, ${mismatches.length} different, ${exempt} exempt (<select>)`
	);
	console.log(`  live-DOM audit: ${problems.length} problems, ${dialogs.length} dialogs`);
	for (const mismatch of mismatches.slice(0, 20))
		console.log('  MISMATCH', JSON.stringify(mismatch));
	for (const problem of problems.slice(0, 20)) console.log('  AUDIT', problem);
	process.exitCode = mismatches.length > 0 || problems.length > 0 || dialogs.length > 0 ? 1 : 0;
} finally {
	rmSync(work, { recursive: true, force: true });
}
// esbuild's service would otherwise keep the process alive.
process.exit();
