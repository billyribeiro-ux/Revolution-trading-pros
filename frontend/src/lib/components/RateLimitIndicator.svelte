<script lang="ts">
	/**
	 * API Rate Limit Indicator - Apple ICT9+ Design
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * Shows API rate limit status for connected services.
	 *
	 * @version 1.0.0
	 */

	import { onMount, onDestroy } from 'svelte';
	import { fade, scale } from 'svelte/transition';
	import { connections, type ConnectionStatus } from '$lib/stores/connections';
	import {
		IconPlugConnected,
		IconChevronDown,
		IconAlertTriangle,
		IconCircleCheck,
		IconClock
	} from '@tabler/icons-svelte';

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
	let refreshInterval: ReturnType<typeof setInterval>;

	// Mock data - in production, this would come from actual API responses
	function generateMockRateLimits(): RateLimitInfo[] {
		const mockServices = [
			{ service: 'Google Analytics', limit: 10000 },
			{ service: 'Mailchimp', limit: 100 },
			{ service: 'Stripe', limit: 1000 },
			{ service: 'SendGrid', limit: 500 }
		];

		return mockServices.map(s => {
			const remaining = Math.floor(Math.random() * s.limit);
			const percentage = (remaining / s.limit) * 100;
			let status: 'ok' | 'warning' | 'critical' = 'ok';
			if (percentage < 10) status = 'critical';
			else if (percentage < 25) status = 'warning';

			return {
				...s,
				remaining,
				percentage,
				status,
				resetsAt: new Date(Date.now() + Math.random() * 3600000)
			};
		});
	}

	let overallStatus = $derived(rateLimits.some(r => r.status === 'critical')
		? 'critical'
		: rateLimits.some(r => r.status === 'warning')
		? 'warning'
		: 'ok');

	let lowestPercentage = $derived(rateLimits.length > 0
		? Math.min(...rateLimits.map(r => r.percentage))
		: 100);

	function getStatusColor(status: string) {
		switch (status) {
			case 'critical': return '#ef4444';
			case 'warning': return '#f59e0b';
			default: return '#10b981';
		}
	}

	function getStatusIcon(status: string) {
		switch (status) {
			case 'critical': return IconAlertTriangle;
			case 'warning': return IconClock;
			default: return IconCircleCheck;
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
		rateLimits = generateMockRateLimits();
		refreshInterval = setInterval(() => {
			rateLimits = generateMockRateLimits();
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
		onclick={(e) => { e.stopPropagation(); toggle(); }}
		title="API Rate Limits"
	>
		<IconPlugConnected size={18} />
		<span class="limit-percentage">{Math.round(lowestPercentage)}%</span>
		<span class="chevron" class:rotated={isOpen}>
			<IconChevronDown size={14} />
		</span>
	</button>

	{#if isOpen}
		<div
			class="rate-limit-dropdown"
			transition:scale={{ duration: 200, start: 0.95 }}
			onclick={(e) => e.stopPropagation()}
		>
			<div class="dropdown-header">
				<h4>API Rate Limits</h4>
				<span class="header-status" style="color: {getStatusColor(overallStatus)}">
					<svelte:component this={getStatusIcon(overallStatus)} size={14} />
					{overallStatus === 'ok' ? 'All services healthy' : overallStatus === 'warning' ? 'Some limits low' : 'Critical limits reached'}
				</span>
			</div>

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

			<div class="dropdown-footer">
				<span class="footer-note">Rate limits refresh every minute</span>
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
		background: rgba(16, 185, 129, 0.1);
		border: 1px solid rgba(16, 185, 129, 0.2);
		border-radius: 8px;
		color: #10b981;
		font-size: 0.8125rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.rate-limit-btn:hover {
		background: rgba(16, 185, 129, 0.2);
	}

	.rate-limit-btn.warning {
		background: rgba(245, 158, 11, 0.1);
		border-color: rgba(245, 158, 11, 0.2);
		color: #f59e0b;
	}

	.rate-limit-btn.warning:hover {
		background: rgba(245, 158, 11, 0.2);
	}

	.rate-limit-btn.critical {
		background: rgba(239, 68, 68, 0.1);
		border-color: rgba(239, 68, 68, 0.2);
		color: #ef4444;
	}

	.rate-limit-btn.critical:hover {
		background: rgba(239, 68, 68, 0.2);
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
		color: #f1f5f9;
		margin: 0 0 0.375rem 0;
	}

	.header-status {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.75rem;
	}

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
		color: #e2e8f0;
	}

	.limit-status {
		font-size: 0.75rem;
		font-weight: 600;
	}

	.limit-bar-container {
		height: 4px;
		background: rgba(99, 102, 241, 0.1);
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
		color: #64748b;
	}

	.percentage {
		font-size: 0.6875rem;
		color: #94a3b8;
	}

	.dropdown-footer {
		padding: 0.75rem 1rem;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
		background: rgba(15, 23, 42, 0.5);
	}

	.footer-note {
		font-size: 0.6875rem;
		color: #64748b;
	}
</style>
