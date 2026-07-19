import type { SEOInput } from '$lib/seo/types';
import { buildBreadcrumb, buildEvent, buildService } from '$lib/seo/schemas';

const SITE = 'https://revolutiontradingpros.com';

// +page.ts
export const prerender = true;

export const load = () => ({
	seo: {
		title: 'Small Account Options Room — PDT-Free SPX Strategy for Accounts Under $25k',
		description:
			'A dedicated small-account options room focused on cash-account execution, PDT-free routines, SPX 0DTE risk control, and account-preservation habits.',
		og: {
			type: 'website',
			image: `${SITE}/images/day-trading-og.jpg`,
			imageAlt: 'Small Account Options Room — PDT-Free SPX Strategies'
		},
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Live Trading Rooms', url: `${SITE}/live-trading-rooms` },
				{ name: 'Small Accounts', url: `${SITE}/live-trading-rooms/small-accounts` }
			]),
			buildService({
				name: 'Small Account Options Room',
				description:
					'Live trading room designed for growing small accounts (under $25k) using PDT-free cash-account routines and SPX 0DTE risk control.',
				url: `${SITE}/live-trading-rooms/small-accounts`,
				provider: { name: 'Revolution Trading Pros', url: SITE },
				serviceType: 'Live Trading Room',
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
