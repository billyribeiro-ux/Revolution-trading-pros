import sharp from 'sharp';

export const MAX_INPUT_BYTES = 50 * 1024 * 1024;
export const MAX_DECODED_PIXELS = 16_000_000;
export const MAX_FRAMES = 256;

export interface ImageDetails {
	format: string;
	width: number;
	height: number;
	pages: number;
	animated: boolean;
	bytes: number;
}

export interface WebpOptions {
	quality?: number;
	effort?: number;
	width?: number;
	height?: number;
	lossless?: boolean;
}

export interface OptimizedImage {
	bytes: Buffer;
	format: string;
	width: number;
	height: number;
	pages: number;
	originalBytes: number;
	savedBytes: number;
	usedOriginal: boolean;
}

function validateBuffer(input: Buffer): void {
	if (!Buffer.isBuffer(input) || input.length === 0 || input.length > MAX_INPUT_BYTES) {
		throw new Error('Input must contain 1–50 MiB of image bytes');
	}
	const header = input.subarray(0, 12);
	const raster =
		(header[0] === 0xff && header[1] === 0xd8 && header[2] === 0xff) ||
		header.subarray(0, 8).equals(Buffer.from('89504e470d0a1a0a', 'hex')) ||
		['GIF87a', 'GIF89a'].includes(header.subarray(0, 6).toString('ascii')) ||
		(header.subarray(0, 4).toString('ascii') === 'RIFF' &&
			header.subarray(8, 12).toString('ascii') === 'WEBP') ||
		(header.subarray(4, 8).toString('ascii') === 'ftyp' &&
			['avif', 'avis'].includes(header.subarray(8, 12).toString('ascii')));
	if (!raster) throw new Error('Only JPEG, PNG, GIF, WebP and AVIF raster images are supported');
}

export async function inspectImage(input: Buffer): Promise<ImageDetails> {
	validateBuffer(input);
	const metadata = await sharp(input, {
		limitInputPixels: MAX_DECODED_PIXELS,
		failOn: 'error',
		animated: true
	}).metadata();
	const pages = metadata.pages ?? 1;
	const frameHeight = metadata.pageHeight ?? metadata.height ?? 0;
	const width = metadata.width ?? 0;
	if (
		!width ||
		!frameHeight ||
		pages > MAX_FRAMES ||
		width * frameHeight * pages > MAX_DECODED_PIXELS
	) {
		throw new Error('Image exceeds the pixel or frame limit');
	}
	const rotated = [5, 6, 7, 8].includes(metadata.orientation ?? 1);
	return {
		format: metadata.format ?? 'unknown',
		width: rotated ? frameHeight : width,
		height: rotated ? width : frameHeight,
		pages,
		animated: pages > 1,
		bytes: input.length
	};
}

function validInteger(value: number | undefined, min: number, max: number, label: string): void {
	if (value !== undefined && (!Number.isSafeInteger(value) || value < min || value > max)) {
		throw new Error(`${label} must be an integer from ${min} to ${max}`);
	}
}

/** Encode image bytes as WebP. Caller paths and URLs are never opened or fetched. */
export async function encodeWebp(
	input: Buffer,
	options: WebpOptions = {}
): Promise<OptimizedImage> {
	validInteger(options.quality, 1, 100, 'quality');
	validInteger(options.effort, 0, 6, 'effort');
	validInteger(options.width, 1, 8192, 'width');
	validInteger(options.height, 1, 8192, 'height');
	const details = await inspectImage(input);
	let pipeline = sharp(input, {
		limitInputPixels: MAX_DECODED_PIXELS,
		failOn: 'error',
		animated: details.animated
	})
		.timeout({ seconds: 25 })
		.rotate();
	if (options.width !== undefined || options.height !== undefined) {
		pipeline = pipeline.resize({
			width: options.width,
			height: options.height,
			fit: 'inside',
			withoutEnlargement: true
		});
	}
	const { data, info } = await pipeline
		.webp({
			quality: options.quality ?? 80,
			effort: options.effort ?? 4,
			lossless: options.lossless ?? false
		})
		.toBuffer({ resolveWithObject: true });
	if (data.length > MAX_INPUT_BYTES) throw new Error('WebP output exceeds 50 MiB');
	return {
		bytes: data,
		format: 'webp',
		width: info.width,
		height: info.pageHeight ?? info.height,
		pages: info.pages ?? 1,
		originalBytes: input.length,
		savedBytes: input.length - data.length,
		usedOriginal: false
	};
}

/** Use the original only when a full-size WebP would cost more bytes. */
export async function optimizeWebp(
	input: Buffer,
	options: WebpOptions = {}
): Promise<OptimizedImage> {
	if (options.width !== undefined || options.height !== undefined) {
		throw new Error('Size comparison is only valid for full-size images');
	}
	const converted = await encodeWebp(input, options);
	if (converted.bytes.length < input.length) return converted;
	const details = await inspectImage(input);
	return {
		bytes: input,
		format: details.format,
		width: details.width,
		height: details.height,
		pages: details.pages,
		originalBytes: input.length,
		savedBytes: 0,
		usedOriginal: true
	};
}
