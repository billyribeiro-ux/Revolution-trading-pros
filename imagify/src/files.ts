import { createHash, randomUUID } from 'node:crypto';
import { lstat, mkdir, readFile, readdir, rename, rm, writeFile } from 'node:fs/promises';
import { basename, dirname, extname, join, relative, resolve, sep } from 'node:path';
import { optimizeWebp, type WebpOptions } from './index.ts';

const EXTENSIONS = new Set(['.jpg', '.jpeg', '.png', '.gif', '.webp', '.avif']);

export interface FileResult {
	source: string;
	output: string;
	format: string;
	width: number;
	height: number;
	pages: number;
	originalBytes: number;
	outputBytes: number;
	savedBytes: number;
	usedOriginal: boolean;
}

async function writeManifest(
	inputRoot: string,
	outputRoot: string,
	results: FileResult[]
): Promise<void> {
	const manifest = {
		version: 1,
		policy: 'Use WebP when its full-size bytes are smaller; retain originals otherwise',
		results: results.map((result) => ({
			...result,
			source: relative(inputRoot, result.source),
			output: relative(outputRoot, result.output)
		}))
	};
	await atomicWrite(
		join(outputRoot, 'imagify-manifest.json'),
		Buffer.from(JSON.stringify(manifest, null, 2))
	);
}

async function atomicWrite(path: string, bytes: Buffer): Promise<void> {
	await mkdir(dirname(path), { recursive: true });
	const temporary = `${path}.${randomUUID()}.tmp`;
	try {
		await writeFile(temporary, bytes, { flag: 'wx' });
		await rename(temporary, path);
	} catch (error) {
		await rm(temporary, { force: true });
		throw error;
	}
}

export async function optimizeFile(
	source: string,
	outputDirectory: string,
	options: WebpOptions = {}
): Promise<FileResult> {
	const sourcePath = resolve(source);
	const outputRoot = resolve(outputDirectory);
	if (sourcePath === outputRoot || sourcePath.startsWith(`${outputRoot}${sep}`)) {
		throw new Error('The output directory must not contain the input file');
	}
	const stat = await lstat(sourcePath);
	if (!stat.isFile() || stat.isSymbolicLink()) throw new Error('Input must be a regular file');
	if (!EXTENSIONS.has(extname(sourcePath).toLowerCase()))
		throw new Error('Unsupported image extension');
	const input = await readFile(sourcePath);
	const image = await optimizeWebp(input, options);
	// A source named photo.png.webp must never overwrite photo.png's WebP output.
	const suffix = createHash('sha256').update(sourcePath).digest('hex').slice(0, 24);
	const readableName = Array.from(basename(sourcePath)).slice(0, 48).join('');
	const extension = image.usedOriginal ? extname(sourcePath).slice(1) : 'webp';
	const fileName = `${readableName}.${suffix}.${extension}`;
	const outputPath = join(outputRoot, fileName);
	await atomicWrite(outputPath, image.bytes);
	return {
		source: sourcePath,
		output: outputPath,
		format: image.format,
		width: image.width,
		height: image.height,
		pages: image.pages,
		originalBytes: input.length,
		outputBytes: image.bytes.length,
		savedBytes: image.savedBytes,
		usedOriginal: image.usedOriginal
	};
}

async function* listImages(directory: string): AsyncGenerator<string> {
	const entries = await readdir(directory, { withFileTypes: true });
	for (const entry of entries.sort((left, right) => left.name.localeCompare(right.name))) {
		const path = join(directory, entry.name);
		if (entry.isDirectory()) yield* listImages(path);
		else if (entry.isFile() && EXTENSIONS.has(extname(entry.name).toLowerCase())) yield path;
	}
}

export async function optimizeDirectory(
	inputDirectory: string,
	outputDirectory: string,
	options: WebpOptions = {}
): Promise<FileResult[]> {
	const inputRoot = resolve(inputDirectory);
	const outputRoot = resolve(outputDirectory);
	if (
		outputRoot === inputRoot ||
		outputRoot.startsWith(`${inputRoot}${sep}`) ||
		inputRoot.startsWith(`${outputRoot}${sep}`)
	) {
		throw new Error('Input and output directories must be separate');
	}
	const stat = await lstat(inputRoot);
	if (!stat.isDirectory() || stat.isSymbolicLink())
		throw new Error('Input must be a regular directory');
	const results: FileResult[] = [];
	for await (const source of listImages(inputRoot)) {
		const destination = join(outputRoot, dirname(relative(inputRoot, source)));
		results.push(await optimizeFile(source, destination, options));
	}
	await writeManifest(inputRoot, outputRoot, results);
	return results;
}

/** Polling avoids platform-specific recursive file watcher behavior. */
export async function watchDirectory(
	inputDirectory: string,
	outputDirectory: string,
	options: WebpOptions = {},
	intervalMs = 2000
): Promise<() => void> {
	if (!Number.isSafeInteger(intervalMs) || intervalMs < 250)
		throw new Error('intervalMs must be at least 250');
	const inputRoot = resolve(inputDirectory);
	const outputRoot = resolve(outputDirectory);
	if (
		outputRoot === inputRoot ||
		outputRoot.startsWith(`${inputRoot}${sep}`) ||
		inputRoot.startsWith(`${outputRoot}${sep}`)
	)
		throw new Error('Input and output directories must be separate');
	const inputStat = await lstat(inputRoot);
	if (!inputStat.isDirectory() || inputStat.isSymbolicLink())
		throw new Error('Input must be a regular directory');
	const fingerprints = new Map<string, string>();
	const outputs = new Map<string, FileResult>();
	let busy = false;
	let initialized = false;
	const scan = async () => {
		if (busy) return;
		busy = true;
		try {
			const current = new Set<string>();
			let changed = false;
			for await (const path of listImages(inputRoot)) {
				const stat = await lstat(path);
				current.add(path);
				const fingerprint = `${stat.size}:${stat.mtimeMs}`;
				if (fingerprints.get(path) === fingerprint) continue;
				const destination = join(outputRoot, dirname(relative(inputRoot, path)));
				const result = await optimizeFile(path, destination, options);
				const previous = outputs.get(path);
				if (previous && previous.output !== result.output)
					await rm(previous.output, { force: true });
				fingerprints.set(path, fingerprint);
				outputs.set(path, result);
				changed = true;
			}
			for (const [path, result] of outputs) {
				if (current.has(path)) continue;
				await rm(result.output, { force: true });
				outputs.delete(path);
				fingerprints.delete(path);
				changed = true;
			}
			if (changed || !initialized) {
				await writeManifest(inputRoot, outputRoot, [...outputs.values()]);
				initialized = true;
			}
		} finally {
			busy = false;
		}
	};
	await scan();
	const timer = setInterval(() => {
		void scan().catch((error: unknown) => console.error('Imagify scan failed:', error));
	}, intervalMs);
	return () => clearInterval(timer);
}
