import { writable, derived } from 'svelte/store';
import type { Dashboard, DashboardWidget } from '$lib/types/dashboard';
import { dashboardApi } from '$lib/api/dashboard';

interface DashboardState {
	currentDashboard: Dashboard | null;
	widgets: DashboardWidget[];
	isLoading: boolean;
	error: string | null;
	lastRefresh: Date | null;
}

function createDashboardStore() {
	const { subscribe, set, update } = writable<DashboardState>({
		currentDashboard: null,
		widgets: [],
		isLoading: false,
		error: null,
		lastRefresh: null
	});

	return {
		subscribe,

		async loadDashboard(type: 'admin' | 'user' = 'user') {
			update((state) => ({ ...state, isLoading: true, error: null }));

			try {
				const data = await dashboardApi.getDashboards(type);

				update((state) => ({
					...state,
					currentDashboard: data.dashboard,
					widgets: data.widgets,
					isLoading: false,
					lastRefresh: new Date()
				}));
			} catch (error: any) {
				update((state) => ({
					...state,
					error: error.message || 'Failed to load dashboard',
					isLoading: false
				}));
			}
		},

		async updateWidgetLayout(
			widgetId: string,
			layout: { x: number; y: number; width: number; height: number }
		) {
			try {
				const updatedWidget = await dashboardApi.updateWidgetLayout(widgetId, layout);

				update((state) => ({
					...state,
					widgets: state.widgets.map((w) => (w.id === widgetId ? updatedWidget : w))
				}));
			} catch (error: any) {
				update((state) => ({
					...state,
					error: error.message || 'Failed to update widget layout'
				}));
			}
		},

		async addWidget(widget: Partial<DashboardWidget>) {
			update((state) => ({ ...state, isLoading: true }));

			try {
				const currentDashboard = await new Promise<Dashboard>((resolve, _reject) => {
					const unsubscribe = subscribe((state) => {
						if (state.currentDashboard) {
							unsubscribe();
							resolve(state.currentDashboard);
						}
					});
				});

				const newWidget = await dashboardApi.addWidget(currentDashboard.id, widget);

				update((state) => ({
					...state,
					widgets: [...state.widgets, newWidget],
					isLoading: false
				}));
			} catch (error: any) {
				update((state) => ({
					...state,
					error: error.message || 'Failed to add widget',
					isLoading: false
				}));
			}
		},

		async removeWidget(widgetId: string) {
			try {
				await dashboardApi.removeWidget(widgetId);

				update((state) => ({
					...state,
					widgets: state.widgets.filter((w) => w.id !== widgetId)
				}));
			} catch (error: any) {
				update((state) => ({
					...state,
					error: error.message || 'Failed to remove widget'
				}));
			}
		},

		async refreshWidget(widgetId: string) {
			update((state) => ({ ...state, isLoading: true }));

			try {
				const currentDashboard = await new Promise<Dashboard>((resolve) => {
					const unsubscribe = subscribe((state) => {
						if (state.currentDashboard) {
							unsubscribe();
							resolve(state.currentDashboard);
						}
					});
				});

				const data = await dashboardApi.getDashboard(currentDashboard.id);
				const refreshedWidget = data.widgets.find((w) => w.id === widgetId);

				if (refreshedWidget) {
					update((state) => ({
						...state,
						widgets: state.widgets.map((w) => (w.id === widgetId ? refreshedWidget : w)),
						isLoading: false
					}));
				}
			} catch (error: any) {
				update((state) => ({
					...state,
					error: error.message || 'Failed to refresh widget',
					isLoading: false
				}));
			}
		},

		clearError() {
			update((state) => ({ ...state, error: null }));
		},

		reset() {
			set({
				currentDashboard: null,
				widgets: [],
				isLoading: false,
				error: null,
				lastRefresh: null
			});
		}
	};
}

export const dashboardStore = createDashboardStore();

export const visibleWidgets = derived(dashboardStore, ($dashboard) =>
	$dashboard.widgets.filter((w) => w.is_visible)
);

export const widgetsByType = derived(dashboardStore, ($dashboard) => {
	const grouped: Record<string, DashboardWidget[]> = {};
	$dashboard.widgets.forEach((widget) => {
		if (!grouped[widget.widget_type]) {
			grouped[widget.widget_type] = [];
		}
		grouped[widget.widget_type]!.push(widget);
	});
	return grouped;
});
