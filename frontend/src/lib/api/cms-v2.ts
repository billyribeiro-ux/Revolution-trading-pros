/**
 * CMS v2 API Client
 * Apple Principal Engineer ICT 7 Grade - January 2026
 *
 * Type-safe API client for the custom CMS backend.
 * Handles all content, asset, and settings operations.
 */

import type { PageBlock, CmsContentType, CmsContentStatus } from '$lib/page-builder/types';

const API_BASE = '/api/admin/cms-v2';
const PUBLIC_API_BASE = '/api/cms';

// ═══════════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════════

export interface CmsAssetFolder {
	id: string;
	name: string;
	slug: string;
	parent_id: string | null;
	path: string;
	depth: number;
	description: string | null;
	color: string | null;
	icon: string | null;
	is_public: boolean;
	sort_order: number;
	asset_count: number;
	created_at: string;
	updated_at: string;
}

export interface CmsAsset {
	id: string;
	folder_id: string | null;
	filename: string;
	original_filename: string;
	mime_type: string;
	file_size: number;
	file_extension: string;
	storage_provider: string;
	storage_key: string;
	url: string;
	cdn_url: string;
	width: number | null;
	height: number | null;
	aspect_ratio: number | null;
	blurhash: string | null;
	dominant_color: string | null;
	duration_seconds: number | null;
	thumbnail_url: string | null;
	variants: AssetVariant[] | null;
	title: string | null;
	alt_text: string | null;
	caption: string | null;
	description: string | null;
	credits: string | null;
	seo_title: string | null;
	seo_description: string | null;
	tags: string[] | null;
	usage_count: number;
	last_used_at: string | null;
	created_at: string;
	updated_at: string;
}

export interface AssetVariant {
	width: number;
	height: number;
	url: string;
	format: string;
	size: number;
}

export interface CmsAssetSummary {
	id: string;
	folder_id: string | null;
	filename: string;
	mime_type: string;
	file_size: number;
	url: string;
	cdn_url: string;
	width: number | null;
	height: number | null;
	blurhash: string | null;
	thumbnail_url: string | null;
	title: string | null;
	alt_text: string | null;
	usage_count: number;
	created_at: string;
}

export interface CmsContent {
	id: string;
	content_type: CmsContentType;
	slug: string;
	locale: string;
	is_primary_locale: boolean;
	parent_content_id: string | null;
	title: string;
	subtitle: string | null;
	excerpt: string | null;
	content: string | null;
	content_format: 'html' | 'markdown' | 'raw' | null;
	content_blocks: PageBlock[] | null;
	featured_image_id: string | null;
	og_image_id: string | null;
	gallery_ids: string[] | null;
	meta_title: string | null;
	meta_description: string | null;
	meta_keywords: string[] | null;
	canonical_url: string | null;
	robots_directives: string | null;
	structured_data: Record<string, unknown> | null;
	author_id: string | null;
	contributors: string[] | null;
	status: CmsContentStatus;
	published_at: string | null;
	scheduled_publish_at: string | null;
	scheduled_unpublish_at: string | null;
	primary_category_id: string | null;
	categories: string[] | null;
	custom_fields: Record<string, unknown> | null;
	template: string | null;
	version: number;
	created_at: string;
	updated_at: string;
}

export interface CmsContentSummary {
	id: string;
	content_type: CmsContentType;
	slug: string;
	locale: string;
	title: string;
	excerpt: string | null;
	featured_image_id: string | null;
	author_id: string | null;
	status: CmsContentStatus;
	published_at: string | null;
	created_at: string;
	updated_at: string;
}

export interface CmsRevision {
	id: string;
	contentId: string;
	revisionNumber: number;
	isCurrent: boolean;
	data: Record<string, unknown>;
	changeSummary: string | null;
	changedFields: string[] | null;
	createdAt: string;
	createdBy: string | null;
}

export interface CmsTag {
	id: string;
	name: string;
	slug: string;
	parentId: string | null;
	description: string | null;
	color: string | null;
	icon: string | null;
	metaTitle: string | null;
	metaDescription: string | null;
	usageCount: number;
	createdAt: string;
	updatedAt: string;
}

export interface CmsComment {
	id: string;
	contentId: string;
	parentId: string | null;
	threadId: string | null;
	body: string;
	blockId: string | null;
	selectionStart: number | null;
	selectionEnd: number | null;
	isResolved: boolean;
	resolvedBy: string | null;
	resolvedAt: string | null;
	mentionedUsers: string[] | null;
	createdAt: string;
	updatedAt: string;
	createdBy: string | null;
}

export interface CmsSiteSettings {
	id: string;
	siteName: string;
	siteTagline: string | null;
	siteDescription: string | null;
	logoLightId: string | null;
	logoDarkId: string | null;
	faviconId: string | null;
	ogDefaultImageId: string | null;
	contactEmail: string | null;
	supportEmail: string | null;
	phone: string | null;
	address: string | null;
	socialLinks: Record<string, string> | null;
	defaultMetaTitleSuffix: string | null;
	defaultRobots: string | null;
	googleAnalyticsId: string | null;
	googleTagManagerId: string | null;
	maintenanceMode: boolean;
	maintenanceMessage: string | null;
	headScripts: string | null;
	bodyStartScripts: string | null;
	bodyEndScripts: string | null;
	customCss: string | null;
	settings: Record<string, unknown> | null;
	updatedAt: string;
}

export interface CmsNavigationMenu {
	id: string;
	name: string;
	slug: string;
	location: string | null;
	items: NavigationMenuItem[];
	isActive: boolean;
	createdAt: string;
	updatedAt: string;
}

export interface NavigationMenuItem {
	id: string;
	label: string;
	url: string | null;
	contentId: string | null;
	target: string | null;
	icon: string | null;
	cssClass: string | null;
	isVisible: boolean;
	children: NavigationMenuItem[];
}

export interface CmsRedirect {
	id: string;
	sourcePath: string;
	targetPath: string;
	statusCode: number;
	isRegex: boolean;
	preserveQueryString: boolean;
	hitCount: number;
	lastHitAt: string | null;
	isActive: boolean;
	createdAt: string;
	updatedAt: string;
}

export interface PaginatedResponse<T> {
	data: T[];
	meta?: {
		total: number;
		limit: number;
		offset: number;
		hasMore: boolean;
	};
	pagination: {
		total: number;
		page: number;
		per_page: number;
		total_pages: number;
	};
}

export interface CmsStats {
	// Content counts
	total_content: number;
	published_content: number;
	draft_content: number;
	in_review_content: number;
	scheduled_content: number;
	// Asset counts
	total_assets: number;
	total_folders: number;
	// Other counts
	total_tags: number;
	total_menus: number;
	total_redirects: number;
}

// ═══════════════════════════════════════════════════════════════════════════════
// QUERY PARAMS
// ═══════════════════════════════════════════════════════════════════════════════

export interface ContentListParams {
	contentType?: CmsContentType;
	status?: CmsContentStatus;
	authorId?: string;
	locale?: string;
	search?: string;
	tagId?: string;
	categoryId?: string;
	sortBy?: 'created_at' | 'updated_at' | 'published_at' | 'title';
	sortOrder?: 'ASC' | 'DESC';
	limit?: number;
	offset?: number;
	includeDeleted?: boolean;
}

export interface AssetListParams {
	folder_id?: string;
	mime_type?: string;
	mime_type_filter?: string;
	search?: string;
	tags?: string[];
	page?: number;
	per_page?: number;
	sort_by?: 'created_at' | 'filename' | 'file_size' | 'usage_count';
	sortOrder?: 'ASC' | 'DESC';
	limit?: number;
	offset?: number;
	includeDeleted?: boolean;
}

// ═══════════════════════════════════════════════════════════════════════════════
// REQUEST TYPES
// ═══════════════════════════════════════════════════════════════════════════════

export interface CreateContentRequest {
	contentType: CmsContentType;
	slug: string;
	locale?: string;
	title: string;
	subtitle?: string;
	excerpt?: string;
	content?: string;
	contentFormat?: 'html' | 'markdown' | 'raw';
	contentBlocks?: PageBlock[];
	featuredImageId?: string;
	ogImageId?: string;
	metaTitle?: string;
	metaDescription?: string;
	metaKeywords?: string[];
	canonicalUrl?: string;
	authorId?: string;
	customFields?: Record<string, unknown>;
	template?: string;
}

export interface UpdateContentRequest {
	slug?: string;
	title?: string;
	subtitle?: string;
	excerpt?: string;
	content?: string;
	contentFormat?: 'html' | 'markdown' | 'raw';
	contentBlocks?: PageBlock[];
	featuredImageId?: string;
	ogImageId?: string;
	galleryIds?: string[];
	metaTitle?: string;
	metaDescription?: string;
	metaKeywords?: string[];
	canonicalUrl?: string;
	robotsDirectives?: string;
	structuredData?: Record<string, unknown>;
	authorId?: string;
	contributors?: string[];
	primaryCategoryId?: string;
	categories?: string[];
	customFields?: Record<string, unknown>;
	template?: string;
}

export interface TransitionStatusRequest {
	status: CmsContentStatus;
	comment?: string;
	scheduledAt?: string;
}

export interface CreateAssetFolderRequest {
	name: string;
	parentId?: string;
	description?: string;
	color?: string;
	icon?: string;
	isPublic?: boolean;
}

export interface UpdateAssetFolderRequest {
	name?: string;
	parentId?: string;
	description?: string;
	color?: string;
	icon?: string;
	isPublic?: boolean;
	sortOrder?: number;
}

export interface CreateAssetRequest {
	folderId?: string;
	filename: string;
	originalFilename: string;
	mimeType: string;
	fileSize: number;
	fileExtension: string;
	storageKey: string;
	cdnUrl: string;
	width?: number;
	height?: number;
	blurhash?: string;
	dominantColor?: string;
	title?: string;
	altText?: string;
	tags?: string[];
}

export interface UpdateAssetRequest {
	folderId?: string;
	title?: string;
	altText?: string;
	caption?: string;
	description?: string;
	credits?: string;
	seoTitle?: string;
	seoDescription?: string;
	tags?: string[];
}

export interface CreateTagRequest {
	name: string;
	parentId?: string;
	description?: string;
	color?: string;
	icon?: string;
}

export interface CreateCommentRequest {
	parentId?: string;
	body: string;
	blockId?: string;
	selectionStart?: number;
	selectionEnd?: number;
	mentionedUsers?: string[];
}

export interface UpdateSiteSettingsRequest {
	siteName?: string;
	siteTagline?: string;
	siteDescription?: string;
	logoLightId?: string;
	logoDarkId?: string;
	faviconId?: string;
	ogDefaultImageId?: string;
	contactEmail?: string;
	supportEmail?: string;
	phone?: string;
	address?: string;
	socialLinks?: Record<string, string>;
	defaultMetaTitleSuffix?: string;
	defaultRobots?: string;
	googleAnalyticsId?: string;
	googleTagManagerId?: string;
	maintenanceMode?: boolean;
	maintenanceMessage?: string;
	headScripts?: string;
	bodyStartScripts?: string;
	bodyEndScripts?: string;
	customCss?: string;
	settings?: Record<string, unknown>;
}

export interface CreateNavigationMenuRequest {
	name: string;
	location?: string;
	items: NavigationMenuItem[];
}

export interface UpdateNavigationMenuRequest {
	name?: string;
	location?: string;
	items?: NavigationMenuItem[];
	isActive?: boolean;
}

export interface CreateRedirectRequest {
	sourcePath: string;
	targetPath: string;
	statusCode?: number;
	isRegex?: boolean;
	preserveQueryString?: boolean;
}

// ═══════════════════════════════════════════════════════════════════════════════
// API CLIENT
// ═══════════════════════════════════════════════════════════════════════════════

async function fetchApi<T>(
	endpoint: string,
	options: RequestInit = {}
): Promise<T> {
	const response = await fetch(endpoint, {
		...options,
		headers: {
			'Content-Type': 'application/json',
			...options.headers
		},
		credentials: 'include'
	});

	if (!response.ok) {
		const error = await response.json().catch(() => ({ error: 'Unknown error' }));
		throw new Error(error.error || `HTTP ${response.status}`);
	}

	return response.json();
}

function buildQueryString(params: Record<string, unknown>): string {
	const searchParams = new URLSearchParams();
	for (const [key, value] of Object.entries(params)) {
		if (value !== undefined && value !== null) {
			if (Array.isArray(value)) {
				value.forEach((v) => searchParams.append(key, String(v)));
			} else {
				searchParams.set(key, String(value));
			}
		}
	}
	return searchParams.toString();
}

// ═══════════════════════════════════════════════════════════════════════════════
// STATS
// ═══════════════════════════════════════════════════════════════════════════════

export async function getCmsStats(): Promise<CmsStats> {
	return fetchApi<CmsStats>(`${API_BASE}/stats`);
}

// ═══════════════════════════════════════════════════════════════════════════════
// ASSET FOLDERS
// ═══════════════════════════════════════════════════════════════════════════════

export async function listAssetFolders(): Promise<CmsAssetFolder[]> {
	return fetchApi<CmsAssetFolder[]>(`${API_BASE}/folders`);
}

export async function getAssetFolder(id: string): Promise<CmsAssetFolder> {
	return fetchApi<CmsAssetFolder>(`${API_BASE}/folders/${id}`);
}

export async function createAssetFolder(
	request: CreateAssetFolderRequest
): Promise<CmsAssetFolder> {
	return fetchApi<CmsAssetFolder>(`${API_BASE}/folders`, {
		method: 'POST',
		body: JSON.stringify(request)
	});
}

export async function updateAssetFolder(
	id: string,
	request: UpdateAssetFolderRequest
): Promise<CmsAssetFolder> {
	return fetchApi<CmsAssetFolder>(`${API_BASE}/folders/${id}`, {
		method: 'PUT',
		body: JSON.stringify(request)
	});
}

export async function deleteAssetFolder(
	id: string,
	moveToId?: string
): Promise<void> {
	const query = moveToId ? `?move_to=${moveToId}` : '';
	await fetchApi<{ message: string }>(`${API_BASE}/folders/${id}${query}`, {
		method: 'DELETE'
	});
}

// ═══════════════════════════════════════════════════════════════════════════════
// ASSETS
// ═══════════════════════════════════════════════════════════════════════════════

export async function listAssets(
	params: AssetListParams = {}
): Promise<PaginatedResponse<CmsAssetSummary>> {
	const query = buildQueryString(params as Record<string, unknown>);
	return fetchApi<PaginatedResponse<CmsAssetSummary>>(
		`${API_BASE}/assets${query ? `?${query}` : ''}`
	);
}

export async function getAsset(id: string): Promise<CmsAsset> {
	return fetchApi<CmsAsset>(`${API_BASE}/assets/${id}`);
}

export async function createAsset(request: CreateAssetRequest): Promise<CmsAsset> {
	return fetchApi<CmsAsset>(`${API_BASE}/assets`, {
		method: 'POST',
		body: JSON.stringify(request)
	});
}

export async function updateAsset(
	id: string,
	request: UpdateAssetRequest
): Promise<CmsAsset> {
	return fetchApi<CmsAsset>(`${API_BASE}/assets/${id}`, {
		method: 'PUT',
		body: JSON.stringify(request)
	});
}

export async function deleteAsset(id: string): Promise<void> {
	await fetchApi<{ message: string }>(`${API_BASE}/assets/${id}`, {
		method: 'DELETE'
	});
}

// ═══════════════════════════════════════════════════════════════════════════════
// CONTENT
// ═══════════════════════════════════════════════════════════════════════════════

export async function listContent(
	params: ContentListParams = {}
): Promise<PaginatedResponse<CmsContentSummary>> {
	const query = buildQueryString(params as Record<string, unknown>);
	return fetchApi<PaginatedResponse<CmsContentSummary>>(
		`${API_BASE}/content${query ? `?${query}` : ''}`
	);
}

export async function getContent(id: string): Promise<CmsContent> {
	return fetchApi<CmsContent>(`${API_BASE}/content/${id}`);
}

export async function getContentBySlug(
	contentType: CmsContentType,
	slug: string,
	locale?: string
): Promise<CmsContent> {
	const query = locale ? `?locale=${locale}` : '';
	return fetchApi<CmsContent>(
		`${PUBLIC_API_BASE}/content/${contentType}/${slug}${query}`
	);
}

export async function createContent(request: CreateContentRequest): Promise<CmsContent> {
	return fetchApi<CmsContent>(`${API_BASE}/content`, {
		method: 'POST',
		body: JSON.stringify(request)
	});
}

export async function updateContent(
	id: string,
	request: UpdateContentRequest
): Promise<CmsContent> {
	return fetchApi<CmsContent>(`${API_BASE}/content/${id}`, {
		method: 'PUT',
		body: JSON.stringify(request)
	});
}

export async function transitionContentStatus(
	id: string,
	request: TransitionStatusRequest
): Promise<CmsContent> {
	return fetchApi<CmsContent>(`${API_BASE}/content/${id}/status`, {
		method: 'POST',
		body: JSON.stringify(request)
	});
}

export async function deleteContent(id: string): Promise<void> {
	await fetchApi<{ message: string }>(`${API_BASE}/content/${id}`, {
		method: 'DELETE'
	});
}

// ═══════════════════════════════════════════════════════════════════════════════
// REVISIONS
// ═══════════════════════════════════════════════════════════════════════════════

export async function getContentRevisions(
	contentId: string,
	limit = 25
): Promise<CmsRevision[]> {
	return fetchApi<CmsRevision[]>(
		`${API_BASE}/content/${contentId}/revisions?limit=${limit}`
	);
}

export async function restoreRevision(
	contentId: string,
	revisionNumber: number
): Promise<CmsContent> {
	return fetchApi<CmsContent>(
		`${API_BASE}/content/${contentId}/revisions/${revisionNumber}/restore`,
		{ method: 'POST' }
	);
}

// ═══════════════════════════════════════════════════════════════════════════════
// TAGS
// ═══════════════════════════════════════════════════════════════════════════════

export async function listTags(): Promise<CmsTag[]> {
	return fetchApi<CmsTag[]>(`${API_BASE}/tags`);
}

export async function createTag(request: CreateTagRequest): Promise<CmsTag> {
	return fetchApi<CmsTag>(`${API_BASE}/tags`, {
		method: 'POST',
		body: JSON.stringify(request)
	});
}

export async function getContentTags(contentId: string): Promise<CmsTag[]> {
	return fetchApi<CmsTag[]>(`${API_BASE}/content/${contentId}/tags`);
}

export async function addTagToContent(
	contentId: string,
	tagId: string
): Promise<void> {
	await fetchApi<{ message: string }>(
		`${API_BASE}/content/${contentId}/tags/${tagId}`,
		{ method: 'POST' }
	);
}

export async function removeTagFromContent(
	contentId: string,
	tagId: string
): Promise<void> {
	await fetchApi<{ message: string }>(
		`${API_BASE}/content/${contentId}/tags/${tagId}`,
		{ method: 'DELETE' }
	);
}

// ═══════════════════════════════════════════════════════════════════════════════
// COMMENTS
// ═══════════════════════════════════════════════════════════════════════════════

export async function getContentComments(contentId: string): Promise<CmsComment[]> {
	return fetchApi<CmsComment[]>(`${API_BASE}/content/${contentId}/comments`);
}

export async function createComment(
	contentId: string,
	request: CreateCommentRequest
): Promise<CmsComment> {
	return fetchApi<CmsComment>(`${API_BASE}/content/${contentId}/comments`, {
		method: 'POST',
		body: JSON.stringify(request)
	});
}

export async function resolveComment(commentId: string): Promise<void> {
	await fetchApi<{ message: string }>(
		`${API_BASE}/comments/${commentId}/resolve`,
		{ method: 'POST' }
	);
}

// ═══════════════════════════════════════════════════════════════════════════════
// SITE SETTINGS
// ═══════════════════════════════════════════════════════════════════════════════

export async function getSiteSettings(): Promise<CmsSiteSettings> {
	return fetchApi<CmsSiteSettings>(`${API_BASE}/settings`);
}

export async function updateSiteSettings(
	request: UpdateSiteSettingsRequest
): Promise<CmsSiteSettings> {
	return fetchApi<CmsSiteSettings>(`${API_BASE}/settings`, {
		method: 'PUT',
		body: JSON.stringify(request)
	});
}

export async function getPublicSiteSettings(): Promise<CmsSiteSettings> {
	return fetchApi<CmsSiteSettings>(`${PUBLIC_API_BASE}/settings`);
}

// ═══════════════════════════════════════════════════════════════════════════════
// NAVIGATION MENUS
// ═══════════════════════════════════════════════════════════════════════════════

export async function listNavigationMenus(): Promise<CmsNavigationMenu[]> {
	return fetchApi<CmsNavigationMenu[]>(`${API_BASE}/menus`);
}

export async function getNavigationMenu(slugOrId: string): Promise<CmsNavigationMenu> {
	return fetchApi<CmsNavigationMenu>(`${API_BASE}/menus/${slugOrId}`);
}

export async function createNavigationMenu(
	request: CreateNavigationMenuRequest
): Promise<CmsNavigationMenu> {
	return fetchApi<CmsNavigationMenu>(`${API_BASE}/menus`, {
		method: 'POST',
		body: JSON.stringify(request)
	});
}

export async function updateNavigationMenu(
	id: string,
	request: UpdateNavigationMenuRequest
): Promise<CmsNavigationMenu> {
	return fetchApi<CmsNavigationMenu>(`${API_BASE}/menus/${id}`, {
		method: 'PUT',
		body: JSON.stringify(request)
	});
}

export async function getPublicNavigationMenus(): Promise<CmsNavigationMenu[]> {
	return fetchApi<CmsNavigationMenu[]>(`${PUBLIC_API_BASE}/menus`);
}

export async function getPublicNavigationMenu(slug: string): Promise<CmsNavigationMenu> {
	return fetchApi<CmsNavigationMenu>(`${PUBLIC_API_BASE}/menus/${slug}`);
}

// ═══════════════════════════════════════════════════════════════════════════════
// REDIRECTS
// ═══════════════════════════════════════════════════════════════════════════════

export async function listRedirects(): Promise<CmsRedirect[]> {
	return fetchApi<CmsRedirect[]>(`${API_BASE}/redirects`);
}

export async function createRedirect(
	request: CreateRedirectRequest
): Promise<CmsRedirect> {
	return fetchApi<CmsRedirect>(`${API_BASE}/redirects`, {
		method: 'POST',
		body: JSON.stringify(request)
	});
}

export async function deleteRedirect(id: string): Promise<void> {
	await fetchApi<{ message: string }>(`${API_BASE}/redirects/${id}`, {
		method: 'DELETE'
	});
}

// ═══════════════════════════════════════════════════════════════════════════════
// ALIAS EXPORTS (for backward compatibility)
// ═══════════════════════════════════════════════════════════════════════════════

export type AssetListQuery = AssetListParams;
export type ContentListQuery = ContentListParams;

// ═══════════════════════════════════════════════════════════════════════════════
// API OBJECT EXPORT
// ═══════════════════════════════════════════════════════════════════════════════

export const cmsApi = {
	// Stats
	getStats: getCmsStats,

	// Folders
	listAssetFolders,
	getAssetFolder,
	createAssetFolder,
	updateAssetFolder,
	deleteAssetFolder,

	// Assets
	listAssets,
	getAsset,
	createAsset,
	updateAsset,
	deleteAsset,

	// Content
	listContent,
	getContent,
	getContentBySlug,
	createContent,
	updateContent,
	transitionContentStatus,
	deleteContent,

	// Revisions
	getContentRevisions,
	restoreRevision,

	// Tags
	listTags,
	createTag,
	getContentTags,
	addTagToContent,
	removeTagFromContent,

	// Comments
	getContentComments,
	createComment,
	resolveComment,

	// Settings
	getSiteSettings,
	updateSiteSettings,
	getPublicSiteSettings,

	// Menus
	listNavigationMenus,
	getNavigationMenu,
	createNavigationMenu,
	updateNavigationMenu,
	getPublicNavigationMenus,
	getPublicNavigationMenu,

	// Redirects
	listRedirects,
	createRedirect,
	deleteRedirect
};
