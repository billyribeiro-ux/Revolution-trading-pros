import type { PageLoad } from './$types';
import type { SEOInput } from '$lib/seo/types';

export const prerender = true;

export const load: PageLoad = () => {
	const seo: SEOInput = {
		title: 'Terms of Service',
		titleTemplate: '%s — Revolution Trading Pros',
		description:
			'The legal agreement between you and Revolution Trading Pros covering use of our website, trading education, and live trading rooms.',
		og: {
			type: 'website',
			title: 'Terms of Service — Revolution Trading Pros',
			description:
				'The legal agreement between you and Revolution Trading Pros covering use of our website, trading education, and live trading rooms.'
		},
		jsonld: [
			{
				'@context': 'https://schema.org',
				'@type': 'WebPage',
				name: 'Terms of Service',
				description: 'Revolution Trading Pros terms of service and user agreement.',
				inLanguage: 'en-US',
				isPartOf: {
					'@type': 'WebSite',
					name: 'Revolution Trading Pros',
					url: 'https://revolution-trading-pros.pages.dev'
				}
			},
			{
				'@context': 'https://schema.org',
				'@type': 'BreadcrumbList',
				itemListElement: [
					{
						'@type': 'ListItem',
						position: 1,
						name: 'Home',
						item: 'https://revolution-trading-pros.pages.dev/'
					},
					{
						'@type': 'ListItem',
						position: 2,
						name: 'Terms of Service',
						item: 'https://revolution-trading-pros.pages.dev/terms-of-service'
					}
				]
			}
		]
	};

	return { seo };
};
