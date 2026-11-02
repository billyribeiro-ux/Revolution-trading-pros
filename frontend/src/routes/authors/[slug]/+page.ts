import type { PageLoad, EntryGenerator } from './$types';
import { error } from '@sveltejs/kit';
import type { SEOInput } from '#lib/seo/types.js';
import { buildBreadcrumb, buildProfilePage } from '#lib/seo/schemas.js';
import { authors, findAuthor } from '#lib/authors/authors.js';
import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';

const SITE = PRODUCTION_ORIGIN;

export const prerender = true;

export const entries: EntryGenerator = () => authors.map((a) => ({ slug: a.slug }));

export const load: PageLoad = ({ params }) => {
	const author = findAuthor(params.slug);
	if (!author) {
		error(404, 'Author not found');
	}

	const url = `${SITE}/authors/${author.slug}`;
	const description =
		author.bio || `${author.name} — ${author.jobTitle ?? 'Revolution Trading Pros team member'}`;

	const seo: SEOInput = {
		title: `${author.name} — ${author.jobTitle ?? 'Author'}`,
		description,
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Our Team', url: `${SITE}/team` },
				{ name: author.name, url }
			]),
			buildProfilePage({
				url,
				name: `${author.name} — Author Profile`,
				description,
				dateCreated: '2025-01-01',
				dateModified: '2026-05-25',
				person: {
					name: author.name,
					url,
					jobTitle: author.jobTitle,
					description: author.bio || undefined,
					image: author.image ?? `${SITE}/revolution-trading-pros.png`,
					sameAs: author.sameAs,
					knowsAbout: author.knowsAbout,
					worksFor: { name: 'Revolution Trading Pros', url: `${SITE}/` }
				}
			})
		]
	};

	return { author, seo };
};
