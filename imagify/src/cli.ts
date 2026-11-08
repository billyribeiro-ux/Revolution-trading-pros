#!/usr/bin/env node
import { lstat } from 'node:fs/promises';
import { resolve } from 'node:path';
import { optimizeDirectory, optimizeFile, watchDirectory } from './files.ts';
import type { WebpOptions } from './index.ts';

async function main() {
	const [input, output, ...flags] = process.argv.slice(2);
	if (!input || !output) {
		throw new Error(
			'Usage: imagify <input-file-or-directory> <output-directory> [--watch] [--quality=1..100] [--effort=0..6]'
		);
	}
	const options: WebpOptions = {};
	let watch = false;
	for (const flag of flags) {
		if (flag === '--watch') watch = true;
		else if (flag.startsWith('--quality=')) options.quality = Number(flag.slice(10));
		else if (flag.startsWith('--effort=')) options.effort = Number(flag.slice(9));
		else throw new Error(`Unknown option: ${flag}`);
	}
	const source = resolve(input);
	const target = resolve(output);
	const stat = await lstat(source);
	if (stat.isSymbolicLink()) throw new Error('Symbolic link inputs are not supported');
	if (watch && !stat.isDirectory()) throw new Error('--watch requires a directory');
	if (watch) {
		await watchDirectory(source, target, options);
		console.log(`Watching ${source} for images`);
	} else if (stat.isDirectory()) {
		const results = await optimizeDirectory(source, target, options);
		console.log(
			JSON.stringify({
				images: results.length,
				bytesSaved: results.reduce((sum, item) => sum + item.savedBytes, 0),
				results
			})
		);
	} else {
		console.log(JSON.stringify(await optimizeFile(source, target, options)));
	}
}

main().catch((error: unknown) => {
	console.error(error instanceof Error ? error.message : error);
	process.exitCode = 1;
});
