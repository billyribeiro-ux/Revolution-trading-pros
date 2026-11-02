<script lang="ts">
	/**
	 * API Rate Limit Indicator - Apple ICT9+ Design
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * Shows API rate limit status for connected services.
	 * Shows "No services connected" when nothing is connected instead of fake 100%.
	 *
	 * @version 2.0.0 - Fixed misleading UX
	 */

	import { onMount } from 'svelte';
	import { scale } from 'svelte/transition';
	import IconGauge from '@tabler/icons-svelte-runes/icons/gauge';
	import IconPlugConnectedX from '@tabler/icons-svelte-runes/icons/plug-connected-x';
	import IconAlertTriangle from '@tabler/icons-svelte-runes/icons/alert-triangle';
	import IconCircleCheck from '@tabler/icons-svelte-runes/icons/circle-check';
	import IconClock from '@tabler/icons-svelte-runes/icons/clock';
	import IconPlugOff from '@tabler/icons-svelte-runes/icons/plug-off';
	import { adminFetch } from '#lib/utils/adminFetch.js';

	interface RateLimitInfo {
		service: string;
		limit: number;
		remaining: number;
		resetsAt: Date | null;
		percentage: number;
		status: 'ok' | 'warning' | 'critical';
	}

	let isOpen = $state(false);
	let rateLimits = $state<RateLimitInfo[]>([]);
	let isLoading = $state(true);
	let hasConnectedServices = $state(false);

	/**
	 * Fetch real rate limit data from connected services
	 * Returns empty array if no services are connected
	 */
	async function fetchRateLimits(): Promise<RateLimitInfo[]> {
		try {
			// ICT11+ Pattern: Use adminFetch for authenticated admin API calls
			const data = await adminFetch('/api/admin/connections', { skipAuthRedirect: true });
			const limits: RateLimitInfo[] = [];

			// Transform connection status to rate limit info
			if (data.connections && Array.isArray(data.connections)) {
				for (const conn of data.connections) {
					// Only include services that are actually connected and have rate limit info
					if (conn.status === 'connected' && conn.rate_limit) {
						const limit = conn.rate_limit.limit || 1000;
						const remaining = conn.rate_limit.remaining ?? limit;
						const percentage = (remaining / limit) * 100;
						let status: 'ok' | 'warning' | 'critical' = 'ok';
						if (percentage < 10) status = 'critical';
						else if (percentage < 25) status = 'warning';

						limits.push({
							service: conn.name || conn.key,
							limit,
							remaining,
							percentage,
							status,
							resetsAt: conn.rate_limit.resets_at ? new Date(conn.rate_limit.resets_at) : null
						});
					}
				}
			}

			// Update connected services flag
			hasConnectedServices = limits.length > 0;
			return limits;
		} catch (error) {
			console.warn('Failed to fetch rate limits:', error);
			hasConnectedServices = false;
			return [];
		}
	}

	// Calculate overall status - only if we have connected services
	let overallStatus = $derived.by(() => {
		if (!hasConnectedServices || rateLimits.length === 0) return 'none';
		if (rateLimits.some((r) => r.status === 'critical')) return 'critical';
		if (rateLimits.some((r) => r.status === 'warning')) return 'warning';
		return 'ok';
	});

	// Calculate lowest percentage - only if we have connected services
	let lowestPercentage = $derived.by(() => {
		if (!hasConnectedServices || rateLimits.length === 0) return null;
		return Math.min(...rateLimits.map((r) => r.percentage));
	});

	function getStatusColor(status: string) {
		switch (status) {
			case 'critical':
				return '#ef4444';
			case 'warning':
				return '#f59e0b';
			case 'ok':
				return '#10b981';
			case 'none':
				return '#6b7280';
			default:
				return '#6b7280';
		}
	}

	function getStatusIcon(status: string) {
		switch (status) {
			case 'critical':
				return IconAlertTriangle;
			case 'warning':
				return IconClock;
			case 'ok':
				return IconCircleCheck;
			default:
				return IconPlugConnectedX;
		}
	}

	function formatTimeUntilReset(date: Date | null): string {
		if (!date) return 'N/A';
		const diff = date.getTime() - Date.now();
		if (diff <= 0) return 'Now';
		const mins = Math.floor(diff / 60000);
		if (mins < 60) return `${mins}m`;
		return `${Math.floor(mins / 60)}h ${mins % 60}m`;
	}

	function toggle() {
		isOpen = !isOpen;
	}

	function close() {
		isOpen = false;
	}

	onMount(() => {
		let isMounted = true;

		async function refreshRateLimits() {
			const nextRateLimits = await fetchRateLimits();
			if (isMounted) {
				rateLimits = nextRateLimits;
				isLoading = false;
			}
		}

		isLoading = true;
		void refreshRateLimits();

		// Refresh every minute
		const refreshInterval = setInterval(() => {
			void refreshRateLimits();
		}, 60000);

		return () => {
			isMounted = false;
			clearInterval(refreshInterval);
		};
	});
</script>

<svelte:window onclick={close} />

<div class="rate-limit-container">
	<button
		type="button"
		class={[
			'icon-btn',
			'rate-limit-btn',
			{
				warning: overallStatus === 'warning',
				critical: overallStatus === 'critical',
				ok: overallStatus === 'ok'
			}
		]}
		onclick={(e: MouseEvent) => {
			e.stopPropagation();
			toggle();
		}}
		title="API rate limits"
		aria-label={hasConnectedServices
			? `API rate limits, lowest remaining ${Math.round(lowestPercentage ?? 0)}%`
			: 'API rate limits, no services connected'}
		aria-haspopup="menu"
		aria-expanded={isOpen}
	>
		{#if hasConnectedServices}
			<IconGauge size={18} aria-hidden="true" />
		{:else}
			<IconPlugConnectedX size={18} aria-hidden="true" />
		{/if}
		{#if overallStatus !== 'none'}
			<span class="status-dot" aria-hidden="true"></span>
		{/if}
	</button>

	{#if isOpen}
		{const status = overallStatus}
		{const StatusIcon = getStatusIcon(status)}
		<div
			class="rate-limit-dropdown"
			transition:scale={{ duration: 200, start: 0.95 }}
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
			role="menu"
			tabindex="-1"
		>
			<div class="dropdown-header">
				<h4>API Rate Limits</h4>
				<span class="header-status" style:color={getStatusColor(status)}>
					<StatusIcon size={14} />
					{#if status === 'none'}
						No services connected
					{:else if status === 'ok'}
						All services healthy
					{:else if status === 'warning'}
						Some limits low
					{:else}
						Critical limits reached
					{/if}
				</span>
			</div>

			{#if isLoading}
				<div class="limits-loading">
					<div class="loading-spinner"></div>
					<span>Checking connections...</span>
				</div>
			{:else if !hasConnectedServices}
				<!-- No services connected - show helpful message -->
				<div class="no-services">
					<div class="no-services-icon">
						<IconPlugOff size={32} />
					</div>
					<p class="no-services-title">No Services Connected</p>
					<p class="no-services-desc">
						Connect external services like Stripe, SendGrid, or OpenAI to monitor their API rate
						limits here.
					</p>
					<a href="/admin/connections" class="connect-btn"> Connect Services </a>
				</div>
			{:else}
				<div class="limits-list">
					{#each rateLimits as limit (limit.service)}
						<div class="limit-item">
							<div class="limit-header">
								<span class="service-name">{limit.service}</span>
								<span class="limit-status" style:color={getStatusColor(limit.status)}>
									{limit.remaining.toLocaleString()} / {limit.limit.toLocaleString()}
								</span>
							</div>
							<div class="limit-bar-container">
								<div
									class="limit-bar"
									style:width={`${limit.percentage}%`}
									style:background={getStatusColor(limit.status)}
								></div>
							</div>
							<div class="limit-footer">
								<span class="reset-time">
									<IconClock size={12} />
									Resets in {formatTimeUntilReset(limit.resetsAt)}
								</span>
								<span class="percentage">{Math.round(limit.percentage)}% remaining</span>
							</div>
						</div>
					{/each}
				</div>
			{/if}

			<div class="dropdown-footer">
				{#if hasConnectedServices}
					<span class="footer-note">Rate limits refresh every minute</span>
				{:else}
					<span class="footer-note">Connect services to monitor rate limits</span>
				{/if}
			</div>
		</div>
	{/if}
</div>

<style>
	.rate-limit-container {
		position: relative;
	}

	/* Sizing, hover and focus come from the shared `.icon-btn` in admin.css so the
	   header toolbar stays uniform. Only the status dot is local. */
	.status-dot {
		position: absolute;
		top: 5px;
		right: 5px;
		width: 8px;
		height: 8px;
		border-radius: var(--radius-full, 9999px);
		background: var(--text-tertiary, #6e7681);
		border: 2px solid var(--bg-surface, #1c2128);
		box-sizing: content-box;
	}

	.rate-limit-btn.ok .status-dot {
		background: var(--success-emphasis, #3fb950);
	}

	.rate-limit-btn.warning .status-dot {
		background: var(--warning-emphasis, #d29922);
	}

	.rate-limit-btn.critical .status-dot {
		background: var(--error-emphasis, #f85149);
	}

	.rate-limit-dropdown {
		position: absolute;
		top: calc(100% + 0.5rem);
		right: 0;
		width: 320px;
		max-width: calc(100vw - 2rem);
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-default, #30363d);
		border-radius: var(--radius-xl, 10px);
		box-shadow: var(--shadow-lg, 0 8px 24px rgba(0, 0, 0, 0.35));
		overflow: hidden;
		z-index: var(--z-dropdown, 100);
	}

	.dropdown-header {
		padding: 1rem;
		border-bottom: 1px solid var(--border-muted, #21262d);
	}

	.dropdown-header h4 {
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
		margin: 0 0 0.375rem 0;
	}

	.header-status {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.75rem;
	}

	.limits-loading {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 2rem;
		gap: 0.75rem;
		color: var(--text-secondary, #8b949e);
		font-size: 0.8125rem;
	}

	.loading-spinner {
		width: 24px;
		height: 24px;
		border: 2px solid var(--border-default, #30363d);
		border-top-color: var(--primary-500, #e6b800);
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.loading-spinner {
			animation-duration: 2s;
		}
	}

	.no-services {
		display: flex;
		flex-direction: column;
		align-items: center;
		text-align: center;
		padding: 1.5rem;
	}

	.no-services-icon {
		width: 56px;
		height: 56px;
		border-radius: var(--radius-xl, 10px);
		background: var(--bg-surface, #1c2128);
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--text-tertiary, #6e7681);
		margin-bottom: 1rem;
	}

	.no-services-title {
		font-size: 0.9375rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
		margin: 0 0 0.5rem 0;
	}

	.no-services-desc {
		font-size: 0.8125rem;
		color: var(--text-secondary, #8b949e);
		margin: 0 0 1rem 0;
		line-height: 1.5;
	}

	.connect-btn {
		display: inline-flex;
		align-items: center;
		min-height: 36px;
		padding: 0 1.25rem;
		background: var(--primary-500, #e6b800);
		color: var(--bg-base, #0d1117);
		font-size: 0.8125rem;
		font-weight: 600;
		border-radius: var(--radius-md, 6px);
		text-decoration: none;
		transition: background-color 0.2s ease;
	}

	.connect-btn:hover {
		background: var(--primary-400, #ffd11a);
	}

	.limits-list {
		padding: 0.75rem;
		max-height: 300px;
		overflow-y: auto;
	}

	.limit-item {
		padding: 0.75rem;
		background: var(--bg-surface, #1c2128);
		border-radius: var(--radius-lg, 8px);
		margin-bottom: 0.5rem;
	}

	.limit-item:last-child {
		margin-bottom: 0;
	}

	.limit-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.5rem;
	}

	.service-name {
		font-size: 0.8125rem;
		font-weight: 500;
		color: var(--text-primary, #f0f6fc);
	}

	.limit-status {
		font-size: 0.75rem;
		font-weight: 600;
	}

	.limit-bar-container {
		height: 4px;
		background: var(--bg-base, #0d1117);
		border-radius: 2px;
		overflow: hidden;
		margin-bottom: 0.5rem;
	}

	.limit-bar {
		height: 100%;
		border-radius: 2px;
		transition: width 0.3s ease;
	}

	.limit-footer {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.reset-time {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-size: 0.75rem;
		color: var(--text-secondary, #8b949e);
	}

	.percentage {
		font-size: 0.75rem;
		color: var(--text-secondary, #8b949e);
	}

	.dropdown-footer {
		padding: 0.75rem 1rem;
		border-top: 1px solid var(--border-muted, #21262d);
		background: var(--bg-base, #0d1117);
	}

	.footer-note {
		font-size: 0.75rem;
		color: var(--text-secondary, #8b949e);
	}
</style>
