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
import type { RequestEvent } from '@sveltejs/kit';

export const GET = async ({ params }: RequestEvent) => {
	const { slug } = params;
	redirect(301, `/dashboard/small-account-mentorship/video/${slug}`);
};
