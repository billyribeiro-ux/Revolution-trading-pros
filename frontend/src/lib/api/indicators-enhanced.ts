/**
 * Enhanced Indicators API - Revolution Trading Pros
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Apple Principal Engineer ICT 7 Grade - January 2026
 *
 * API client for:
 * - Indicator Management (CRUD)
 * - Platform Management
 * - Platform-specific Files
 * - Videos & Documentation
 * - TradingView Access Management
 * - Download Tracking
 */

// ICT 11+ CORB Fix: Use same-origin endpoints to prevent CORB
const API_BASE = '';

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

export interface Indicator {
	id: number;
	name: string;
	slug: string;
	subtitle?: string;
	description?: string;
	description_html?: string;
	short_description?: string;
	thumbnail_url?: string;
	preview_image_url?: string;
	preview_video_url?: string;
	category?: string;
	tags: string[];
	version: string;
	version_notes?: string;
	release_date?: string;
	is_published: boolean;
	is_featured: boolean;
	is_free: boolean;
	required_plan_id?: number;
	price_cents?: number;
	total_downloads: number;
	supported_platform_ids: number[];
	has_tradingview_access: boolean;
	tradingview_invite_only: boolean;
	videos?: IndicatorVideo[];
	platform_files?: PlatformFile[];
	documentation?: Documentation[];
	created_at: string;
	updated_at?: string;
}

export interface Platform {
	id: number;
	name: string;
	slug: string;
	display_name: string;
	icon_url?: string;
	file_extension?: string;
	installation_instructions?: string;
	is_active: boolean;
}

export interface PlatformSummary {
	id: number;
	name: string;
	display_name: string;
	icon_url?: string;
}

export interface IndicatorVideo {
	id: number;
	title: string;
	description?: string;
	video_url?: string;
	embed_url?: string;
	thumbnail_url?: string;
	duration_seconds?: number;
	formatted_duration?: string;
	video_type: string;
	sort_order: number;
	is_preview: boolean;
	is_published: boolean;
	view_count: number;
}

export interface PlatformFile {
	id: number;
	platform: PlatformSummary;
	file_url: string;
	file_name: string;
	file_size_bytes?: number;
	formatted_size: string;
	version: string;
	version_notes?: string;
	installation_notes?: string;
	is_latest: boolean;
	download_count: number;
	created_at: string;
}

export interface Documentation {
	id: number;
	title: string;
	doc_type: string;
	content_html?: string;
	file_url?: string;
	file_name?: string;
	sort_order: number;
	is_published: boolean;
}

export interface TradingViewAccess {
	id: number;
	user_id: number;
	user_email?: string;
	user_name?: string;
	tradingview_username: string;
	access_type: string;
	granted_at: string;
	expires_at?: string;
	is_active: boolean;
	is_expired: boolean;
	synced_to_tradingview: boolean;
	last_sync_at?: string;
	sync_error?: string;
	notes?: string;
}

export interface DownloadLog {
	id: number;
	user_id: number;
	user_email?: string;
	indicator_name: string;
	platform_name: string;
	file_name: string;
	downloaded_at: string;
}

export interface IndicatorStats {
	total_indicators: number;
	published_indicators: number;
	total_downloads: number;
	total_tradingview_users: number;
	downloads_by_platform: { platform_name: string; download_count: number }[];
	top_indicators: { indicator_id: number; indicator_name: string; download_count: number }[];
}

export interface CreateIndicatorRequest {
	name: string;
	subtitle?: string;
	description?: string;
	description_html?: string;
	short_description?: string;
	thumbnail_url?: string;
	preview_image_url?: string;
	preview_video_url?: string;
	category?: string;
	tags?: string[];
	version?: string;
	version_notes?: string;
	is_published?: boolean;
	is_featured?: boolean;
	is_free?: boolean;
	required_plan_id?: number;
	price_cents?: number;
	has_tradingview_access?: boolean;
	tradingview_invite_only?: boolean;
}

export interface CreatePlatformRequest {
	name: string;
	display_name: string;
	icon_url?: string;
	file_extension?: string;
	installation_instructions?: string;
	is_active?: boolean;
}

export interface CreateVideoRequest {
	title: string;
	description?: string;
	video_url?: string;
	bunny_video_guid?: string;
	thumbnail_url?: string;
	video_type?: string;
	is_preview?: boolean;
	is_published?: boolean;
}

export interface CreatePlatformFileRequest {
	platform_id: number;
	file_url: string;
	file_name: string;
	file_size_bytes?: number;
	version?: string;
	version_notes?: string;
	installation_notes?: string;
	is_latest?: boolean;
	checksum_sha256?: string;
}

export interface CreateDocumentationRequest {
	title: string;
	doc_type?: string;
	content_html?: string;
	file_url?: string;
	file_name?: string;
	is_published?: boolean;
}

export interface GrantTradingViewAccessRequest {
	user_id: number;
	tradingview_username: string;
	access_type?: string;
	expires_at?: string;
	notes?: string;
}

export interface IndicatorListQuery {
	page?: number;
	per_page?: number;
	category?: string;
	platform_id?: number;
	is_published?: boolean;
	is_featured?: boolean;
	has_tradingview?: boolean;
	search?: string;
}

export interface ApiResult<T> {
	success: boolean;
	data?: T;
	error?: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// API FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

async function apiRequest<T>(endpoint: string, options: RequestInit = {}): Promise<ApiResult<T>> {
	try {
		const response = await fetch(`${API_BASE}/admin/indicators-enhanced${endpoint}`, {
			...options,
			headers: {
				'Content-Type': 'application/json',
				...options.headers
			},
			credentials: 'include'
		});

		const data = await response.json();

		if (!response.ok) {
			return { success: false, error: data.error || `HTTP ${response.status}` };
		}

		return { success: true, data };
	} catch (error) {
		return { success: false, error: String(error) };
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// INDICATOR API
// ═══════════════════════════════════════════════════════════════════════════

export const indicatorsApi = {
	list: (query: IndicatorListQuery = {}) => {
		const params = new URLSearchParams();
		Object.entries(query).forEach(([key, value]) => {
			if (value !== undefined) params.append(key, String(value));
		});
		return apiRequest<{ indicators: Indicator[]; pagination: any }>(`?${params}`);
	},

	get: (indicatorId: number) => {
		return apiRequest<Indicator>(`/${indicatorId}`);
	},

	create: (data: CreateIndicatorRequest) => {
		return apiRequest<{ success: boolean; indicator: { id: number; name: string; slug: string } }>(
			'',
			{ method: 'POST', body: JSON.stringify(data) }
		);
	},

	update: (indicatorId: number, data: Partial<CreateIndicatorRequest>) => {
		return apiRequest<{ success: boolean }>(`/${indicatorId}`, {
			method: 'PUT',
			body: JSON.stringify(data)
		});
	},

	delete: (indicatorId: number) => {
		return apiRequest<{ success: boolean }>(`/${indicatorId}`, {
			method: 'DELETE'
		});
	},

	getStats: () => {
		return apiRequest<IndicatorStats>('/stats');
	},

	getDownloadLog: (
		query: {
			page?: number;
			per_page?: number;
			user_id?: number;
			platform_id?: number;
			from_date?: string;
			to_date?: string;
		} = {}
	) => {
		const params = new URLSearchParams();
		Object.entries(query).forEach(([key, value]) => {
			if (value !== undefined) params.append(key, String(value));
		});
		return apiRequest<{ downloads: DownloadLog[]; pagination: any }>(`/downloads?${params}`);
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// PLATFORM API
// ═══════════════════════════════════════════════════════════════════════════

export const platformsApi = {
	list: () => {
		return apiRequest<{ platforms: Platform[] }>('/platforms');
	},

	create: (data: CreatePlatformRequest) => {
		return apiRequest<{ success: boolean; platform: { id: number; name: string } }>('/platforms', {
			method: 'POST',
			body: JSON.stringify(data)
		});
	},

	update: (platformId: number, data: Partial<CreatePlatformRequest>) => {
		return apiRequest<{ success: boolean }>(`/platforms/${platformId}`, {
			method: 'PUT',
			body: JSON.stringify(data)
		});
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// VIDEO API
// ═══════════════════════════════════════════════════════════════════════════

export const indicatorVideosApi = {
	create: (indicatorId: number, data: CreateVideoRequest) => {
		return apiRequest<{ success: boolean; video: { id: number; title: string } }>(
			`/${indicatorId}/videos`,
			{ method: 'POST', body: JSON.stringify(data) }
		);
	},

	update: (indicatorId: number, videoId: number, data: Partial<CreateVideoRequest>) => {
		return apiRequest<{ success: boolean }>(`/${indicatorId}/videos/${videoId}`, {
			method: 'PUT',
			body: JSON.stringify(data)
		});
	},

	delete: (indicatorId: number, videoId: number) => {
		return apiRequest<{ success: boolean }>(`/${indicatorId}/videos/${videoId}`, {
			method: 'DELETE'
		});
	},

	reorder: (indicatorId: number, items: { id: number; sort_order: number }[]) => {
		return apiRequest<{ success: boolean }>(`/${indicatorId}/videos/reorder`, {
			method: 'PUT',
			body: JSON.stringify({ items })
		});
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// PLATFORM FILE API
// ═══════════════════════════════════════════════════════════════════════════

export const platformFilesApi = {
	create: (indicatorId: number, data: CreatePlatformFileRequest) => {
		return apiRequest<{ success: boolean; file: { id: number; file_name: string } }>(
			`/${indicatorId}/files`,
			{ method: 'POST', body: JSON.stringify(data) }
		);
	},

	update: (indicatorId: number, fileId: number, data: Partial<CreatePlatformFileRequest>) => {
		return apiRequest<{ success: boolean }>(`/${indicatorId}/files/${fileId}`, {
			method: 'PUT',
			body: JSON.stringify(data)
		});
	},

	delete: (indicatorId: number, fileId: number) => {
		return apiRequest<{ success: boolean }>(`/${indicatorId}/files/${fileId}`, {
			method: 'DELETE'
		});
	},

	bulkUpload: (
		indicatorId: number,
		files: {
			platform_id: number;
			file_url: string;
			file_name: string;
			file_size_bytes?: number;
			version?: string;
		}[]
	) => {
		return apiRequest<{ success: boolean; created: number }>(`/${indicatorId}/files/bulk`, {
			method: 'POST',
			body: JSON.stringify({ files })
		});
	},

	logDownload: (indicatorId: number, fileId: number, userId: number) => {
		return apiRequest<{ success: boolean }>(`/${indicatorId}/files/${fileId}/download`, {
			method: 'POST',
			body: JSON.stringify({ user_id: userId })
		});
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// DOCUMENTATION API
// ═══════════════════════════════════════════════════════════════════════════

export const documentationApi = {
	create: (indicatorId: number, data: CreateDocumentationRequest) => {
		return apiRequest<{ success: boolean; documentation: { id: number; title: string } }>(
			`/${indicatorId}/docs`,
			{ method: 'POST', body: JSON.stringify(data) }
		);
	},

	update: (indicatorId: number, docId: number, data: Partial<CreateDocumentationRequest>) => {
		return apiRequest<{ success: boolean }>(`/${indicatorId}/docs/${docId}`, {
			method: 'PUT',
			body: JSON.stringify(data)
		});
	},

	delete: (indicatorId: number, docId: number) => {
		return apiRequest<{ success: boolean }>(`/${indicatorId}/docs/${docId}`, {
			method: 'DELETE'
		});
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// TRADINGVIEW ACCESS API
// ═══════════════════════════════════════════════════════════════════════════

export const tradingViewAccessApi = {
	list: (
		indicatorId: number,
		query: {
			page?: number;
			per_page?: number;
			is_active?: boolean;
			is_synced?: boolean;
			search?: string;
		} = {}
	) => {
		const params = new URLSearchParams();
		Object.entries(query).forEach(([key, value]) => {
			if (value !== undefined) params.append(key, String(value));
		});
		return apiRequest<{ accesses: TradingViewAccess[]; pagination: any }>(
			`/${indicatorId}/tradingview?${params}`
		);
	},

	grant: (indicatorId: number, data: GrantTradingViewAccessRequest) => {
		return apiRequest<{ success: boolean; access: { id: number; tradingview_username: string } }>(
			`/${indicatorId}/tradingview`,
			{ method: 'POST', body: JSON.stringify(data) }
		);
	},

	bulkGrant: (
		indicatorId: number,
		accesses: { user_id: number; tradingview_username: string; access_type?: string }[]
	) => {
		return apiRequest<{ success: boolean; granted: number; errors: string[] }>(
			`/${indicatorId}/tradingview/bulk`,
			{ method: 'POST', body: JSON.stringify({ accesses }) }
		);
	},

	revoke: (indicatorId: number, accessId: number) => {
		return apiRequest<{ success: boolean }>(`/${indicatorId}/tradingview/${accessId}`, {
			method: 'DELETE'
		});
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// USER ACCESS API
// ═══════════════════════════════════════════════════════════════════════════

export const userAccessApi = {
	grant: (indicatorId: number, userId: number, source?: string, expiresAt?: string) => {
		return apiRequest<{ success: boolean; access: { id: number } }>(`/${indicatorId}/access`, {
			method: 'POST',
			body: JSON.stringify({ user_id: userId, access_source: source, expires_at: expiresAt })
		});
	},

	revoke: (indicatorId: number, userId: number) => {
		return apiRequest<{ success: boolean }>(`/${indicatorId}/access/${userId}`, {
			method: 'DELETE'
		});
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

export function formatFileSize(bytes: number): string {
	if (bytes < 1024) return `${bytes} B`;
	if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
	if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
	return `${(bytes / (1024 * 1024 * 1024)).toFixed(2)} GB`;
}

export function formatDuration(seconds: number): string {
	const hours = Math.floor(seconds / 3600);
	const minutes = Math.floor((seconds % 3600) / 60);
	const secs = seconds % 60;

	if (hours > 0) {
		return `${hours}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
	}
	return `${minutes}:${secs.toString().padStart(2, '0')}`;
}

export function validateTradingViewUsername(username: string): boolean {
	if (username.length < 3 || username.length > 20) return false;
	if (!/^[a-zA-Z]/.test(username)) return false;
	return /^[a-zA-Z0-9_]+$/.test(username);
}

export function getPlatformIcon(platformName: string): string {
	const icons: Record<string, string> = {
		thinkorswim: '/icons/platforms/thinkorswim.svg',
		tradingview: '/icons/platforms/tradingview.svg',
		ninjatrader: '/icons/platforms/ninjatrader.svg',
		metatrader: '/icons/platforms/metatrader.svg',
		tradestation: '/icons/platforms/tradestation.svg'
	};
	return icons[platformName.toLowerCase()] || '/icons/platforms/default.svg';
}
