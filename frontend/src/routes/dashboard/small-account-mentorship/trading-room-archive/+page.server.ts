/**
 * Trading Room Archive Page Server
 * ═══════════════════════════════════════════════════════════════════════════
 * Apple ICT 11+ Principal Engineer Implementation
 *
 * Fetches room archive videos
 *
 * @version 3.0.0
 */

import { env } from '$env/dynamic/private';
import type { PageServerLoad } from './$types';

// Video response from API
interface VideoResponse {
	id: number;
	title: string;
	slug: string;
	description: string | null;
	video_url: string;
	thumbnail_url: string | null;
	duration: number | null;
	formatted_duration: string;
	content_type: string;
	video_date: string;
	formatted_date: string;
	is_published: boolean;
	trader: {
		id: number;
		name: string;
		slug: string;
	} | null;
	rooms: Array<{
		id: number;
		name: string;
		slug: string;
	}>;
	tags: string[];
	[key: string]: unknown;
}

// Page data type export for +page.svelte
export interface ArchivePageData {
	videos: VideoResponse[];
	meta: {
		current_page: number;
		per_page: number;
		total: number;
		last_page: number;
	};
	search: string;
	error: string | null;
}

export const load: PageServerLoad = async ({ url }): Promise<ArchivePageData> => {
	void env;

	// Get query params
	const page = url.searchParams.get('page') || '1';
	void page;
	const search = url.searchParams.get('search') || '';

	// TODO: Implement new video fetching approach
	// Returning empty data until new implementation is ready
	return {
		videos: [],
		meta: { current_page: 1, per_page: 50, total: 0, last_page: 1 },
		search,
		error: null
	};
};
