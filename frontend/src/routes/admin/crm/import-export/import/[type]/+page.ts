import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import type { ImportType } from '#lib/crm/types.js';
import type { SEOInput } from '#lib/seo/types.js';
export const load: PageLoad = ({ params }) => {
	if (
		!['contacts', 'tags', 'lists', 'sequences', 'campaigns', 'automations', 'templates'].includes(
			params.type
		)
	)
		error(404, 'Transfer type not found');
	return {
		kind: params.type as ImportType,
		seo: {
			title: `Import ${params.type} | CRM Admin`,
			description:
				'Upload and review saved CRM import data, map supported fields, inspect validation errors and follow recorded transfer status in the administration dashboard.'
		} satisfies SEOInput
	};
};
