import type { SEOInput } from '#lib/seo/types.js';
export const load = () => ({
	seo: {
		title: 'New Email Sequence | Admin',
		description:
			'Create an email sequence draft with ordered messages, sender settings, conditions and delays for confirmed contacts in the administration dashboard.'
	} satisfies SEOInput
});
