<script lang="ts">
import { logger } from '$lib/utils/logger';
	/**
	 * API Rate Limit Indicator - Apple ICT9+ Design
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * Shows API rate limit status for connected services.
	 * Shows "No services connected" when nothing is connected instead of fake 100%.
	 *
	 * @version 2.0.0 - Fixed misleading UX
	 */

	import { onMount, onDestroy } from 'svelte';
	import { scale } from 'svelte/transition';
	import IconPlugConnected from '@tabler/icons-svelte-runes/icons/plug-connected';
	import IconPlugConnectedX from '@tabler/icons-svelte-runes/icons/plug-connected-x';
	import IconChevronDown from '@tabler/icons-svelte-runes/icons/chevron-down';
	import IconAlertTriangle from '@tabler/icons-svelte-runes/icons/alert-triangle';
	import IconCircleCheck from '@tabler/icons-svelte-runes/icons/circle-check';
	import IconClock from '@tabler/icons-svelte-runes/icons/clock';
	import IconPlugOff from '@tabler/icons-svelte-runes/icons/plug-off';
	import { adminFetch } from '$lib/utils/adminFetch';

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
	let refreshInterval = $state<ReturnType<typeof setInterval> | undefined>(undefined);

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
			logger.warn('Failed to fetch rate limits:', error);
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
				return 'var(--color-loss)';
			case 'warning':
				return 'var(--color-warning)';
			case 'ok':
				return 'var(--color-success)';
			case 'none':
				return 'var(--color-text-muted)';
			default:
				return 'var(--color-text-muted)';
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

	onMount(async () => {
		isLoading = true;
		rateLimits = await fetchRateLimits();
		isLoading = false;

		// Refresh every minute
		refreshInterval = setInterval(async () => {
			rateLimits = await fetchRateLimits();
		}, 60000);
	});

	onDestroy(() => {
		if (refreshInterval) clearInterval(refreshInterval);
	});
</script>

<svelte:window onclick={close} />

<div class="rate-limit-container">
	<button
		class="rate-limit-btn"
		class:warning={overallStatus === 'warning'}
		class:critical={overallStatus === 'critical'}
		class:disconnected={overallStatus === 'none'}
		onclick={(e: MouseEvent) => {
			e.stopPropagation();
			toggle();
		}}
		title="API Rate Limits"
	>
		{#if hasConnectedServices}
			<IconPlugConnected size={18} />
			<span class="limit-percentage"
				>{lowestPercentage !== null ? Math.round(lowestPercentage ?? 0) : '--'}%</span
			>
		{:else}
			<IconPlugConnectedX size={18} />
			<span class="limit-percentage">--</span>
		{/if}
		<span class="chevron" class:rotated={isOpen}>
			<IconChevronDown size={14} />
		</span>
	</button>

	{#if isOpen}
		{@const status = overallStatus}
		{@const StatusIcon = getStatusIcon(status)}
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
				<span class="header-status" style="color: {getStatusColor(status)}">
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
					{#each rateLimits as limit}
						<div class="limit-item">
							<div class="limit-header">
								<span class="service-name">{limit.service}</span>
								<span class="limit-status" style="color: {getStatusColor(limit.status)}">
									{limit.remaining.toLocaleString()} / {limit.limit.toLocaleString()}
								</span>
							</div>
							<div class="limit-bar-container">
								<div
									class="limit-bar"
									style="width: {limit.percentage}%; background: {getStatusColor(limit.status)}"
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

	.rate-limit-btn {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 0.75rem;
		background: var(--color-success-bg);
		border: 1px solid var(--color-success-border);
		border-radius: 8px;
		color: var(--color-success);
		font-size: 0.8125rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.rate-limit-btn:hover {
		background: var(--color-success-bg-hover);
	}

	.rate-limit-btn.warning {
		background: var(--color-warning-bg);
		border-color: var(--color-warning-border);
		color: var(--color-warning);
	}

	.rate-limit-btn.warning:hover {
		background: var(--color-warning-bg-hover);
	}

	.rate-limit-btn.critical {
		background: var(--color-loss-bg);
		border-color: var(--color-loss-border);
		color: var(--color-loss);
	}

	.rate-limit-btn.critical:hover {
		background: var(--color-loss-bg-hover);
	}

	.rate-limit-btn.disconnected {
		background: var(--color-bg-subtle);
		border-color: var(--color-border-subtle);
		color: var(--color-text-muted);
	}

	.rate-limit-btn.disconnected:hover {
		background: var(--color-bg-hover);
	}

	.chevron {
		transition: transform 0.2s ease;
	}

	.chevron.rotated {
		transform: rotate(180deg);
	}

	.rate-limit-dropdown {
		position: absolute;
		top: calc(100% + 0.5rem);
		right: 0;
		width: 320px;
		background: linear-gradient(135deg, rgba(30, 41, 59, 0.98), rgba(15, 23, 42, 0.98));
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 14px;
		box-shadow: 0 20px 50px rgba(0, 0, 0, 0.4);
		overflow: hidden;
		z-index: 100;
	}

	.dropdown-header {
		padding: 1rem;
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
	}

	.dropdown-header h4 {
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--color-text-primary);
		margin: 0 0 0.375rem 0;
	}

	.header-status {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.75rem;
	}

	/* Loading State */
	.limits-loading {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 2rem;
		gap: 0.75rem;
		color: var(--color-text-muted);
		font-size: 0.8125rem;
	}

	.loading-spinner {
		width: 24px;
		height: 24px;
		border: 2px solid var(--color-brand-primary-subtle);
		border-top-color: var(--color-brand-primary);
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* No Services State */
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
		border-radius: 14px;
		background: var(--color-bg-subtle);
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--color-text-muted);
		margin-bottom: 1rem;
	}

	.no-services-title {
		font-size: 0.9375rem;
		font-weight: 600;
		color: var(--color-text-primary);
		margin: 0 0 0.5rem 0;
	}

	.no-services-desc {
		font-size: 0.8125rem;
		color: var(--color-text-muted);
		margin: 0 0 1rem 0;
		line-height: 1.5;
	}

	.connect-btn {
		display: inline-flex;
		align-items: center;
		padding: 0.625rem 1.25rem;
		background: var(--color-brand-gradient);
		color: var(--color-bg-card);
		font-size: 0.8125rem;
		font-weight: 600;
		border-radius: 8px;
		text-decoration: none;
		transition: all 0.2s;
	}

	.connect-btn:hover {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px var(--color-brand-shadow);
	}

	/* Limits List */
	.limits-list {
		padding: 0.75rem;
		max-height: 300px;
		overflow-y: auto;
	}

	.limits-list::-webkit-scrollbar {
		width: 4px;
	}

	.limits-list::-webkit-scrollbar-thumb {
		background: rgba(99, 102, 241, 0.3);
		border-radius: 2px;
	}

	.limit-item {
		padding: 0.75rem;
		background: rgba(99, 102, 241, 0.05);
		border-radius: 10px;
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
		color: var(--color-text-primary);
	}

	.limit-status {
		font-size: 0.75rem;
		font-weight: 600;
	}

	.limit-bar-container {
		height: 4px;
		background: var(--color-bg-subtle);
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
		font-size: 0.6875rem;
		color: var(--color-text-muted);
	}

	.percentage {
		font-size: 0.6875rem;
		color: var(--color-text-tertiary);
	}

	.dropdown-footer {
		padding: 0.75rem 1rem;
		border-top: 1px solid var(--color-border-subtle);
		background: var(--color-bg-subtle);
	}

	.footer-note {
		font-size: 0.6875rem;
		color: var(--color-text-muted);
	}
</style>
