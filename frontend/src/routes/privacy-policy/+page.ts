import type { PageLoad } from './$types';
import type { SEOInput } from '$lib/seo/types';

export const prerender = true;

export const load: PageLoad = () => {
	const seo: SEOInput = {
		title: 'Privacy Policy',
		titleTemplate: '%s — Revolution Trading Pros',
		description:
			'How Revolution Trading Pros collects, uses, and protects your personal information. GDPR, CCPA, and global data-rights compliant.',
		og: {
			type: 'website',
			title: 'Privacy Policy — Revolution Trading Pros',
			description:
				'How Revolution Trading Pros collects, uses, and protects your personal information.'
		},
		schema: [
			{
				'@context': 'https://schema.org',
				'@type': 'WebPage',
				name: 'Privacy Policy',
				description: 'Revolution Trading Pros privacy policy and data protection statement.',
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
						name: 'Privacy Policy',
						item: 'https://revolution-trading-pros.pages.dev/privacy-policy'
					}
				]
			}
		]
	};

	return { seo };
};
