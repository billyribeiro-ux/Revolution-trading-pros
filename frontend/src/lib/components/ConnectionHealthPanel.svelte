<script lang="ts">
	/**
	 * Connection Health Panel - Apple ICT9+ Design
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * Comprehensive API connection health monitoring with:
	 * - Real-time status indicators
	 * - Response time tracking
	 * - Error rate monitoring
	 * - Reconnection capabilities
	 *
	 * @version 1.0.0
	 */

	import { fade, fly } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';
	import {
		connections,
		allConnectionStatuses,
		type ConnectionState
	} from '$lib/stores/connections';
	import {
		IconX,
		IconPlugConnected,
		IconPlugConnectedX,
		IconRefresh,
		IconLoader,
		IconCheck,
		IconAlertTriangle,
		IconAlertCircle,
		IconClock,
		IconActivity,
		IconChartLine,
		IconExternalLink
	} from '$lib/icons';

	interface Props {
		isOpen?: boolean;
		onclose?: () => void;
	}

	let { isOpen = $bindable(false), onclose }: Props = $props();

	let refreshingService = $state<string | null>(null);
	let activeTab = $state<'overview' | 'details'>('overview');

	// Mock response time data (would come from real API)
	const mockMetrics: Record<string, { responseTime: number; uptime: number; errorRate: number }> = {
		wordpress: { responseTime: 145, uptime: 99.9, errorRate: 0.1 },
		analytics: { responseTime: 220, uptime: 99.5, errorRate: 0.3 },
		seo: { responseTime: 180, uptime: 99.8, errorRate: 0.2 },
		email: { responseTime: 95, uptime: 99.95, errorRate: 0.05 },
		crm: { responseTime: 310, uptime: 98.5, errorRate: 1.2 },
		behavior: { responseTime: 125, uptime: 99.7, errorRate: 0.15 }
	};

	const serviceLabels: Record<string, string> = {
		wordpress: 'WordPress',
		analytics: 'Analytics',
		seo: 'SEO',
		email: 'Email',
		crm: 'CRM',
		behavior: 'Behavior'
	};

	const serviceDescriptions: Record<string, string> = {
		wordpress: 'Blog posts, pages, and media management',
		analytics: 'Traffic and user engagement tracking',
		seo: 'Search engine optimization settings',
		email: 'Email campaigns and templates',
		crm: 'Customer relationship management',
		behavior: 'User behavior and session tracking'
	};

	function getStatusIcon(status: ConnectionState) {
		switch (status) {
			case 'connected': return IconCheck;
			case 'connecting': return IconLoader;
			case 'error': return IconAlertCircle;
			default: return IconPlugConnectedX;
		}
	}

	function getStatusColor(status: ConnectionState) {
		switch (status) {
			case 'connected': return '#10b981';
			case 'connecting': return '#f59e0b';
			case 'error': return '#ef4444';
			default: return '#64748b';
		}
	}

	function getStatusLabel(status: ConnectionState) {
		switch (status) {
			case 'connected': return 'Connected';
			case 'connecting': return 'Connecting...';
			case 'error': return 'Error';
			default: return 'Disconnected';
		}
	}

	function getResponseTimeColor(ms: number) {
		if (ms < 200) return '#10b981';
		if (ms < 500) return '#f59e0b';
		return '#ef4444';
	}

	function getUptimeColor(uptime: number) {
		if (uptime >= 99.5) return '#10b981';
		if (uptime >= 98) return '#f59e0b';
		return '#ef4444';
	}

	async function refreshConnection(service: string) {
		refreshingService = service;
		try {
			await connections.load(true);
		} finally {
			refreshingService = null;
		}
	}

	async function refreshAll() {
		for (const service of Object.keys(serviceLabels)) {
			await refreshConnection(service);
		}
	}

	function close() {
		isOpen = false;
		onclose?.();
	}

	let connectedCount = $derived(Object.values($allConnectionStatuses).filter(s => s === 'connected').length);
	let totalCount = $derived(Object.keys($allConnectionStatuses).length);
	let overallHealth = $derived(connectedCount === totalCount ? 'healthy' : connectedCount > 0 ? 'partial' : 'unhealthy');
</script>

{#if isOpen}
	<div
		class="health-overlay"
		transition:fade={{ duration: 150 }}
		onclick={close}
		onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && close()}
		role="button"
		tabindex="0"
		aria-label="Close connection health panel"
	>
		<div
			class="health-panel"
			transition:fly={{ x: 400, duration: 300, easing: quintOut }}
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
			role="presentation"
		>
			<!-- Header -->
			<div class="panel-header">
				<div class="header-left">
					<div class="header-icon" class:healthy={overallHealth === 'healthy'} class:partial={overallHealth === 'partial'} class:unhealthy={overallHealth === 'unhealthy'}>
						<IconPlugConnected size={20} />
					</div>
					<div class="header-info">
						<h2>Connection Health</h2>
						<span class="connection-summary">
							{connectedCount}/{totalCount} services connected
						</span>
					</div>
				</div>
				<div class="header-actions">
					<button class="refresh-all-btn" onclick={refreshAll} disabled={refreshingService !== null}>
						<IconRefresh size={16} class={refreshingService ? 'spinning' : ''} />
						Refresh All
					</button>
					<button class="close-btn" onclick={close}>
						<IconX size={20} />
					</button>
				</div>
			</div>

			<!-- Overall Status Bar -->
			<div class="status-bar" class:healthy={overallHealth === 'healthy'} class:partial={overallHealth === 'partial'} class:unhealthy={overallHealth === 'unhealthy'}>
				<div class="status-indicator">
					{#if overallHealth === 'healthy'}
						<IconCheck size={18} />
						<span>All Systems Operational</span>
					{:else if overallHealth === 'partial'}
						<IconAlertTriangle size={18} />
						<span>Some Services Degraded</span>
					{:else}
						<IconAlertCircle size={18} />
						<span>Services Unavailable</span>
					{/if}
				</div>
				<span class="last-check">Last checked: Just now</span>
			</div>

			<!-- Tabs -->
			<div class="panel-tabs">
				<button
					class="tab"
					class:active={activeTab === 'overview'}
					onclick={() => activeTab = 'overview'}
				>
					<IconPlugConnected size={16} />
					Overview
				</button>
				<button
					class="tab"
					class:active={activeTab === 'details'}
					onclick={() => activeTab = 'details'}
				>
					<IconChartLine size={16} />
					Metrics
				</button>
			</div>

			<!-- Content -->
			<div class="panel-content">
				{#if activeTab === 'overview'}
					<div class="services-list" in:fade={{ duration: 150 }}>
						{#each Object.entries($allConnectionStatuses) as [service, status]}
							{@const metrics = mockMetrics[service] || { responseTime: 0, uptime: 0, errorRate: 0 }}
							{@const StatusIcon = getStatusIcon(status)}
							<div class="service-card" class:connected={status === 'connected'} class:error={status === 'error'}>
								<div class="service-status" style="color: {getStatusColor(status)}">
									<StatusIcon size={20} class={status === 'connecting' ? 'spinning' : ''} />
								</div>
								<div class="service-info">
									<div class="service-header">
										<span class="service-name">{serviceLabels[service]}</span>
										<span class="status-badge" style="background: {getStatusColor(status)}20; color: {getStatusColor(status)}">
											{getStatusLabel(status)}
										</span>
									</div>
									<p class="service-description">{serviceDescriptions[service]}</p>
									{#if status === 'connected'}
										<div class="quick-metrics">
											<span class="metric">
												<IconClock size={12} />
												{metrics.responseTime}ms
											</span>
											<span class="metric">
												<IconActivity size={12} />
												{metrics.uptime}% uptime
											</span>
										</div>
									{/if}
								</div>
								<div class="service-actions">
									<button
										class="action-btn"
										onclick={() => refreshConnection(service)}
										disabled={refreshingService === service}
										title="Refresh connection"
									>
										<IconRefresh size={16} class={refreshingService === service ? 'spinning' : ''} />
									</button>
									<a href="/admin/connections" class="action-btn" title="Configure">
										<IconExternalLink size={16} />
									</a>
								</div>
							</div>
						{/each}
					</div>
				{:else}
					<div class="metrics-view" in:fade={{ duration: 150 }}>
						{#each Object.entries($allConnectionStatuses) as [service, status]}
							{@const metrics = mockMetrics[service] || { responseTime: 0, uptime: 0, errorRate: 0 }}
							<div class="metrics-card">
								<div class="metrics-header">
									<span class="service-name">{serviceLabels[service]}</span>
									<span class="status-dot" style="background: {getStatusColor(status)}"></span>
								</div>
								<div class="metrics-grid">
									<div class="metric-item">
										<span class="metric-label">Response Time</span>
										<span class="metric-value" style="color: {getResponseTimeColor(metrics.responseTime)}">
											{metrics.responseTime}ms
										</span>
										<div class="metric-bar">
											<div
												class="metric-bar-fill"
												style="width: {Math.min(metrics.responseTime / 5, 100)}%; background: {getResponseTimeColor(metrics.responseTime)}"
											></div>
										</div>
									</div>
									<div class="metric-item">
										<span class="metric-label">Uptime</span>
										<span class="metric-value" style="color: {getUptimeColor(metrics.uptime)}">
											{metrics.uptime}%
										</span>
										<div class="metric-bar">
											<div
												class="metric-bar-fill"
												style="width: {metrics.uptime}%; background: {getUptimeColor(metrics.uptime)}"
											></div>
										</div>
									</div>
									<div class="metric-item">
										<span class="metric-label">Error Rate</span>
										<span class="metric-value" style="color: {metrics.errorRate > 1 ? '#ef4444' : metrics.errorRate > 0.5 ? '#f59e0b' : '#10b981'}">
											{metrics.errorRate}%
										</span>
										<div class="metric-bar">
											<div
												class="metric-bar-fill error"
												style="width: {Math.min(metrics.errorRate * 10, 100)}%"
											></div>
										</div>
									</div>
								</div>
							</div>
						{/each}
					</div>
				{/if}
			</div>

			<!-- Footer -->
			<div class="panel-footer">
				<a href="/admin/site-health" class="view-all-link">
					<IconActivity size={16} />
					View Full Site Health Dashboard
					<IconExternalLink size={14} />
				</a>
			</div>
		</div>
	</div>
{/if}

<style>
	.health-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.5);
		backdrop-filter: blur(4px);
		z-index: 9998;
	}

	.health-panel {
		position: fixed;
		top: 0;
		right: 0;
		bottom: 0;
		width: 100%;
		max-width: 480px;
		background: linear-gradient(135deg, rgba(30, 41, 59, 0.98), rgba(15, 23, 42, 0.98));
		border-left: 1px solid rgba(99, 102, 241, 0.2);
		display: flex;
		flex-direction: column;
		box-shadow: -20px 0 60px rgba(0, 0, 0, 0.4);
	}

	.panel-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1.25rem;
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
	}

	.header-left {
		display: flex;
		align-items: center;
		gap: 0.875rem;
	}

	.header-icon {
		width: 42px;
		height: 42px;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 12px;
		color: white;
	}

	.header-icon.healthy {
		background: linear-gradient(135deg, #10b981, #059669);
	}

	.header-icon.partial {
		background: linear-gradient(135deg, #f59e0b, #d97706);
	}

	.header-icon.unhealthy {
		background: linear-gradient(135deg, #ef4444, #dc2626);
	}

	.header-info h2 {
		font-size: 1.125rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.connection-summary {
		font-size: 0.8125rem;
		color: #64748b;
	}

	.header-actions {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.refresh-all-btn {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 0.75rem;
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		color: #a5b4fc;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.refresh-all-btn:hover:not(:disabled) {
		background: rgba(99, 102, 241, 0.2);
	}

	.refresh-all-btn:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.close-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(99, 102, 241, 0.1);
		border: none;
		border-radius: 10px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.close-btn:hover {
		background: rgba(99, 102, 241, 0.2);
		color: #f1f5f9;
	}

	.status-bar {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.875rem 1.25rem;
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
	}

	.status-bar.healthy {
		background: rgba(16, 185, 129, 0.1);
	}

	.status-bar.partial {
		background: rgba(245, 158, 11, 0.1);
	}

	.status-bar.unhealthy {
		background: rgba(239, 68, 68, 0.1);
	}

	.status-indicator {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.875rem;
		font-weight: 500;
	}

	.status-bar.healthy .status-indicator {
		color: #10b981;
	}

	.status-bar.partial .status-indicator {
		color: #f59e0b;
	}

	.status-bar.unhealthy .status-indicator {
		color: #ef4444;
	}

	.last-check {
		font-size: 0.75rem;
		color: #64748b;
	}

	.panel-tabs {
		display: flex;
		padding: 0 1.25rem;
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
	}

	.tab {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.875rem 1rem;
		background: none;
		border: none;
		border-bottom: 2px solid transparent;
		color: #64748b;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.tab:hover {
		color: #94a3b8;
	}

	.tab.active {
		color: #a5b4fc;
		border-bottom-color: #6366f1;
	}

	.panel-content {
		flex: 1;
		overflow-y: auto;
		padding: 1rem;
	}

	.panel-content::-webkit-scrollbar {
		width: 6px;
	}

	.panel-content::-webkit-scrollbar-track {
		background: transparent;
	}

	.panel-content::-webkit-scrollbar-thumb {
		background: rgba(99, 102, 241, 0.3);
		border-radius: 3px;
	}

	.services-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.service-card {
		display: flex;
		align-items: flex-start;
		gap: 0.875rem;
		padding: 1rem;
		background: rgba(99, 102, 241, 0.05);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 14px;
		transition: all 0.2s;
	}

	.service-card:hover {
		background: rgba(99, 102, 241, 0.08);
	}

	.service-card.connected {
		border-color: rgba(16, 185, 129, 0.2);
	}

	.service-card.error {
		border-color: rgba(239, 68, 68, 0.2);
	}

	.service-status {
		width: 40px;
		height: 40px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(99, 102, 241, 0.1);
		border-radius: 10px;
		flex-shrink: 0;
	}

	.service-info {
		flex: 1;
		min-width: 0;
	}

	.service-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 0.25rem;
	}

	.service-name {
		font-size: 0.9375rem;
		font-weight: 600;
		color: #f1f5f9;
	}

	.status-badge {
		padding: 0.125rem 0.5rem;
		border-radius: 6px;
		font-size: 0.6875rem;
		font-weight: 600;
	}

	.service-description {
		font-size: 0.8125rem;
		color: #64748b;
		margin: 0 0 0.5rem;
		line-height: 1.4;
	}

	.quick-metrics {
		display: flex;
		gap: 0.875rem;
	}

	.quick-metrics .metric {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-size: 0.75rem;
		color: #94a3b8;
	}

	.service-actions {
		display: flex;
		gap: 0.375rem;
	}

	.action-btn {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(99, 102, 241, 0.1);
		border: none;
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
		text-decoration: none;
		transition: all 0.2s;
	}

	.action-btn:hover:not(:disabled) {
		background: rgba(99, 102, 241, 0.2);
		color: #a5b4fc;
	}

	.action-btn:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.metrics-view {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.metrics-card {
		padding: 1rem;
		background: rgba(99, 102, 241, 0.05);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 14px;
	}

	.metrics-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 1rem;
	}

	.status-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
	}

	.metrics-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 1rem;
	}

	.metric-item {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
	}

	.metric-label {
		font-size: 0.6875rem;
		font-weight: 500;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.metric-value {
		font-size: 1.125rem;
		font-weight: 700;
	}

	.metric-bar {
		height: 4px;
		background: rgba(99, 102, 241, 0.1);
		border-radius: 2px;
		overflow: hidden;
	}

	.metric-bar-fill {
		height: 100%;
		border-radius: 2px;
		transition: width 0.3s ease;
	}

	.metric-bar-fill.error {
		background: #ef4444;
	}

	.panel-footer {
		padding: 1rem 1.25rem;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
	}

	.view-all-link {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.75rem;
		background: rgba(99, 102, 241, 0.1);
		border-radius: 10px;
		color: #a5b4fc;
		font-size: 0.875rem;
		font-weight: 500;
		text-decoration: none;
		transition: all 0.2s;
	}

	.view-all-link:hover {
		background: rgba(99, 102, 241, 0.2);
	}

	:global(.spinning) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}

	@media (max-width: 480px) {
		.health-panel {
			max-width: 100%;
		}

		.metrics-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
