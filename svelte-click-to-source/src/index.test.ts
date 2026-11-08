import { afterEach, describe, expect, it } from 'vitest';
import { createServer, type ViteDevServer } from 'vite';
import {
	chmodSync,
	existsSync,
	mkdtempSync,
	readFileSync,
	realpathSync,
	rmSync,
	symlinkSync,
	writeFileSync
} from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { preprocess } from 'svelte/compiler';
import { clickToSource, resolveSourceFile } from './index';
import { createPreprocessor } from './preprocessor';

const cleanup: Array<() => Promise<void> | void> = [];

afterEach(async () => {
	for (const dispose of cleanup.splice(0).reverse()) await dispose();
});

describe('click-to-source request boundary', () => {
	it('allows only real Svelte files within the project root', () => {
		const root = mkdtempSync(join(tmpdir(), 'click-source-root-'));
		const outside = mkdtempSync(join(tmpdir(), 'click-source-outside-'));
		cleanup.push(() => {
			rmSync(root, { recursive: true, force: true });
			rmSync(outside, { recursive: true, force: true });
		});
		writeFileSync(join(root, 'safe.svelte'), '<p>safe</p>');
		writeFileSync(join(outside, 'outside.svelte'), '<p>outside</p>');
		symlinkSync(join(outside, 'outside.svelte'), join(root, 'escape.svelte'));

		expect(resolveSourceFile(root, 'safe.svelte')).toBe(realpathSync(join(root, 'safe.svelte')));
		expect(resolveSourceFile(root, '../outside.svelte')).toBeNull();
		expect(resolveSourceFile(root, join(outside, 'outside.svelte'))).toBeNull();
		expect(resolveSourceFile(root, 'escape.svelte')).toBeNull();
		expect(resolveSourceFile(root, 'missing.svelte')).toBeNull();
		expect(resolveSourceFile(root, 'safe.ts')).toBeNull();
	});

	it('rejects GET and unauthorized POST before launching an editor', async () => {
		const root = mkdtempSync(join(tmpdir(), 'click-source-server-'));
		cleanup.push(() => rmSync(root, { recursive: true, force: true }));
		writeFileSync(join(root, 'safe.svelte'), '<p>safe</p>');
		let server: ViteDevServer | undefined;
		try {
			server = await createServer({
				root,
				configFile: false,
				server: { host: '127.0.0.1', port: 0 },
				plugins: [clickToSource({ editor: 'vscode' })]
			});
			cleanup.push(() => server?.close());
			await server.listen();
			const base = server.resolvedUrls?.local[0];
			expect(base).toBeDefined();
			const endpoint = new URL('/__click-to-source-open?file=safe.svelte&line=1&col=1', base);
			expect((await fetch(endpoint)).status).toBe(405);
			expect((await fetch(endpoint, { method: 'POST' })).status).toBe(403);
			expect(
				(await fetch(endpoint, { method: 'POST', headers: { 'X-Click-To-Source-Token': 'wrong' } }))
					.status
			).toBe(403);
		} finally {
			if (!server) rmSync(root, { recursive: true, force: true });
		}
	});

	it('annotates a project file using a safe relative path', async () => {
		const root = mkdtempSync(join(tmpdir(), 'click-source-preprocess-'));
		cleanup.push(() => rmSync(root, { recursive: true, force: true }));
		const filename = join(root, 'quoted"name.svelte');
		const result = await preprocess(
			'<div>hello</div>',
			createPreprocessor({ enabled: true, root }),
			{ filename }
		);
		expect(result.code).toContain('data-source="quoted&quot;name.svelte:1:1"');
	});

	it('opens a valid source without interpreting shell characters', async () => {
		const root = mkdtempSync(join(tmpdir(), 'click-source-exec-'));
		cleanup.push(() => rmSync(root, { recursive: true, force: true }));
		const filename = 'quoted"source.svelte';
		writeFileSync(join(root, filename), '<p>safe</p>');
		const marker = join(root, 'editor-args.json');
		const editor = join(root, 'fake-editor');
		writeFileSync(
			editor,
			`#!/usr/bin/env node\nrequire('node:fs').writeFileSync(${JSON.stringify(marker)}, JSON.stringify(process.argv.slice(2)))\n`
		);
		chmodSync(editor, 0o755);
		const server = await createServer({
			root,
			configFile: false,
			server: { host: '127.0.0.1', port: 0 },
			plugins: [clickToSource({ editorCommand: editor })]
		});
		cleanup.push(() => server.close());
		await server.listen();
		const transformed = await server.transformRequest('virtual:click-to-source');
		const token = transformed?.code.match(/var REQUEST_TOKEN = "([^"]+)"/)?.[1];
		expect(token).toBeDefined();
		const endpoint = new URL('/__click-to-source-open', server.resolvedUrls?.local[0]);
		endpoint.searchParams.set('file', filename);
		endpoint.searchParams.set('line', '3');
		endpoint.searchParams.set('col', '5');
		const response = await fetch(endpoint, {
			method: 'POST',
			headers: { 'X-Click-To-Source-Token': token ?? '' }
		});
		expect(response.status).toBe(200);
		await expect.poll(() => existsSync(marker)).toBe(true);
		expect(JSON.parse(readFileSync(marker, 'utf8'))).toEqual([
			'--goto',
			`${realpathSync(join(root, filename))}:3:5`
		]);

		endpoint.searchParams.set('file', '../outside.svelte');
		expect(
			(
				await fetch(endpoint, {
					method: 'POST',
					headers: { 'X-Click-To-Source-Token': token ?? '' }
				})
			).status
		).toBe(400);
	});
});
