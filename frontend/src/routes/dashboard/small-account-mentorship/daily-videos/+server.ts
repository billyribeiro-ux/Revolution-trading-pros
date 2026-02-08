/**
 * Redirect Handler - Old URL to New Structure
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Redirects /dashboard/day-trading-room/daily-videos
 * to /dashboard/day-trading-room/daily-videos (new dynamic structure)
 *
 * @version 3.0.0 - January 2026
 */

import { redirect } from '@sveltejs/kit';
import type { RequestEvent } from '@sveltejs/kit';

export const GET = async ({ url }: RequestEvent) => {
	// Preserve query parameters (page, search, etc.)
	const searchParams = url.searchParams.toString();
	const newUrl = `/dashboard/small-account-mentorship/daily-videos${searchParams ? '?' + searchParams : ''}`;

	redirect(301, newUrl);
};
