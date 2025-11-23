/**
 * Blog Post Types
 */

export interface Author {
	id: number;
	name: string;
}

export interface Post {
	id: number;
	title: string;
	slug: string;
	excerpt: string | null;
	content_blocks: any[] | null;
	featured_image: string | null;
	published_at: string;
	author?: Author;
	meta_title?: string | null;
	meta_description?: string | null;
	canonical_url?: string | null;
	schema_markup?: any | null;
	indexable?: boolean;
}

export interface PaginatedPosts {
	data: Post[];
	current_page: number;
	last_page: number;
	per_page: number;
	total: number;
	links: {
		first: string | null;
		last: string | null;
		prev: string | null;
		next: string | null;
	};
}
