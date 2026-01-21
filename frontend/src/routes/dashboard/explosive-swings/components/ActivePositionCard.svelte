<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * ActivePositionCard Component - Active Position Display
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Displays an active position with entry, targets, stop, and progress bar
	 * @version 4.0.0 - January 2026 - Nuclear Build Specification
	 * @standards Apple Principal Engineer ICT 7+ Standards
	 */
	import type { ActivePosition } from '../types';
	import { formatPercent, formatPrice } from '../utils/formatters';
	import { getPositionStatusColor } from '../utils/calculations';

	interface Props {
		position: ActivePosition;
	}

	const { position }: Props = $props();

	const statusColor = $derived(
		getPositionStatusColor(position.unrealizedPercent, position.status)
	);

	const unrealizedColor = $derived(
		position.unrealizedPercent === null
			? 'text-slate-600'
			: position.unrealizedPercent >= 0
				? 'text-emerald-600'
				: 'text-red-600'
	);

	const progressBarColor = $derived(
		position.unrealizedPercent === null
			? 'bg-slate-300'
			: position.unrealizedPercent >= 0
				? 'bg-emerald-500'
				: 'bg-red-500'
	);
</script>

<article class="position-card" aria-label="{position.ticker} position">
	<!-- Header Row -->
	<div class="position-header">
		<h3 class="position-ticker">{position.ticker}</h3>
		<span class="position-status {statusColor}">{position.status}</span>
	</div>

	<div class="position-divider"></div>

	<!-- Price Row -->
	<div class="position-prices">
		{#if position.status === 'WATCHING' && position.entryZone}
			<div class="price-row">
				<span class="price-label">Entry Zone:</span>
				<span class="price-value"
					>{formatPrice(position.entryZone.low)} - {formatPrice(position.entryZone.high)}</span
				>
			</div>
			<div class="price-row">
				<span class="price-label">Current:</span>
				<span class="price-value">{formatPrice(position.currentPrice)}</span>
			</div>
		{:else if position.entryPrice}
			<div class="price-row">
				<span class="price-label">Entry:</span>
				<span class="price-value">{formatPrice(position.entryPrice)}</span>
				<span class="price-separator">|</span>
				<span class="price-label">Now:</span>
				<span class="price-value">{formatPrice(position.currentPrice)}</span>
			</div>
		{/if}
	</div>

	<!-- Unrealized P&L -->
	{#if position.unrealizedPercent !== null}
		<div class="unrealized-row">
			<span class="unrealized-value {unrealizedColor}">
				{formatPercent(position.unrealizedPercent)} unrealized
			</span>
		</div>
	{/if}

	<!-- Notes -->
	{#if position.notes}
		<p class="position-notes">{position.notes}</p>
	{/if}

	<!-- Targets and Stop -->
	<div class="targets-section">
		{#each position.targets as target}
			<div class="target-row">
				<span class="target-label">{target.label}:</span>
				<span class="target-value"
					>{formatPrice(target.price)} ({formatPercent(target.percentFromEntry)})</span
				>
			</div>
		{/each}
		<div class="target-row stop-row">
			<span class="target-label">Stop:</span>
			<span class="target-value stop-value"
				>{formatPrice(position.stopLoss.price)} ({formatPercent(position.stopLoss.percentFromEntry)})</span
			>
		</div>
	</div>

	<!-- Progress Bar (only for ENTRY/ACTIVE with targets) -->
	{#if position.status !== 'WATCHING' && position.targets.length > 0}
		<div class="progress-section">
			<div class="progress-bar" role="progressbar" aria-valuenow={position.progressToTarget1} aria-valuemin={0} aria-valuemax={100}>
				<div class="progress-fill {progressBarColor}" style="width: {position.progressToTarget1}%"></div>
			</div>
			<span class="progress-label">{position.progressToTarget1.toFixed(0)}% to T1</span>
		</div>
	{/if}
</article>

<style>
	.position-card {
		background: #fff;
		border: 1px solid #e2e8f0;
		border-radius: 12px;
		padding: 20px;
		min-width: 320px;
		max-width: 400px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
		transition: box-shadow 0.2s ease-out;
	}

	.position-card:hover {
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
	}

	.position-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 12px;
	}

	.position-ticker {
		font-size: 20px;
		font-weight: 700;
		color: #0f172a;
		margin: 0;
		text-transform: uppercase;
		letter-spacing: 0.025em;
	}

	.position-status {
		font-size: 11px;
		font-weight: 600;
		padding: 4px 10px;
		border-radius: 6px;
		border: 1px solid;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.position-divider {
		height: 1px;
		background: #e2e8f0;
		margin-bottom: 12px;
	}

	.position-prices {
		margin-bottom: 8px;
	}

	.price-row {
		display: flex;
		align-items: center;
		gap: 6px;
		flex-wrap: wrap;
	}

	.price-label {
		font-size: 13px;
		color: #64748b;
	}

	.price-value {
		font-size: 14px;
		font-weight: 600;
		color: #0f172a;
		font-variant-numeric: tabular-nums;
	}

	.price-separator {
		color: #cbd5e1;
		margin: 0 4px;
	}

	.unrealized-row {
		margin-bottom: 12px;
	}

	.unrealized-value {
		font-size: 20px;
		font-weight: 700;
		font-variant-numeric: tabular-nums;
	}

	.position-notes {
		font-size: 13px;
		color: #64748b;
		font-style: italic;
		margin: 0 0 12px 0;
		line-height: 1.5;
	}

	.targets-section {
		display: flex;
		flex-direction: column;
		gap: 6px;
		margin-bottom: 16px;
	}

	.target-row {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.target-label {
		font-size: 13px;
		color: #64748b;
		min-width: 60px;
	}

	.target-value {
		font-size: 13px;
		font-weight: 500;
		color: #334155;
		font-variant-numeric: tabular-nums;
	}

	.stop-row .target-label {
		color: #dc2626;
	}

	.stop-value {
		color: #dc2626;
	}

	.progress-section {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.progress-bar {
		flex: 1;
		height: 6px;
		background: #e2e8f0;
		border-radius: 3px;
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		border-radius: 3px;
		transition: width 0.5s ease-out;
	}

	.progress-label {
		font-size: 12px;
		color: #64748b;
		font-variant-numeric: tabular-nums;
		white-space: nowrap;
	}
</style>
