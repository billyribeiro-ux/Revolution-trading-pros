import type { PageLoad } from './$types';
import { error } from '@sveltejs/kit';
import type { SEOInput } from '$lib/seo/types';
import { buildBreadcrumb, buildPerson, buildWebPage } from '$lib/seo/schemas';
import { findAuthor } from '$lib/authors/authors';

const SITE = 'https://revolutiontradingpros.com';

export const prerender = false;

export const load: PageLoad = ({ params }) => {
	const author = findAuthor(params.slug);
	if (!author) {
		error(404, { message: 'Author not found' });
	}

	const url = `${SITE}/authors/${author.slug}`;
	const description =
		author.bio || `${author.name} — ${author.jobTitle ?? 'Revolution Trading Pros team member'}`;

	const seo: SEOInput = {
		title: `${author.name} — Revolution Trading Pros`,
		description,
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Team', url: `${SITE}/team` },
				{ name: author.name, url }
			]),
			buildWebPage({
				url,
				name: `${author.name} — Author Profile`,
				description,
				dateModified: '2026-05-25',
				pageType: 'ProfilePage'
			}),
			buildPerson({
				name: author.name,
				url,
				jobTitle: author.jobTitle,
				description: author.bio || undefined,
				image: author.image,
				sameAs: author.sameAs,
				knowsAbout: author.knowsAbout,
				worksFor: { name: 'Revolution Trading Pros', url: SITE }
			})
		]
	};

	return { author, seo };
};
