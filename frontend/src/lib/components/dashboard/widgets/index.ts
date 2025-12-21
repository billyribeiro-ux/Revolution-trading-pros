/**
 * Dashboard Widgets Index
 *
 * Enterprise-grade dashboard widget components
 * Inspired by Apple.com design language
 *
 * @version 1.0.0
 */

// Core Widget Components
export { default as AppleWidgetCard } from './AppleWidgetCard.svelte';

// SEO & Analytics Widgets
export { default as SeoScoreWidget } from './SeoScoreWidget.svelte';
export { default as KeywordRankingsWidget } from './KeywordRankingsWidget.svelte';
export { default as VisitorAnalyticsWidget } from './VisitorAnalyticsWidget.svelte';
export { default as TrafficSourcesWidget } from './TrafficSourcesWidget.svelte';
export { default as CoreWebVitalsWidget } from './CoreWebVitalsWidget.svelte';
export { default as BacklinkOverviewWidget } from './BacklinkOverviewWidget.svelte';

// Integration Widgets
export { default as GoogleIntegrationWidget } from './GoogleIntegrationWidget.svelte';

// Widget type definitions
export type WidgetType =
	| 'seo_score'
	| 'keyword_rankings'
	| 'visitor_analytics'
	| 'traffic_sources'
	| 'core_web_vitals'
	| 'backlink_overview'
	| 'google_integrations'
	| 'system_health'
	| 'revenue_mrr'
	| 'user_growth'
	| 'subscription_churn'
	| 'recent_activity'
	| 'email_performance'
	| 'crm_pipeline'
	| 'subscription_status'
	| 'recent_courses'
	| 'trading_performance'
	| 'notifications';

export interface WidgetConfig {
	id: string;
	type: WidgetType;
	title: string;
	subtitle?: string;
	position: { x: number; y: number };
	size: { width: number; height: number };
	refreshInterval?: number;
	config?: Record<string, any>;
}

// Widget registry for dynamic rendering
export const widgetRegistry: Record<
	WidgetType,
	{
		component: any;
		defaultSize: { width: number; height: number };
		minSize: { width: number; height: number };
		maxSize: { width: number; height: number };
		icon: string;
		category: string;
	}
> = {
	seo_score: {
		component: () => import('./SeoScoreWidget.svelte'),
		defaultSize: { width: 6, height: 5 },
		minSize: { width: 4, height: 4 },
		maxSize: { width: 12, height: 8 },
		icon: 'chart',
		category: 'SEO'
	},
	keyword_rankings: {
		component: () => import('./KeywordRankingsWidget.svelte'),
		defaultSize: { width: 6, height: 6 },
		minSize: { width: 4, height: 4 },
		maxSize: { width: 12, height: 10 },
		icon: 'search',
		category: 'SEO'
	},
	visitor_analytics: {
		component: () => import('./VisitorAnalyticsWidget.svelte'),
		defaultSize: { width: 6, height: 5 },
		minSize: { width: 4, height: 4 },
		maxSize: { width: 12, height: 8 },
		icon: 'users',
		category: 'Analytics'
	},
	traffic_sources: {
		component: () => import('./TrafficSourcesWidget.svelte'),
		defaultSize: { width: 6, height: 6 },
		minSize: { width: 4, height: 5 },
		maxSize: { width: 8, height: 10 },
		icon: 'globe',
		category: 'Analytics'
	},
	core_web_vitals: {
		component: () => import('./CoreWebVitalsWidget.svelte'),
		defaultSize: { width: 8, height: 6 },
		minSize: { width: 6, height: 5 },
		maxSize: { width: 12, height: 8 },
		icon: 'analytics',
		category: 'Performance'
	},
	backlink_overview: {
		component: () => import('./BacklinkOverviewWidget.svelte'),
		defaultSize: { width: 6, height: 7 },
		minSize: { width: 4, height: 5 },
		maxSize: { width: 12, height: 10 },
		icon: 'chart',
		category: 'SEO'
	},
	google_integrations: {
		component: () => import('./GoogleIntegrationWidget.svelte'),
		defaultSize: { width: 6, height: 6 },
		minSize: { width: 4, height: 5 },
		maxSize: { width: 8, height: 8 },
		icon: 'globe',
		category: 'Integrations'
	},
	// Placeholder entries for other widget types
	system_health: {
		component: () => Promise.resolve({ default: null }),
		defaultSize: { width: 6, height: 4 },
		minSize: { width: 4, height: 3 },
		maxSize: { width: 12, height: 6 },
		icon: 'analytics',
		category: 'System'
	},
	revenue_mrr: {
		component: () => Promise.resolve({ default: null }),
		defaultSize: { width: 6, height: 4 },
		minSize: { width: 4, height: 3 },
		maxSize: { width: 12, height: 6 },
		icon: 'revenue',
		category: 'Finance'
	},
	user_growth: {
		component: () => Promise.resolve({ default: null }),
		defaultSize: { width: 4, height: 4 },
		minSize: { width: 3, height: 3 },
		maxSize: { width: 8, height: 6 },
		icon: 'users',
		category: 'Users'
	},
	subscription_churn: {
		component: () => Promise.resolve({ default: null }),
		defaultSize: { width: 4, height: 4 },
		minSize: { width: 3, height: 3 },
		maxSize: { width: 8, height: 6 },
		icon: 'chart',
		category: 'Finance'
	},
	recent_activity: {
		component: () => Promise.resolve({ default: null }),
		defaultSize: { width: 4, height: 8 },
		minSize: { width: 3, height: 4 },
		maxSize: { width: 6, height: 12 },
		icon: 'chart',
		category: 'Activity'
	},
	email_performance: {
		component: () => Promise.resolve({ default: null }),
		defaultSize: { width: 4, height: 4 },
		minSize: { width: 3, height: 3 },
		maxSize: { width: 8, height: 6 },
		icon: 'chart',
		category: 'Marketing'
	},
	crm_pipeline: {
		component: () => Promise.resolve({ default: null }),
		defaultSize: { width: 4, height: 4 },
		minSize: { width: 3, height: 3 },
		maxSize: { width: 8, height: 6 },
		icon: 'users',
		category: 'CRM'
	},
	subscription_status: {
		component: () => Promise.resolve({ default: null }),
		defaultSize: { width: 6, height: 4 },
		minSize: { width: 4, height: 3 },
		maxSize: { width: 8, height: 6 },
		icon: 'revenue',
		category: 'Finance'
	},
	recent_courses: {
		component: () => Promise.resolve({ default: null }),
		defaultSize: { width: 6, height: 4 },
		minSize: { width: 4, height: 3 },
		maxSize: { width: 12, height: 6 },
		icon: 'chart',
		category: 'Learning'
	},
	trading_performance: {
		component: () => Promise.resolve({ default: null }),
		defaultSize: { width: 8, height: 6 },
		minSize: { width: 6, height: 4 },
		maxSize: { width: 12, height: 8 },
		icon: 'chart',
		category: 'Trading'
	},
	notifications: {
		component: () => Promise.resolve({ default: null }),
		defaultSize: { width: 4, height: 6 },
		minSize: { width: 3, height: 4 },
		maxSize: { width: 6, height: 10 },
		icon: 'chart',
		category: 'System'
	}
};
