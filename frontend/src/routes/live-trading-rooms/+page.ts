/**
 * Live Trading Rooms - Page Configuration
 *
 * ARCHITECTURE NOTE: This parent page uses SSR (not prerender) because:
 * 1. Child routes (/day-trading, /swing-trading, /small-accounts) are prerendered
 * 2. SvelteKit cannot create both /live-trading-rooms.html AND /live-trading-rooms/ directory
 * 3. SSR with caching headers (set in +page.server.ts) provides equivalent performance
 * 4. The +page.server.ts already sets Cache-Control for edge caching
 *
 * @see https://kit.svelte.dev/docs/page-options
 * @module routes/live-trading-rooms/+page
 * @version 1.0.2
 */

import type { PageLoad } from './$types';
import type { SEOInput } from '$lib/seo/types';

// SSR parent page - child routes prerender into /live-trading-rooms/ directory
export const prerender = false;
export const ssr = true;
export const csr = true;

export const load: PageLoad = ({ data }) => {
	const seo: SEOInput = {
		title: 'Live Trading Rooms | Professional Trading Mentorship | Revolution Trading Pros',
		description: 'Join expert traders in our live trading rooms. Learn day trading, swing trading, and small account strategies with real-time alerts from institutional professionals with 15+ years experience.',
		canonical: '/live-trading-rooms',
		og: {
			type: 'website',
			image: 'https://revolution-trading-pros.pages.dev/images/live-trading-rooms-og.jpg',
			imageAlt: 'Professional Live Trading Rooms with Expert Mentors',
			imageWidth: 1200,
			imageHeight: 630
		}
	};
	return { ...data, seo };
};
