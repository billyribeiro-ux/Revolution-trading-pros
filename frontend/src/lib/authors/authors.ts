/**
 * Author / instructor registry.
 *
 * Each entry seeds the `/authors/[slug]` profile page, `/team` listing,
 * and `Article.author` / `ProfilePage` JSON-LD schemas.
 * Sourced from `frontend/src/routes/about/+page.svelte:127-149`.
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
		jobTitle: 'Founder & Head Trader',
		bio: 'Billy Ribeiro is a globally recognized trader and market strategist and creator of the "Move Prior to The Move" price-action system. He leads live trading rooms and risk-management mentorship at Revolution Trading Pros.',
		knowsAbout: [
			'Day Trading',
			'SPX Options',
			'Price Action',
			'Market Structure',
			'Risk Management'
		]
	},
	{
		slug: 'freddie-ferber',
		name: 'Freddie Ferber',
		jobTitle: 'Co-Founder & Options Mentor',
		bio: 'Freddie Ferber is Co-Founder and Chart Master at Revolution Trading Pros, specializing in equity and options structure, technical analysis, and disciplined multi-timeframe execution.',
		knowsAbout: ['Options Strategies', 'Technical Analysis', 'Price Action', 'Market Structure']
	},
	{
		slug: 'shao-wen',
		name: 'Shao Wen',
		jobTitle: 'Mentor & Chart Master',
		bio: 'Shao Wen is a Senior Trading Mentor and Chart Master at Revolution Trading Pros, blending institutional price-action structure with multi-session technical analysis and precision charting.',
		knowsAbout: ['East-West Fusion', 'Precision Charting', 'Technical Analysis', 'Swing Trading']
	}
];

export function findAuthor(slug: string): Author | undefined {
	return authors.find((a) => a.slug === slug);
}
