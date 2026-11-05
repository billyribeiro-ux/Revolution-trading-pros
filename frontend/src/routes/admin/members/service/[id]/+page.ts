import type { PageLoad } from './$types';
import type { SEOInput } from '#lib/seo/types.js';
export const load: PageLoad = ({ params }) => ({
	seo: {
		title: `Service ${params.id} Members | Admin`,
		description:
			'Review recorded members, membership status and order value for this selected membership plan, with search and pagination.'
	} satisfies SEOInput
});
