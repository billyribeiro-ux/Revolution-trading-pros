import { readFile } from 'node:fs/promises';
import { Buffer } from 'node:buffer';
import { expect, test, type APIRequestContext } from '@playwright/test';
const api = 'http://127.0.0.1:8281/api/admin/media';
interface Media {
	id: number;
	path: string;
	url: string;
	content_version: number;
	width: number;
	height: number;
	is_optimized: boolean;
	alt_text: string;
}
interface Variant {
	id: number;
	url: string;
	file_size: number;
	source_version: number;
}
interface Version {
	id: string;
	version_number: number;
	url: string;
}
async function optimize(request: APIRequestContext, headers: Record<string, string>, id: number) {
	const response = await request.post(`${api}/files/${id}/optimize`, { headers, data: {} });
	expect(response.status(), await response.text()).toBe(202);
	const jobId = ((await response.json()) as { data: { id: number } }).data.id;
	await expect
		.poll(
			async () =>
				(
					(await (await request.get(`${api}/jobs/${jobId}`, { headers })).json()) as {
						data: { status: string };
					}
				).data.status,
			{ timeout: 15000 }
		)
		.toBe('completed');
	return (
		(await (await request.get(`${api}/files/${id}/variants`, { headers })).json()) as {
			data: Variant[];
		}
	).data;
}
test.beforeEach(({ baseURL }) => expect(baseURL).toBe('http://127.0.0.1:5274'));
test('replace, optimize the current revision, restore, and delete every retained object', async ({
	request
}) => {
	const login = await request.post('http://127.0.0.1:8281/api/auth/login', {
		data: { email: 'admin@admin-completion.test', password: 'OnlyForDisposableTest123!' }
	});
	expect(login.ok()).toBe(true);
	const headers = { Authorization: `Bearer ${((await login.json()) as { token: string }).token}` };
	const before = (
		(await (await request.get(`${api}/statistics`, { headers })).json()) as {
			data: { total_storage: number };
		}
	).data.total_storage;
	const original = await readFile(new URL('../fixtures/media-processing.png', import.meta.url));
	const replacement = await readFile(new URL('../fixtures/media-replacement.png', import.meta.url));
	const uploaded = await request.post(`${api}/upload`, {
		headers,
		multipart: {
			file: { name: 'version-original.png', mimeType: 'image/png', buffer: original },
			alt_text: 'Preserve metadata'
		}
	});
	expect(uploaded.ok(), await uploaded.text()).toBe(true);
	const first = ((await uploaded.json()) as { data: Media[] }).data[0];
	expect(first.content_version).toBe(1);
	const initial = (
		(await (await request.get(`${api}/files/${first.id}/versions`, { headers })).json()) as {
			versions: Version[];
		}
	).versions;
	expect(initial).toHaveLength(1);
	const originalVersion = initial[0];
	const originalVariants = await optimize(request, headers, first.id);
	expect(originalVariants).toHaveLength(3);
	const changed = await request.post(`${api}/${first.id}/replace`, {
		headers,
		multipart: {
			file: { name: 'version-replacement.png', mimeType: 'image/png', buffer: replacement },
			changes: 'Replacement fixture'
		}
	});
	expect(changed.ok(), await changed.text()).toBe(true);
	const second = ((await changed.json()) as { data: Media }).data;
	expect(second).toMatchObject({
		id: first.id,
		content_version: 2,
		width: 200,
		height: 100,
		is_optimized: false,
		alt_text: 'Preserve metadata'
	});
	expect(second.url).not.toBe(first.url);
	expect(await (await request.get(first.url)).body()).toEqual(original);
	expect(await (await request.get(second.url)).body()).toEqual(replacement);
	const empty = (
		(await (await request.get(`${api}/files/${first.id}/variants`, { headers })).json()) as {
			data: Variant[];
		}
	).data;
	expect(empty).toEqual([]);
	expect(
		(
			await request.get(`${api}/content/${first.id}?variant_id=${originalVariants[0]?.id}`, {
				headers
			})
		).status()
	).toBe(404);
	const nextVariants = await optimize(request, headers, first.id);
	expect(nextVariants).toHaveLength(1);
	expect(nextVariants[0]?.source_version).toBe(2);
	const restored = await request.post(
		`${api}/files/${first.id}/versions/${originalVersion?.id}/restore`,
		{ headers }
	);
	expect(restored.ok(), await restored.text()).toBe(true);
	const third = ((await restored.json()) as { data: Media }).data;
	expect(third).toMatchObject({
		id: first.id,
		content_version: 3,
		width: 1000,
		height: 400,
		is_optimized: false,
		alt_text: 'Preserve metadata',
		path: first.path,
		url: first.url
	});
	expect(await (await request.get(`${api}/content/${first.id}`, { headers })).body()).toEqual(
		original
	);
	const history = (await (
		await request.get(`${api}/files/${first.id}/versions?per_page=1&page=2`, { headers })
	).json()) as { versions: Version[]; meta: { total: number; last_page: number } };
	expect(history.meta).toMatchObject({ total: 3, last_page: 3 });
	expect(history.versions[0]?.version_number).toBe(2);
	const recorded = await request.get(
		`${api}/content/${first.id}?version_id=${originalVersion?.id}`,
		{ headers }
	);
	expect(recorded.ok()).toBe(true);
	expect(await recorded.body()).toEqual(original);
	const invalid = await request.post(`${api}/${first.id}/replace`, {
		headers,
		multipart: {
			file: {
				name: 'invalid.png',
				mimeType: 'image/png',
				buffer: Buffer.from('89504e470d0a1a0a', 'hex')
			}
		}
	});
	expect(invalid.status(), await invalid.text()).toBe(400);
	const threat = Buffer.from(
		'WDVPIVAlQEFQWzRcUFpYNTQoUF4pN0NDKTd9JEVJQ0FSLVNUQU5EQVJELUFOVElWSVJVUy1URVNULUZJTEUhJEgrSCo=',
		'base64'
	);
	const infected = await request.post(`${api}/${first.id}/replace`, {
		headers,
		multipart: { file: { name: 'infected.txt', mimeType: 'text/plain', buffer: threat } }
	});
	expect(infected.status()).toBe(400);
	const retained = (
		(await (await request.get(`${api}/${first.id}`, { headers })).json()) as { data: Media }
	).data;
	expect(retained.content_version).toBe(3);
	expect(retained.url).toBe(first.url);
	const physical = (
		(await (await request.get(`${api}/statistics`, { headers })).json()) as {
			data: { total_storage: number };
		}
	).data.total_storage;
	expect(physical - before).toBe(
		original.length +
			replacement.length +
			[...originalVariants, ...nextVariants].reduce((sum, v) => sum + v.file_size, 0)
	);
	expect((await request.get(`${api}/files/${first.id}/versions`)).status()).toBe(401);
	const deleted = await request.delete(`${api}/${first.id}`, { headers });
	expect(deleted.ok(), await deleted.text()).toBe(true);
	for (const url of [
		first.url,
		second.url,
		...originalVariants.map((v) => v.url),
		...nextVariants.map((v) => v.url)
	])
		expect((await request.get(url)).status()).toBe(404);
	expect((await request.get(`${api}/files/${first.id}/versions`, { headers })).status()).toBe(404);
});
