/**
 * Blog Post Types
 */

import type { JsonValue } from '$lib/types/common';

export interface Author {
	id: number;
	name: string;
}

export interface Post {
	id: number;
	title: string;
	slug: string;
	excerpt: string | null;
	// Heterogeneous editor block list (JSONB on the backend). Kept as `any[]`
	// because consumers assign `post.content_blocks ?? []` into concretely
	// typed `Draft.content_blocks: Block[]` fields (offline/db.ts,
	// offline/index.ts, offline/sync.ts) — widening to `unknown[]` would break
	// those assignments. Callers narrow to `Block[]` at use site.
	// eslint-disable-next-line @typescript-eslint/no-explicit-any -- see note above; `unknown[]` cascades into Block[] consumers
	content_blocks: any[] | null;
	featured_image: string | null;
	published_at: string;
	author?: Author;
	meta_title?: string | null;
	meta_description?: string | null;
	canonical_url?: string | null;
	schema_markup?: JsonValue | null;
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
