/**
 * Bing SEO API Client
 * ===============================================================================
 *
 * Enterprise Bing SEO integration for instant URL indexing and search optimization.
 * This gives your content a MASSIVE competitive advantage - while competitors wait
 * days/weeks for indexing, your content appears in minutes.
 *
 * @version 1.0.0
 */

import { getAuthToken } from '#lib/stores/auth.svelte.js';
import type { JsonValue, QueryParams } from './_types';

// ICT 11+ CORB Fix: Use same-origin endpoints to prevent CORB

// ===============================================================================
// Type Definitions
// ===============================================================================

export interface IndexNowResponse {
	success: boolean;
	url: string;
	status_code: number;
	message: string;
	timestamp: string;
}

export interface BingSubmission {
	id: number;
	url: string;
	submission_type: 'indexnow' | 'indexnow_batch' | 'bing_webmaster' | 'sitemap';
	success: boolean;
	status_code: number;
	response_message: string;
	created_at: string;
}

export interface BingSearchPerformance {
	date: string;
	query: string;
	page_url: string;
	impressions: number;
	clicks: number;
	ctr: number;
	position: number;
}

export interface CoreWebVitals {
	url: string | null;
	device: 'all' | 'mobile' | 'desktop' | 'unknown';
	lcp: number | null;
	fid: number | null;
	cls: number | null;
	inp: number | null;
	ttfb: number | null;
	fcp: number | null;
	lcp_rating: string | null;
	fid_rating: string | null;
	cls_rating: string | null;
	inp_rating: string | null;
	overall_score: number | null;
	sample_counts: Record<string, number>;
	window_days: number;
	aggregation: 'p75';
	source: 'first_party_observations';
}
export interface PerformanceFilters {
	url?: string;
	device?: string;
	days?: number;
}

export interface BingMetaTags {
	'msvalidate.01': string;
	bingbot: string;
	msnbot: string;
	'geo.region': string;
	'geo.placename': string;
	'geo.position': string;
}

export interface BingSeoStats {
	total_submissions: number;
	successful_submissions: number;
	failed_submissions: number;
	success_rate: number;
	submissions_today: number;
	submissions_this_week: number;
	last_submission: string | null;
}

export interface PerformanceDashboard {
	server: {
		cpu_usage: number | null;
		memory_usage: number | null;
		disk_usage: number | null;
		uptime: string | null;
		status: string;
		scope: string;
		measured_at: string | null;
	};
	database: {
		query_count: number | null;
		slow_queries: number | null;
		connection_pool: number;
		avg_query_time: number | null;
		query_statistics_status: string;
		query_statistics_requirement: string | null;
	};
	cache: {
		hit_rate: number | null;
		memory_used: string;
		keys_count: number;
		evictions: number;
		scope: string;
		redis_configured: boolean;
	};
	core_web_vitals: CoreWebVitals;
}
export interface CacheWarmResult {
	success: boolean;
	views: number;
	routes: number;
	configs: number;
	rooms: number;
	plans: number;
	total: number;
	errors: string[];
}

export interface OptimizationRecommendation {
	category: string;
	priority: 'low' | 'medium' | 'high' | 'critical';
	title: string;
	description: string;
	impact: string;
	implementation: string;
}

// ===============================================================================
// API Client Class
// ===============================================================================

class BingSeoAPI {
	private async request<T>(
		method: string,
		endpoint: string,
		data?: unknown,
		params?: QueryParams
	): Promise<T> {
		// Use secure auth store token (memory-only, not localStorage)
		const token = getAuthToken();

		// ICT 11+ Fix: Add /api prefix if endpoint doesn't already have it
		const apiEndpoint = endpoint.startsWith('/api/') ? endpoint : `/api${endpoint}`;
		const search = new URLSearchParams();
		if (params) {
			// R13-A: handle `QueryParams` arrays (`?tag=a&tag=b`) — pre-R13-A
			// `Record<string, any>` typed the input, but the loop body called
			// `String(value)` which would serialise arrays to `"a,b"`. Match
			// the buildUrl shape used by `client.svelte.ts` / `config.ts`.
			for (const [key, value] of Object.entries(params)) {
				if (value === undefined || value === null) continue;
				if (Array.isArray(value)) {
					for (const item of value) search.append(key, String(item));
				} else {
					search.append(key, String(value));
				}
			}
		}

		const headers: Record<string, string> = {
			'Content-Type': 'application/json'
		};

		if (token) {
			headers['Authorization'] = `Bearer ${token}`;
		}

		const fetchOptions: RequestInit = {
			method,
			headers,
			...(data ? { body: JSON.stringify(data) } : {}),
			credentials: 'include'
		};

		const response = await fetch(`${apiEndpoint}${search.size ? `?${search}` : ''}`, fetchOptions);

		if (!response.ok) {
			// R13-A: `response.json()` is `Promise<unknown>` after tightening;
			// narrow to read `.message` instead of letting `any` paper over it.
			const parsed: unknown = await response.json().catch(() => null);
			const detail =
				parsed && typeof parsed === 'object' && 'error' in parsed ? parsed.error : parsed;
			const message =
				detail && typeof detail === 'object' && 'message' in detail
					? String(detail.message)
					: typeof detail === 'string'
						? detail
						: `HTTP ${response.status}`;
			throw new Error(message);
		}

		return response.json();
	}

	// ===============================================================================
	// IndexNow - Instant URL Indexing
	// ===============================================================================

	/**
	 * Submit a single URL to Bing IndexNow for instant indexing
	 * URLs get indexed in MINUTES instead of days/weeks
	 */
	async submitUrl(url: string): Promise<IndexNowResponse> {
		const response = await this.request<{ data: IndexNowResponse }>(
			'POST',
			'/seo/bing/submit-url',
			{
				url
			}
		);
		return response.data;
	}

	/**
	 * Submit multiple URLs in a single batch (up to 10,000 URLs)
	 */
	async submitBatch(
		urls: string[]
	): Promise<{ success: boolean; submitted: number; failed: number }> {
		const response = await this.request<{
			data: { success: boolean; submitted: number; failed: number };
		}>('POST', '/seo/bing/submit-batch', { urls });
		return response.data;
	}

	/**
	 * Get recent submission history
	 */
	async getSubmissions(params?: {
		page?: number;
		per_page?: number;
		type?: string;
		success?: boolean;
	}): Promise<{ submissions: BingSubmission[]; total: number }> {
		const response = await this.request<{ data: { submissions: BingSubmission[]; total: number } }>(
			'GET',
			'/seo/bing/submissions',
			undefined,
			params
		);
		return response.data;
	}

	/**
	 * Get submission statistics
	 */
	async getStats(): Promise<BingSeoStats> {
		const response = await this.request<{ data: BingSeoStats }>('GET', '/seo/bing/stats');
		return response.data;
	}

	// ===============================================================================
	// Bing Meta Tags
	// ===============================================================================

	/**
	 * Generate Bing-specific meta tags for your pages
	 */
	async getMetaTags(): Promise<BingMetaTags> {
		const response = await this.request<{ data: BingMetaTags }>('GET', '/seo/bing/meta-tags');
		return response.data;
	}

	// ===============================================================================
	// Search Performance (requires Bing Webmaster Tools API key)
	// ===============================================================================

	/**
	 * Get search performance data from Bing
	 */
	async getSearchPerformance(params?: {
		start_date?: string;
		end_date?: string;
		query?: string;
	}): Promise<BingSearchPerformance[]> {
		const response = await this.request<{ data: BingSearchPerformance[] }>(
			'GET',
			'/seo/bing/performance',
			undefined,
			params
		);
		return response.data;
	}

	/**
	 * Get top performing queries
	 */
	async getTopQueries(limit?: number): Promise<BingSearchPerformance[]> {
		const response = await this.request<{ data: BingSearchPerformance[] }>(
			'GET',
			'/seo/bing/top-queries',
			undefined,
			{ limit }
		);
		return response.data;
	}

	// ===============================================================================
	// Performance Optimization
	// ===============================================================================

	/**
	 * Get comprehensive performance dashboard
	 */
	async getPerformanceDashboard(filters: PerformanceFilters = {}): Promise<PerformanceDashboard> {
		const response = await this.request<{ data: PerformanceDashboard }>(
			'GET',
			'/performance/dashboard',
			undefined,
			{ ...filters }
		);
		return response.data;
	}

	/**
	 * Get Core Web Vitals for a URL
	 */
	async getCoreWebVitals(url?: string): Promise<CoreWebVitals> {
		const response = await this.request<{ data: CoreWebVitals }>(
			'GET',
			'/performance/core-web-vitals',
			undefined,
			{ url }
		);
		return response.data;
	}

	/**
	 * Get optimization recommendations
	 */
	async getRecommendations(filters: PerformanceFilters = {}): Promise<{
		recommendations: OptimizationRecommendation[];
		cdn: JsonValue | null;
		core_web_vitals: JsonValue | null;
	}> {
		// R13-A: `cdn` / `core_web_vitals` are heterogeneous JSON blobs the
		// only consumer (admin/performance/+page.svelte) doesn't read.
		// `JsonValue | null` preserves the wire-format escape hatch without
		// the `any` poison — if a future caller wants `.cdn.provider`,
		// they'll have to narrow.
		const response = await this.request<{
			data: {
				recommendations: OptimizationRecommendation[];
				cdn: JsonValue | null;
				core_web_vitals: JsonValue | null;
			};
		}>('GET', '/performance/recommendations', undefined, { ...filters });
		return response.data;
	}

	/**
	 * Warm caches for optimal performance
	 */
	async warmCaches(): Promise<CacheWarmResult> {
		const response = await this.request<{ data: CacheWarmResult }>(
			'POST',
			'/performance/warm-caches'
		);
		return response.data;
	}

	// ===============================================================================
	// Sitemap Management
	// ===============================================================================

	/**
	 * Submit sitemap to Bing
	 */
	async submitSitemap(sitemapUrl?: string): Promise<{ success: boolean; message: string }> {
		const response = await this.request<{ data: { success: boolean; message: string } }>(
			'POST',
			'/seo/bing/submit-sitemap',
			{ sitemap_url: sitemapUrl }
		);
		return response.data;
	}

	/**
	 * Get sitemap submission status
	 */
	async getSitemapStatus(): Promise<{
		submitted: boolean;
		last_submitted: string | null;
		urls_indexed: number;
	}> {
		const response = await this.request<{
			data: { submitted: boolean; last_submitted: string | null; urls_indexed: number };
		}>('GET', '/seo/bing/sitemap-status');
		return response.data;
	}
}

export const bingSeoApi = new BingSeoAPI();
export default bingSeoApi;
