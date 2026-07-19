import type { SEOInput } from '$lib/seo/types';
import { buildBreadcrumb, buildHowTo } from '$lib/seo/schemas';

const SITE = 'https://revolutiontradingpros.com';

export const prerender = true;

export const load = () => ({
	seo: {
		title: 'Swing Trade Entry Guide',
		description:
			'High-probability swing trade entry criteria: support/demand zones, volume confirmation, multi-timeframe alignment, and the entry checklist used by professional swing traders.',
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Guides', url: `${SITE}/guides` },
				{ name: 'Swing Entry', url: `${SITE}/guides/swing-entry` }
			]),
			buildHowTo({
				name: 'Execute a High-Probability Swing Trade Entry',
				description:
					'A checklist for entering swing trades only when multiple confirmation factors align.',
				steps: [
					{
						name: 'Identify the higher-timeframe trend',
						text: 'Confirm the weekly and daily timeframes agree on direction. Counter-trend trades go in size only when confluence is exceptional.'
					},
					{
						name: 'Locate the demand or supply zone',
						text: 'Mark the most recent swing high/low and the volume node where institutional accumulation occurred.'
					},
					{
						name: 'Wait for confirmation',
						text: 'Require at least one confirmation signal: bullish/bearish engulfing candle, RSI divergence, or volume spike on rejection of the zone.'
					},
					{
						name: 'Define entry, stop, and target before pulling the trigger',
						text: 'Write the trade plan: entry price, stop-loss invalidation level, and at least two profit targets. Calculate position size from the plan.'
					}
				]
			})
		]
	} satisfies SEOInput
});
