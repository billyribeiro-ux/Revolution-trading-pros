import { createHash, timingSafeEqual } from 'node:crypto';
import { createServer, type IncomingMessage, type ServerResponse } from 'node:http';
import sharp from 'sharp';

export const MAX_BYTES = 50 * 1024 * 1024;
export const MAX_PIXELS = 16_000_000;
sharp.cache({ memory: 64, files: 0, items: 32 });
sharp.concurrency(2);

export interface ProcessorOptions {
	token: string;
	maxBytes?: number;
	maxPixels?: number;
	maxConcurrent?: number;
}
class HttpError extends Error {
	readonly status: number;
	constructor(status: number, message: string) {
		super(message);
		this.status = status;
	}
}
function integer(raw: string | null, fallback: number | undefined, maximum: number) {
	if (raw === null) return fallback;
	const value = Number(raw);
	if (!Number.isSafeInteger(value) || value < 1 || value > maximum)
		throw new HttpError(400, `Expected an integer from 1 to ${maximum}`);
	return value;
}
function json(response: ServerResponse, status: number, body: unknown) {
	response.writeHead(status, {
		'Content-Type': 'application/json',
		'Cache-Control': 'no-store',
		'X-Content-Type-Options': 'nosniff'
	});
	response.end(JSON.stringify(body));
}
async function body(request: IncomingMessage, maximum: number) {
	const declared = request.headers['content-length'];
	if (declared !== undefined && (!/^\d+$/.test(declared) || Number(declared) > maximum))
		throw new HttpError(413, 'Image exceeds the input size limit');
	const chunks: Buffer[] = [];
	let size = 0;
	for await (const chunk of request) {
		const data = typeof chunk === 'string' ? Buffer.from(chunk) : Buffer.from(chunk as Uint8Array);
		size += data.length;
		if (size > maximum) throw new HttpError(413, 'Image exceeds the input size limit');
		chunks.push(data);
	}
	if (size === 0) throw new HttpError(400, 'Image body is required');
	return Buffer.concat(chunks, size);
}
function isRaster(data: Buffer) {
	return (
		(data[0] === 0xff && data[1] === 0xd8 && data[2] === 0xff) ||
		data.subarray(0, 8).equals(Buffer.from('89504e470d0a1a0a', 'hex')) ||
		['GIF87a', 'GIF89a'].includes(data.subarray(0, 6).toString('ascii')) ||
		(data.subarray(0, 4).toString('ascii') === 'RIFF' &&
			data.subarray(8, 12).toString('ascii') === 'WEBP') ||
		(data.subarray(4, 8).toString('ascii') === 'ftyp' &&
			['avif', 'avis'].includes(data.subarray(8, 12).toString('ascii')))
	);
}

/** Processes bytes only. It never fetches a URL or opens a caller-supplied path. */
export function createProcessor(options: ProcessorOptions) {
	if (!/^[\x21-\x7e]{32,256}$/.test(options.token))
		throw new Error('MEDIA_PROCESSOR_TOKEN must contain 32–256 visible ASCII characters');
	const maxBytes = options.maxBytes ?? MAX_BYTES;
	const maxPixels = options.maxPixels ?? MAX_PIXELS;
	const maxConcurrent = options.maxConcurrent ?? 1;
	for (const [value, maximum] of [
		[maxBytes, MAX_BYTES],
		[maxPixels, MAX_PIXELS],
		[maxConcurrent, 4]
	]) {
		if (!Number.isSafeInteger(value) || value < 1 || value > maximum)
			throw new Error('Invalid processor resource limit');
	}
	const tokenHash = createHash('sha256').update(`Bearer ${options.token}`).digest();
	let active = 0;
	const handle = async (request: IncomingMessage, response: ServerResponse) => {
		const authorization = request.headers.authorization ?? '';
		if (!timingSafeEqual(tokenHash, createHash('sha256').update(authorization).digest()))
			throw new HttpError(401, 'Unauthorized');
		const url = new URL(request.url ?? '/', 'http://processor.local');
		if (url.pathname === '/health' && request.method === 'GET') {
			json(response, 200, { available: true, engine: 'sharp', version: sharp.versions.sharp });
			return;
		}
		if (!['/inspect', '/process'].includes(url.pathname))
			throw new HttpError(404, 'Unknown processor endpoint');
		if (request.method !== 'POST') throw new HttpError(405, 'Use POST with image bytes');
		if (active >= maxConcurrent) throw new HttpError(503, 'Processor is busy; retry later');
		for (const key of url.searchParams.keys())
			if (!['format', 'quality', 'width', 'height', 'lossless'].includes(key))
				throw new HttpError(400, 'Unsupported processing option');
		const format = url.searchParams.get('format') ?? 'webp';
		if (!['jpeg', 'png', 'webp', 'avif', 'gif'].includes(format))
			throw new HttpError(400, 'Unsupported output format');
		const width = integer(url.searchParams.get('width'), undefined, 8192);
		const height = integer(url.searchParams.get('height'), undefined, 8192);
		const quality = integer(url.searchParams.get('quality'), 80, 100) ?? 80;
		const losslessValue = url.searchParams.get('lossless');
		if (losslessValue !== null && !['true', 'false'].includes(losslessValue))
			throw new HttpError(400, 'Lossless must be true or false');
		const lossless = losslessValue === 'true';
		if (lossless && format === 'jpeg')
			throw new HttpError(
				400,
				'JPEG re-encoding cannot guarantee lossless pixels; use WebP, AVIF or PNG'
			);
		active++;
		try {
			const input = await body(request, maxBytes);
			if (!isRaster(input))
				throw new HttpError(
					415,
					'Supported inputs are JPEG, PNG, GIF, WebP and AVIF raster images'
				);
			const metadata = await sharp(input, {
				limitInputPixels: maxPixels,
				failOn: 'error',
				animated: true
			}).metadata();
			const pages = metadata.pages ?? 1;
			const frameHeight = metadata.pageHeight ?? metadata.height ?? 0;
			if (
				!metadata.width ||
				!frameHeight ||
				pages > 256 ||
				metadata.width * frameHeight * pages > maxPixels
			)
				throw new HttpError(422, 'Image exceeds the pixel or frame limit');
			if (url.pathname === '/inspect') {
				const swap = [5, 6, 7, 8].includes(metadata.orientation ?? 1);
				json(response, 200, {
					format: metadata.format,
					width: swap ? frameHeight : metadata.width,
					height: swap ? metadata.width : frameHeight,
					pages,
					animated: pages > 1,
					size: input.length
				});
				return;
			}
			if (pages > 1 && !['webp', 'gif'].includes(format))
				throw new HttpError(
					422,
					'Animated inputs require WebP or GIF output to preserve their frames'
				);
			let pipeline = sharp(input, {
				limitInputPixels: maxPixels,
				failOn: 'error',
				animated: pages > 1
			})
				.timeout({ seconds: 25 })
				.rotate();
			if (width !== undefined || height !== undefined)
				pipeline = pipeline.resize({ width, height, fit: 'inside', withoutEnlargement: true });
			switch (format) {
				case 'webp':
					pipeline = pipeline.webp({ quality, lossless, effort: 4 });
					break;
				case 'avif':
					pipeline = pipeline.avif({ quality, lossless, effort: 4 });
					break;
				case 'jpeg':
					pipeline = pipeline.jpeg({ quality, mozjpeg: true });
					break;
				case 'png':
					pipeline = pipeline.png({ quality, palette: !lossless, compressionLevel: 9 });
					break;
				case 'gif':
					pipeline = pipeline.gif();
					break;
			}
			const { data, info } = await pipeline.toBuffer({ resolveWithObject: true });
			if (data.length > maxBytes)
				throw new HttpError(413, 'Processed image exceeds the output size limit');
			response.writeHead(200, {
				'Content-Type': `image/${format}`,
				'Content-Length': data.length,
				'Cache-Control': 'no-store',
				'X-Content-Type-Options': 'nosniff',
				'X-Image-Width': info.width,
				'X-Image-Height': info.pageHeight ?? info.height,
				'X-Image-Pages': info.pages ?? 1,
				'X-Image-Format': info.format
			});
			response.end(data);
		} catch (error) {
			if (error instanceof HttpError) throw error;
			throw new HttpError(
				422,
				error instanceof Error ? error.message.slice(0, 240) : 'Image could not be processed'
			);
		} finally {
			active--;
		}
	};
	const server = createServer({ maxHeaderSize: 8192 }, (request, response) => {
		handle(request, response).catch((error: unknown) => {
			request.resume();
			if (response.headersSent || response.destroyed) {
				response.destroy();
				return;
			}
			if (error instanceof HttpError) json(response, error.status, { error: error.message });
			else {
				console.error('[media-processor] Request failed', error);
				json(response, 500, { error: 'Processor request failed' });
			}
		});
	});
	server.requestTimeout = 35000;
	server.headersTimeout = 10000;
	server.keepAliveTimeout = 5000;
	return server;
}
