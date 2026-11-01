import type { SEOInput } from '#lib/seo/types.js';
import { buildBreadcrumb, buildEvent, buildProduct } from '#lib/seo/schemas.js';
import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';

const SITE = PRODUCTION_ORIGIN;

export const prerender = true;

export const load = () => ({
	seo: {
		title: 'Live SPX 0DTE Day Trading Room',
		description:
			'Join our live options trading room. Watch professional traders execute SPX 0DTE strategies with live voice commentary and 1080p screen share.',
		og: {
			type: 'website',
			image: `${SITE}/revolution-trading-pros.png`,
			imageAlt: 'Live SPX Day Trading Room — Trade with Professional Traders'
		},
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Live Trading Rooms', url: `${SITE}/live-trading-rooms` },
				{ name: 'Day Trading Room', url: `${SITE}/live-trading-rooms/day-trading` }
			]),
			buildProduct({
				name: 'Live SPX Day Trading Room',
				description:
					'Professional day trading community specializing in SPX 0DTE options with live voice commentary, 1080p screen sharing, and real-time trade alerts.',
				url: `${SITE}/live-trading-rooms/day-trading`,
				image: `${SITE}/revolution-trading-pros.png`,
				offers: { price: 197, priceCurrency: 'USD' }
			}),
			buildEvent({
				name: 'Live SPX 0DTE Trading Room — Daily Session',
				description:
					'Recurring weekday live trading session with real-time voice commentary and screen share for SPX 0DTE options.',
				url: `${SITE}/live-trading-rooms/day-trading`,
				startDate: '2026-05-26T09:30:00-04:00',
				eventAttendanceMode: 'OnlineEventAttendanceMode',
				organizer: { name: 'Revolution Trading Pros', url: SITE },
				isLiveBroadcast: true,
				location: { name: 'Revolution Trading Pros — Discord', url: SITE }
			})
		]
	} satisfies SEOInput
});
