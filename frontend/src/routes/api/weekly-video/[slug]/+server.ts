/**
 * Weekly Video API - Get current weekly video for a room
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * GET /api/weekly-video/[room-slug] - Get current weekly video
 * POST /api/weekly-video/[room-slug] - Publish new weekly video (admin)
 *
 * Connects to backend at /api/room-content/rooms/:slug/weekly-video
 *
 * @version 1.0.0 - ICT 11 Principal Engineer Grade
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
// R20-A: migrated off local `fetchFromBackend` helper to shared
// `$lib/server/proxy-fetch` (CLAUDE.md URL-fallback pinned once,
// `Promise<unknown>` return; `extractBackendData` preserves the
// R18-A Latent Bug §3 fix for `{ data: null }` semantics).
import { fetchBackend, extractBackendData } from '$lib/server/proxy-fetch';

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

// R22-A: `WeeklyVideo` interface dropped — the only caller was the mock POST
// path that minted a fake row with `Date.now()` as the id.

/** Shape of the incoming POST body — every field optional except `video_url`/`video_title`. */
interface WeeklyVideoPostBody {
	week_of?: string;
	week_title?: string;
	video_title?: string;
	video_url?: string;
	video_platform?: string;
	thumbnail_url?: string;
	duration?: string;
	description?: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// R22-A: Deleted `mockWeeklyVideos` (2 fake placeholder videos for
//   `spx-profit-pulse` and `weekly-watchlist` pointing at
//   `iframe.mediadelivery.net/embed/585929/placeholder-*` URLs that DO NOT
//   resolve to real streams — anyone hitting these on a backend hiccup got
//   a broken iframe). The `explosive-swings` entry was already `null`.
//   On backend failure:
//     - GET returned the mock placeholder for matching slugs and `null`
//       for unknown slugs, with `_source: 'mock'`.
//     - POST mutated the in-memory map and returned 200 — admins saw
//       "Weekly video published" while nothing persisted.
//   Both now surface backend failure as 502.
// ═══════════════════════════════════════════════════════════════════════════

// ═══════════════════════════════════════════════════════════════════════════
// GET - Get current weekly video
// ═══════════════════════════════════════════════════════════════════════════

export const GET: RequestHandler = async ({ params, request, cookies }) => {
	const { slug } = params;

	if (!slug) {
		error(400, 'Room slug is required');
	}

	// Get auth headers
	const authHeader = request.headers.get('Authorization');
	// FIX-2026-04-26: comment-out, verify, delete in follow-up. Wrong cookie name — login proxy sets rtp_access_token, not session.
	// const sessionCookie = cookies.get('session');
	const sessionCookie = cookies.get('rtp_access_token');
	const headers: Record<string, string> = {};

	if (authHeader) {
		headers['Authorization'] = authHeader;
	} else if (sessionCookie) {
		headers['Cookie'] = `session=${sessionCookie}`;
	}

	// Call backend at /api/room-content/rooms/:slug/weekly-video
	const backendData = await fetchBackend(
		`/api/room-content/rooms/${slug}/weekly-video`,
		{ headers },
		'[Weekly Video API]'
	);

	if (backendData) {
		return json({
			success: true,
			data: extractBackendData(backendData),
			_source: 'backend'
		});
	}

	// R22-A: was: return `mockWeeklyVideos[slug] || null` with
	// `_source: 'mock'` — broken iframe URLs for matching slugs. Now: 502
	// so the weekly-video card shows a real error.
	console.error(`[Weekly Video API] GET backend unavailable for slug '${slug}'`);
	return json(
		{ success: false, error: 'Unable to load weekly video — backend is unavailable.' },
		{ status: 502 }
	);
};

// ═══════════════════════════════════════════════════════════════════════════
// POST - Publish new weekly video (archives previous)
// ═══════════════════════════════════════════════════════════════════════════

export const POST: RequestHandler = async ({ params, request, cookies }) => {
	const { slug } = params;
	const authHeader = request.headers.get('Authorization');
	// FIX-2026-04-26: comment-out, verify, delete in follow-up. Wrong cookie name — login proxy sets rtp_access_token, not session.
	// const sessionCookie = cookies.get('session');
	const sessionCookie = cookies.get('rtp_access_token');

	if (!authHeader && !sessionCookie) {
		error(401, 'Authentication required');
	}

	if (!slug) {
		error(400, 'Room slug is required');
	}

	const body = (await request.json()) as WeeklyVideoPostBody;

	// Validate required fields
	if (!body.video_url || !body.video_title) {
		error(400, 'Video URL and title are required');
	}

	// Build headers
	const headers: Record<string, string> = {};
	if (authHeader) {
		headers['Authorization'] = authHeader;
	} else if (sessionCookie) {
		headers['Cookie'] = `session=${sessionCookie}`;
	}

	// Call backend at /api/admin/room-content/weekly-video
	const backendData = await fetchBackend(
		`/api/admin/room-content/weekly-video`,
		{
			method: 'POST',
			headers,
			body: JSON.stringify({
				room_slug: slug,
				week_of: body.week_of || new Date().toISOString().split('T')[0],
				week_title:
					body.week_title ||
					`Week of ${new Date().toLocaleDateString('en-US', { month: 'long', day: 'numeric', year: 'numeric' })}`,
				video_title: body.video_title,
				video_url: body.video_url,
				video_platform: body.video_platform || 'bunny',
				thumbnail_url: body.thumbnail_url || '',
				duration: body.duration || '',
				description: body.description || ''
			})
		},
		'[Weekly Video API]'
	);

	if (backendData) {
		return json({
			success: true,
			data: backendData,
			message: 'Weekly video published successfully',
			_source: 'backend'
		});
	}

	// R22-A: was: mint a new `WeeklyVideo` with `Date.now()` as the id,
	// store it in `mockWeeklyVideos[slug]`, return 200. Fake-success on a
	// mutating endpoint — admins saw "Published" while nothing persisted.
	// Now: 502.
	console.error(`[Weekly Video API] POST backend unavailable for slug '${slug}'`);
	return json(
		{ success: false, error: 'Unable to publish weekly video — backend is unavailable.' },
		{ status: 502 }
	);
};
