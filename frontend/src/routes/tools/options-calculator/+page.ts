import type { PageLoad } from './$types';
import type { SEOInput } from '#lib/seo/types.js';
import { generateOGMeta, generateStructuredData } from '#lib/options-calculator/utils/seo.js';

// Head tags come from the root layout's <Seo> (the site's single SEO owner, see
// #lib/seo/README.md), fed by `data.seo`: the title is templated to
// "… | Revolution Trading Pros", and the canonical / og:url derive from the pathname.
export const load: PageLoad = () => {
	const ogMeta = generateOGMeta();

	const seo: SEOInput = {
		title: 'Black-Scholes Options Calculator',
		description:
			'Free Black-Scholes options calculator with real-time Greeks (Delta, Gamma, Theta, Vega), payoff diagrams, strategy builder, and Monte Carlo simulation.',
		og: {
			title: ogMeta['og:title'],
			description: ogMeta['og:description'],
			type: ogMeta['og:type']
		},
		twitter: {
			card: ogMeta['twitter:card'],
			title: ogMeta['twitter:title'],
			description: ogMeta['twitter:description']
		},
		// FAQPage markup is deliberately left out: Google dropped FAQ rich results
		// (see generateFAQStructuredData's @deprecated note).
		jsonld: [generateStructuredData()]
	};

	return { seo };
};
