#!/usr/bin/env node
/**
 * Runs `svelte-mcp svelte-autofixer` on every .svelte file under ./src (single-file tool).
 * Usage: node scripts/run-svelte-autofixer.mjs [--strict] [--limit N]
 *   --strict  exit 1 if any file has suggestions (default: only `issues` fail the run)
 *   --limit N process only the first N files (sorted paths) for a quick sample
 */
import { execFileSync } from 'node:child_process';
import { readdir } from 'node:fs/promises';
import { dirname, join, relative } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const frontendRoot = join(__dirname, '..');
const srcRoot = join(frontendRoot, 'src');

const strict = process.argv.includes('--strict');
const limitArg = process.argv.find((a) => a.startsWith('--limit='));
const limitIdx = process.argv.indexOf('--limit');
const limit =
	limitArg != null
		? Number.parseInt(limitArg.slice('--limit='.length), 10)
		: limitIdx !== -1 && process.argv[limitIdx + 1]
			? Number.parseInt(process.argv[limitIdx + 1], 10)
			: NaN;
const fileLimit = Number.isFinite(limit) && limit > 0 ? limit : 0;

/** @param {string} dir */
async function* walkSvelte(dir) {
	const entries = await readdir(dir, { withFileTypes: true });
	for (const e of entries) {
		const p = join(dir, e.name);
		if (e.name === 'node_modules' || e.name === '.svelte-kit') continue;
		if (e.isDirectory()) yield* walkSvelte(p);
		else if (e.isFile() && e.name.endsWith('.svelte')) yield p;
	}
}

/** svelte-mcp prints a JS object literal (not JSON: unquoted keys, single-quoted strings). */
/** @param {string} stdout */
function parseAutofixerOutput(stdout) {
	const i = stdout.indexOf('{');
	if (i === -1) throw new Error('No object in output');
	const literal = stdout.slice(i).trim();
	return /** @type {{ issues?: unknown[]; suggestions?: string[] }} */ (
		new Function(`return (${literal})`)()
	);
}

function runOne(filePath) {
	const out = execFileSync(
		'pnpm',
		['exec', 'svelte-mcp', 'svelte-autofixer', filePath, '--svelte-version', '5'],
		{
			cwd: frontendRoot,
			encoding: 'utf8',
			maxBuffer: 10 * 1024 * 1024
		}
	);
	return parseAutofixerOutput(out);
}

async function main() {
	const files = [];
	for await (const f of walkSvelte(srcRoot)) files.push(f);
	files.sort();
	if (fileLimit > 0) files.length = Math.min(files.length, fileLimit);

	let issueCount = 0;
	let suggestionCount = 0;

	for (const abs of files) {
		const rel = relative(frontendRoot, abs);
		let result;
		try {
			result = runOne(abs);
		} catch (e) {
			console.error(`${rel}: ${e instanceof Error ? e.message : e}`);
			process.exitCode = 1;
			continue;
		}
		const issues = result.issues ?? [];
		const suggestions = result.suggestions ?? [];
		if (issues.length || suggestions.length) {
			console.log(`\n${rel}`);
			if (issues.length) {
				issueCount += issues.length;
				for (const x of issues)
					console.log(`  issue: ${typeof x === 'string' ? x : JSON.stringify(x)}`);
			}
			if (suggestions.length) {
				suggestionCount += suggestions.length;
				for (const s of suggestions) console.log(`  suggestion: ${s}`);
			}
		}
	}

	const totalNote =
		fileLimit > 0 ? ` (limited to first ${fileLimit} of full tree)` : '';
	console.log(
		`\nsvelte-autofixer: ${files.length} files${totalNote}, ${issueCount} issue(s), ${suggestionCount} suggestion(s)`
	);

	if (issueCount > 0) process.exitCode = 1;
	else if (strict && suggestionCount > 0) process.exitCode = 1;
}

main().catch((e) => {
	console.error(e);
	process.exit(1);
});
