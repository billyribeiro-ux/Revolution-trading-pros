import { readFile } from 'node:fs/promises';
import { basename } from 'node:path';
import { performance } from 'node:perf_hooks';
import sharp from 'sharp';
import { encodeWebp, inspectImage } from './index.ts';

const paths = process.argv.slice(2);
if (paths.length === 0) {
	console.error('Usage: pnpm --filter @revolution/imagify bench -- <image> [image...]');
	process.exitCode = 1;
} else {
	for (const path of paths) {
		const input = await readFile(path);
		const info = await inspectImage(input);
		const cases: Array<{ name: string; run: () => Promise<Buffer> }> = [
			{
				name: 'webp-q80-effort0',
				run: async () => (await encodeWebp(input, { quality: 80, effort: 0 })).bytes
			},
			{
				name: 'webp-q80-effort4',
				run: async () => (await encodeWebp(input, { quality: 80, effort: 4 })).bytes
			},
			{
				name: 'webp-q80-effort6',
				run: async () => (await encodeWebp(input, { quality: 80, effort: 6 })).bytes
			}
		];
		if (!info.animated) {
			cases.push({
				name: 'avif-q60-effort4',
				run: () => sharp(input).rotate().avif({ quality: 60, effort: 4 }).toBuffer()
			});
		}
		for (const candidate of cases) {
			const start = performance.now();
			const output = await candidate.run();
			const milliseconds = performance.now() - start;
			console.log(
				JSON.stringify({
					file: basename(path),
					originalBytes: input.length,
					encoder: candidate.name,
					outputBytes: output.length,
					savedPercent: Number((100 * (1 - output.length / input.length)).toFixed(2)),
					encodeMs: Number(milliseconds.toFixed(2))
				})
			);
		}
	}
}
