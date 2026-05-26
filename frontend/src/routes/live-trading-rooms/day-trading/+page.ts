import type { SEOInput } from '$lib/seo/types';
import { buildBreadcrumb, buildEvent, buildFAQPage, buildService } from '$lib/seo/schemas';

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
			}),
			buildFAQPage([
				{
					q: 'Do I need to be an expert to join?',
					a: "No. While the trading is fast-paced, we prioritize education. We provide a comprehensive 'New Member' onboarding video series to help you understand our levels, terminology, and platform execution before you take your first trade."
				},
				{
					q: 'What platform is the room hosted on?',
					a: 'We host our live room on a private, boosted Discord server. This allows for low-latency Voice Channels (sub-200ms) and crisp 1080p Screen Share, accessible from desktop or mobile.'
				},
				{
					q: 'What specific instruments do you trade?',
					a: 'We specialize in SPX (S&P 500 Index) options, specifically 0DTE (Zero Days to Expiration). We occasionally trade SPY, QQQ, and futures (ES/NQ) when setups present high probability, but SPX is our primary focus due to tax benefits and liquidity.'
				}
			])
		]
	} satisfies SEOInput
});
