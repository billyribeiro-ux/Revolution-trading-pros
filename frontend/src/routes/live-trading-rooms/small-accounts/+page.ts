import type { SEOInput } from '#lib/seo/types.js';
import { buildBreadcrumb, buildEvent, buildProduct } from '#lib/seo/schemas.js';
import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';

const SITE = PRODUCTION_ORIGIN;

export const prerender = true;

export const load = () => ({
	seo: {
		title: 'Small Account Options Trading Room',
		description:
			'Dedicated small-account options room focused on cash-account execution, PDT-free routines, SPX 0DTE risk control, and account preservation.',
		og: {
			type: 'website',
			image: `${SITE}/revolution-trading-pros.png`,
			imageAlt: 'Small Account Options Room — PDT-Free SPX Strategies'
		},
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Live Trading Rooms', url: `${SITE}/live-trading-rooms` },
				{ name: 'Small Accounts Mentorship', url: `${SITE}/live-trading-rooms/small-accounts` }
			]),
			buildProduct({
				name: 'Small Account Options Room',
				description:
					'Live trading room designed for growing small accounts (under $25k) using PDT-free cash-account routines and SPX 0DTE risk control.',
				url: `${SITE}/live-trading-rooms/small-accounts`,
				image: `${SITE}/revolution-trading-pros.png`,
				offers: { price: 197, priceCurrency: 'USD' }
			}),
			buildEvent({
				name: 'Small Account Options Room — Daily Session',
				description: 'Recurring weekday live session for traders growing accounts under $25k.',
				url: `${SITE}/live-trading-rooms/small-accounts`,
				startDate: '2026-05-26T09:30:00-04:00',
				eventAttendanceMode: 'OnlineEventAttendanceMode',
				organizer: { name: 'Revolution Trading Pros', url: SITE },
				isLiveBroadcast: true,
				location: { name: 'Revolution Trading Pros — Discord', url: SITE }
			})
		]
	} satisfies SEOInput
});
