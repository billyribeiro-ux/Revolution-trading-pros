/**
 * Room Resources API Client
 * Apple Principal Engineer ICT 7 Grade - January 2026
 *
 * Unified resource management for trading rooms:
 * - Videos (Bunny.net, Vimeo, YouTube, Direct)
 * - PDFs (Trade plans, guides)
 * - Documents (Word, Excel)
 * - Images (Charts, screenshots)
 *
 * @version 1.0.0
 */

import { API_BASE_URL } from './config';

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

export type ResourceType = 'video' | 'pdf' | 'document' | 'image' | 'spreadsheet' | 'archive' | 'other';

export type ContentType = 
	| 'tutorial' 
	| 'daily_video' 
	| 'weekly_watchlist' 
	| 'trade_plan' 
	| 'guide' 
	| 'chart' 
	| 'screenshot' 
	| 'template' 
	| 'cheat_sheet' 
	| 'archive' 
	| 'other';

export type VideoPlatform = 'bunny' | 'vimeo' | 'youtube' | 'wistia' | 'direct';

export type DifficultyLevel = 'beginner' | 'intermediate' | 'advanced';

export interface RoomResource {
	id: number;
	title: string;
	slug: string;
	description?: string;
	resource_type: ResourceType;
	content_type: ContentType;
	file_url: string;
	embed_url: string;
	mime_type?: string;
	file_size?: number;
	formatted_size: string;
	video_platform?: VideoPlatform;
	duration?: number;
	formatted_duration: string;
	thumbnail_url?: string;
	width?: number;
	height?: number;
	trading_room_id: number;
	trader_id?: number;
	resource_date: string;
	formatted_date: string;
	is_published: boolean;
	is_featured: boolean;
	is_pinned: boolean;
	category?: string;
	tags: string[];
	difficulty_level?: DifficultyLevel;
	views_count: number;
	downloads_count: number;
	created_at: string;
}

export interface CreateResourceRequest {
	title: string;
	description?: string;
	resource_type: ResourceType;
	content_type: ContentType;
	file_url: string;
	mime_type?: string;
	file_size?: number;
	video_platform?: VideoPlatform;
	bunny_video_guid?: string;
	bunny_library_id?: number;
	duration?: number;
	thumbnail_url?: string;
	width?: number;
	height?: number;
	trading_room_id: number;
	trader_id?: number;
	resource_date?: string;
	is_published?: boolean;
	is_featured?: boolean;
	is_pinned?: boolean;
	category?: string;
	tags?: string[];
	difficulty_level?: DifficultyLevel;
}

export interface UpdateResourceRequest {
	title?: string;
	description?: string;
	resource_type?: ResourceType;
	content_type?: ContentType;
	file_url?: string;
	mime_type?: string;
	file_size?: number;
	video_platform?: VideoPlatform;
	bunny_video_guid?: string;
	bunny_library_id?: number;
	duration?: number;
	thumbnail_url?: string;
	width?: number;
	height?: number;
	trader_id?: number;
	resource_date?: string;
	is_published?: boolean;
	is_featured?: boolean;
	is_pinned?: boolean;
	category?: string;
	tags?: string[];
	difficulty_level?: DifficultyLevel;
}

export interface ResourceListQuery {
	page?: number;
	per_page?: number;
	room_id?: number;
	resource_type?: ResourceType;
	content_type?: ContentType;
	is_featured?: boolean;
	is_published?: boolean;
	tags?: string;
	difficulty_level?: DifficultyLevel;
	search?: string;
}

export interface PaginationMeta {
	current_page: number;
	per_page: number;
	total: number;
	last_page: number;
}

export interface ResourceListResponse {
	success: boolean;
	data: RoomResource[];
	meta: PaginationMeta;
}

export interface ResourceResponse {
	success: boolean;
	data: RoomResource;
	message?: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

function buildQueryString(params: Record<string, any>): string {
	const query = new URLSearchParams();
	Object.entries(params).forEach(([key, value]) => {
		if (value !== undefined && value !== null && value !== '') {
			query.append(key, String(value));
		}
	});
	return query.toString();
}

async function handleResponse<T>(response: Response): Promise<T> {
	if (!response.ok) {
		const error = await response.json().catch(() => ({ error: 'Request failed' }));
		throw new Error(error.error || error.message || `HTTP ${response.status}`);
	}
	return response.json();
}

// ═══════════════════════════════════════════════════════════════════════════
// PUBLIC API
// ═══════════════════════════════════════════════════════════════════════════

/**
 * List resources (public - published only)
 */
export async function listResources(
	params: ResourceListQuery = {},
	fetchFn: typeof fetch = fetch
): Promise<ResourceListResponse> {
	const query = buildQueryString(params);
	const url = `${API_BASE_URL}/api/room-resources${query ? `?${query}` : ''}`;
	const response = await fetchFn(url);
	return handleResponse<ResourceListResponse>(response);
}

/**
 * Get single resource by ID or slug
 */
export async function getResource(
	idOrSlug: number | string,
	fetchFn: typeof fetch = fetch
): Promise<ResourceResponse> {
	const url = `${API_BASE_URL}/api/room-resources/${idOrSlug}`;
	const response = await fetchFn(url);
	return handleResponse<ResourceResponse>(response);
}

/**
 * Track resource download
 */
export async function trackDownload(
	id: number,
	fetchFn: typeof fetch = fetch
): Promise<{ success: boolean }> {
	const url = `${API_BASE_URL}/api/room-resources/${id}/download`;
	const response = await fetchFn(url, { method: 'POST' });
	return handleResponse<{ success: boolean }>(response);
}

// ═══════════════════════════════════════════════════════════════════════════
// ADMIN API
// ═══════════════════════════════════════════════════════════════════════════

/**
 * List all resources (admin - includes unpublished)
 */
export async function adminListResources(
	params: ResourceListQuery = {},
	fetchFn: typeof fetch = fetch
): Promise<ResourceListResponse> {
	const query = buildQueryString(params);
	const url = `${API_BASE_URL}/api/admin/room-resources${query ? `?${query}` : ''}`;
	const response = await fetchFn(url, { credentials: 'include' });
	return handleResponse<ResourceListResponse>(response);
}

/**
 * Create a new resource
 */
export async function createResource(
	data: CreateResourceRequest,
	fetchFn: typeof fetch = fetch
): Promise<ResourceResponse> {
	const url = `${API_BASE_URL}/api/admin/room-resources`;
	const response = await fetchFn(url, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify(data),
		credentials: 'include'
	});
	return handleResponse<ResourceResponse>(response);
}

/**
 * Update a resource
 */
export async function updateResource(
	id: number,
	data: UpdateResourceRequest,
	fetchFn: typeof fetch = fetch
): Promise<ResourceResponse> {
	const url = `${API_BASE_URL}/api/admin/room-resources/${id}`;
	const response = await fetchFn(url, {
		method: 'PUT',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify(data),
		credentials: 'include'
	});
	return handleResponse<ResourceResponse>(response);
}

/**
 * Delete a resource
 */
export async function deleteResource(
	id: number,
	fetchFn: typeof fetch = fetch
): Promise<{ success: boolean; message: string }> {
	const url = `${API_BASE_URL}/api/admin/room-resources/${id}`;
	const response = await fetchFn(url, {
		method: 'DELETE',
		credentials: 'include'
	});
	return handleResponse<{ success: boolean; message: string }>(response);
}

// ═══════════════════════════════════════════════════════════════════════════
// CONVENIENCE FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Get featured tutorial video for a room
 */
export async function getRoomTutorialVideo(
	roomId: number,
	fetchFn: typeof fetch = fetch
): Promise<RoomResource | null> {
	const response = await listResources({
		room_id: roomId,
		resource_type: 'video',
		content_type: 'tutorial',
		is_featured: true,
		per_page: 1
	}, fetchFn);
	return response.data?.[0] || null;
}

/**
 * Get latest daily videos for a room
 */
export async function getRoomDailyVideos(
	roomId: number,
	limit: number = 6,
	fetchFn: typeof fetch = fetch
): Promise<RoomResource[]> {
	const response = await listResources({
		room_id: roomId,
		resource_type: 'video',
		content_type: 'daily_video',
		per_page: limit
	}, fetchFn);
	return response.data || [];
}

/**
 * Get PDFs/documents for a room
 */
export async function getRoomDocuments(
	roomId: number,
	contentType?: ContentType,
	fetchFn: typeof fetch = fetch
): Promise<RoomResource[]> {
	const response = await listResources({
		room_id: roomId,
		resource_type: 'pdf',
		content_type: contentType,
		per_page: 50
	}, fetchFn);
	return response.data || [];
}

/**
 * Get images/charts for a room
 */
export async function getRoomImages(
	roomId: number,
	fetchFn: typeof fetch = fetch
): Promise<RoomResource[]> {
	const response = await listResources({
		room_id: roomId,
		resource_type: 'image',
		per_page: 50
	}, fetchFn);
	return response.data || [];
}

// ═══════════════════════════════════════════════════════════════════════════
// UTILITY FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Detect video platform from URL
 */
export function detectVideoPlatform(url: string): VideoPlatform {
	if (url.includes('vimeo.com')) return 'vimeo';
	if (url.includes('youtube.com') || url.includes('youtu.be')) return 'youtube';
	if (url.includes('bunny.net') || url.includes('b-cdn.net')) return 'bunny';
	if (url.includes('wistia.com')) return 'wistia';
	return 'direct';
}

/**
 * Detect resource type from MIME type
 */
export function detectResourceType(mimeType: string): ResourceType {
	if (mimeType.startsWith('video/')) return 'video';
	if (mimeType === 'application/pdf') return 'pdf';
	if (mimeType.startsWith('image/')) return 'image';
	if (
		mimeType.includes('spreadsheet') ||
		mimeType.includes('excel') ||
		mimeType === 'text/csv'
	) return 'spreadsheet';
	if (
		mimeType.includes('document') ||
		mimeType.includes('word') ||
		mimeType === 'text/plain'
	) return 'document';
	if (
		mimeType.includes('zip') ||
		mimeType.includes('compressed') ||
		mimeType.includes('archive')
	) return 'archive';
	return 'other';
}

/**
 * Get file extension from URL or filename
 */
export function getFileExtension(url: string): string {
	const match = url.match(/\.([a-zA-Z0-9]+)(?:\?|$)/);
	return match ? match[1].toLowerCase() : '';
}

/**
 * Get icon name for resource type
 */
export function getResourceIcon(resource: RoomResource): string {
	switch (resource.resource_type) {
		case 'video': return 'video';
		case 'pdf': return 'file-text';
		case 'document': return 'file-text';
		case 'image': return 'photo';
		case 'spreadsheet': return 'table';
		case 'archive': return 'archive';
		default: return 'file';
	}
}

/**
 * Format resource for display
 */
export function formatResourceForDisplay(resource: RoomResource): {
	icon: string;
	typeLabel: string;
	contentLabel: string;
	actionLabel: string;
	actionUrl: string;
} {
	const icon = getResourceIcon(resource);
	
	const typeLabels: Record<ResourceType, string> = {
		video: 'Video',
		pdf: 'PDF',
		document: 'Document',
		image: 'Image',
		spreadsheet: 'Spreadsheet',
		archive: 'Archive',
		other: 'File'
	};

	const contentLabels: Record<ContentType, string> = {
		tutorial: 'Tutorial',
		daily_video: 'Daily Video',
		weekly_watchlist: 'Weekly Watchlist',
		trade_plan: 'Trade Plan',
		guide: 'Guide',
		chart: 'Chart',
		screenshot: 'Screenshot',
		template: 'Template',
		cheat_sheet: 'Cheat Sheet',
		archive: 'Archive',
		other: 'Resource'
	};

	const actionLabel = resource.resource_type === 'video' ? 'Watch Now' : 'Download';
	const actionUrl = resource.resource_type === 'video' ? resource.embed_url : resource.file_url;

	return {
		icon,
		typeLabel: typeLabels[resource.resource_type] || 'File',
		contentLabel: contentLabels[resource.content_type] || 'Resource',
		actionLabel,
		actionUrl
	};
}

// ═══════════════════════════════════════════════════════════════════════════
// EXPORT API OBJECT
// ═══════════════════════════════════════════════════════════════════════════

export const roomResourcesApi = {
	// Public
	list: listResources,
	get: getResource,
	trackDownload,
	
	// Admin
	adminList: adminListResources,
	create: createResource,
	update: updateResource,
	delete: deleteResource,
	
	// Convenience
	getRoomTutorialVideo,
	getRoomDailyVideos,
	getRoomDocuments,
	getRoomImages,
	
	// Utilities
	detectVideoPlatform,
	detectResourceType,
	getFileExtension,
	getResourceIcon,
	formatResourceForDisplay
};

export default roomResourcesApi;
