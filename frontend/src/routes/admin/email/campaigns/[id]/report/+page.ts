/**
 * Email Campaign Report - Page Load
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Loads campaign ID from URL parameters for the report page.
 *
 * @version 1.0.0 - January 2026
 */

import type { SEOInput } from '#lib/seo/types.js';
import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';

// Disable prerendering for dynamic route
export const prerender = false;

export const load: PageLoad = ({ params }) => {
	if (!/^[1-9]\d*$/.test(params.id) || !Number.isSafeInteger(Number(params.id)))
		error(404, 'Campaign not found');
	return {
		seo: {
			title: 'Campaign Report | Admin',
			description: 'Recorded email campaign delivery receipts and observed engagement.',
			robots: { index: false, follow: false }
		} satisfies SEOInput,
		campaignId: params.id
	};
};
