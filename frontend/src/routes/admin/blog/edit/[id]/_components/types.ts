// Shared types for the admin/blog/edit/[id] route extraction.
// The parent owns the canonical `post` $state; child components bind into it.

export type PostState = {
	id: number;
	title: string;
	slug: string;
	excerpt: string;
	content_blocks: unknown[];
	featured_image: string;
	featured_image_alt: string;
	featured_image_title: string;
	featured_image_caption: string;
	featured_image_description: string;
	featured_media_id: number | null;
	status: string;
	published_at: string;
	allow_comments: boolean;
	meta_title: string;
	meta_description: string;
	meta_keywords: string[];
	indexable: boolean;
	canonical_url: string;
	categories: string[];
	tags: number[];
};

export type TagRow = { id: number; name?: string; color?: string };
