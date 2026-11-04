import { error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { API_BASE_URL, BACKEND_URL } from '$app/env/private';
import { requireAdmin } from '#lib/server/auth.js';

export const GET: RequestHandler = async (event) => {
	const { token } = requireAdmin(event);
	const { id } = event.params;
	if (!id || !/^\d+$/.test(id)) error(400, 'Invalid media ID');
	const params = new URLSearchParams();
	for (const name of ['variant_id', 'version_id']) {
		const value = event.url.searchParams.get(name);
		if (value !== null) {
			if (!/^\d+$/.test(value)) error(400, 'Invalid media revision ID');
			params.set(name, value);
		}
	}
	const suffix = params.size ? `?${params.toString()}` : '';
	const response = await event.fetch(
		`${API_BASE_URL || BACKEND_URL || 'http://localhost:8080'}/api/admin/media/content/${id}${suffix}`,
		{
			headers: { Authorization: `Bearer ${token}` },
			signal: AbortSignal.timeout(35000)
		}
	);
	const headers = new Headers({
		'Cache-Control': 'private, no-store',
		'X-Content-Type-Options': 'nosniff'
	});
	for (const name of ['content-type', 'content-disposition', 'x-media-scan-status']) {
		const value = response.headers.get(name);
		if (value) headers.set(name, value);
	}
	return new Response(response.body, { status: response.status, headers });
};
