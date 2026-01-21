<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * ActivePositionCard Component - Active Position Display
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Displays an active position with entry, targets, stop, and progress
	 * @version 4.1.0 - Visual Polish Pass
	 * @standards Apple Principal Engineer ICT 7+ Standards
	 */
	import type { ActivePosition } from '../types';
	import { formatPercent, formatPrice } from '../utils/formatters';

	interface Props {
		position: ActivePosition;
	}

	const { position }: Props = $props();

	// Determine card accent based on status and P&L
	const statusConfig = $derived(() => {
		switch (position.status) {
			case 'ENTRY':
				return {
					label: 'ENTRY',
					borderColor: 'border-l-teal-500',
					badgeClass: 'badge-entry',
					bgTint: 'bg-teal-50/30'
				};
			case 'WATCHING':
				return {
					label: 'WATCHING',
					borderColor: 'border-l-amber-500',
					badgeClass: 'badge-watching',
					bgTint: 'bg-amber-50/30'
				};
			case 'ACTIVE':
				if (position.unrealizedPercent !== null && position.unrealizedPercent >= 0) {
					return {
						label: 'ACTIVE',
						borderColor: 'border-l-emerald-500',
						badgeClass: 'badge-active-profit',
						bgTint: 'bg-emerald-50/30'
					};
				} else {
					return {
						label: 'ACTIVE',
						borderColor: 'border-l-red-500',
						badgeClass: 'badge-active-loss',
						bgTint: 'bg-red-50/30'
					};
				}
			default:
				return {
					label: position.status,
					borderColor: 'border-l-slate-500',
					badgeClass: 'badge-default',
					bgTint: ''
				};
		}
	});

	const unrealizedColorClass = $derived(
		position.unrealizedPercent === null
			? 'text-slate-500'
			: position.unrealizedPercent >= 0
				? 'text-emerald-600'
				: 'text-red-600'
	);

	const progressBarBg = $derived(
		position.unrealizedPercent === null
			? 'bg-slate-300'
			: position.unrealizedPercent >= 0
				? 'bg-gradient-to-r from-emerald-400 to-emerald-500'
				: 'bg-gradient-to-r from-red-400 to-red-500'
	);
</script>

<article 
	class="position-card {statusConfig().borderColor} {statusConfig().bgTint}"
	aria-label="{position.ticker} position"
>
	<!-- Header Row -->
	<div class="card-header">
		<h3 class="ticker">{position.ticker}</h3>
		<span class="status-badge {statusConfig().badgeClass}">
			{statusConfig().label}
		</span>
	</div>

	<div class="divider"></div>

	<!-- Price Information -->
	<div class="price-section">
		{#if position.status === 'WATCHING' && position.entryZone}
			<div class="price-row">
				<span class="price-label">Entry Zone:</span>
				<span class="price-value">{formatPrice(position.entryZone.low)} - {formatPrice(position.entryZone.high)}</span>
			</div>
			<div class="price-row">
				<span class="price-label">Current:</span>
				<span class="price-value">{formatPrice(position.currentPrice)}</span>
			</div>
		{:else if position.entryPrice}
			<div class="price-row inline">
				<span class="price-label">Entry:</span>
				<span class="price-value">{formatPrice(position.entryPrice)}</span>
				<span class="separator">|</span>
				<span class="price-label">Now:</span>
				<span class="price-value">{formatPrice(position.currentPrice)}</span>
			</div>
		{/if}
	</div>

	<!-- Unrealized P&L -->
	{#if position.unrealizedPercent !== null}
		<div class="unrealized-pnl">
			<span class="unrealized-value {unrealizedColorClass}">
				{formatPercent(position.unrealizedPercent)}
			</span>
			<span class="unrealized-label">unrealized</span>
		</div>
	{/if}

	<!-- Notes -->
	{#if position.notes}
		<p class="position-notes">{position.notes}</p>
	{/if}

	<!-- Targets and Stop -->
	<div class="targets-container">
		{#each position.targets as target}
			<div class="target-row">
				<span class="target-label">{target.label}:</span>
				<span class="target-price">{formatPrice(target.price)}</span>
				<span class="target-percent positive">({formatPercent(target.percentFromEntry)})</span>
			</div>
		{/each}
		<div class="target-row stop">
			<span class="target-label">Stop:</span>
			<span class="target-price stop-price">{formatPrice(position.stopLoss.price)}</span>
			<span class="target-percent negative">({formatPercent(position.stopLoss.percentFromEntry)})</span>
		</div>
	</div>

	<!-- Progress Bar -->
	{#if position.status !== 'WATCHING' && position.targets.length > 0}
		<div class="progress-container">
			<div 
				class="progress-track"
				role="progressbar" 
				aria-valuenow={position.progressToTarget1} 
				aria-valuemin={0} 
				aria-valuemax={100}
				aria-label="Progress to Target 1"
			>
				<div 
					class="progress-fill {progressBarBg}" 
					style="width: {Math.min(100, Math.max(0, position.progressToTarget1))}%"
				></div>
			</div>
			<span class="progress-text">{position.progressToTarget1.toFixed(0)}% to T1</span>
		</div>
	{/if}
</article>

<style>
	.position-card {
		background: #ffffff;
		border: 1px solid #e2e8f0;
		border-left-width: 4px;
		border-radius: 12px;
		padding: 20px;
		min-width: 320px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.04), 0 1px 2px rgba(0, 0, 0, 0.06);
		transition: all 0.2s ease-out;
	}

	.position-card:hover {
		box-shadow: 0 10px 25px rgba(0, 0, 0, 0.08), 0 4px 10px rgba(0, 0, 0, 0.04);
		transform: translateY(-2px);
	}

	/* Header */
	.card-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 12px;
	}

	.ticker {
		font-size: 22px;
		font-weight: 800;
		color: #0f172a;
		margin: 0;
		text-transform: uppercase;
		letter-spacing: 0.02em;
	}

	/* Status Badges */
	.status-badge {
		font-size: 10px;
		font-weight: 700;
		padding: 5px 10px;
		border-radius: 6px;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		border: 1px solid;
	}

	.badge-entry {
		background: linear-gradient(135deg, #ccfbf1 0%, #99f6e4 100%);
		color: #0f766e;
		border-color: #5eead4;
	}

	.badge-watching {
		background: linear-gradient(135deg, #fef3c7 0%, #fde68a 100%);
		color: #92400e;
		border-color: #fcd34d;
	}

	.badge-active-profit {
		background: linear-gradient(135deg, #dcfce7 0%, #bbf7d0 100%);
		color: #166534;
		border-color: #86efac;
	}

	.badge-active-loss {
		background: linear-gradient(135deg, #fee2e2 0%, #fecaca 100%);
		color: #991b1b;
		border-color: #fca5a5;
	}

	.badge-default {
		background: #f1f5f9;
		color: #475569;
		border-color: #cbd5e1;
	}

	/* Divider */
	.divider {
		height: 1px;
		background: linear-gradient(90deg, #e2e8f0 0%, transparent 100%);
		margin-bottom: 14px;
	}

	/* Price Section */
	.price-section {
		margin-bottom: 8px;
	}

	.price-row {
		display: flex;
		align-items: center;
		gap: 6px;
		margin-bottom: 4px;
	}

	.price-row.inline {
		flex-wrap: wrap;
	}

	.price-label {
		font-size: 13px;
		color: #64748b;
		font-weight: 500;
	}

	.price-value {
		font-size: 14px;
		font-weight: 700;
		color: #0f172a;
		font-variant-numeric: tabular-nums;
	}

	.separator {
		color: #cbd5e1;
		margin: 0 4px;
		font-weight: 300;
	}

	/* Unrealized P&L */
	.unrealized-pnl {
		display: flex;
		align-items: baseline;
		gap: 6px;
		margin-bottom: 12px;
	}

	.unrealized-value {
		font-size: 24px;
		font-weight: 800;
		font-variant-numeric: tabular-nums;
		line-height: 1;
	}

	.unrealized-label {
		font-size: 13px;
		color: #94a3b8;
		font-weight: 500;
	}

	/* Notes */
	.position-notes {
		font-size: 13px;
		color: #64748b;
		font-style: italic;
		margin: 0 0 14px 0;
		line-height: 1.5;
		padding: 10px 12px;
		background: #f8fafc;
		border-radius: 8px;
		border-left: 3px solid #cbd5e1;
	}

	/* Targets */
	.targets-container {
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
		font-size: 12px;
		color: #64748b;
		min-width: 58px;
		font-weight: 500;
	}

	.target-price {
		font-size: 13px;
		font-weight: 600;
		color: #334155;
		font-variant-numeric: tabular-nums;
	}

	.target-percent {
		font-size: 12px;
		font-weight: 600;
		font-variant-numeric: tabular-nums;
	}

	.target-percent.positive {
		color: #059669;
	}

	.target-percent.negative {
		color: #dc2626;
	}

	.target-row.stop .target-label {
		color: #b91c1c;
	}

	.stop-price {
		color: #dc2626;
	}

	/* Progress Bar */
	.progress-container {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.progress-track {
		flex: 1;
		height: 8px;
		background: #e2e8f0;
		border-radius: 4px;
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		border-radius: 4px;
		transition: width 0.6s cubic-bezier(0.4, 0, 0.2, 1);
	}

	.progress-text {
		font-size: 12px;
		color: #64748b;
		font-weight: 600;
		font-variant-numeric: tabular-nums;
		white-space: nowrap;
		min-width: 70px;
		text-align: right;
	}
</style>