/**
 * Author / instructor registry.
 *
 * Each entry seeds the `/authors/[slug]` profile page and (eventually) the
 * BlogPosting.author byline schema. Entries MUST be human-curated — do not
 * synthesize bios. Empty `bio` indicates the placeholder is awaiting copy.
 */

export interface Author {
	slug: string;
	name: string;
	jobTitle?: string;
	bio: string;
	image?: string;
	sameAs?: string[];
	knowsAbout?: string[];
}

export const authors: Author[] = [
	{
		slug: 'billy-ribeiro',
		name: 'Billy Ribeiro',
		jobTitle: 'Founder & Lead Trader',
		bio: '',
		knowsAbout: ['Day Trading', 'SPX Options', 'Risk Management']
	},
	{
		slug: 'freddie-ferber',
		name: 'Freddie Ferber',
		jobTitle: 'Co-Founder & Options Mentor',
		bio: '',
		knowsAbout: ['Options Strategies', 'Technical Analysis']
	},
	{
		slug: 'shao-wen',
		name: 'Shao Wen',
		jobTitle: 'Mentor & Chart Master',
		bio: '',
		knowsAbout: ['East-West Fusion', 'Precision Charting']
	}
];

export function findAuthor(slug: string): Author | undefined {
	return authors.find((a) => a.slug === slug);
}
