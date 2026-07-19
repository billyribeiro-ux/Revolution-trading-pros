import type { SEOInput } from '$lib/seo/types';
import { buildBreadcrumb, buildHowTo } from '$lib/seo/schemas';

const SITE = 'https://revolutiontradingpros.com';

export const prerender = true;

export const load = () => ({
	seo: {
		title: 'Risk Management Guide for Swing Trading',
		description:
			'Protect your capital and trade with confidence. The 2% rule, position sizing, stop loss placement, and risk/reward ratios used by professional swing traders.',
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Guides', url: `${SITE}/guides` },
				{ name: 'Risk Management', url: `${SITE}/guides/risk-management` }
			]),
			buildHowTo({
				name: 'Apply Professional Risk Management to Swing Trading',
				description:
					'Step-by-step playbook for sizing positions, placing stops, and managing risk/reward on every swing trade.',
				steps: [
					{
						name: 'Apply the 2% Rule',
						text: 'Never risk more than 2% of total account equity on any single trade. This caps drawdown and preserves capital across losing streaks.'
					},
					{
						name: 'Calculate Position Size',
						text: 'Shares to buy = (Account Size × Risk %) ÷ (Entry Price - Stop Loss). Lets risk drive size, not gut feel.'
					},
					{
						name: 'Place a Hard Stop Loss',
						text: 'Use technical stops (below support / swing low), ATR stops (1.5-2x ATR), or percentage stops (5-8% for swing trades). Never trade without one.'
					},
					{
						name: 'Demand at Least 2:1 Risk/Reward',
						text: 'Target a minimum 2:1 reward-to-risk ratio. Skip setups that do not meet this; over enough trades, R/R is what makes the system profitable.'
					},
					{
						name: 'Scale Out at Targets',
						text: 'Lock in profits at predefined targets and let runners pay for the position. Reduce emotion by automating exits where possible.'
					}
				]
			})
		]
	} satisfies SEOInput
});
