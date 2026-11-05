import type { SEOInput } from '#lib/seo/types.js';
export const load = () => ({
	seo: {
		title: 'Import and Export | CRM Admin',
		description:
			'Review recorded CRM transfers, upload contact files, map fields, inspect validation results and download filtered CSV or JSON contact snapshots.'
	} satisfies SEOInput
});
