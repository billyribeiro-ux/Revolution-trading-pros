import type { SEOInput } from '$lib/seo/types';
import { buildBreadcrumb, buildEvent, buildService } from '$lib/seo/schemas';

const SITE = 'https://revolution-trading-pros.pages.dev';

// +page.ts
export const prerender = true;

export const load = () => ({
	seo: {
		title: 'Live SPX 0DTE Trading Room — Real-Time Voice & Screen Share',
		description:
			'Join the #1 live options trading room. Watch professional traders execute SPX 0DTE strategies with live voice commentary, 1080p screen sharing, and real-time mentorship.',
		og: {
			type: 'website',
			image: `${SITE}/images/og-live-room.jpg`,
			imageAlt: 'Live SPX Day Trading Room — Trade with Professional Traders'
		},
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Live Trading Rooms', url: `${SITE}/live-trading-rooms` },
				{ name: 'Day Trading', url: `${SITE}/live-trading-rooms/day-trading` }
			]),
			buildService({
				name: 'Live SPX Day Trading Room',
				description:
					'Professional day trading community specializing in SPX 0DTE options. Live voice commentary, 1080p screen sharing, and real-time trade alerts.',
				url: `${SITE}/live-trading-rooms/day-trading`,
				provider: { name: 'Revolution Trading Pros', url: SITE },
				serviceType: 'Live Trading Room'
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
