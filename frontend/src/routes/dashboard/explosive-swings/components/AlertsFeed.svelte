<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * AlertsFeed Component - Live Alerts Feed with Filter Tabs
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Container for live alerts with filtering and pagination
	 * @version 4.0.0 - January 2026 - Nuclear Build Specification
	 * @standards Apple Principal Engineer ICT 7+ Standards
	 */
	import type { Alert, AlertFilterType } from '../types';
	import AlertCard from './AlertCard.svelte';

	interface Props {
		alerts: Alert[];
		isLoading?: boolean;
		isAdmin?: boolean;
		onViewTradePlan?: (alert: Alert) => void;
		onEdit?: (alert: Alert) => void;
		onDelete?: (alertId: string) => void;
		onNewAlert?: () => void;
	}

	const {
		alerts,
		isLoading = false,
		isAdmin = false,
		onViewTradePlan,
		onEdit,
		onDelete,
		onNewAlert
	}: Props = $props();

	let activeFilter = $state<AlertFilterType>('all');

	const filterOptions: { value: AlertFilterType; label: string }[] = [
		{ value: 'all', label: 'All' },
		{ value: 'entries', label: 'Entries' },
		{ value: 'updates', label: 'Updates' },
		{ value: 'exits', label: 'Exits' }
	];

	const filteredAlerts = $derived(() => {
		if (activeFilter === 'all') return alerts;
		const typeMap: Record<AlertFilterType, string> = {
			all: '',
			entries: 'ENTRY',
			updates: 'UPDATE',
			exits: 'EXIT'
		};
		return alerts.filter((a) => a.type === typeMap[activeFilter]);
	});

	const counts = $derived({
		all: alerts.length,
		entries: alerts.filter((a) => a.type === 'ENTRY').length,
		updates: alerts.filter((a) => a.type === 'UPDATE').length,
		exits: alerts.filter((a) => a.type === 'EXIT').length
	});

	function setFilter(filter: AlertFilterType) {
		activeFilter = filter;
	}
</script>

<section class="alerts-feed" aria-labelledby="alerts-heading">
	<!-- Header Row -->
	<div class="feed-header">
		<h2 id="alerts-heading" class="feed-title">Live Alerts</h2>
		{#if isAdmin && onNewAlert}
			<button class="new-alert-btn" onclick={onNewAlert}>
				<svg viewBox="0 0 20 20" fill="currentColor" class="w-4 h-4">
					<path d="M10.75 4.75a.75.75 0 00-1.5 0v4.5h-4.5a.75.75 0 000 1.5h4.5v4.5a.75.75 0 001.5 0v-4.5h4.5a.75.75 0 000-1.5h-4.5v-4.5z" />
				</svg>
				New Alert
			</button>
		{/if}
	</div>

	<!-- Filter Tabs -->
	<div class="filter-tabs" role="tablist" aria-label="Filter alerts by type">
		{#each filterOptions as option}
			<button
				class="filter-tab"
				class:active={activeFilter === option.value}
				role="tab"
				aria-selected={activeFilter === option.value}
				onclick={() => setFilter(option.value)}
			>
				{option.label}
				{#if counts[option.value] > 0}
					<span class="filter-count">({counts[option.value]})</span>
				{/if}
			</button>
		{/each}
	</div>

	<!-- Alerts List -->
	<div class="alerts-list" role="feed" aria-busy={isLoading}>
		{#if isLoading}
			{#each Array(3) as _}
				<div class="alert-skeleton"></div>
			{/each}
		{:else if filteredAlerts().length === 0}
			<div class="empty-state">
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" class="empty-icon">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M14.857 17.082a23.848 23.848 0 005.454-1.31A8.967 8.967 0 0118 9.75v-.7V9A6 6 0 006 9v.75a8.967 8.967 0 01-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 01-5.714 0m5.714 0a3 3 0 11-5.714 0" />
				</svg>
				<p>No {activeFilter === 'all' ? '' : activeFilter.slice(0, -1)} alerts to display</p>
			</div>
		{:else}
			{#each filteredAlerts() as alert (alert.id)}
				<AlertCard
					{alert}
					{isAdmin}
					{onViewTradePlan}
					{onEdit}
					{onDelete}
				/>
			{/each}
		{/if}
	</div>
</section>

<style>
	.alerts-feed {
		contain: layout style;
	}

	.feed-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 16px;
	}

	.feed-title {
		font-size: 18px;
		font-weight: 600;
		color: #0f172a;
		margin: 0;
	}

	.new-alert-btn {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 8px 16px;
		font-size: 13px;
		font-weight: 600;
		background: #143e59;
		color: #fff;
		border: none;
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.2s;
	}

	.new-alert-btn:hover {
		background: #1e5175;
	}

	.filter-tabs {
		display: flex;
		gap: 4px;
		margin-bottom: 20px;
		border-bottom: 1px solid #e2e8f0;
		padding-bottom: 0;
	}

	.filter-tab {
		display: flex;
		align-items: center;
		gap: 4px;
		padding: 10px 16px;
		font-size: 14px;
		font-weight: 500;
		color: #64748b;
		background: transparent;
		border: none;
		border-bottom: 2px solid transparent;
		cursor: pointer;
		transition: all 0.2s;
		margin-bottom: -1px;
	}

	.filter-tab:hover {
		color: #0f172a;
	}

	.filter-tab.active {
		color: #0f172a;
		font-weight: 600;
		border-bottom-color: #143e59;
	}

	.filter-count {
		font-size: 12px;
		color: #94a3b8;
	}

	.filter-tab.active .filter-count {
		color: #64748b;
	}

	.alerts-list {
		display: flex;
		flex-direction: column;
		contain: layout;
	}

	.alert-skeleton {
		height: 180px;
		background: linear-gradient(90deg, #f1f5f9 25%, #e2e8f0 50%, #f1f5f9 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 12px;
		margin-bottom: 16px;
	}

	@keyframes shimmer {
		0% {
			background-position: 200% 0;
		}
		100% {
			background-position: -200% 0;
		}
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 48px 24px;
		text-align: center;
		color: #64748b;
	}

	.empty-icon {
		width: 48px;
		height: 48px;
		margin-bottom: 16px;
		opacity: 0.5;
	}

	.empty-state p {
		font-size: 14px;
		margin: 0;
	}

	@media (max-width: 640px) {
		.filter-tabs {
			overflow-x: auto;
			-webkit-overflow-scrolling: touch;
		}

		.filter-tab {
			padding: 8px 12px;
			font-size: 13px;
			white-space: nowrap;
		}
	}
</style>
