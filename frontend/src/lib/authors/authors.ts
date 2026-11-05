import authorRecords from './authors.json';

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

export const authors: Author[] = authorRecords;

export function findAuthor(slug: string): Author | undefined {
	return authors.find((a) => a.slug === slug);
}
