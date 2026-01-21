<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * AlertCard Component - Live Alert Display (Nuclear Build)
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Displays a single alert with visual differentiation by type
	 * @version 4.0.0 - January 2026 - Nuclear Build Specification
	 * @standards Apple Principal Engineer ICT 7+ Standards
	 *
	 * Visual Hierarchy:
	 * - ENTRY: Full-width, teal accent, prominent CTA
	 * - UPDATE: Condensed, amber accent, collapsible
	 * - EXIT: Compact, green/red based on result
	 */
	import type { Alert } from '../types';
	import { alertColors } from '../types';
	import { formatRelativeTime, formatPrice, formatPercent } from '../utils/formatters';

	interface Props {
		alert: Alert;
		isAdmin?: boolean;
		onViewTradePlan?: (alert: Alert) => void;
		onEdit?: (alert: Alert) => void;
		onDelete?: (alertId: string) => void;
	}

	const { alert, isAdmin = false, onViewTradePlan, onEdit, onDelete }: Props = $props();

	let isExpanded = $state(true);

	const colors = $derived(() => {
		if (alert.type === 'ENTRY') return alertColors.entry;
		if (alert.type === 'UPDATE') return alertColors.update;
		if (alert.type === 'EXIT') {
			return (alert.resultPercent ?? 0) >= 0 ? alertColors.exitWin : alertColors.exitLoss;
		}
		return alertColors.entry;
	});

	const borderClass = $derived(colors().border);
	const bgClass = $derived(colors().bg);
	const badgeClass = $derived(colors().badge);

	function toggleExpanded() {
		isExpanded = !isExpanded;
	}
</script>

<article
	class="alert-card {borderClass} {bgClass}"
	class:is-new={alert.isNew}
	aria-label="{alert.type} alert for {alert.ticker}"
>
	<!-- Header Row -->
	<div class="alert-header">
		<div class="alert-header-left">
			<span class="alert-ticker">{alert.ticker}</span>
			<span class="alert-badge {badgeClass}">{alert.type}</span>
			{#if alert.isNew}
				<span class="new-badge pulse-new">NEW</span>
			{/if}
		</div>
		<div class="alert-header-right">
			<time class="alert-time" datetime={alert.timestamp.toISOString()}>
				{formatRelativeTime(alert.timestamp)}
			</time>
			{#if alert.type === 'UPDATE'}
				<button
					class="expand-btn"
					onclick={toggleExpanded}
					aria-expanded={isExpanded}
					aria-label={isExpanded ? 'Collapse details' : 'Expand details'}
				>
					<svg
						viewBox="0 0 20 20"
						fill="currentColor"
						class="w-5 h-5 transition-transform"
						class:rotate-180={isExpanded}
					>
						<path
							fill-rule="evenodd"
							d="M5.23 7.21a.75.75 0 011.06.02L10 11.168l3.71-3.938a.75.75 0 111.08 1.04l-4.25 4.5a.75.75 0 01-1.08 0l-4.25-4.5a.75.75 0 01.02-1.06z"
							clip-rule="evenodd"
						/>
					</svg>
				</button>
			{/if}
		</div>
	</div>

	<!-- Alert Title -->
	<h3 class="alert-title">{alert.title}</h3>

	<!-- Content (collapsible for UPDATE type) -->
	{#if alert.type !== 'UPDATE' || isExpanded}
		<div class="alert-content">
			<!-- ENTRY Alert: Show entry/target/stop grid -->
			{#if alert.type === 'ENTRY' && (alert.entryPrice || alert.targetPrice || alert.stopPrice)}
				<div class="trade-details-grid">
					{#if alert.entryPrice}
						<div class="trade-detail">
							<span class="detail-label">Entry</span>
							<span class="detail-value">{formatPrice(alert.entryPrice)}</span>
						</div>
					{/if}
					{#if alert.targetPrice && alert.entryPrice}
						<div class="trade-detail">
							<span class="detail-label">Target</span>
							<span class="detail-value positive">
								{formatPrice(alert.targetPrice)}
								<span class="detail-percent">
									({formatPercent(((alert.targetPrice - alert.entryPrice) / alert.entryPrice) * 100)})
								</span>
							</span>
						</div>
					{/if}
					{#if alert.stopPrice && alert.entryPrice}
						<div class="trade-detail">
							<span class="detail-label">Stop</span>
							<span class="detail-value negative">
								{formatPrice(alert.stopPrice)}
								<span class="detail-percent">
									({formatPercent(((alert.stopPrice - alert.entryPrice) / alert.entryPrice) * 100)})
								</span>
							</span>
						</div>
					{/if}
				</div>
			{/if}

			<!-- EXIT Alert: Show result prominently -->
			{#if alert.type === 'EXIT' && alert.resultPercent !== undefined}
				<div class="exit-result">
					<span
						class="result-value"
						class:positive={alert.resultPercent >= 0}
						class:negative={alert.resultPercent < 0}
					>
						{formatPercent(alert.resultPercent)}
					</span>
				</div>
			{/if}

			<!-- Description -->
			<p class="alert-description">{alert.description}</p>

			<!-- Notes -->
			{#if alert.notes}
				<p class="alert-notes">{alert.notes}</p>
			{/if}
		</div>
	{/if}

	<!-- Actions Row -->
	<div class="alert-actions">
		{#if alert.type === 'ENTRY' && onViewTradePlan}
			<button class="action-btn primary" onclick={() => onViewTradePlan(alert)}>
				View Trade Plan
			</button>
		{/if}
		{#if alert.hasVideo && alert.videoUrl}
			<a href={alert.videoUrl} class="action-btn secondary" target="_blank" rel="noopener">
				<svg viewBox="0 0 20 20" fill="currentColor" class="w-4 h-4">
					<path d="M6.3 2.841A1.5 1.5 0 004 4.11V15.89a1.5 1.5 0 002.3 1.269l9.344-5.89a1.5 1.5 0 000-2.538L6.3 2.84z" />
				</svg>
				Watch Video
			</a>
		{/if}

		<!-- Admin Actions -->
		{#if isAdmin}
			<div class="admin-actions">
				{#if onEdit}
					<button class="admin-btn" onclick={() => onEdit(alert)} aria-label="Edit alert">
						<svg viewBox="0 0 20 20" fill="currentColor" class="w-4 h-4">
							<path d="M2.695 14.763l-1.262 3.154a.5.5 0 00.65.65l3.155-1.262a4 4 0 001.343-.885L17.5 5.5a2.121 2.121 0 00-3-3L3.58 13.42a4 4 0 00-.885 1.343z" />
						</svg>
					</button>
				{/if}
				{#if onDelete}
					<button class="admin-btn danger" onclick={() => onDelete(alert.id)} aria-label="Delete alert">
						<svg viewBox="0 0 20 20" fill="currentColor" class="w-4 h-4">
							<path fill-rule="evenodd" d="M8.75 1A2.75 2.75 0 006 3.75v.443c-.795.077-1.584.176-2.365.298a.75.75 0 10.23 1.482l.149-.022.841 10.518A2.75 2.75 0 007.596 19h4.807a2.75 2.75 0 002.742-2.53l.841-10.52.149.023a.75.75 0 00.23-1.482A41.03 41.03 0 0014 4.193V3.75A2.75 2.75 0 0011.25 1h-2.5z" clip-rule="evenodd" />
						</svg>
					</button>
				{/if}
			</div>
		{/if}
	</div>
</article>

<style>
	.alert-card {
		position: relative;
		border-left: 4px solid;
		border-radius: 12px;
		padding: 20px;
		margin-bottom: 16px;
		transition: all 0.2s ease-out;
		animation: slideInUp 0.3s ease-out;
	}

	.alert-card:hover {
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
	}

	.alert-card.is-new {
		box-shadow: 0 0 0 2px rgba(20, 184, 166, 0.3);
	}

	@keyframes slideInUp {
		from {
			opacity: 0;
			transform: translateY(8px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.alert-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 12px;
	}

	.alert-header-left {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.alert-header-right {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.alert-ticker {
		font-size: 24px;
		font-weight: 700;
		color: #0f172a;
		text-transform: uppercase;
		letter-spacing: 0.025em;
	}

	.alert-badge {
		font-size: 11px;
		font-weight: 600;
		padding: 4px 10px;
		border-radius: 6px;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.new-badge {
		font-size: 10px;
		font-weight: 700;
		padding: 3px 8px;
		border-radius: 4px;
		background: #dc2626;
		color: #fff;
		text-transform: uppercase;
		letter-spacing: 0.1em;
	}

	.pulse-new {
		animation: pulse-subtle 2s ease-in-out infinite;
	}

	@keyframes pulse-subtle {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.7;
		}
	}

	.alert-time {
		font-size: 13px;
		color: #64748b;
	}

	.expand-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		border: none;
		background: rgba(0, 0, 0, 0.05);
		border-radius: 6px;
		color: #64748b;
		cursor: pointer;
		transition: all 0.2s;
	}

	.expand-btn:hover {
		background: rgba(0, 0, 0, 0.1);
		color: #0f172a;
	}

	.rotate-180 {
		transform: rotate(180deg);
	}

	.alert-title {
		font-size: 16px;
		font-weight: 600;
		color: #1e293b;
		margin: 0 0 12px 0;
		line-height: 1.4;
	}

	.alert-content {
		margin-bottom: 16px;
	}

	.trade-details-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 16px;
		padding: 16px;
		background: rgba(255, 255, 255, 0.6);
		border-radius: 8px;
		margin-bottom: 12px;
	}

	.trade-detail {
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.detail-label {
		font-size: 12px;
		font-weight: 500;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.detail-value {
		font-size: 16px;
		font-weight: 600;
		color: #0f172a;
		font-variant-numeric: tabular-nums;
	}

	.detail-value.positive {
		color: #059669;
	}

	.detail-value.negative {
		color: #dc2626;
	}

	.detail-percent {
		font-size: 13px;
		font-weight: 500;
	}

	.exit-result {
		margin-bottom: 12px;
	}

	.result-value {
		font-size: 32px;
		font-weight: 700;
		font-variant-numeric: tabular-nums;
	}

	.result-value.positive {
		color: #059669;
	}

	.result-value.negative {
		color: #dc2626;
	}

	.alert-description {
		font-size: 14px;
		color: #475569;
		line-height: 1.6;
		margin: 0;
	}

	.alert-notes {
		font-size: 13px;
		color: #64748b;
		font-style: italic;
		margin: 8px 0 0 0;
		line-height: 1.5;
	}

	.alert-actions {
		display: flex;
		align-items: center;
		gap: 12px;
		flex-wrap: wrap;
	}

	.action-btn {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 10px 18px;
		font-size: 14px;
		font-weight: 600;
		border-radius: 8px;
		border: none;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
	}

	.action-btn.primary {
		background: #0f766e;
		color: #fff;
	}

	.action-btn.primary:hover {
		background: #0d9488;
	}

	.action-btn.secondary {
		background: rgba(0, 0, 0, 0.05);
		color: #475569;
	}

	.action-btn.secondary:hover {
		background: rgba(0, 0, 0, 0.1);
		color: #0f172a;
	}

	.admin-actions {
		display: flex;
		gap: 8px;
		margin-left: auto;
	}

	.admin-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		border: 1px solid #e2e8f0;
		background: #fff;
		border-radius: 6px;
		color: #64748b;
		cursor: pointer;
		transition: all 0.2s;
	}

	.admin-btn:hover {
		border-color: #cbd5e1;
		color: #0f172a;
	}

	.admin-btn.danger:hover {
		border-color: #fecaca;
		background: #fef2f2;
		color: #dc2626;
	}

	@media (max-width: 640px) {
		.alert-card {
			padding: 16px;
		}

		.alert-ticker {
			font-size: 20px;
		}

		.trade-details-grid {
			grid-template-columns: 1fr;
			gap: 12px;
		}

		.alert-actions {
			flex-direction: column;
			align-items: stretch;
		}

		.admin-actions {
			margin-left: 0;
			justify-content: flex-end;
		}
	}
</style>
