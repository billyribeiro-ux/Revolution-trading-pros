import assert from 'node:assert/strict';
import { mkdtemp, readFile, readdir, stat, symlink, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { test } from 'node:test';
import sharp from 'sharp';
import { encodeWebp, inspectImage, optimizeWebp } from '../src/index.ts';
import { optimizeDirectory, watchDirectory } from '../src/files.ts';

void test('converts actual raster bytes and retains dimensions and transparency', async () => {
	const input = await sharp({
		create: { width: 1000, height: 400, channels: 4, background: '#ff000080' }
	})
		.png()
		.toBuffer();
	const output = await optimizeWebp(input);
	assert.equal(output.format, 'webp');
	assert.equal(output.usedOriginal, false);
	assert.ok(output.bytes.length < input.length);
	const decoded = await sharp(output.bytes).metadata();
	assert.equal(decoded.width, 1000);
	assert.equal(decoded.height, 400);
	assert.equal(decoded.hasAlpha, true);
});

void test('avoids a larger re-encode of an already small source', async () => {
	const input = await sharp({
		create: { width: 80, height: 40, channels: 3, background: '#123456' }
	})
		.webp({ quality: 1 })
		.toBuffer();
	const output = await optimizeWebp(input, { quality: 95 });
	assert.equal(output.usedOriginal, true);
	assert.deepEqual(output.bytes, input);
	assert.equal(output.savedBytes, 0);
});

void test('keeps animation frames and rejects vector input and oversized options', async () => {
	const red = await sharp({ create: { width: 8, height: 4, channels: 3, background: 'red' } })
		.png()
		.toBuffer();
	const blue = await sharp({ create: { width: 8, height: 4, channels: 3, background: 'blue' } })
		.png()
		.toBuffer();
	const animated = await sharp([red, blue], { join: { animated: true } })
		.gif()
		.toBuffer();
	const output = await encodeWebp(animated);
	assert.equal((await inspectImage(output.bytes)).pages, 2);
	await assert.rejects(encodeWebp(Buffer.from('<svg/>')), /raster images/);
	await assert.rejects(encodeWebp(red, { width: 8193 }), /width/);
});

void test('processes a directory, writes a manifest, and ignores symlinks', async () => {
	const root = await mkdtemp(join(tmpdir(), 'imagify-test-'));
	const source = join(root, 'source');
	const output = join(root, 'output');
	const { mkdir, rm } = await import('node:fs/promises');
	try {
		await mkdir(source);
		const png = await sharp({ create: { width: 400, height: 150, channels: 3, background: 'red' } })
			.png()
			.toBuffer();
		await writeFile(join(source, 'photo.png'), png);
		await symlink(join(source, 'photo.png'), join(source, 'alias.png'));
		const results = await optimizeDirectory(source, output);
		assert.equal(results.length, 1);
		assert.equal(results[0]?.source, join(source, 'photo.png'));
		assert.ok((await readFile(results[0]!.output)).length > 0);
		const manifest = JSON.parse(await readFile(join(output, 'imagify-manifest.json'), 'utf8')) as {
			results: unknown[];
		};
		assert.equal(manifest.results.length, 1);
	} finally {
		await rm(root, { recursive: true, force: true });
	}
});

void test('gives similar source names separate stable output paths', async () => {
	const root = await mkdtemp(join(tmpdir(), 'imagify-names-'));
	const source = join(root, 'source');
	const output = join(root, 'output');
	const { mkdir, rm } = await import('node:fs/promises');
	try {
		await mkdir(source);
		const png = await sharp({ create: { width: 300, height: 100, channels: 3, background: 'red' } })
			.png()
			.toBuffer();
		const webp = await sharp(png).webp({ quality: 1 }).toBuffer();
		await writeFile(join(source, 'photo.png'), png);
		await writeFile(join(source, 'photo.png.webp'), webp);
		const results = await optimizeDirectory(source, output);
		assert.equal(results.length, 2);
		assert.notEqual(results[0]?.output, results[1]?.output);
		for (const result of results) assert.ok((await readFile(result.output)).length > 0);
	} finally {
		await rm(root, { recursive: true, force: true });
	}
});

void test('watch mode converts newly added images without a manual command', async () => {
	const root = await mkdtemp(join(tmpdir(), 'imagify-watch-'));
	const source = join(root, 'downloads');
	const output = join(root, 'optimized');
	const { mkdir, rm } = await import('node:fs/promises');
	await mkdir(source);
	let stop: (() => void) | undefined;
	try {
		stop = await watchDirectory(source, output, {}, 250);
		const input = await sharp({
			create: { width: 400, height: 150, channels: 3, background: 'blue' }
		})
			.png()
			.toBuffer();
		await writeFile(join(source, 'download.png'), input);
		let files: string[] = [];
		for (let attempt = 0; attempt < 20; attempt++) {
			await new Promise((resolve) => setTimeout(resolve, 100));
			files = await readdir(output);
			if (files.some((file) => file.endsWith('.webp'))) break;
		}
		assert.ok(files.some((file) => file.endsWith('.webp')));
		const firstName = files.find(
			(file) => file.startsWith('download.png.') && file.endsWith('.webp')
		);
		assert.ok(firstName);
		const firstOutput = join(output, firstName);
		const firstModified = (await stat(firstOutput)).mtimeMs;
		await writeFile(join(source, 'second.png'), input);
		for (let attempt = 0; attempt < 20; attempt++) {
			await new Promise((resolve) => setTimeout(resolve, 100));
			if ((await readdir(output)).some((file) => file.startsWith('second.png.'))) break;
		}
		assert.ok((await readdir(output)).some((file) => file.startsWith('second.png.')));
		assert.equal((await stat(firstOutput)).mtimeMs, firstModified);
		await rm(join(source, 'download.png'));
		for (let attempt = 0; attempt < 20; attempt++) {
			await new Promise((resolve) => setTimeout(resolve, 100));
			if (!(await readdir(output)).includes(firstName)) break;
		}
		assert.ok(!(await readdir(output)).includes(firstName));
	} finally {
		stop?.();
		await rm(root, { recursive: true, force: true });
	}
});
