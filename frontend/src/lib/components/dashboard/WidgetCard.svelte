<script lang="ts">
	import type { DashboardWidget } from '$lib/types/dashboard';
	import { dashboardStore } from '$lib/stores/dashboard';

	// Widget components
	import SystemHealthWidget from './widgets/SystemHealthWidget.svelte';
	import RevenueMRRWidget from './widgets/RevenueMRRWidget.svelte';
	import UserGrowthWidget from './widgets/UserGrowthWidget.svelte';
	import RecentActivityWidget from './widgets/RecentActivityWidget.svelte';
	import GenericWidget from './widgets/GenericWidget.svelte';

	interface Props {
		widget: DashboardWidget;
		ondragstart?: () => void;
		ondragend?: () => void;
		onlayoutchange?: (e: { detail: any }) => void;
	}

	let { widget, ondragstart, ondragend, onlayoutchange }: Props = $props();

	let isRefreshing = $state(false);

	async function handleRefresh() {
		isRefreshing = true;
		await dashboardStore.refreshWidget(widget.id);
		isRefreshing = false;
	}

	async function handleRemove() {
		if (confirm(`Remove widget "${widget.title}"?`)) {
			await dashboardStore.removeWidget(widget.id);
		}
	}

	function getWidgetComponent(type: string) {
		switch (type) {
			case 'system_health':
				return SystemHealthWidget;
			case 'revenue_mrr':
				return RevenueMRRWidget;
			case 'user_growth':
				return UserGrowthWidget;
			case 'recent_activity':
				return RecentActivityWidget;
			default:
				return GenericWidget;
		}
	}

	let WidgetComponent = $derived(getWidgetComponent(widget.widget_type));
	let gridColumn = $derived(`${widget.position_x + 1} / span ${widget.width}`);
	let gridRow = $derived(`${widget.position_y + 1} / span ${widget.height}`);
</script>

<div
	class="widget-card"
	style="
    grid-column: {gridColumn};
    grid-row: {gridRow};
  "
	draggable="true"
	role="button"
	tabindex="0"
	aria-label="Widget card"
	aria-roledescription="Widget card container"
	ondragstart={ondragstart}
	ondragend={ondragend}
>
	<div class="widget-header">
		<h3 class="widget-title">{widget.title}</h3>
		<div class="widget-actions">
			<button class="action-btn" onclick={handleRefresh} disabled={isRefreshing} title="Refresh">
				<svg
					class:spin={isRefreshing}
					width="16"
					height="16"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<path
						d="M21.5 2v6h-6M2.5 22v-6h6M2 11.5a10 10 0 0 1 18.8-4.3M22 12.5a10 10 0 0 1-18.8 4.2"
					/>
				</svg>
			</button>
			<button class="action-btn" onclick={handleRemove} title="Remove">
				<svg
					width="16"
					height="16"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<path d="M18 6L6 18M6 6l12 12" />
				</svg>
			</button>
		</div>
	</div>

	<div class="widget-content">
		<WidgetComponent data={widget.data} config={widget.config} />
	</div>
</div>

<style>
	.widget-card {
		background: white;
		border-radius: 12px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
		display: flex;
		flex-direction: column;
		overflow: hidden;
		transition: box-shadow 0.2s;
	}

	.widget-card:hover {
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
	}

	.widget-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem 1.25rem;
		border-bottom: 1px solid #e5e7eb;
		background: #fafafa;
	}

	.widget-title {
		font-size: 0.95rem;
		font-weight: 600;
		color: #1f2937;
		margin: 0;
	}

	.widget-actions {
		display: flex;
		gap: 0.5rem;
	}

	.action-btn {
		background: none;
		border: none;
		padding: 0.25rem;
		cursor: pointer;
		color: #6b7280;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 4px;
		transition: all 0.2s;
	}

	.action-btn:hover {
		background: #e5e7eb;
		color: #1f2937;
	}

	.action-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.widget-content {
		flex: 1;
		padding: 1.25rem;
		overflow: auto;
	}

	.spin {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>
