/**
 * Redirect Handler - Old Video URL to New Structure
 * ═══════════════════════════════════════════════════════════════════════════
 * 
 * Redirects /dashboard/day-trading-room/video/[slug]
 * to /dashboard/day-trading-room/video/[slug] (new dynamic structure)
 * 
 * @version 3.0.0 - January 2026
 */

import { redirect } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async ({ params }) => {
	const { slug } = params;
	throw redirect(301, `/dashboard/day-trading-room/video/${slug}`);
};
