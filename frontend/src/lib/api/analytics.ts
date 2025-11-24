/**
 * Analytics API Client - Enterprise Analytics Engine
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Complete API client for the enterprise analytics engine.
 * Provides type-safe access to KPIs, funnels, cohorts, attribution, and more.
 *
 * @version 1.0.0
 */

import { get } from 'svelte/store';
import { authStore } from '$lib/stores/auth';

// ═══════════════════════════════════════════════════════════════════════════
// Configuration
// ═══════════════════════════════════════════════════════════════════════════

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8000/api';

// ═══════════════════════════════════════════════════════════════════════════
// Type Definitions
// ═══════════════════════════════════════════════════════════════════════════

export interface KpiDefinition {
	key: string;
	name: string;
	description?: string;
	category: string;
	type: string;
	format: string;
	icon?: string;
	color?: string;
	is_primary: boolean;
}

export interface KpiValue {
	kpi_key: string;
	name: string;
	category: string;
	value: number;
	formatted_value: string;
	change_percentage: number;
	trend: 'up' | 'down' | 'stable';
	target_status?: string;
	is_anomaly: boolean;
	icon?: string;
	color?: string;
	is_primary: boolean;
}

export interface KpiTimeSeries {
	date: string;
	value: number;
	is_anomaly: boolean;
}

export interface KpiDetails {
	kpi: {
		key: string;
		name: string;
		category: string;
		format: string;
	};
	current_value: number;
	formatted_value: string;
	change_percentage: number;
	trend: string;
	time_series: KpiTimeSeries[];
}

export interface FunnelStep {
	step_number?: number;
	name: string;
	event_name?: string;
	count: number;
	conversion_rate: number;
	overall_conversion_rate?: number;
	drop_off?: number;
	drop_off_rate: number;
}

export interface FunnelAnalysis {
	funnel: {
		key: string;
		name: string;
		total_steps: number;
	};
	period: {
		start: string;
		end: string;
	};
	steps: FunnelStep[];
	summary: {
		total_entries: number;
		total_conversions: number;
		overall_conversion_rate: number;
		biggest_drop_off_step?: string;
		avg_conversion_rate: number;
	};
}

export interface FunnelDefinition {
	key: string;
	name: string;
	description?: string;
	type: string;
	steps_count: number;
	steps: {
		number: number;
		name: string;
		event: string;
	}[];
}

export interface CohortPeriod {
	active_users: number;
	retention_rate: number;
	total_revenue: number;
	avg_revenue_per_user: number;
}

export interface CohortRow {
	cohort?: string; // Alternative cohort identifier
	cohort_date?: string;
	size?: number; // Alternative size field
	cohort_size?: number;
	periods?: Record<number, CohortPeriod> | number[]; // Support both formats
}

export interface CohortDefinition {
	key: string;
	name: string;
	description?: string;
	type: string;
	grouping: string;
	metric_type: string;
}

export interface Cohort {
	key: string;
	name: string;
	description?: string;
	type: string;
	granularity: string;
	retention_matrix: CohortRow[];
	summary?: {
		avg_retention: number;
		best_cohort: string;
		worst_cohort: string;
	};
}

export interface RetentionCurvePoint {
	period: number;
	retention: number;
	cohorts: number;
}

export interface ChannelAttribution {
	channel: string;
	attributed_conversions: number;
	attributed_revenue: number;
	assisted_conversions: number;
	touchpoints: number;
	conversion_share: number;
	revenue_share: number;
	// Attribution model shares
	first_touch_share?: number;
	last_touch_share?: number;
	linear_share?: number;
	time_decay_share?: number;
	position_based_share?: number;
}

export interface AttributionReport {
	model: string;
	period: {
		start: string;
		end: string;
	};
	summary: {
		total_conversions: number;
		total_revenue: number;
	};
	channels: ChannelAttribution[];
	conversion_paths?: ConversionPath[];
}

export interface ConversionPath {
	path: string;
	conversions: number;
	revenue: number;
}

export interface Segment {
	key: string;
	name: string;
	description?: string;
	type: string;
	user_count: number;
	percentage: number;
	color?: string;
	icon?: string;
	is_system: boolean;
	rules?: any[]; // Segment rules/conditions
}

export interface Forecast {
	date: string;
	predicted_value: number;
	lower_bound: number;
	upper_bound: number;
	confidence: number;
}

export interface ForecastResult {
	metric_key: string;
	model: string;
	granularity: string;
	historical_points: number;
	model_accuracy: number;
	forecasts: Forecast[];
}

export interface RealTimeMetrics {
	active_users: number;
	events: number;
	page_views: number;
	conversions: number;
	revenue: number;
	top_pages: { page_path: string; views: number }[];
	top_events: { event_name: string; count: number }[];
}

export interface DashboardData {
	kpis: KpiValue[];
	realtime: RealTimeMetrics;
	anomalies: {
		anomalies_count: number;
		alerts_triggered: number;
		critical_count: number;
		recent_anomalies: any[];
		recent_alerts: any[];
	};
	funnels?: Array<{
		key: string;
		name: string;
		steps: FunnelStep[];
		overall_conversion: number;
	}>;
	cohorts?: Array<{
		key: string;
		name: string;
		retention_matrix: Array<{
			cohort: string;
			size: number;
			periods: number[];
		}>;
	}>;
	attribution?: {
		channels: ChannelAttribution[];
	};
	time_series?: {
		revenue?: Array<{ date: string; value: number }>;
		users?: Array<{ date: string; value: number }>;
	};
	top_pages?: Array<{ page_path: string; views: number }>;
	top_events?: Array<{ event_name: string; count: number }>;
	period: string;
	generated_at: string;
}

export interface AnalyticsEvent {
	id: string;
	event_name: string;
	event_type: string;
	event_category?: string;
	user_id?: number;
	session_id?: string;
	page_path?: string;
	channel?: string;
	properties?: Record<string, any>;
	created_at: string;
}

export interface Report {
	id: string;
	name: string;
	description?: string;
	type: 'dashboard' | 'table' | 'chart' | 'mixed';
	config: {
		metrics: string[];
		dimensions: string[];
	};
	schedule?: {
		frequency: 'daily' | 'weekly' | 'monthly';
		recipients: string[];
	};
	status: 'active' | 'paused' | 'draft';
	created_at: string;
	last_run?: string;
}

export interface ConversionPathData {
	channels: string[];
	conversions: number;
	revenue: number;
}

export interface TimeSeriesData {
	period: string;
	event_count: number;
	unique_users: number;
	sessions: number;
	conversions: number;
	total_revenue: number;
}

export interface BreakdownData {
	[key: string]: number;
}

// ═══════════════════════════════════════════════════════════════════════════
// API Client
// ═══════════════════════════════════════════════════════════════════════════

class AnalyticsApiClient {
	private getAuthHeaders(): Record<string, string> {
		const auth = get(authStore);
		const headers: Record<string, string> = {
			'Content-Type': 'application/json',
			Accept: 'application/json'
		};

		if (auth.token) {
			headers['Authorization'] = `Bearer ${auth.token}`;
		}

		return headers;
	}

	private async request<T>(
		method: string,
		endpoint: string,
		body?: any,
		params?: Record<string, any>
	): Promise<T> {
		let url = `${API_BASE_URL}${endpoint}`;

		if (params) {
			const searchParams = new URLSearchParams();
			Object.entries(params).forEach(([key, value]) => {
				if (value !== undefined && value !== null) {
					searchParams.append(key, String(value));
				}
			});
			const queryString = searchParams.toString();
			if (queryString) {
				url += `?${queryString}`;
			}
		}

		const response = await fetch(url, {
			method,
			headers: this.getAuthHeaders(),
			body: body ? JSON.stringify(body) : undefined
		});

		if (!response.ok) {
			const error = await response.json().catch(() => ({ message: 'Request failed' }));
			throw new Error(error.message || `HTTP ${response.status}`);
		}

		return response.json();
	}

	// ═══════════════════════════════════════════════════════════════════════
	// Event Tracking (Public)
	// ═══════════════════════════════════════════════════════════════════════

	async trackEvent(eventData: {
		event_name: string;
		event_category?: string;
		event_type?: string;
		entity_type?: string;
		entity_id?: number;
		properties?: Record<string, any>;
		event_value?: number;
		revenue?: number;
	}): Promise<{ success: boolean; event_id: string }> {
		return this.request('POST', '/analytics/track', eventData);
	}

	async trackPageView(data: {
		page_url?: string;
		page_title?: string;
		page_type?: string;
		referrer?: string;
	}): Promise<{ success: boolean; event_id: string }> {
		return this.request('POST', '/analytics/pageview', data);
	}

	async trackBatch(
		events: Array<{
			event_name: string;
			event_category?: string;
			properties?: Record<string, any>;
		}>
	): Promise<{ success: boolean; processed: number }> {
		return this.request('POST', '/analytics/batch', { events });
	}

	// ═══════════════════════════════════════════════════════════════════════
	// Dashboard & KPIs
	// ═══════════════════════════════════════════════════════════════════════

	async getDashboard(period: string = '30d'): Promise<DashboardData> {
		return this.request('GET', '/admin/analytics/dashboard', undefined, { period });
	}

	async getKpiDefinitions(): Promise<{ definitions: KpiDefinition[] }> {
		return this.request('GET', '/admin/analytics/kpis');
	}

	async getKpi(kpiKey: string, period: string = '30d', granularity: string = 'daily'): Promise<KpiDetails> {
		return this.request('GET', `/admin/analytics/kpis/${kpiKey}`, undefined, { period, granularity });
	}

	// ═══════════════════════════════════════════════════════════════════════
	// Funnel Analytics
	// ═══════════════════════════════════════════════════════════════════════

	async getFunnelAnalysis(funnelKey: string, period: string = '30d'): Promise<FunnelAnalysis> {
		return this.request('GET', `/admin/analytics/funnels/${funnelKey}`, undefined, { period });
	}

	async getFunnelDropOff(funnelKey: string, period: string = '30d'): Promise<any> {
		return this.request('GET', `/admin/analytics/funnels/${funnelKey}/dropoff`, undefined, { period });
	}

	async getFunnelBySegment(
		funnelKey: string,
		segmentField: string,
		period: string = '30d'
	): Promise<any> {
		return this.request('GET', `/admin/analytics/funnels/${funnelKey}/segment`, undefined, {
			segment_field: segmentField,
			period
		});
	}

	// ═══════════════════════════════════════════════════════════════════════
	// Cohort Analytics
	// ═══════════════════════════════════════════════════════════════════════

	async getCohortMatrix(cohortKey: string, period: string = '90d'): Promise<{ cohort_key: string; matrix: CohortRow[] }> {
		return this.request('GET', `/admin/analytics/cohorts/${cohortKey}/matrix`, undefined, { period });
	}

	async getCohortCurve(cohortKey: string, period: string = '90d'): Promise<{ cohort_key: string; curve: RetentionCurvePoint[] }> {
		return this.request('GET', `/admin/analytics/cohorts/${cohortKey}/curve`, undefined, { period });
	}

	async getCohortLTV(cohortKey: string, period: string = '90d'): Promise<any> {
		return this.request('GET', `/admin/analytics/cohorts/${cohortKey}/ltv`, undefined, { period });
	}

	// ═══════════════════════════════════════════════════════════════════════
	// Attribution Analytics
	// ═══════════════════════════════════════════════════════════════════════

	async getCampaignAttribution(
		model: string = 'linear',
		period: string = '30d'
	): Promise<{ campaigns: any[] }> {
		return this.request('GET', '/admin/analytics/attribution/campaigns', undefined, { model, period });
	}

	async getConversionPaths(period: string = '30d', limit: number = 20): Promise<{ paths: ConversionPath[] }> {
		return this.request('GET', '/admin/analytics/attribution/paths', undefined, { period, limit });
	}

	async compareAttributionModels(period: string = '30d'): Promise<{ models: Record<string, any> }> {
		return this.request('GET', '/admin/analytics/attribution/compare', undefined, { period });
	}

	// ═══════════════════════════════════════════════════════════════════════
	// Forecasting
	// ═══════════════════════════════════════════════════════════════════════

	async getForecast(
		kpiKey: string,
		periodsAhead: number = 14,
		model: string = 'linear',
		granularity: string = 'daily'
	): Promise<ForecastResult> {
		return this.request('GET', `/admin/analytics/forecast/${kpiKey}`, undefined, {
			periods_ahead: periodsAhead,
			model,
			granularity
		});
	}

	async getForecastAccuracy(kpiKey: string, granularity: string = 'daily'): Promise<any> {
		return this.request('GET', `/admin/analytics/forecast/${kpiKey}/accuracy`, undefined, { granularity });
	}

	// ═══════════════════════════════════════════════════════════════════════
	// Segments
	// ═══════════════════════════════════════════════════════════════════════

	async getSegments(): Promise<{ segments: Segment[] }> {
		return this.request('GET', '/admin/analytics/segments');
	}

	async getSegment(segmentKey: string): Promise<{ segment: Segment & { rules?: any } }> {
		return this.request('GET', `/admin/analytics/segments/${segmentKey}`);
	}

	// ═══════════════════════════════════════════════════════════════════════
	// Event Explorer
	// ═══════════════════════════════════════════════════════════════════════

	async getEventTimeSeries(
		eventName?: string,
		granularity: string = 'day',
		period: string = '30d'
	): Promise<{ time_series: TimeSeriesData[] }> {
		return this.request('GET', '/admin/analytics/events/timeseries', undefined, {
			event_name: eventName,
			granularity,
			period
		});
	}

	async getEventBreakdown(
		dimension: 'channel' | 'device_type' | 'country_code' | 'browser' | 'event_name',
		period: string = '30d'
	): Promise<{ breakdown: Array<{ [key: string]: any; count: number }> }> {
		return this.request('GET', '/admin/analytics/events/breakdown', undefined, { dimension, period });
	}

	async getRealTimeMetrics(): Promise<{
		metrics: RealTimeMetrics;
		counters: any;
		timestamp: string;
	}> {
		return this.request('GET', '/admin/analytics/realtime');
	}

	// ═══════════════════════════════════════════════════════════════════════
	// Events API
	// ═══════════════════════════════════════════════════════════════════════

	async getEvents(params: {
		period?: string;
		event_type?: string;
		search?: string;
		page?: number;
		per_page?: number;
	}): Promise<{
		events: AnalyticsEvent[];
		event_types: Array<{ name: string; count: number }>;
		pagination: {
			page: number;
			per_page: number;
			total: number;
			total_pages: number;
		};
	}> {
		return this.request('GET', '/admin/analytics/events', undefined, params);
	}

	// ═══════════════════════════════════════════════════════════════════════
	// Funnel Management
	// ═══════════════════════════════════════════════════════════════════════

	async getFunnels(period?: string): Promise<{
		funnels: Array<{
			key: string;
			name: string;
			description?: string;
			steps: Array<{
				name: string;
				count: number;
				conversion_rate: number;
				drop_off_rate: number;
			}>;
			overall_conversion: number;
			avg_completion_time?: number;
		}>;
	}> {
		return this.request('GET', '/admin/analytics/funnels', undefined, { period });
	}

	async createFunnel(data: {
		name: string;
		description?: string;
		steps: Array<{
			step_number: number;
			name: string;
			event_name: string;
		}>;
	}): Promise<{ success: boolean; funnel_key: string }> {
		return this.request('POST', '/admin/analytics/funnels', data);
	}

	// ═══════════════════════════════════════════════════════════════════════
	// Cohort Management
	// ═══════════════════════════════════════════════════════════════════════

	async getCohorts(params?: {
		period?: string;
		granularity?: string;
	}): Promise<{
		cohorts: Array<{
			key: string;
			name: string;
			description?: string;
			type: string;
			granularity: string;
			retention_matrix: Array<{
				cohort: string;
				size: number;
				periods: number[];
			}>;
			summary?: {
				avg_retention: number;
				best_cohort: string;
				worst_cohort: string;
			};
		}>;
	}> {
		return this.request('GET', '/admin/analytics/cohorts', undefined, params);
	}

	async createCohort(data: {
		name: string;
		description?: string;
		type: 'signup' | 'first_purchase' | 'custom';
		granularity: 'daily' | 'weekly' | 'monthly';
		start_event?: string;
		return_event?: string;
	}): Promise<{ success: boolean; cohort_key: string }> {
		return this.request('POST', '/admin/analytics/cohorts', data);
	}

	// ═══════════════════════════════════════════════════════════════════════
	// Segment Management
	// ═══════════════════════════════════════════════════════════════════════

	async createSegment(data: {
		name: string;
		description?: string;
		type: 'static' | 'dynamic' | 'computed';
		rules: {
			conditions: Array<{
				field: string;
				operator: string;
				value: string;
			}>;
		};
	}): Promise<{ success: boolean; segment_key: string }> {
		return this.request('POST', '/admin/analytics/segments', data);
	}

	async updateSegment(
		segmentKey: string,
		data: {
			name?: string;
			description?: string;
			rules?: any;
		}
	): Promise<{ success: boolean }> {
		return this.request('PUT', `/admin/analytics/segments/${segmentKey}`, data);
	}

	async deleteSegment(segmentKey: string): Promise<{ success: boolean }> {
		return this.request('DELETE', `/admin/analytics/segments/${segmentKey}`);
	}

	// ═══════════════════════════════════════════════════════════════════════
	// Attribution Enhanced
	// ═══════════════════════════════════════════════════════════════════════

	async getChannelAttribution(
		model: string = 'linear',
		period: string = '30d'
	): Promise<AttributionReport & { conversion_paths?: ConversionPathData[] }> {
		return this.request('GET', '/admin/analytics/attribution/channels', undefined, { model, period });
	}

	// ═══════════════════════════════════════════════════════════════════════
	// Reports
	// ═══════════════════════════════════════════════════════════════════════

	async getReports(): Promise<{ reports: Report[] }> {
		return this.request('GET', '/admin/analytics/reports');
	}

	async getReport(reportId: string): Promise<{ report: Report; data: any }> {
		return this.request('GET', `/admin/analytics/reports/${reportId}`);
	}

	async createReport(data: {
		name: string;
		description?: string;
		type: 'dashboard' | 'table' | 'chart' | 'mixed';
		config: {
			metrics: string[];
			dimensions: string[];
		};
		schedule?: {
			frequency: 'daily' | 'weekly' | 'monthly';
			recipients: string[];
		};
	}): Promise<{ success: boolean; report_id: string }> {
		return this.request('POST', '/admin/analytics/reports', data);
	}

	async updateReport(
		reportId: string,
		data: {
			name?: string;
			description?: string;
			config?: any;
			schedule?: any;
			status?: string;
		}
	): Promise<{ success: boolean }> {
		return this.request('PUT', `/admin/analytics/reports/${reportId}`, data);
	}

	async deleteReport(reportId: string): Promise<{ success: boolean }> {
		return this.request('DELETE', `/admin/analytics/reports/${reportId}`);
	}

	async runReport(reportId: string): Promise<{ success: boolean; data: any }> {
		return this.request('POST', `/admin/analytics/reports/${reportId}/run`);
	}
}

// Export singleton instance
export const analyticsApi = new AnalyticsApiClient();
export default analyticsApi;
