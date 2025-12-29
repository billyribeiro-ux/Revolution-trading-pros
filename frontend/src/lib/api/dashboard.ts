import { apiClient } from './client';
import type { DashboardData, DashboardWidget, WidgetLayout } from '$lib/types/dashboard';

export const dashboardApi = {
	async getDashboards(type: 'admin' | 'user' = 'user'): Promise<DashboardData> {
		const response = (await apiClient.get(`/admin/dashboards?type=${type}`)) as { data: DashboardData };
		return response.data;
	},

	async getDashboard(id: string): Promise<DashboardData> {
		const response = (await apiClient.get(`/admin/dashboards/${id}`)) as { data: DashboardData };
		return response.data;
	},

	async updateWidgetLayout(widgetId: string, layout: WidgetLayout): Promise<DashboardWidget> {
		const response = (await apiClient.put(`/admin/widgets/${widgetId}/layout`, layout)) as {
			data: DashboardWidget;
		};
		return response.data;
	},

	async addWidget(dashboardId: string, widget: Partial<DashboardWidget>): Promise<DashboardWidget> {
		const response = (await apiClient.post(`/admin/dashboards/${dashboardId}/widgets`, widget)) as {
			data: DashboardWidget;
		};
		return response.data;
	},

	async removeWidget(widgetId: string): Promise<void> {
		await apiClient.delete(`/admin/widgets/${widgetId}`);
	}
};
