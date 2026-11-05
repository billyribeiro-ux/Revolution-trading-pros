import type { SEOInput } from '#lib/seo/types.js';
export const load = () => ({
	seo: {
		title: 'Import Contacts | CRM Admin',
		description:
			'Upload a contact file, preview saved rows, map fields and apply a validated contact import with recorded results in the administration dashboard.'
	} satisfies SEOInput
});
