/**
 * Learning Content API Service
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Fetch educational videos and resources from backend
 * NO MOCK DATA - All data from backend API
 *
 * @version 1.0.0
 */

import { browser } from '$app/environment';
import { authStore } from '$lib/stores/auth.svelte';

// ICT 7 FIX: VITE_API_URL does NOT include /api suffix (per config.ts pattern)
const PROD_API_ROOT = 'https://revolution-trading-pros-api.fly.dev';
const API_ROOT = browser ? (import.meta.env['VITE_API_URL'] || PROD_API_ROOT) : '';
const API_BASE = API_ROOT ? `${API_ROOT}/api` : '';

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

export interface Video {
	id: string;
	title: string;
	description?: string;
	duration: string;
	thumbnail_url?: string;
	video_url: string;
	category: string;
	membership_slug: string;
	order: number;
	created_at: string;
}

export interface VideoCategory {
	title: string;
	videos: Video[];
}

export interface Resource {
	id: string;
	title: string;
	description?: string;
	type: 'PDF' | 'Excel' | 'Word' | 'Other';
	file_size: string;
	download_url: string;
	category: string;
	membership_slug: string;
	created_at: string;
}

export interface ResourceCategory {
	category: string;
	items: Resource[];
}

// ═══════════════════════════════════════════════════════════════════════════
// HTTP UTILITIES
// ═══════════════════════════════════════════════════════════════════════════

async function getAuthHeaders(): Promise<Record<string, string>> {
	const token = authStore.getToken();
	const headers: Record<string, string> = {
		'Content-Type': 'application/json',
		Accept: 'application/json'
	};

	if (token) {
		headers['Authorization'] = `Bearer ${token}`;
	}

	return headers;
}

async function handleResponse<T>(response: Response): Promise<T> {
	if (!response.ok) {
		const error = await response.json().catch(() => ({ message: 'Request failed' }));
		throw new Error(error.message || 'Request failed');
	}

	const data = await response.json();
	return data.data ?? data;
}

// ═══════════════════════════════════════════════════════════════════════════
// API FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Get learning videos for a membership
 */
export async function getLearningVideos(membershipSlug: string): Promise<VideoCategory[]> {
	if (!browser) {
		return [];
	}

	const url = `${API_BASE}/memberships/${membershipSlug}/videos`;

	try {
		const response = await fetch(url, {
			method: 'GET',
			headers: await getAuthHeaders(),
			credentials: 'include'
		});

		const videos = await handleResponse<Video[]>(response);

		// Group videos by category
		const categorized = videos.reduce((acc, video) => {
			const existing = acc.find(cat => cat.title === video.category);
			if (existing) {
				existing.videos.push(video);
			} else {
				acc.push({
					title: video.category,
					videos: [video]
				});
			}
			return acc;
		}, [] as VideoCategory[]);

		// Sort videos within each category by order
		categorized.forEach(cat => {
			cat.videos.sort((a, b) => a.order - b.order);
		});

		return categorized;
	} catch (error) {
		console.error('[LearningContent] Error fetching videos:', error);
		return [];
	}
}

/**
 * Get resources for a membership
 */
export async function getResources(membershipSlug: string): Promise<ResourceCategory[]> {
	if (!browser) {
		return [];
	}

	const url = `${API_BASE}/memberships/${membershipSlug}/resources`;

	try {
		const response = await fetch(url, {
			method: 'GET',
			headers: await getAuthHeaders(),
			credentials: 'include'
		});

		const resources = await handleResponse<Resource[]>(response);

		// Group resources by category
		const categorized = resources.reduce((acc, resource) => {
			const existing = acc.find(cat => cat.category === resource.category);
			if (existing) {
				existing.items.push(resource);
			} else {
				acc.push({
					category: resource.category,
					items: [resource]
				});
			}
			return acc;
		}, [] as ResourceCategory[]);

		return categorized;
	} catch (error) {
		console.error('[LearningContent] Error fetching resources:', error);
		return [];
	}
}

export default {
	getLearningVideos,
	getResources
};
