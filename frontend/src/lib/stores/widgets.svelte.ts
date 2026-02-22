/**
 * Dashboard Widgets Store - Apple ICT9+ Design (Svelte 5 Runes)
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Manages dashboard widget configuration with:
 * - Widget visibility and ordering
 * - Persistent storage
 * - Drag and drop reordering
 * - Size configuration
 *
 * @version 2.0.0 - Svelte 5 Runes Migration
 */

import { browser } from '$app/environment';
import { logger } from '$lib/utils/logger';

export type WidgetSize = 'small' | 'medium' | 'large' | 'full';

export interface DashboardWidget {
	id: string;
	title: string;
	description: string;
	icon: string;
	size: WidgetSize;
	visible: boolean;
	order: number;
	category: 'analytics' | 'content' | 'commerce' | 'system';
	component?: string;
	refreshInterval?: number; // in seconds
	lastRefreshed?: number;
}

interface WidgetConfig {
	widgets: DashboardWidget[];
	layout: 'grid' | 'list';
	autoRefresh: boolean;
	refreshInterval: number;
}

const STORAGE_KEY = 'rtp_dashboard_widgets';

// Default widget configuration
const defaultWidgets: DashboardWidget[] = [
	{
		id: 'analytics-overview',
		title: 'Analytics Overview',
		description: 'Page views, visitors, and engagement metrics',
		icon: 'chart-line',
		size: 'large',
		visible: true,
		order: 0,
		category: 'analytics',
		refreshInterval: 300
	},
	{
		id: 'traffic-sources',
		title: 'Traffic Sources',
		description: 'Where your visitors come from',
		icon: 'world',
		size: 'medium',
		visible: true,
		order: 1,
		category: 'analytics',
		refreshInterval: 300
	},
	{
		id: 'recent-posts',
		title: 'Recent Posts',
		description: 'Latest blog posts and articles',
		icon: 'article',
		size: 'medium',
		visible: true,
		order: 2,
		category: 'content',
		refreshInterval: 600
	},
	{
		id: 'member-activity',
		title: 'Member Activity',
		description: 'New signups and active members',
		icon: 'users',
		size: 'medium',
		visible: true,
		order: 3,
		category: 'content',
		refreshInterval: 300
	},
	{
		id: 'revenue-chart',
		title: 'Revenue',
		description: 'Sales and subscription revenue',
		icon: 'currency-dollar',
		size: 'large',
		visible: true,
		order: 4,
		category: 'commerce',
		refreshInterval: 600
	},
	{
		id: 'recent-orders',
		title: 'Recent Orders',
		description: 'Latest product orders',
		icon: 'shopping-cart',
		size: 'medium',
		visible: true,
		order: 5,
		category: 'commerce',
		refreshInterval: 120
	},
	{
		id: 'api-status',
		title: 'API Status',
		description: 'Connected services health',
		icon: 'plug',
		size: 'small',
		visible: true,
		order: 6,
		category: 'system',
		refreshInterval: 60
	},
	{
		id: 'site-health',
		title: 'Site Health',
		description: 'Performance and security status',
		icon: 'heart-rate-monitor',
		size: 'small',
		visible: true,
		order: 7,
		category: 'system',
		refreshInterval: 300
	},
	{
		id: 'quick-actions',
		title: 'Quick Actions',
		description: 'Common admin tasks',
		icon: 'bolt',
		size: 'small',
		visible: true,
		order: 8,
		category: 'system'
	},
	{
		id: 'email-stats',
		title: 'Email Stats',
		description: 'Campaign performance metrics',
		icon: 'mail',
		size: 'medium',
		visible: false,
		order: 9,
		category: 'analytics',
		refreshInterval: 600
	},
	{
		id: 'seo-snapshot',
		title: 'SEO Snapshot',
		description: 'Search engine visibility',
		icon: 'search',
		size: 'medium',
		visible: false,
		order: 10,
		category: 'analytics',
		refreshInterval: 3600
	},
	{
		id: 'behavior-insights',
		title: 'Behavior Insights',
		description: 'User behavior patterns',
		icon: 'eye',
		size: 'medium',
		visible: false,
		order: 11,
		category: 'analytics',
		refreshInterval: 300
	}
];

const defaultConfig: WidgetConfig = {
	widgets: defaultWidgets,
	layout: 'grid',
	autoRefresh: true,
	refreshInterval: 300
};

// Load from localStorage
function loadConfig(): WidgetConfig {
	if (!browser) return defaultConfig;

	try {
		const stored = localStorage.getItem(STORAGE_KEY);
		if (stored) {
			const parsed = JSON.parse(stored);
			// Merge with defaults to handle new widgets
			const storedIds = new Set(parsed.widgets.map((w: DashboardWidget) => w.id));
			const newWidgets = defaultWidgets.filter((w) => !storedIds.has(w.id));

			return {
				...defaultConfig,
				...parsed,
				widgets: [...parsed.widgets, ...newWidgets]
			};
		}
	} catch (e) {
		logger.error('Failed to load widget config:', e);
	}

	return defaultConfig;
}

// Save to localStorage
function saveConfig(config: WidgetConfig) {
	if (!browser) return;

	try {
		localStorage.setItem(STORAGE_KEY, JSON.stringify(config));
	} catch (e) {
		logger.error('Failed to save widget config:', e);
	}
}

// ═══════════════════════════════════════════════════════════════════════════════
// State (Svelte 5 Runes)
// ═══════════════════════════════════════════════════════════════════════════════

let widgetConfig = $state<WidgetConfig>(loadConfig());

// Auto-save on changes (skip initial run to prevent hydration issues)
if (browser) {
	$effect.root(() => {
		let isFirstRun = true;
		$effect(() => {
			// Read the state to create dependency
			const config = widgetConfig;
			// Skip initial run to prevent infinite loop during hydration
			if (isFirstRun) {
				isFirstRun = false;
				return;
			}
			saveConfig(config);
		});
	});
}

// ═══════════════════════════════════════════════════════════════════════════════
// Widget Store API
// ═══════════════════════════════════════════════════════════════════════════════

export const widgetStore = {
	get config() {
		return widgetConfig;
	},

	get widgets() {
		return widgetConfig.widgets;
	},

	get layout() {
		return widgetConfig.layout;
	},

	get autoRefresh() {
		return widgetConfig.autoRefresh;
	},

	// Toggle widget visibility
	toggleWidget(id: string) {
		widgetConfig = {
			...widgetConfig,
			widgets: widgetConfig.widgets.map((w) => (w.id === id ? { ...w, visible: !w.visible } : w))
		};
	},

	// Update widget size
	setWidgetSize(id: string, size: WidgetSize) {
		widgetConfig = {
			...widgetConfig,
			widgets: widgetConfig.widgets.map((w) => (w.id === id ? { ...w, size } : w))
		};
	},

	// Reorder widgets
	reorderWidgets(fromIndex: number, toIndex: number) {
		const widgets = [...widgetConfig.widgets];
		const [removed] = widgets.splice(fromIndex, 1);
		if (!removed) return;
		widgets.splice(toIndex, 0, removed);

		// Update order values
		widgetConfig = {
			...widgetConfig,
			widgets: widgets.map((w, i) => ({ ...w, order: i }))
		};
	},

	// Move widget by id
	moveWidget(id: string, direction: 'up' | 'down') {
		const widgets = [...widgetConfig.widgets].sort((a, b) => a.order - b.order);
		const currentIndex = widgets.findIndex((w) => w.id === id);

		if (currentIndex === -1) return;

		const targetIndex =
			direction === 'up'
				? Math.max(0, currentIndex - 1)
				: Math.min(widgets.length - 1, currentIndex + 1);

		if (currentIndex === targetIndex) return;

		const [removed] = widgets.splice(currentIndex, 1);
		if (!removed) return;
		widgets.splice(targetIndex, 0, removed);

		widgetConfig = {
			...widgetConfig,
			widgets: widgets.map((w, i) => ({ ...w, order: i }))
		};
	},

	// Set layout mode
	setLayout(layout: 'grid' | 'list') {
		widgetConfig = { ...widgetConfig, layout };
	},

	// Toggle auto-refresh
	toggleAutoRefresh() {
		widgetConfig = { ...widgetConfig, autoRefresh: !widgetConfig.autoRefresh };
	},

	// Set refresh interval
	setRefreshInterval(interval: number) {
		widgetConfig = { ...widgetConfig, refreshInterval: interval };
	},

	// Mark widget as refreshed
	markRefreshed(id: string) {
		widgetConfig = {
			...widgetConfig,
			widgets: widgetConfig.widgets.map((w) =>
				w.id === id ? { ...w, lastRefreshed: Date.now() } : w
			)
		};
	},

	// Reset to defaults
	resetToDefaults() {
		widgetConfig = { ...defaultConfig };
	},

	// Get visible widgets sorted by order
	getVisibleWidgets(): DashboardWidget[] {
		return widgetConfig.widgets.filter((w) => w.visible).sort((a, b) => a.order - b.order);
	}
};

// ═══════════════════════════════════════════════════════════════════════════════
// Getter Functions (Svelte 5 - cannot export $derived from modules)
// ═══════════════════════════════════════════════════════════════════════════════

export function getVisibleWidgets() {
	return widgetConfig.widgets.filter((w) => w.visible).sort((a, b) => a.order - b.order);
}

export function getHiddenWidgets() {
	return widgetConfig.widgets.filter((w) => !w.visible).sort((a, b) => a.order - b.order);
}

export function getWidgetsByCategory() {
	const categories: Record<string, DashboardWidget[]> = {
		analytics: [],
		content: [],
		commerce: [],
		system: []
	};

	for (const widget of widgetConfig.widgets) {
		const categoryArray = categories[widget.category];
		if (categoryArray) {
			categoryArray.push(widget);
		}
	}

	return categories;
}

export function getWidgetLayout() {
	return widgetConfig.layout;
}
export function getAutoRefreshEnabled() {
	return widgetConfig.autoRefresh;
}
