/**
 * R28-C extraction (2026-05-20): hoisted form shapes for the blog
 * categories/tags admin page. Children import these from here so the
 * parent stays the single source of truth for state, and prop typing
 * is consistent across CategoryFormModal / TagFormModal.
 */

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
