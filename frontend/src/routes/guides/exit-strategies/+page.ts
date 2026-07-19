import type { SEOInput } from '$lib/seo/types';
import { buildBreadcrumb, buildHowTo } from '$lib/seo/schemas';

const SITE = 'https://revolutiontradingpros.com';

export const prerender = true;

export const load = () => ({
	seo: {
		title: 'Exit Strategies Guide for Swing Trading',
		description:
			'Know when to take profits and protect gains. Learn the scaling-out method, target-based exits, and stop trailing techniques used by professional swing traders.',
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Guides', url: `${SITE}/guides` },
				{ name: 'Exit Strategies', url: `${SITE}/guides/exit-strategies` }
			]),
			buildHowTo({
				name: 'Build a Repeatable Swing-Trade Exit Plan',
				description:
					'A step-by-step plan for scaling out of swing trades and locking in profits without emotional interference.',
				steps: [
					{
						name: 'Define targets before entry',
						text: 'Calculate at least three profit targets (T1, T2, T3) based on supply zones and prior swing highs. Write them down before placing the trade.'
					},
					{
						name: 'Scale out at T1',
						text: 'Sell one-third to one-half of the position at the first target to lock in gains and reduce remaining risk to zero or near-zero.'
					},
					{
						name: 'Trail the stop',
						text: 'After T1, raise the stop loss to the entry price or just under, so the rest of the trade is risk-free.'
					},
					{
						name: 'Take final profit at T2 / T3',
						text: 'Exit the remainder at predetermined targets or on a confirmed trend break. Never let a winner turn into a loser.'
					}
				]
			})
		]
	} satisfies SEOInput
});
