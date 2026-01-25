<script lang="ts">
	/**
	 * ActivePositionCard - Compact Trading Dashboard Card
	 * @version 8.0.0 - Professional admin controls with dropdown menu
	 */
	import type { ActivePosition } from '../types';
	import { formatPercent, formatPrice } from '../utils/formatters';

	interface Props {
		position: ActivePosition;
		isAdmin?: boolean;
		onClose?: (position: ActivePosition) => void;
		onUpdate?: (position: ActivePosition) => void;
	}

	const { position, isAdmin = false, onClose, onUpdate }: Props = $props();

	let menuOpen = $state(false);

	const isProfit = $derived(position.unrealizedPercent !== null && position.unrealizedPercent >= 0);
	const statusClass = $derived(
		position.status === 'WATCHING' ? 'watching' : 
		position.status === 'ENTRY' ? 'entry' : 
		isProfit ? 'profit' : 'loss'
	);

	function toggleMenu(e: MouseEvent) {
		e.stopPropagation();
		menuOpen = !menuOpen;
	}

	function handleClose(e: MouseEvent) {
		e.stopPropagation();
		menuOpen = false;
		onClose?.(position);
	}

	function handleUpdate(e: MouseEvent) {
		e.stopPropagation();
		menuOpen = false;
		onUpdate?.(position);
	}

	function handleClickOutside(e: MouseEvent) {
		if (menuOpen) {
			menuOpen = false;
		}
	}
</script>

<svelte:window onclick={handleClickOutside} />

<div class="card {statusClass}">
	<!-- Row 1: Ticker + Status + P&L -->
	<div class="row-main">
		<span class="ticker">{position.ticker}</span>
		<span class="status">{position.status}</span>
		<span class="pnl" class:profit={isProfit} class:loss={!isProfit && position.unrealizedPercent !== null}>
			{position.unrealizedPercent !== null ? formatPercent(position.unrealizedPercent) : '—'}
		</span>
	</div>

	<!-- Row 2: Entry → Current | Stop | T1 -->
	<div class="row-prices">
		{#if position.entryPrice}
			<span>E:{formatPrice(position.entryPrice)} → {formatPrice(position.currentPrice)}</span>
		{:else if position.entryZone}
			<span>Zone:{formatPrice(position.entryZone.low)}-{formatPrice(position.entryZone.high)}</span>
		{/if}
		<span class="sep">|</span>
		<span class="stop">S:{formatPrice(position.stopLoss.price)}</span>
		{#if position.targets[0]}
			<span class="sep">|</span>
			<span class="target">T1:{formatPrice(position.targets[0].price)}</span>
		{/if}
	</div>

	<!-- Row 3: Progress bar -->
	{#if position.status !== 'WATCHING' && position.targets.length > 0}
		<div class="row-progress">
			<div class="bar">
				<div class="fill" class:profit={isProfit} style="width:{Math.min(100, position.progressToTarget1)}%"></div>
			</div>
			<span class="pct">{position.progressToTarget1.toFixed(0)}%</span>
		</div>
	{/if}

	<!-- Admin Menu -->
	{#if isAdmin && (onClose || onUpdate)}
		<div class="menu-container">
			<button 
				type="button" 
				class="menu-trigger"
				onclick={toggleMenu}
				aria-label="Position actions"
				aria-expanded={menuOpen}
				aria-haspopup="menu"
			>
				<svg viewBox="0 0 24 24" fill="currentColor" width="18" height="18">
					<circle cx="12" cy="5" r="2"/>
					<circle cx="12" cy="12" r="2"/>
					<circle cx="12" cy="19" r="2"/>
				</svg>
			</button>

			{#if menuOpen}
				<div class="menu-dropdown" role="menu">
					{#if onUpdate}
						<button 
							type="button" 
							class="menu-item"
							onclick={handleUpdate}
							role="menuitem"
						>
							<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="16" height="16">
								<path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/>
								<path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/>
							</svg>
							Update Position
						</button>
					{/if}
					{#if onClose && position.status === 'ACTIVE'}
						<button 
							type="button" 
							class="menu-item danger"
							onclick={handleClose}
							role="menuitem"
						>
							<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="16" height="16">
								<circle cx="12" cy="12" r="10"/>
								<path d="M15 9l-6 6M9 9l6 6"/>
							</svg>
							Close Position
						</button>
					{/if}
				</div>
			{/if}
		</div>
	{/if}
</div>

<style>
	.card {
		position: relative;
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-default);
		border-left: 3px solid var(--color-border-strong);
		border-radius: 8px;
		padding: 12px 14px;
		font-size: 13px;
	}
	.card.profit { border-left-color: var(--color-profit); }
	.card.loss { border-left-color: var(--color-loss); }
	.card.watching { border-left-color: var(--color-watching); }
	.card.entry { border-left-color: var(--color-entry); }

	/* Row 1 */
	.row-main {
		display: flex;
		align-items: center;
		gap: 8px;
		margin-bottom: 6px;
		padding-right: 28px; /* Space for menu trigger */
	}
	.ticker {
		font-weight: 700;
		font-size: 15px;
		color: var(--color-text-primary);
	}
	.status {
		font-size: 10px;
		font-weight: 600;
		text-transform: uppercase;
		padding: 2px 6px;
		border-radius: 4px;
		background: var(--color-bg-subtle);
		color: var(--color-text-tertiary);
	}
	.pnl {
		margin-left: auto;
		font-weight: 700;
		font-size: 14px;
		font-variant-numeric: tabular-nums;
		color: var(--color-text-muted);
	}
	.pnl.profit { color: var(--color-profit); }
	.pnl.loss { color: var(--color-loss); }

	/* Row 2 */
	.row-prices {
		display: flex;
		align-items: center;
		gap: 6px;
		font-size: 12px;
		color: var(--color-text-secondary);
		font-variant-numeric: tabular-nums;
		margin-bottom: 2px;
	}
	.sep { color: var(--color-border-strong); }
	.stop { color: var(--color-loss); }
	.target { color: var(--color-profit); }

	/* Row 3 */
	.row-progress {
		display: flex;
		align-items: center;
		gap: 8px;
		margin-top: 8px;
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
		background: var(--color-loss);
		border-radius: 2px;
	}
	.fill.profit { background: var(--color-profit); }
	.pct {
		font-size: 11px;
		font-weight: 600;
		color: var(--color-text-muted);
		min-width: 28px;
		text-align: right;
	}

	/* Menu */
	.menu-container {
		position: absolute;
		top: 10px;
		right: 10px;
	}

	.menu-trigger {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		padding: 0;
		border: none;
		background: transparent;
		color: var(--color-text-muted);
		cursor: pointer;
		border-radius: 6px;
		transition: background 0.15s, color 0.15s;
	}
	.menu-trigger:hover {
		background: var(--color-bg-subtle);
		color: var(--color-text-secondary);
	}
	.menu-trigger:focus-visible {
		outline: 2px solid var(--color-brand-primary);
		outline-offset: 2px;
	}

	.menu-dropdown {
		position: absolute;
		top: calc(100% + 4px);
		right: 0;
		min-width: 160px;
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-default);
		border-radius: 8px;
		box-shadow: var(--shadow-lg);
		padding: 4px;
		z-index: 50;
		animation: menuFadeIn 0.15s ease-out;
	}

	@keyframes menuFadeIn {
		from {
			opacity: 0;
			transform: translateY(-4px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.menu-item {
		display: flex;
		align-items: center;
		gap: 10px;
		width: 100%;
		padding: 10px 12px;
		border: none;
		background: transparent;
		color: var(--color-text-secondary);
		font-size: 13px;
		font-weight: 500;
		text-align: left;
		cursor: pointer;
		border-radius: 6px;
		transition: background 0.15s, color 0.15s;
	}
	.menu-item:hover {
		background: var(--color-bg-subtle);
		color: var(--color-text-primary);
	}
	.menu-item:focus-visible {
		outline: 2px solid var(--color-brand-primary);
		outline-offset: -2px;
	}
	.menu-item.danger {
		color: var(--color-loss);
	}
	.menu-item.danger:hover {
		background: var(--color-loss-bg);
		color: var(--color-loss);
	}
</style>