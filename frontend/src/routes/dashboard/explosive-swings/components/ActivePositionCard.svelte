<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * ActivePositionCard Component - Active Position Display
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Displays an active position with entry, targets, stop, and progress
	 * @version 6.0.0 - Nuclear Refactor: Design Tokens + Accessibility
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

	// Status configuration using design tokens
	const statusConfig = $derived.by(() => {
		switch (position.status) {
			case 'ENTRY':
				return {
					label: 'ENTRY',
					borderColor: 'var(--color-entry)',
					bgColor: 'var(--color-entry-bg)',
					badgeBg: 'var(--color-entry-bg)',
					badgeText: 'var(--color-entry-text)',
					badgeBorder: 'var(--color-entry-border)'
				};
			case 'WATCHING':
				return {
					label: 'WATCHING',
					borderColor: 'var(--color-watching)',
					bgColor: 'var(--color-watching-bg)',
					badgeBg: 'var(--color-watching-bg)',
					badgeText: 'var(--color-watching-text)',
					badgeBorder: 'var(--color-watching-border)'
				};
			case 'ACTIVE':
				const isProfit = position.unrealizedPercent !== null && position.unrealizedPercent >= 0;
				return isProfit
					? {
							label: 'ACTIVE',
							borderColor: 'var(--color-profit)',
							bgColor: 'var(--color-profit-bg-subtle)',
							badgeBg: 'var(--color-profit-bg)',
							badgeText: 'var(--color-profit)',
							badgeBorder: 'var(--color-profit-border)'
						}
					: {
							label: 'ACTIVE',
							borderColor: 'var(--color-loss)',
							bgColor: 'var(--color-loss-bg-subtle)',
							badgeBg: 'var(--color-loss-bg)',
							badgeText: 'var(--color-loss)',
							badgeBorder: 'var(--color-loss-border)'
						};
			default:
				return {
					label: position.status,
					borderColor: 'var(--color-border-strong)',
					bgColor: 'var(--color-bg-card)',
					badgeBg: 'var(--color-bg-subtle)',
					badgeText: 'var(--color-text-secondary)',
					badgeBorder: 'var(--color-border-default)'
				};
		}
	});

	const pnlColorVar = $derived(
		position.unrealizedPercent === null
			? 'var(--color-text-muted)'
			: position.unrealizedPercent >= 0
				? 'var(--color-profit)'
				: 'var(--color-loss)'
	);

	const progressGradient = $derived(
		position.unrealizedPercent === null || position.unrealizedPercent < 0
			? 'linear-gradient(to right, var(--color-loss-light), var(--color-loss))'
			: 'linear-gradient(to right, var(--color-profit-light), var(--color-profit))'
	);
</script>

<!-- SEMANTIC HTML: <article> for grouping, explicit <button> for close action -->
<article 
	class="position-card"
	style="
		--card-border-color: {statusConfig.borderColor};
		--card-bg-color: {statusConfig.bgColor};
	"
	aria-labelledby="position-{position.id}-title"
>
	<!-- Header Row -->
	<header class="card-header">
		<h3 id="position-{position.id}-title" class="ticker">{position.ticker}</h3>
		<span 
			class="status-badge"
			style="
				background: {statusConfig.badgeBg};
				color: {statusConfig.badgeText};
				border-color: {statusConfig.badgeBorder};
			"
		>
			{statusConfig.label}
		</span>
	</header>

	<div class="divider" aria-hidden="true"></div>

	<!-- Price Information -->
	<div class="price-section">
		{#if position.status === 'WATCHING' && position.entryZone}
			<dl class="price-grid">
				<div class="price-item">
					<dt>Entry Zone</dt>
					<dd>{formatPrice(position.entryZone.low)} – {formatPrice(position.entryZone.high)}</dd>
				</div>
				<div class="price-item">
					<dt>Current</dt>
					<dd>{formatPrice(position.currentPrice)}</dd>
				</div>
			</dl>
		{:else if position.entryPrice}
			<dl class="price-grid inline">
				<div class="price-item">
					<dt>Entry</dt>
					<dd>{formatPrice(position.entryPrice)}</dd>
				</div>
				<div class="price-item">
					<dt>Now</dt>
					<dd>{formatPrice(position.currentPrice)}</dd>
				</div>
			</dl>
		{/if}
	</div>

	<!-- Unrealized P&L -->
	{#if position.unrealizedPercent !== null}
		<div class="unrealized-pnl">
			<span class="unrealized-value" style="color: {pnlColorVar}">
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
	<dl class="targets-list">
		{#each position.targets as target}
			<div class="target-row">
				<dt>{target.label}</dt>
				<dd>
					<span class="target-price">{formatPrice(target.price)}</span>
					<span class="target-percent profit">{formatPercent(target.percentFromEntry)}</span>
				</dd>
			</div>
		{/each}
		<div class="target-row stop-row">
			<dt>Stop</dt>
			<dd>
				<span class="target-price stop">{formatPrice(position.stopLoss.price)}</span>
				<span class="target-percent loss">{formatPercent(position.stopLoss.percentFromEntry)}</span>
			</dd>
		</div>
	</dl>

	<!-- Progress Bar -->
	{#if position.status !== 'WATCHING' && position.targets.length > 0}
		<div class="progress-container">
			<div 
				class="progress-track"
				role="progressbar" 
				aria-valuenow={position.progressToTarget1} 
				aria-valuemin={0} 
				aria-valuemax={100}
				aria-label="Progress to Target 1: {position.progressToTarget1.toFixed(0)}%"
			>
				<div 
					class="progress-fill" 
					style="
						background: {progressGradient}; 
						width: {Math.min(100, Math.max(0, position.progressToTarget1))}%;
					"
				></div>
			</div>
			<span class="progress-text" aria-hidden="true">
				{position.progressToTarget1.toFixed(0)}% to T1
			</span>
		</div>
	{/if}

	<!-- Admin Close Button - EXPLICIT, not hidden behind card click -->
	{#if isAdmin && onClose && position.status === 'ACTIVE'}
		<div class="card-actions">
			<button 
				type="button"
				class="close-position-btn"
				onclick={handleClosePosition}
				aria-label="Close {position.ticker} position"
			>
				<svg 
					viewBox="0 0 24 24" 
					fill="none" 
					stroke="currentColor" 
					stroke-width="2" 
					width="16" 
					height="16"
					aria-hidden="true"
				>
					<path d="M18 6L6 18M6 6l12 12" />
				</svg>
				Close Position
			</button>
		</div>
	{/if}
</article>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   POSITION CARD - Design Token Implementation
	   ═══════════════════════════════════════════════════════════════════════════ */
	.position-card {
		background: var(--card-bg-color, var(--color-bg-card));
		border: 1px solid var(--color-border-default);
		border-left: 4px solid var(--card-border-color, var(--color-border-strong));
		border-radius: var(--radius-lg);
		padding: var(--space-5);
		box-shadow: var(--shadow-sm);
		transition: var(--transition-shadow), var(--transition-transform);
		contain: layout style;
	}

	.position-card:hover {
		box-shadow: var(--shadow-md);
		transform: translateY(-2px);
	}

	/* Header */
	.card-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: var(--space-3);
	}

	.ticker {
		font-size: var(--text-xl);
		font-weight: var(--font-extrabold);
		color: var(--color-text-primary);
		margin: 0;
		text-transform: uppercase;
		letter-spacing: var(--tracking-wide);
		font-family: var(--font-display);
	}

	.status-badge {
		font-size: var(--text-xs);
		font-weight: var(--font-bold);
		padding: var(--space-1) var(--space-2);
		border-radius: var(--radius-sm);
		text-transform: uppercase;
		letter-spacing: var(--tracking-wider);
		border: 1px solid;
	}

	.divider {
		height: 1px;
		background: linear-gradient(90deg, var(--color-border-default) 0%, transparent 100%);
		margin-bottom: var(--space-3);
	}

	/* Price Grid - using <dl> for semantic markup */
	.price-section {
		margin-bottom: var(--space-2);
	}

	.price-grid {
		display: flex;
		gap: var(--space-4);
		margin: 0 0 var(--space-2) 0;
	}

	.price-grid.inline {
		flex-wrap: wrap;
	}

	.price-item {
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
	}

	.price-item dt {
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
		font-weight: var(--font-medium);
	}

	.price-item dd {
		font-size: var(--text-base);
		font-weight: var(--font-bold);
		color: var(--color-text-primary);
		font-variant-numeric: tabular-nums;
		margin: 0;
	}

	/* Unrealized P&L */
	.unrealized-pnl {
		display: flex;
		align-items: baseline;
		gap: var(--space-2);
		margin-bottom: var(--space-3);
	}

	.unrealized-value {
		font-size: var(--text-2xl);
		font-weight: var(--font-extrabold);
		font-variant-numeric: tabular-nums;
		line-height: var(--leading-none);
	}

	.unrealized-label {
		font-size: var(--text-sm);
		color: var(--color-text-muted);
		font-weight: var(--font-medium);
	}

	/* Notes */
	.position-notes {
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
		font-style: italic;
		margin: 0 0 var(--space-3) 0;
		line-height: var(--leading-normal);
		padding: var(--space-2) var(--space-3);
		background: var(--color-bg-subtle);
		border-radius: var(--radius-md);
		border-left: 3px solid var(--color-border-strong);
	}

	/* Targets List - using <dl> for semantic markup */
	.targets-list {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
		margin: 0 0 var(--space-4) 0;
	}

	.target-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.target-row dt {
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		font-weight: var(--font-medium);
		min-width: 60px;
	}

	.target-row dd {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		margin: 0;
	}

	.target-price {
		font-size: var(--text-sm);
		font-weight: var(--font-semibold);
		color: var(--color-text-secondary);
		font-variant-numeric: tabular-nums;
	}

	.target-percent {
		font-size: var(--text-xs);
		font-weight: var(--font-semibold);
		font-variant-numeric: tabular-nums;
	}

	.target-percent.profit {
		color: var(--color-profit);
	}

	.target-percent.loss {
		color: var(--color-loss);
	}

	.stop-row dt {
		color: var(--color-loss);
	}

	.target-price.stop {
		color: var(--color-loss);
	}

	/* Progress Bar */
	.progress-container {
		display: flex;
		align-items: center;
		gap: var(--space-3);
	}

	.progress-track {
		flex: 1;
		height: 10px;
		background: var(--color-bg-muted);
		border-radius: var(--radius-sm);
		overflow: hidden;
		box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.06);
	}

	.progress-fill {
		height: 100%;
		border-radius: var(--radius-sm);
		transition: width var(--duration-slow) var(--ease-out);
		position: relative;
	}

	.progress-fill::after {
		content: '';
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		height: 50%;
		background: linear-gradient(to bottom, rgba(255, 255, 255, 0.25), transparent);
		border-radius: var(--radius-sm) var(--radius-sm) 0 0;
	}

	.progress-text {
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		font-weight: var(--font-semibold);
		font-variant-numeric: tabular-nums;
		white-space: nowrap;
		min-width: 70px;
		text-align: right;
	}

	/* Admin Actions */
	.card-actions {
		margin-top: var(--space-4);
		padding-top: var(--space-3);
		border-top: 1px solid var(--color-border-subtle);
	}

	.close-position-btn {
		display: inline-flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-2) var(--space-3);
		background: transparent;
		border: 1px solid var(--color-loss);
		color: var(--color-loss);
		border-radius: var(--radius-md);
		font-size: var(--text-sm);
		font-weight: var(--font-semibold);
		cursor: pointer;
		transition: var(--transition-colors);
	}

	.close-position-btn:hover {
		background: var(--color-loss);
		color: white;
	}

	.close-position-btn:focus-visible {
		outline: 2px solid var(--color-loss);
		outline-offset: 2px;
	}

	/* Responsive */
	@media (max-width: 640px) {
		.position-card {
			padding: var(--space-4);
		}

		.ticker {
			font-size: var(--text-lg);
		}

		.unrealized-value {
			font-size: var(--text-xl);
		}
	}
</style>