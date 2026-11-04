import { Buffer } from 'node:buffer';
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
