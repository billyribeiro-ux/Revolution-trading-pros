/**
 * Dashboard Widgets Store - Apple ICT9+ Design
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Manages dashboard widget configuration with:
 * - Widget visibility and ordering
 * - Persistent storage
 * - Drag and drop reordering
 * - Size configuration
 *
 * @version 1.0.0
 */

import { writable, derived, get } from 'svelte/store';
import { browser } from '$app/environment';

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
			const newWidgets = defaultWidgets.filter(w => !storedIds.has(w.id));

			return {
				...defaultConfig,
				...parsed,
				widgets: [...parsed.widgets, ...newWidgets]
			};
		}
	} catch (e) {
		console.error('Failed to load widget config:', e);
	}

	return defaultConfig;
}

// Save to localStorage
function saveConfig(config: WidgetConfig) {
	if (!browser) return;

	try {
		localStorage.setItem(STORAGE_KEY, JSON.stringify(config));
	} catch (e) {
		console.error('Failed to save widget config:', e);
	}
}

function createWidgetStore() {
	const { subscribe, set, update } = writable<WidgetConfig>(loadConfig());

	// Auto-save on changes
	subscribe(config => {
		saveConfig(config);
	});

	return {
		subscribe,

		// Toggle widget visibility
		toggleWidget(id: string) {
			update(config => ({
				...config,
				widgets: config.widgets.map(w =>
					w.id === id ? { ...w, visible: !w.visible } : w
				)
			}));
		},

		// Update widget size
		setWidgetSize(id: string, size: WidgetSize) {
			update(config => ({
				...config,
				widgets: config.widgets.map(w =>
					w.id === id ? { ...w, size } : w
				)
			}));
		},

		// Reorder widgets
		reorderWidgets(fromIndex: number, toIndex: number) {
			update(config => {
				const widgets = [...config.widgets];
				const [removed] = widgets.splice(fromIndex, 1);
				widgets.splice(toIndex, 0, removed);

				// Update order values
				return {
					...config,
					widgets: widgets.map((w, i) => ({ ...w, order: i }))
				};
			});
		},

		// Move widget by id
		moveWidget(id: string, direction: 'up' | 'down') {
			update(config => {
				const widgets = [...config.widgets].sort((a, b) => a.order - b.order);
				const currentIndex = widgets.findIndex(w => w.id === id);

				if (currentIndex === -1) return config;

				const targetIndex = direction === 'up'
					? Math.max(0, currentIndex - 1)
					: Math.min(widgets.length - 1, currentIndex + 1);

				if (currentIndex === targetIndex) return config;

				const [removed] = widgets.splice(currentIndex, 1);
				widgets.splice(targetIndex, 0, removed);

				return {
					...config,
					widgets: widgets.map((w, i) => ({ ...w, order: i }))
				};
			});
		},

		// Set layout mode
		setLayout(layout: 'grid' | 'list') {
			update(config => ({ ...config, layout }));
		},

		// Toggle auto-refresh
		toggleAutoRefresh() {
			update(config => ({ ...config, autoRefresh: !config.autoRefresh }));
		},

		// Set refresh interval
		setRefreshInterval(interval: number) {
			update(config => ({ ...config, refreshInterval: interval }));
		},

		// Mark widget as refreshed
		markRefreshed(id: string) {
			update(config => ({
				...config,
				widgets: config.widgets.map(w =>
					w.id === id ? { ...w, lastRefreshed: Date.now() } : w
				)
			}));
		},

		// Reset to defaults
		resetToDefaults() {
			set(defaultConfig);
		},

		// Get visible widgets sorted by order
		getVisibleWidgets(): DashboardWidget[] {
			const config = get({ subscribe });
			return config.widgets
				.filter(w => w.visible)
				.sort((a, b) => a.order - b.order);
		}
	};
}

export const widgetStore = createWidgetStore();

// Derived stores
export const visibleWidgets = derived(widgetStore, $config =>
	$config.widgets.filter(w => w.visible).sort((a, b) => a.order - b.order)
);

export const hiddenWidgets = derived(widgetStore, $config =>
	$config.widgets.filter(w => !w.visible).sort((a, b) => a.order - b.order)
);

export const widgetsByCategory = derived(widgetStore, $config => {
	const categories: Record<string, DashboardWidget[]> = {
		analytics: [],
		content: [],
		commerce: [],
		system: []
	};

	for (const widget of $config.widgets) {
		categories[widget.category].push(widget);
	}

	return categories;
});

export const widgetLayout = derived(widgetStore, $config => $config.layout);
export const autoRefreshEnabled = derived(widgetStore, $config => $config.autoRefresh);
