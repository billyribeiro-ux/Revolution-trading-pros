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

// ICT 11+ CORB Fix: Use same-origin endpoints to prevent CORB

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

export type ResourceType =
	| 'video'
	| 'pdf'
	| 'document'
	| 'image'
	| 'spreadsheet'
	| 'archive'
	| 'other';

export type ContentType =
	| 'introduction' // ICT 7: Main videos
	| 'tutorial'
	| 'daily_video'
	| 'weekly_watchlist'
	| 'weekly_alert' // ICT 7: Explosive Swings
	| 'trade_plan'
	| 'guide'
	| 'chart'
	| 'screenshot'
	| 'template'
	| 'cheat_sheet'
	| 'archive'
	| 'other';

// ICT 7: Section types for dashboard organization
export type SectionType =
	| 'introduction'
	| 'latest_updates'
	| 'premium_daily_videos'
	| 'watchlist'
	| 'weekly_alerts'
	| 'learning_center';

export type VideoPlatform = 'bunny' | 'vimeo' | 'youtube' | 'wistia' | 'direct';

export type DifficultyLevel = 'beginner' | 'intermediate' | 'advanced';

// ICT 7 NEW: Access level types
export type AccessLevel = 'free' | 'member' | 'premium' | 'vip';

export interface RoomResource {
	id: number;
	title: string;
	slug: string;
	description?: string;
	resource_type: ResourceType;
	content_type: ContentType;
	section?: SectionType; // ICT 7: Section for dashboard organization
	file_url: string;
	embed_url: string;
	secure_download_url?: string; // ICT 7: Signed URL for secure downloads
	mime_type?: string;
	file_size?: number;
	formatted_size: string;
	video_platform?: VideoPlatform;
	bunny_video_guid?: string;
	bunny_library_id?: string;
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
	// ICT 7 NEW: Access control
	access_level: AccessLevel;
	requires_premium: boolean;
	// ICT 7 NEW: Versioning
	version: number;
	has_previous_version: boolean;
	is_latest_version: boolean;
	// ICT 7 NEW: Course/Lesson linking
	course_id?: number;
	lesson_id?: number;
	course_order?: number;
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
	section?: SectionType; // ICT 7: Section for organization
	category?: string;
	tags?: string[];
	difficulty_level?: DifficultyLevel;
	// ICT 7 NEW: Access control and linking
	access_level?: AccessLevel;
	course_id?: number;
	lesson_id?: number;
	course_order?: number;
	file_hash?: string;
	storage_provider?: 'r2' | 'bunny' | 's3' | 'local';
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
	// ICT 7 NEW: Access control and linking
	access_level?: AccessLevel;
	course_id?: number;
	lesson_id?: number;
	course_order?: number;
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
	// ICT 7 NEW: Access control and versioning filters
	access_level?: AccessLevel;
	course_id?: number;
	lesson_id?: number;
	latest_only?: boolean;
	section?: SectionType;
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
	const url = `/api/room-resources${query ? `?${query}` : ''}`;
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
	const url = `/api/room-resources/${idOrSlug}`;
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
	const url = `/api/room-resources/${id}/download`;
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
	const url = `/api/admin/room-resources${query ? `?${query}` : ''}`;
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
	const url = `/api/admin/room-resources`;
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
	const url = `/api/admin/room-resources/${id}`;
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
	const url = `/api/admin/room-resources/${id}`;
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
	const response = await listResources(
		{
			room_id: roomId,
			resource_type: 'video',
			content_type: 'tutorial',
			is_featured: true,
			per_page: 1
		},
		fetchFn
	);
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
	const response = await listResources(
		{
			room_id: roomId,
			resource_type: 'video',
			content_type: 'daily_video',
			per_page: limit
		},
		fetchFn
	);
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
	const response = await listResources(
		{
			room_id: roomId,
			resource_type: 'pdf',
			content_type: contentType,
			per_page: 50
		},
		fetchFn
	);
	return response.data || [];
}

/**
 * Get images/charts for a room
 */
export async function getRoomImages(
	roomId: number,
	fetchFn: typeof fetch = fetch
): Promise<RoomResource[]> {
	const response = await listResources(
		{
			room_id: roomId,
			resource_type: 'image',
			per_page: 50
		},
		fetchFn
	);
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
	if (mimeType.includes('spreadsheet') || mimeType.includes('excel') || mimeType === 'text/csv')
		return 'spreadsheet';
	if (mimeType.includes('document') || mimeType.includes('word') || mimeType === 'text/plain')
		return 'document';
	if (mimeType.includes('zip') || mimeType.includes('compressed') || mimeType.includes('archive'))
		return 'archive';
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
		case 'video':
			return 'video';
		case 'pdf':
			return 'file-text';
		case 'document':
			return 'file-text';
		case 'image':
			return 'photo';
		case 'spreadsheet':
			return 'table';
		case 'archive':
			return 'archive';
		default:
			return 'file';
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
		introduction: 'Introduction', // ICT 7: Main videos
		tutorial: 'Tutorial',
		daily_video: 'Daily Video',
		weekly_watchlist: 'Weekly Watchlist',
		weekly_alert: 'Weekly Alert', // ICT 7: Explosive Swings
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
// ICT 7 NEW: SECURE DOWNLOAD
// ═══════════════════════════════════════════════════════════════════════════

export interface SecureDownloadResponse {
	success: boolean;
	download_url: string;
	file_url: string;
	filename: string;
	expires_in_hours: number;
}

/**
 * Generate a secure download URL for a resource
 */
export async function getSecureDownloadUrl(
	id: number,
	fetchFn: typeof fetch = fetch
): Promise<SecureDownloadResponse> {
	const url = `/api/room-resources/${id}/secure-download`;
	const response = await fetchFn(url, {
		method: 'POST',
		credentials: 'include'
	});
	return handleResponse<SecureDownloadResponse>(response);
}

// ═══════════════════════════════════════════════════════════════════════════
// ICT 7 NEW: VERSION HISTORY
// ═══════════════════════════════════════════════════════════════════════════

export interface VersionHistoryResponse {
	success: boolean;
	data: RoomResource[];
	total_versions: number;
}

/**
 * Get version history for a resource
 */
export async function getVersionHistory(
	id: number,
	fetchFn: typeof fetch = fetch
): Promise<VersionHistoryResponse> {
	const url = `/api/room-resources/${id}/versions`;
	const response = await fetchFn(url);
	return handleResponse<VersionHistoryResponse>(response);
}

/**
 * Create a new version of a resource
 */
export async function createNewVersion(
	id: number,
	fileUrl: string,
	fileSize?: number,
	fetchFn: typeof fetch = fetch
): Promise<ResourceResponse> {
	const url = `/api/admin/room-resources/${id}/new-version`;
	const response = await fetchFn(url, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ file_url: fileUrl, file_size: fileSize }),
		credentials: 'include'
	});
	return handleResponse<ResourceResponse>(response);
}

// ═══════════════════════════════════════════════════════════════════════════
// ICT 7 NEW: BULK OPERATIONS
// ═══════════════════════════════════════════════════════════════════════════

export interface BulkOperationResponse {
	success: boolean;
	created_count?: number;
	updated_count?: number;
	deleted_count?: number;
	errors?: string[];
	message: string;
}

/**
 * Create multiple resources at once
 */
export async function bulkCreateResources(
	resources: CreateResourceRequest[],
	fetchFn: typeof fetch = fetch
): Promise<BulkOperationResponse> {
	const url = `/api/admin/room-resources/bulk-create`;
	const response = await fetchFn(url, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ resources }),
		credentials: 'include'
	});
	return handleResponse<BulkOperationResponse>(response);
}

export interface BulkUpdateFields {
	is_published?: boolean;
	is_featured?: boolean;
	is_pinned?: boolean;
	access_level?: string;
	category?: string;
	section?: string;
}

/**
 * Update multiple resources at once
 */
export async function bulkUpdateResources(
	ids: number[],
	updates: BulkUpdateFields,
	fetchFn: typeof fetch = fetch
): Promise<BulkOperationResponse> {
	const url = `/api/admin/room-resources/bulk-update`;
	const response = await fetchFn(url, {
		method: 'PUT',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ ids, updates }),
		credentials: 'include'
	});
	return handleResponse<BulkOperationResponse>(response);
}

/**
 * Delete multiple resources at once
 */
export async function bulkDeleteResources(
	ids: number[],
	fetchFn: typeof fetch = fetch
): Promise<BulkOperationResponse> {
	const url = `/api/admin/room-resources/bulk-delete`;
	const response = await fetchFn(url, {
		method: 'DELETE',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify(ids),
		credentials: 'include'
	});
	return handleResponse<BulkOperationResponse>(response);
}

// ═══════════════════════════════════════════════════════════════════════════
// ICT 7 NEW: FILE UPLOAD VALIDATION
// ═══════════════════════════════════════════════════════════════════════════

export interface UploadLimit {
	resource_type: string;
	max_file_size_bytes: number;
	allowed_mime_types: string[];
	allowed_extensions: string[];
	requires_premium: boolean;
}

export interface UploadLimitsResponse {
	success: boolean;
	data: UploadLimit[];
}

export interface ValidateUploadResponse {
	success: boolean;
	valid: boolean;
	error?: string;
	message?: string;
	requires_premium?: boolean;
}

/**
 * Get file upload limits for all resource types
 */
export async function getUploadLimits(
	fetchFn: typeof fetch = fetch
): Promise<UploadLimitsResponse> {
	const url = `/api/admin/room-resources/upload-limits`;
	const response = await fetchFn(url, { credentials: 'include' });
	return handleResponse<UploadLimitsResponse>(response);
}

/**
 * Validate a file before upload
 */
export async function validateUpload(
	resourceType: string,
	fileSize: number,
	mimeType?: string,
	extension?: string,
	fetchFn: typeof fetch = fetch
): Promise<ValidateUploadResponse> {
	const url = `/api/admin/room-resources/validate-upload`;
	const response = await fetchFn(url, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({
			resource_type: resourceType,
			file_size: fileSize,
			mime_type: mimeType,
			extension
		}),
		credentials: 'include'
	});
	return handleResponse<ValidateUploadResponse>(response);
}

// ═══════════════════════════════════════════════════════════════════════════
// ICT 7 NEW: COURSE/LESSON RESOURCES
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Get all resources for a course
 */
export async function getCourseResources(
	courseId: number,
	fetchFn: typeof fetch = fetch
): Promise<ResourceListResponse> {
	const url = `/api/room-resources/by-course/${courseId}`;
	const response = await fetchFn(url);
	return handleResponse<ResourceListResponse>(response);
}

/**
 * Get all resources for a lesson
 */
export async function getLessonResources(
	lessonId: number,
	fetchFn: typeof fetch = fetch
): Promise<ResourceListResponse> {
	const url = `/api/room-resources/by-lesson/${lessonId}`;
	const response = await fetchFn(url);
	return handleResponse<ResourceListResponse>(response);
}

// ═══════════════════════════════════════════════════════════════════════════
// EXPORT API OBJECT
// ═══════════════════════════════════════════════════════════════════════════

// ═══════════════════════════════════════════════════════════════════════════
// ICT 7 NEW: RESOURCE FAVORITES
// ═══════════════════════════════════════════════════════════════════════════

export interface FavoriteResponse {
	success: boolean;
	is_favorited?: boolean;
	added?: boolean;
	removed?: boolean;
	message?: string;
}

/**
 * Check if a resource is favorited by the current user
 */
export async function checkFavorite(
	id: number,
	fetchFn: typeof fetch = fetch
): Promise<FavoriteResponse> {
	const url = `/api/room-resources/${id}/favorite`;
	const response = await fetchFn(url, { credentials: 'include' });
	return handleResponse<FavoriteResponse>(response);
}

/**
 * Add a resource to favorites
 */
export async function addFavorite(
	id: number,
	fetchFn: typeof fetch = fetch
): Promise<FavoriteResponse> {
	const url = `/api/room-resources/${id}/favorite`;
	const response = await fetchFn(url, {
		method: 'POST',
		credentials: 'include'
	});
	return handleResponse<FavoriteResponse>(response);
}

/**
 * Remove a resource from favorites
 */
export async function removeFavorite(
	id: number,
	fetchFn: typeof fetch = fetch
): Promise<FavoriteResponse> {
	const url = `/api/room-resources/${id}/favorite`;
	const response = await fetchFn(url, {
		method: 'DELETE',
		credentials: 'include'
	});
	return handleResponse<FavoriteResponse>(response);
}

/**
 * Get user's favorite resources
 */
export async function getFavoriteResources(
	page: number = 1,
	perPage: number = 20,
	fetchFn: typeof fetch = fetch
): Promise<ResourceListResponse> {
	const query = buildQueryString({ page, per_page: perPage });
	const url = `/api/room-resources/favorites${query ? `?${query}` : ''}`;
	const response = await fetchFn(url, { credentials: 'include' });
	return handleResponse<ResourceListResponse>(response);
}

// ═══════════════════════════════════════════════════════════════════════════
// ICT 7 NEW: RECENTLY ACCESSED
// ═══════════════════════════════════════════════════════════════════════════

export interface RecentlyAccessed {
	id: number;
	user_id: number;
	resource_id: number;
	resource_type: string;
	resource_title: string;
	resource_thumbnail?: string;
	accessed_at: string;
}

export interface RecentlyAccessedResponse {
	success: boolean;
	data: RecentlyAccessed[];
}

/**
 * Track access to a resource
 */
export async function trackAccess(
	id: number,
	fetchFn: typeof fetch = fetch
): Promise<{ success: boolean }> {
	const url = `/api/room-resources/${id}/track-access`;
	const response = await fetchFn(url, {
		method: 'POST',
		credentials: 'include'
	});
	return handleResponse<{ success: boolean }>(response);
}

/**
 * Get recently accessed resources
 */
export async function getRecentlyAccessed(
	limit: number = 10,
	fetchFn: typeof fetch = fetch
): Promise<RecentlyAccessedResponse> {
	const url = `/api/room-resources/recently-accessed?limit=${limit}`;
	const response = await fetchFn(url, { credentials: 'include' });
	return handleResponse<RecentlyAccessedResponse>(response);
}

// ═══════════════════════════════════════════════════════════════════════════
// ICT 7 NEW: STOCK/ETF LISTS
// ═══════════════════════════════════════════════════════════════════════════

export interface StockSymbol {
	symbol: string;
	name?: string;
	sector?: string;
	notes?: string;
	price_target?: number;
	entry_price?: number;
	stop_loss?: number;
}

export interface StockList {
	id: number;
	name: string;
	slug: string;
	description?: string;
	list_type: 'etf' | 'stock' | 'watchlist' | 'sector';
	trading_room_id: number;
	symbols: StockSymbol[];
	is_active: boolean;
	is_featured: boolean;
	sort_order: number;
	week_of?: string;
	created_by?: number;
	created_at: string;
	updated_at: string;
}

export interface StockListResponse {
	success: boolean;
	data: StockList;
	message?: string;
}

export interface StockListsResponse {
	success: boolean;
	data: StockList[];
	meta: PaginationMeta;
}

export interface StockListQuery {
	room_id?: number;
	list_type?: string;
	is_active?: boolean;
	week_of?: string;
	page?: number;
	per_page?: number;
}

/**
 * List stock/ETF lists
 */
export async function listStockLists(
	params: StockListQuery = {},
	fetchFn: typeof fetch = fetch
): Promise<StockListsResponse> {
	const query = buildQueryString(params);
	const url = `/api/room-resources/stock-lists${query ? `?${query}` : ''}`;
	const response = await fetchFn(url);
	return handleResponse<StockListsResponse>(response);
}

/**
 * Get a single stock list
 */
export async function getStockList(
	id: number,
	fetchFn: typeof fetch = fetch
): Promise<StockListResponse> {
	const url = `/api/room-resources/stock-lists/${id}`;
	const response = await fetchFn(url);
	return handleResponse<StockListResponse>(response);
}

/**
 * Get latest watchlist for a room
 */
export async function getLatestWatchlist(
	roomId: number,
	fetchFn: typeof fetch = fetch
): Promise<StockListResponse> {
	const url = `/api/room-resources/stock-lists/latest/${roomId}`;
	const response = await fetchFn(url);
	return handleResponse<StockListResponse>(response);
}

export interface CreateStockListRequest {
	name: string;
	description?: string;
	list_type: string;
	trading_room_id: number;
	symbols: StockSymbol[];
	is_active?: boolean;
	is_featured?: boolean;
	week_of?: string;
}

/**
 * Create a stock list (admin)
 */
export async function createStockList(
	data: CreateStockListRequest,
	fetchFn: typeof fetch = fetch
): Promise<StockListResponse> {
	const url = `/api/admin/room-resources/stock-lists`;
	const response = await fetchFn(url, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify(data),
		credentials: 'include'
	});
	return handleResponse<StockListResponse>(response);
}

/**
 * Update a stock list (admin)
 */
export async function updateStockList(
	id: number,
	data: CreateStockListRequest,
	fetchFn: typeof fetch = fetch
): Promise<StockListResponse> {
	const url = `/api/admin/room-resources/stock-lists/${id}`;
	const response = await fetchFn(url, {
		method: 'PUT',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify(data),
		credentials: 'include'
	});
	return handleResponse<StockListResponse>(response);
}

/**
 * Delete a stock list (admin)
 */
export async function deleteStockList(
	id: number,
	fetchFn: typeof fetch = fetch
): Promise<{ success: boolean; message: string }> {
	const url = `/api/admin/room-resources/stock-lists/${id}`;
	const response = await fetchFn(url, {
		method: 'DELETE',
		credentials: 'include'
	});
	return handleResponse<{ success: boolean; message: string }>(response);
}

// ═══════════════════════════════════════════════════════════════════════════
// ICT 7 NEW: RESOURCE ANALYTICS
// ═══════════════════════════════════════════════════════════════════════════

export interface TypeStats {
	resource_type: string;
	count: number;
	total_views: number;
	total_downloads: number;
}

export interface AccessStats {
	access_level: string;
	count: number;
}

export interface ResourceStats {
	id: number;
	title: string;
	resource_type: string;
	views_count: number;
	downloads_count: number;
	created_at: string;
}

export interface ResourceAnalytics {
	total_resources: number;
	total_views: number;
	total_downloads: number;
	total_favorites: number;
	by_type: TypeStats[];
	by_access_level: AccessStats[];
	top_viewed: ResourceStats[];
	top_downloaded: ResourceStats[];
	recent_uploads: ResourceStats[];
}

export interface AnalyticsResponse {
	success: boolean;
	data: ResourceAnalytics;
}

/**
 * Get resource analytics (admin)
 */
export async function getResourceAnalytics(
	roomId?: number,
	fetchFn: typeof fetch = fetch
): Promise<AnalyticsResponse> {
	const query = roomId ? `?room_id=${roomId}` : '';
	const url = `/api/admin/room-resources/analytics${query}`;
	const response = await fetchFn(url, { credentials: 'include' });
	return handleResponse<AnalyticsResponse>(response);
}

export interface DownloadLog {
	id: number;
	resource_id: number;
	user_id?: number;
	ip_address?: string;
	downloaded_at: string;
}

export interface DownloadLogsResponse {
	success: boolean;
	data: DownloadLog[];
	meta: PaginationMeta;
}

/**
 * Get download logs (admin)
 */
export async function getDownloadLogs(
	resourceId?: number,
	page: number = 1,
	perPage: number = 50,
	fetchFn: typeof fetch = fetch
): Promise<DownloadLogsResponse> {
	const params: Record<string, any> = { page, per_page: perPage };
	if (resourceId) params.resource_id = resourceId;
	const query = buildQueryString(params);
	const url = `/api/admin/room-resources/download-logs${query ? `?${query}` : ''}`;
	const response = await fetchFn(url, { credentials: 'include' });
	return handleResponse<DownloadLogsResponse>(response);
}

// ═══════════════════════════════════════════════════════════════════════════
// EXPORT API OBJECT
// ═══════════════════════════════════════════════════════════════════════════

export const roomResourcesApi = {
	// Public
	list: listResources,
	get: getResource,
	trackDownload,
	getSecureDownloadUrl,
	getVersionHistory,
	getCourseResources,
	getLessonResources,

	// Admin
	adminList: adminListResources,
	create: createResource,
	update: updateResource,
	delete: deleteResource,
	createNewVersion,

	// Bulk Operations
	bulkCreate: bulkCreateResources,
	bulkUpdate: bulkUpdateResources,
	bulkDelete: bulkDeleteResources,

	// Upload Validation
	getUploadLimits,
	validateUpload,

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
	formatResourceForDisplay,

	// ICT 7 NEW: Favorites
	checkFavorite,
	addFavorite,
	removeFavorite,
	getFavoriteResources,

	// ICT 7 NEW: Recently Accessed
	trackAccess,
	getRecentlyAccessed,

	// ICT 7 NEW: Stock Lists
	listStockLists,
	getStockList,
	getLatestWatchlist,
	createStockList,
	updateStockList,
	deleteStockList,

	// ICT 7 NEW: Analytics
	getResourceAnalytics,
	getDownloadLogs
};

export default roomResourcesApi;
