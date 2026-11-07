import { once } from 'node:events';
import { request as httpRequest, type Server } from 'node:http';
import sharp from 'sharp';
import { describe, expect, it } from 'vite-plus/test';
import { createProcessor, type ProcessorOptions } from '../src/app.ts';

const token = 'only-a-disposable-processor-test-secret';
const headers = { Authorization: `Bearer ${token}`, 'Content-Type': 'application/octet-stream' };
const image = () =>
	sharp({
		create: { width: 80, height: 40, channels: 4, background: { r: 180, g: 40, b: 10, alpha: 0.5 } }
	})
		.png()
		.toBuffer();
async function withProcessor(
	options: Partial<ProcessorOptions>,
	run: (origin: string, server: Server) => Promise<void>
) {
	const server = createProcessor({ token, ...options });
	server.listen(0, '127.0.0.1');
	await once(server, 'listening');
	const address = server.address();
	if (!address || typeof address === 'string') throw new Error('Expected a local test socket');
	try {
		await run(`http://127.0.0.1:${address.port}`, server);
	} finally {
		server.closeAllConnections();
		await new Promise<void>((resolve, reject) => {
			server.close((error) => {
				if (error) reject(error);
				else resolve();
			});
		});
	}
}

describe('Sharp processor HTTP contract', () => {
	it('requires an explicit token and rejects unauthenticated requests', async () => {
		expect(() => createProcessor({ token: '' })).toThrow('MEDIA_PROCESSOR_TOKEN');
		await withProcessor({}, async (origin) => {
			expect((await fetch(`${origin}/health`)).status).toBe(401);
			const response = await fetch(`${origin}/health`, { headers });
			expect(await response.json()).toMatchObject({
				available: true,
				engine: 'sharp',
				version: '0.35.5'
			});
		});
	});
	it('inspects actual raster bytes and returns their dimensions and size', async () => {
		await withProcessor({}, async (origin) => {
			const input = await image();
			const response = await fetch(`${origin}/inspect`, { method: 'POST', headers, body: input });
			expect(response.status).toBe(200);
			expect(await response.json()).toMatchObject({
				width: 80,
				height: 40,
				pages: 1,
				animated: false,
				size: input.length,
				format: 'png'
			});
		});
	});
	it('encodes resized WebP bytes with actual dimensions and preserved transparency', async () => {
		await withProcessor({}, async (origin) => {
			const response = await fetch(`${origin}/process?format=webp&width=40&lossless=true`, {
				method: 'POST',
				headers,
				body: await image()
			});
			expect(response.status).toBe(200);
			const data = Buffer.from(await response.arrayBuffer());
			const actual = await sharp(data).metadata();
			expect(actual).toMatchObject({ width: 40, height: 20, format: 'webp', hasAlpha: true });
			expect(response.headers.get('x-image-width')).toBe('40');
			expect(response.headers.get('content-length')).toBe(String(data.length));
		});
	});
	it('produces decodable AVIF and retains original size when enlargement is requested', async () => {
		await withProcessor({}, async (origin) => {
			const response = await fetch(`${origin}/process?format=avif&width=160&quality=60`, {
				method: 'POST',
				headers,
				body: await image()
			});
			expect(response.status).toBe(200);
			expect(await sharp(Buffer.from(await response.arrayBuffer())).metadata()).toMatchObject({
				width: 80,
				height: 40,
				format: 'heif',
				compression: 'av1'
			});
		});
	});
	it('preserves animation frames and rejects formats that would silently flatten them', async () => {
		const first = await sharp({ create: { width: 8, height: 4, channels: 3, background: 'red' } })
			.png()
			.toBuffer();
		const second = await sharp({ create: { width: 8, height: 4, channels: 3, background: 'blue' } })
			.png()
			.toBuffer();
		const animated = await sharp([first, second], { join: { animated: true } })
			.gif({ delay: [100, 100], loop: 0 })
			.toBuffer();
		expect((await sharp(animated, { animated: true }).metadata()).pages).toBe(2);
		await withProcessor({}, async (origin) => {
			const response = await fetch(`${origin}/process?format=webp`, {
				method: 'POST',
				headers,
				body: animated
			});
			expect(response.status).toBe(200);
			expect(
				await sharp(Buffer.from(await response.arrayBuffer()), { animated: true }).metadata()
			).toMatchObject({ pages: 2, pageHeight: 4 });
			expect(
				(await fetch(`${origin}/process?format=jpeg`, { method: 'POST', headers, body: animated }))
					.status
			).toBe(422);
		});
	});
	it('rejects oversized, invalid and vector inputs instead of processing paths or URLs', async () => {
		const compact = await sharp({
			create: { width: 80, height: 40, channels: 3, background: '#123456' }
		})
			.webp({ quality: 1 })
			.toBuffer();
		await withProcessor({ maxBytes: compact.length + 1 }, async (origin) => {
			const response = await fetch(`${origin}/process?format=webp&quality=100`, {
				method: 'POST',
				headers,
				body: compact
			});
			expect(response.status).toBe(413);
		});
		await withProcessor({ maxBytes: 64 }, async (origin) => {
			expect(
				(await fetch(`${origin}/process`, { method: 'POST', headers, body: Buffer.alloc(65) }))
					.status
			).toBe(413);
			expect(
				(await fetch(`${origin}/process`, { method: 'POST', headers, body: '<svg></svg>' })).status
			).toBe(415);
			expect(
				(
					await fetch(`${origin}/process?url=http://127.0.0.1`, {
						method: 'POST',
						headers,
						body: 'x'
					})
				).status
			).toBe(400);
		});
		await withProcessor({ maxPixels: 100 }, async (origin) => {
			expect(
				(await fetch(`${origin}/process`, { method: 'POST', headers, body: await image() })).status
			).toBe(422);
		});
	});
	it('validates format, geometry and lossless options', async () => {
		await withProcessor({}, async (origin) => {
			for (const query of [
				'format=unsupported',
				'width=0',
				'width=9000',
				'quality=101',
				'lossless=maybe',
				'format=jpeg&lossless=true'
			]) {
				expect(
					(
						await fetch(`${origin}/process?${query}`, {
							method: 'POST',
							headers,
							body: await image()
						})
					).status
				).toBe(400);
			}
		});
	});
	it('bounds concurrent work and accepts another request after the first completes', async () => {
		await withProcessor({ maxConcurrent: 1 }, async (origin, server) => {
			const input = await image();
			const received = once(server, 'request');
			const first = httpRequest(`${origin}/process`, {
				method: 'POST',
				headers: { ...headers, 'Content-Length': input.length }
			});
			const complete = new Promise<number>((resolve, reject) => {
				first.on('error', reject);
				first.on('response', (response) => {
					response.resume();
					response.once('end', () => resolve(response.statusCode ?? 0));
				});
			});
			first.flushHeaders();
			await received;
			const busy = await fetch(`${origin}/process`, { method: 'POST', headers, body: input });
			first.end(input);
			expect(await complete).toBe(200);
			expect(busy.status).toBe(503);
			expect(
				(await fetch(`${origin}/process`, { method: 'POST', headers, body: input })).status
			).toBe(200);
		});
	});
});
