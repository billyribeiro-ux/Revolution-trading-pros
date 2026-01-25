<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * ActivePositionCard Component - COMPACT Trading Position Display
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @version 7.0.0 - High Density Refactor: 70px height, single-line data
	 * @standards Apple Principal Engineer ICT 7+ | WCAG 2.1 AA
	 */
	import type { ActivePosition } from '../types';
	import { formatPercent, formatPrice } from '../utils/formatters';

	interface Props {
		position: ActivePosition;
		isAdmin?: boolean;
		onClose?: (position: ActivePosition) => void;
	}

	const { position, isAdmin = false, onClose }: Props = $props();

	function handleClosePosition(e: MouseEvent) {
		e.stopPropagation();
		onClose?.(position);
	}

	// Compact status config
	const statusConfig = $derived.by(() => {
		switch (position.status) {
			case 'ENTRY':
				return { label: 'ENTRY', border: 'var(--color-entry)', bg: 'var(--color-entry-bg)' };
			case 'WATCHING':
				return { label: 'WATCH', border: 'var(--color-watching)', bg: 'var(--color-watching-bg)' };
			case 'ACTIVE':
				const isProfit = position.unrealizedPercent !== null && position.unrealizedPercent >= 0;
				return isProfit
					? { label: 'ACTIVE', border: 'var(--color-profit)', bg: 'var(--color-profit-bg-subtle)' }
					: { label: 'ACTIVE', border: 'var(--color-loss)', bg: 'var(--color-loss-bg-subtle)' };
			default:
				return { label: position.status, border: 'var(--color-border-strong)', bg: 'var(--color-bg-card)' };
		}
	});

	const pnlClass = $derived(
		position.unrealizedPercent === null ? '' : position.unrealizedPercent >= 0 ? 'profit' : 'loss'
	);

	const progressGradient = $derived(
		position.unrealizedPercent === null || position.unrealizedPercent < 0
			? 'linear-gradient(to right, var(--color-loss-light), var(--color-loss))'
			: 'linear-gradient(to right, var(--color-profit-light), var(--color-profit))'
	);

	// Format compact price line
	const pricesLine = $derived.by(() => {
		if (position.status === 'WATCHING' && position.entryZone) {
			return `Zone:${formatPrice(position.entryZone.low)}-${formatPrice(position.entryZone.high)} | Now:${formatPrice(position.currentPrice)}`;
		}
		const parts = [`E:${formatPrice(position.entryPrice ?? 0)} → ${formatPrice(position.currentPrice)}`];
		if (position.stopLoss) parts.push(`S:${formatPrice(position.stopLoss.price)}`);
		if (position.targets[0]) parts.push(`T1:${formatPrice(position.targets[0].price)}`);
		if (position.targets[1]) parts.push(`T2:${formatPrice(position.targets[1].price)}`);
		return parts.join(' | ');
	});
</script>

<article 
	class="card"
	style="--border: {statusConfig.border}; --bg: {statusConfig.bg};"
	aria-label="{position.ticker} {position.status} position"
>
	<!-- Main Row: Ticker + Status + P&L -->
	<div class="row-main">
		<span class="ticker">{position.ticker}</span>
		<span class="status">{statusConfig.label}</span>
		{#if position.unrealizedPercent !== null}
			<span class="pnl {pnlClass}">{formatPercent(position.unrealizedPercent)}</span>
		{/if}
	</div>

	<!-- Prices Row: All on one line -->
	<div class="row-prices">{pricesLine}</div>

	<!-- Progress Row -->
	{#if position.status !== 'WATCHING' && position.targets.length > 0}
		<div class="row-progress">
			<div 
				class="bar"
				role="progressbar" 
				aria-valuenow={position.progressToTarget1} 
				aria-valuemin={0} 
				aria-valuemax={100}
				aria-label="{position.progressToTarget1.toFixed(0)}% to target 1"
			>
				<div class="fill" style="background: {progressGradient}; width: {Math.min(100, Math.max(0, position.progressToTarget1))}%;"></div>
			</div>
			<span class="pct">{position.progressToTarget1.toFixed(0)}%</span>
		</div>
	{/if}

	<!-- Admin Close (hover only) -->
	{#if isAdmin && onClose && position.status === 'ACTIVE'}
		<button 
			type="button"
			class="close-btn"
			onclick={handleClosePosition}
			aria-label="Close {position.ticker}"
			title="Close position"
		>
			<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="12" height="12">
				<path d="M18 6L6 18M6 6l12 12" />
			</svg>
		</button>
	{/if}
</article>

<style>
	/* COMPACT CARD - High Information Density */
	.card {
		position: relative;
		background: var(--bg);
		border: 1px solid var(--color-border-default);
		border-left: 3px solid var(--border);
		border-radius: var(--radius-md);
		padding: 8px 10px;
		box-shadow: var(--shadow-sm);
		transition: var(--transition-shadow);
	}

	.card:hover {
		box-shadow: var(--shadow-md);
	}

	/* Main Row: Ticker + Status + P&L */
	.row-main {
		display: flex;
		align-items: center;
		gap: 8px;
		margin-bottom: 4px;
	}

	.ticker {
		font-size: 14px;
		font-weight: var(--font-bold);
		color: var(--color-text-primary);
		text-transform: uppercase;
		letter-spacing: 0.02em;
		font-family: var(--font-display);
	}

	.status {
		font-size: 9px;
		font-weight: var(--font-bold);
		padding: 2px 6px;
		border-radius: var(--radius-sm);
		text-transform: uppercase;
		letter-spacing: 0.03em;
		background: var(--color-bg-muted);
		color: var(--color-text-secondary);
	}

	.pnl {
		margin-left: auto;
		font-size: 13px;
		font-weight: var(--font-bold);
		font-variant-numeric: tabular-nums;
	}

	.pnl.profit {
		color: var(--color-profit);
	}

	.pnl.loss {
		color: var(--color-loss);
	}

	/* Prices Row: Single line */
	.row-prices {
		font-size: 11px;
		color: var(--color-text-secondary);
		font-variant-numeric: tabular-nums;
		margin-bottom: 4px;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	/* Progress Row */
	.row-progress {
		display: flex;
		align-items: center;
		gap: 6px;
	}

	.bar {
		flex: 1;
		height: 4px;
		background: var(--color-bg-muted);
		border-radius: 2px;
		overflow: hidden;
	}

	.fill {
		height: 100%;
		border-radius: 2px;
		transition: width var(--duration-normal) var(--ease-out);
	}

	.pct {
		font-size: 10px;
		color: var(--color-text-tertiary);
		font-weight: var(--font-semibold);
		font-variant-numeric: tabular-nums;
		min-width: 32px;
		text-align: right;
	}

	/* Admin Close Button - Hover Only */
	.close-btn {
		position: absolute;
		top: 6px;
		right: 6px;
		width: 20px;
		height: 20px;
		padding: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--color-bg-card);
		border: 1px solid var(--color-loss);
		color: var(--color-loss);
		border-radius: var(--radius-sm);
		cursor: pointer;
		opacity: 0;
		transition: var(--transition-colors), opacity var(--duration-fast) var(--ease-out);
	}

	.card:hover .close-btn {
		opacity: 1;
	}

	.close-btn:hover {
		background: var(--color-loss);
		color: white;
	}

	.close-btn:focus-visible {
		opacity: 1;
		outline: 2px solid var(--color-loss);
		outline-offset: 2px;
	}

	/* Responsive */
	@media (max-width: 640px) {
		.card {
			padding: 6px 8px;
		}

		.ticker {
			font-size: 13px;
		}

		.pnl {
			font-size: 12px;
		}

		.row-prices {
			font-size: 10px;
		}
	}
</style>