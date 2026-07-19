import type { SEOInput } from '$lib/seo/types';
import { buildBreadcrumb, buildItemList, buildPerson } from '$lib/seo/schemas';
import { authors } from '$lib/authors/authors';

const SITE = 'https://revolutiontradingpros.com';

export const prerender = true;

export const load = () => ({
	authors,
	seo: {
		title: 'The Revolution Trading Pros Team',
		description:
			'Meet the traders and mentors behind Revolution Trading Pros — their track records, credentials, and trading specialties.',
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Team', url: `${SITE}/team` }
			]),
			buildItemList(
				'Revolution Trading Pros — Team',
				authors.map((a) => ({
					name: a.name,
					url: `${SITE}/authors/${a.slug}`,
					description: a.jobTitle ?? a.bio
				}))
			),
			...authors.map((a) =>
				buildPerson({
					name: a.name,
					url: `${SITE}/authors/${a.slug}`,
					jobTitle: a.jobTitle,
					description: a.bio || undefined,
					sameAs: a.sameAs,
					knowsAbout: a.knowsAbout,
					worksFor: { name: 'Revolution Trading Pros', url: SITE }
				})
			)
		]
	} satisfies SEOInput
});
