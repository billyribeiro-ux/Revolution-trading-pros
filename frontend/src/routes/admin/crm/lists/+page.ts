import type { SEOInput } from '#lib/seo/types.js';
export const load = () => ({
	seo: {
		title: 'CRM Contact Lists | Admin',
		description:
			'Search CRM contact lists, manage list visibility and details, and review membership counts and contacts in the administration dashboard.'
	} satisfies SEOInput
});
