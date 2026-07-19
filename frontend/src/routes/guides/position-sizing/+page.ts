import type { SEOInput } from '$lib/seo/types';
import { buildBreadcrumb, buildHowTo } from '$lib/seo/schemas';

const SITE = 'https://revolutiontradingpros.com';

export const prerender = true;

export const load = () => ({
	seo: {
		title: 'Position Sizing Guide for Swing Trading',
		description:
			'Size every trade so that risk drives the position, not gut feel. Position-sizing formulas, fixed-fractional sizing, and R-multiple thinking for active traders.',
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Guides', url: `${SITE}/guides` },
				{ name: 'Position Sizing', url: `${SITE}/guides/position-sizing` }
			]),
			buildHowTo({
				name: 'Calculate Position Size for Every Trade',
				description:
					'Use risk percentage and stop distance to drive position size, decoupling sizing from emotional bias.',
				steps: [
					{
						name: 'Pick your risk percentage',
						text: 'Choose a fixed-fractional risk per trade (typically 0.5%-2% of account equity). This number stays constant.'
					},
					{
						name: 'Measure the stop distance',
						text: 'Subtract the stop-loss price from the entry price. This is the per-share risk in dollars.'
					},
					{
						name: 'Solve for shares',
						text: 'Position size in shares = (Account Equity × Risk %) ÷ Per-Share Risk. Cap by available buying power.'
					},
					{
						name: 'Validate against R-multiples',
						text: 'Confirm the trade has at least a 2R reward target before committing capital. If not, reduce size or skip.'
					}
				]
			})
		]
	} satisfies SEOInput
});
