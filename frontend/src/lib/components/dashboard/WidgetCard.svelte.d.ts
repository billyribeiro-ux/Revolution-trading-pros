import type { DashboardWidget } from '$lib/types/dashboard';
import type { Component } from 'svelte';

export interface WidgetCardProps {
	widget: DashboardWidget;
	ondragstart?: () => void;
	ondragend?: () => void;
	onlayoutchange?: (event: CustomEvent<any>) => void;
}

declare const WidgetCard: Component<WidgetCardProps>;
export default WidgetCard;
