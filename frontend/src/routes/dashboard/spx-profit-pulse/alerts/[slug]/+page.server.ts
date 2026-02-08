/**
 * SPX Profit Pulse Alert Detail - Server Load Function
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Apple ICT Level 7 Principal Engineer Implementation
 * SSR pre-fetch for individual alert/video pages
 *
 * @version 1.0.0 - January 2026
 */

import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ params, fetch, setHeaders }) => {
	const { slug } = params;

	if (!slug) {
		error(400, 'Alert slug is required');
	}

	// Set cache headers
	setHeaders({
		'Cache-Control': 'public, max-age=3600, stale-while-revalidate=86400'
	});

	try {
		const response = await fetch(`/api/dashboard/spx-profit-pulse/alerts/${slug}`);

		if (!response.ok) {
			if (response.status === 404) {
				error(404, 'Alert not found');
			}
			error(response.status, 'Failed to load alert');
		}

		const result = await response.json();

		if (!result.success || !result.data) {
			error(404, 'Alert not found');
		}

		return {
			alert: result.data
		};
	} catch (err) {
		if (err && typeof err === 'object' && 'status' in err) {
			throw err;
		}
		console.error('[SPX Alert SSR] Error:', err);
		error(500, 'Failed to load alert');
	}
};
