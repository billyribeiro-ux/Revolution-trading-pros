import { Buffer } from 'node:buffer';
import { readFile } from 'node:fs/promises';
import { expect, test, type APIRequestContext } from '@playwright/test';

// Requires the isolated API with real local ClamAV and Adobe S3Mock. S3Mock
// verifies storage effects but does not enforce presigned signatures/expiry.
const api = 'http://127.0.0.1:8281/api/admin/media';
const store = 'http://127.0.0.1:19000/admin-move-media';
const eicar = Buffer.from(
	'WDVPIVAlQEFQWzRcUFpYNTQoUF4pN0NDKTd9JEVJQ0FSLVNUQU5EQVJELUFOVElWSVJVUy1URVNULUZJTEUhJEgrSCo=',
	'base64'
);
type MediaRecord = {
	id: number;
	original_filename: string;
	url: string | null;
	path: string;
	scan_status: string;
	scan_result: { is_clean: boolean };
};

async function login(request: APIRequestContext) {
	const response = await request.post('http://127.0.0.1:8281/api/auth/login', {
		data: { email: 'admin@admin-completion.test', password: 'OnlyForDisposableTest123!' }
	});
	expect(response.ok()).toBe(true);
	const auth = (await response.json()) as { token: string };
	expect(auth.token).toBeTruthy();
	return { Authorization: `Bearer ${auth.token}` };
}
async function objects(request: APIRequestContext) {
	const response = await request.get(store);
	expect(response.ok()).toBe(true);
	return ((await response.text()).match(/<Key>/g) ?? []).length;
}

test.beforeEach(({ baseURL }) => {
	expect(baseURL).toBe('http://127.0.0.1:5274');
});

test('clean upload, actual stored-object scan and deletion persist truthful states', async ({
	request
}) => {
	const headers = await login(request);
	const content = Buffer.from('A harmless local S3 and ClamAV integration fixture.');
	const uploaded = await request.post(`${api}/upload`, {
		headers,
		multipart: {
			file: { name: `storage-clean-${Date.now()}.txt`, mimeType: 'text/plain', buffer: content }
		}
	});
	expect(uploaded.ok(), await uploaded.text()).toBe(true);
	const record = ((await uploaded.json()) as { data: MediaRecord[] }).data[0];
	expect(record).toBeDefined();
	expect(record.scan_status).toBe('clean');
	expect(record.scan_result.is_clean).toBe(true);
	expect(new URL(record.url ?? '').origin).toBe('http://127.0.0.1:19000');
	const stored = await request.get(record.url ?? '');
	expect(stored.ok()).toBe(true);
	expect(await stored.body()).toEqual(content);
	const contentUrl = `${api}/content/${record.id}`;
	expect((await request.get(contentUrl)).status()).toBe(401);
	const member = await request.post('http://127.0.0.1:8281/api/auth/login', {
		data: { email: 'user@admin-completion.test', password: 'OnlyForDisposableTest123!' }
	});
	expect(member.ok()).toBe(true);
	const memberToken = ((await member.json()) as { token: string }).token;
	expect(
		(
			await request.get(contentUrl, { headers: { Authorization: `Bearer ${memberToken}` } })
		).status()
	).toBe(403);
	const protectedContent = await request.get(contentUrl, { headers });
	expect(protectedContent.ok()).toBe(true);
	expect(protectedContent.headers()['content-disposition']).toBe('attachment');
	expect(await protectedContent.body()).toEqual(content);
	const scan = await request.post(`${api}/scan/${record.id}`, { headers });
	expect(scan.ok(), await scan.text()).toBe(true);
	const verdict = (await scan.json()) as {
		data: { scan_result: { status: string; is_clean: boolean } };
	};
	expect(verdict.data.scan_result).toMatchObject({ status: 'clean', is_clean: true });
	const persisted = await request.get(`${api}/${record.id}`, { headers });
	expect(((await persisted.json()) as { data: MediaRecord }).data.scan_status).toBe('clean');
	const deleted = await request.delete(`${api}/${record.id}`, { headers });
	expect(deleted.ok()).toBe(true);
	expect((await request.get(record.url ?? '')).status()).toBe(404);
	expect((await request.get(`${api}/${record.id}`, { headers })).status()).toBe(404);
});

test('real ClamAV rejects EICAR before a direct upload creates an object or row', async ({
	request
}) => {
	const headers = await login(request);
	const beforeObjects = await objects(request);
	const before = (await (await request.get(`${api}/files`, { headers })).json()) as {
		meta: { total: number };
	};
	const response = await request.post(`${api}/upload`, {
		headers,
		multipart: {
			file: { name: `storage-rejected-${Date.now()}.txt`, mimeType: 'text/plain', buffer: eicar }
		}
	});
	expect(response.status()).toBe(400);
	expect(await response.text()).toContain('Malware detected');
	expect(await objects(request)).toBe(beforeObjects);
	const after = (await (await request.get(`${api}/files`, { headers })).json()) as {
		meta: { total: number };
	};
	expect(after.meta.total).toBe(before.meta.total);
});

test('confirming an infected stored object removes it and preserves the infected verdict', async ({
	request
}) => {
	const headers = await login(request);
	const filename = `storage-infected-${Date.now()}.txt`;
	const presigned = await request.post(`${api}/presigned-upload`, {
		headers,
		data: { filename, content_type: 'text/plain', size: eicar.length, collection: 'scan-fixtures' }
	});
	expect(presigned.ok()).toBe(true);
	const signed = (
		(await presigned.json()) as {
			data: { upload_url: string; public_url: string; file_key: string };
		}
	).data;
	expect(new URL(signed.upload_url).origin).toBe('http://127.0.0.1:19000');
	expect(new URL(signed.public_url).origin).toBe('http://127.0.0.1:19000');
	expect(
		(
			await request.put(signed.upload_url, {
				headers: { 'Content-Type': 'text/plain' },
				data: eicar
			})
		).ok()
	).toBe(true);
	const confirmation = await request.post(`${api}/confirm-upload`, {
		headers,
		data: {
			file_key: signed.file_key,
			original_filename: filename,
			content_type: 'text/plain',
			size: eicar.length,
			collection: 'scan-fixtures'
		}
	});
	expect(confirmation.status()).toBe(400);
	expect(await confirmation.text()).toContain('Malware detected');
	expect((await request.get(signed.public_url)).status()).toBe(404);
	const listing = (await (
		await request.get(`${api}/files?collection=scan-fixtures`, { headers })
	).json()) as { data: MediaRecord[] };
	const record = listing.data.find((media) => media.original_filename === filename);
	expect(record).toBeDefined();
	expect(record?.scan_status).toBe('infected');
	expect(record?.scan_result.is_clean).toBe(false);
	expect(record?.url).toBeNull();
	expect((await request.delete(`${api}/${record?.id}`, { headers })).ok()).toBe(true);
});

test('recurring image worker persists real variants, measured sizes, and deletes all objects', async ({
	request,
	page
}) => {
	const headers = await login(request);
	const health = await request.get(`${api}/processor/health`, { headers });
	expect(health.ok(), await health.text()).toBe(true);
	const original = await readFile(new URL('../fixtures/media-processing.png', import.meta.url));
	const upload = await request.post(`${api}/upload`, {
		headers,
		multipart: {
			file: { name: `processing-${Date.now()}.png`, mimeType: 'image/png', buffer: original }
		}
	});
	expect(upload.ok(), await upload.text()).toBe(true);
	const record = ((await upload.json()) as { data: MediaRecord[] }).data[0];
	const queued = await request.post(`${api}/files/${record.id}/optimize`, {
		headers,
		data: { preset: 'balanced' }
	});
	expect(queued.status(), await queued.text()).toBe(202);
	const jobId = ((await queued.json()) as { data: { id: number; media_id: number } }).data.id;
	let job:
		| { status: string; error: string | null; original_size: number; optimized_size: number }
		| undefined;
	await expect
		.poll(
			async () => {
				job = (
					(await (await request.get(`${api}/jobs/${jobId}`, { headers })).json()) as {
						data: typeof job;
					}
				).data;
				return job?.status;
			},
			{ timeout: 45000 }
		)
		.toBe('completed');
	expect(job?.error).toBeNull();
	expect(job?.original_size).toBe(original.length);
	const detail = (
		(await (await request.get(`${api}/${record.id}`, { headers })).json()) as {
			data: MediaRecord & { is_optimized: boolean; width: number; height: number };
		}
	).data;
	expect(detail).toMatchObject({
		is_optimized: true,
		width: 1000,
		height: 400,
		scan_status: 'clean'
	});
	const variants = (
		(await (await request.get(`${api}/files/${record.id}/variants`, { headers })).json()) as {
			data: Array<{
				id: number;
				url: string;
				width: number;
				height: number;
				file_size: number;
				format: string;
				size: string;
			}>;
		}
	).data;
	expect(variants.map((variant) => variant.width).sort((a, b) => a - b)).toEqual([320, 768, 1000]);
	for (const variant of variants) {
		const stored = await request.get(variant.url);
		expect(stored.ok()).toBe(true);
		expect(stored.headers()['content-type']).toBe('image/webp');
		const bytes = await stored.body();
		const authenticated = await request.get(
			`${api}/content/${record.id}?variant_id=${variant.id}`,
			{ headers }
		);
		expect(authenticated.ok(), await authenticated.text()).toBe(true);
		expect(await authenticated.body()).toEqual(bytes);
		expect(bytes.length).toBe(variant.file_size);
		expect(bytes.subarray(0, 4).toString()).toBe('RIFF');
		expect(bytes.subarray(8, 12).toString()).toBe('WEBP');
		const measured = await page.evaluate(async (data) => {
			const bitmap = await createImageBitmap(
				new Blob([new Uint8Array(data)], { type: 'image/webp' })
			);
			const dimensions = { width: bitmap.width, height: bitmap.height };
			bitmap.close();
			return dimensions;
		}, Array.from(bytes));
		expect(measured).toEqual({ width: variant.width, height: variant.height });
	}
	expect(job?.optimized_size).toBe(variants.find((variant) => variant.size === 'full')?.file_size);
	expect(await (await request.get(record.url ?? '')).body()).toEqual(original);
	const stats = (
		(await (await request.get(`${api}/statistics`, { headers })).json()) as {
			data: { total_variants: number; total_storage: number; job_stats: { completed: number } };
		}
	).data;
	expect(stats.total_variants).toBeGreaterThanOrEqual(3);
	expect(stats.total_storage).toBeGreaterThanOrEqual(
		original.length + variants.reduce((sum, v) => sum + v.file_size, 0)
	);
	expect(stats.job_stats.completed).toBeGreaterThanOrEqual(1);
	const removed = await request.delete(`${api}/${record.id}`, { headers });
	expect(removed.ok(), await removed.text()).toBe(true);
	for (const url of [record.url ?? '', ...variants.map((variant) => variant.url)]) {
		expect((await request.get(url)).status()).toBe(404);
	}
	expect((await request.get(`${api}/jobs/${jobId}`, { headers })).status()).toBe(404);
});

test('decode failure retains a retry error and never publishes optimized media', async ({
	request
}) => {
	const headers = await login(request);
	const original = await readFile(new URL('../fixtures/media-processing.png', import.meta.url));
	const upload = await request.post(`${api}/upload`, {
		headers,
		multipart: {
			file: { name: `broken-${Date.now()}.png`, mimeType: 'image/png', buffer: original }
		}
	});
	expect(upload.ok(), await upload.text()).toBe(true);
	const record = ((await upload.json()) as { data: MediaRecord[] }).data[0];
	// Controlled S3Mock fault injection after a valid upload. Its open local
	// adapter accepts this PUT; the real worker must reject corrupt stored bytes.
	expect(
		(
			await request.put(record.url ?? '', {
				headers: { 'Content-Type': 'image/png' },
				data: Buffer.from('89504e470d0a1a0a', 'hex')
			})
		).ok()
	).toBe(true);
	const response = await request.post(`${api}/files/${record.id}/optimize`, { headers, data: {} });
	expect(response.status()).toBe(202);
	const jobId = ((await response.json()) as { data: { id: number } }).data.id;
	await expect
		.poll(
			async () => {
				const job = (
					(await (await request.get(`${api}/jobs/${jobId}`, { headers })).json()) as {
						data: {
							attempts: number;
							status: string;
							error: string | null;
							optimized_size: number | null;
						};
					}
				).data;
				if (job.attempts < 1 || job.status === 'processing') return false;
				expect(job.status).toBe('pending');
				expect(job.error).toContain('processor returned');
				expect(job.optimized_size).toBeNull();
				return true;
			},
			{ timeout: 15000 }
		)
		.toBe(true);
	const detail = (
		(await (await request.get(`${api}/${record.id}`, { headers })).json()) as {
			data: { is_optimized: boolean };
		}
	).data;
	expect(detail.is_optimized).toBe(false);
	expect(
		(
			(await (await request.get(`${api}/files/${record.id}/variants`, { headers })).json()) as {
				data: unknown[];
			}
		).data
	).toEqual([]);
	expect((await request.delete(`${api}/${record.id}`, { headers })).ok()).toBe(true);
});

test('confirmation checks stored bytes, is idempotent, and publishes an immutable scanned key', async ({
	request
}) => {
	const headers = await login(request);
	const content = Buffer.from('Immutable confirmed upload fixture');
	const response = await request.post(`${api}/presigned-upload`, {
		headers,
		data: { filename: 'confirmed.txt', content_type: 'text/plain', size: content.length }
	});
	expect(response.ok(), await response.text()).toBe(true);
	const signed = (
		(await response.json()) as {
			data: { file_key: string; upload_url: string; public_url: string };
		}
	).data;
	const payload = {
		file_key: signed.file_key,
		original_filename: 'confirmed.txt',
		content_type: 'text/plain',
		size: content.length
	};
	const missing = await request.post(`${api}/confirm-upload`, { headers, data: payload });
	expect(missing.status()).toBe(503);
	expect(
		(
			await request.put(signed.upload_url, {
				headers: { 'Content-Type': 'text/plain' },
				data: content
			})
		).ok()
	).toBe(true);
	expect(
		(
			await request.post(`${api}/confirm-upload`, { headers, data: { ...payload, size: 1 } })
		).status()
	).toBe(400);
	expect(
		(
			await request.post(`${api}/confirm-upload`, {
				headers,
				data: { ...payload, file_key: 'uploads/not-issued.txt' }
			})
		).status()
	).toBe(400);
	const confirmed = await request.post(`${api}/confirm-upload`, { headers, data: payload });
	expect(confirmed.ok(), await confirmed.text()).toBe(true);
	const record = ((await confirmed.json()) as { data: MediaRecord }).data;
	expect(record.path).not.toBe(signed.file_key);
	expect(record.scan_status).toBe('clean');
	expect(await (await request.get(record.url ?? '')).body()).toEqual(content);
	expect((await request.get(signed.public_url)).status()).toBe(404);
	const repeated = await request.post(`${api}/confirm-upload`, { headers, data: payload });
	expect(repeated.ok()).toBe(true);
	expect(((await repeated.json()) as { data: MediaRecord }).data.id).toBe(record.id);
	// S3Mock deliberately does not enforce expiry/signatures. This asserts that
	// a second PUT to the old key cannot change the bytes published as clean.
	expect(
		(
			await request.put(signed.upload_url, {
				headers: { 'Content-Type': 'text/plain' },
				data: Buffer.from('changed pending object')
			})
		).ok()
	).toBe(true);
	expect(await (await request.get(record.url ?? '')).body()).toEqual(content);
	expect((await request.delete(signed.public_url)).ok()).toBe(true);
	expect((await request.delete(`${api}/${record.id}`, { headers })).ok()).toBe(true);
});
