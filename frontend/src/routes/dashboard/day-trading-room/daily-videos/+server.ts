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
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async ({ url }) => {
	// Preserve query parameters (page, search, etc.)
	const searchParams = url.searchParams.toString();
	const newUrl = `/dashboard/day-trading-room/daily-videos${searchParams ? '?' + searchParams : ''}`;
	
	throw redirect(301, newUrl);
};
