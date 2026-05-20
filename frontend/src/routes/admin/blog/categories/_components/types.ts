/**
 * R27-E extraction (2026-05-20): shared types for the blog
 * categories/tags admin page. Re-exports the canonical `Category` /
 * `Tag` entity types from `$lib/api/admin`, plus the LOCAL form
 * shapes used by the create/edit modals (the form is a subset of the
 * entity — no `id`, `post_count`, timestamps, etc.).
 */
export type { Category, Tag } from '$lib/api/admin';

export interface CategoryFormData {
	name: string;
	slug: string;
	description: string;
	color: string;
	is_visible: boolean;
}

export interface TagFormData {
	name: string;
	slug: string;
	color: string;
	is_visible: boolean;
}

export type ToastType = 'success' | 'error';
