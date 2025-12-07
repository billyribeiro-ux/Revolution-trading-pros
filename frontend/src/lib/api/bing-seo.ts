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

import { getAuthToken as getAuthStoreToken } from '$lib/stores/auth';

const API_BASE = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8000/api';

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
	url: string;
	device: 'mobile' | 'desktop';
	lcp: number;
	fid: number;
	cls: number;
	inp: number;
	ttfb: number;
	fcp: number;
	lcp_rating: 'good' | 'needs_improvement' | 'poor';
	fid_rating: 'good' | 'needs_improvement' | 'poor';
	cls_rating: 'good' | 'needs_improvement' | 'poor';
	inp_rating: 'good' | 'needs_improvement' | 'poor';
	overall_score: number;
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
		cpu_usage: number;
		memory_usage: number;
		disk_usage: number;
		uptime: string;
	};
	database: {
		query_count: number;
		slow_queries: number;
		connection_pool: number;
		avg_query_time: number;
	};
	cache: {
		hit_rate: number;
		memory_used: string;
		keys_count: number;
		evictions: number;
	};
	core_web_vitals: CoreWebVitals | null;
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
		data?: any,
		params?: Record<string, any>
	): Promise<T> {
		// Use secure auth store token (memory-only, not localStorage)
		const token = getAuthStoreToken();

		const url = new URL(`${API_BASE}${endpoint}`);
		if (params) {
			Object.entries(params).forEach(([key, value]) => {
				if (value !== undefined && value !== null) {
					url.searchParams.append(key, String(value));
				}
			});
		}

		const headers: Record<string, string> = {
			'Content-Type': 'application/json'
		};

		if (token) {
			headers['Authorization'] = `Bearer ${token}`;
		}

		const response = await fetch(url.toString(), {
			method,
			headers,
			body: data ? JSON.stringify(data) : undefined,
			credentials: 'include'
		});

		if (!response.ok) {
			const error = await response.json().catch(() => ({ message: 'Request failed' }));
			throw new Error(error.message || `HTTP ${response.status}`);
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
		const response = await this.request<{ data: IndexNowResponse }>('POST', '/seo/bing/submit-url', {
			url
		});
		return response.data;
	}

	/**
	 * Submit multiple URLs in a single batch (up to 10,000 URLs)
	 */
	async submitBatch(urls: string[]): Promise<{ success: boolean; submitted: number; failed: number }> {
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
	async getPerformanceDashboard(): Promise<PerformanceDashboard> {
		const response = await this.request<{ data: PerformanceDashboard }>(
			'GET',
			'/performance/dashboard'
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
	async getRecommendations(): Promise<{
		recommendations: OptimizationRecommendation[];
		cdn: any;
		core_web_vitals: any;
	}> {
		const response = await this.request<{
			data: {
				recommendations: OptimizationRecommendation[];
				cdn: any;
				core_web_vitals: any;
			};
		}>('GET', '/performance/recommendations');
		return response.data;
	}

	/**
	 * Warm caches for optimal performance
	 */
	async warmCaches(): Promise<{ views: number; routes: number; configs: number }> {
		const response = await this.request<{ data: { views: number; routes: number; configs: number } }>(
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
