<script lang="ts">
	/**
	 * ActivePositionCard - Compact Trading Dashboard Card
	 * @version 9.0.0 - Full admin menu (Update, Invalidate, Close, Delete)
	 */
	import type { ActivePosition } from '../types';
	import { formatPercent, formatPrice } from '../utils/formatters';
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';

	interface Props {
		position: ActivePosition;
		isAdmin?: boolean;
		onUpdate?: (position: ActivePosition) => void;
		onInvalidate?: (position: ActivePosition) => void;
		onClose?: (position: ActivePosition) => void;
		onDelete?: (position: ActivePosition) => void;
	}

	const { position, isAdmin = false, onUpdate, onInvalidate, onClose, onDelete }: Props = $props();

	let menuOpen = $state(false);
	let showDeleteModal = $state(false);

	const isProfit = $derived(position.unrealizedPercent !== null && position.unrealizedPercent >= 0);
	const statusClass = $derived(
		position.status === 'WATCHING'
			? 'watching'
			: position.status === 'ENTRY'
				? 'entry'
				: isProfit
					? 'profit'
					: 'loss'
	);

	function toggleMenu(e: MouseEvent) {
		e.stopPropagation();
		menuOpen = !menuOpen;
	}

	function handleUpdate(e: MouseEvent) {
		e.stopPropagation();
		menuOpen = false;
		onUpdate?.(position);
	}

	function handleInvalidate(e: MouseEvent) {
		e.stopPropagation();
		menuOpen = false;
		onInvalidate?.(position);
	}

	function handleClose(e: MouseEvent) {
		e.stopPropagation();
		menuOpen = false;
		onClose?.(position);
	}

	function handleDelete(e: MouseEvent) {
		e.stopPropagation();
		menuOpen = false;
		showDeleteModal = true;
	}

	function confirmDelete() {
		showDeleteModal = false;
		onDelete?.(position);
	}

	function handleClickOutside() {
		if (menuOpen) {
			menuOpen = false;
		}
	}
</script>

<svelte:window onclick={handleClickOutside} />

<div class="card {statusClass}">
	<!-- Updated Badge -->
	{#if position.wasUpdated}
		<span class="updated-badge">UPDATED</span>
	{/if}

	<!-- Row 1: Ticker + Status + P&L -->
	<div class="row-main">
		<span class="ticker">{position.ticker}</span>
		<span class="status">{position.status}</span>
		<span
			class="pnl"
			class:profit={isProfit}
			class:loss={!isProfit && position.unrealizedPercent !== null}
		>
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
				<div
					class="fill"
					class:profit={isProfit}
					style="width:{Math.min(100, position.progressToTarget1)}%"
				></div>
			</div>
			<span class="pct">{position.progressToTarget1.toFixed(0)}%</span>
		</div>
	{/if}

	<!-- Admin Menu -->
	{#if isAdmin && (onUpdate || onInvalidate || onClose || onDelete)}
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
					<circle cx="12" cy="5" r="2" />
					<circle cx="12" cy="12" r="2" />
					<circle cx="12" cy="19" r="2" />
				</svg>
			</button>

			{#if menuOpen}
				<div class="menu-dropdown" role="menu">
					<!-- Update Position -->
					{#if onUpdate}
						<button type="button" class="menu-item" onclick={handleUpdate} role="menuitem">
							<svg
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
								width="16"
								height="16"
							>
								<path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7" />
								<path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z" />
							</svg>
							Update Position
						</button>
					{/if}

					<!-- Invalidate (for trades that didn't trigger) -->
					{#if onInvalidate}
						<button
							type="button"
							class="menu-item warning"
							onclick={handleInvalidate}
							role="menuitem"
						>
							<svg
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
								width="16"
								height="16"
							>
								<circle cx="12" cy="12" r="10" />
								<path d="M12 8v4M12 16h.01" />
							</svg>
							Invalidate
						</button>
					{/if}

					<!-- Close Trade -->
					{#if onClose}
						<button type="button" class="menu-item" onclick={handleClose} role="menuitem">
							<svg
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
								width="16"
								height="16"
							>
								<path d="M9 12l2 2 4-4" />
								<circle cx="12" cy="12" r="10" />
							</svg>
							Close Trade
						</button>
					{/if}

					<!-- Divider before dangerous action -->
					{#if onDelete && (onUpdate || onInvalidate || onClose)}
						<div class="menu-divider"></div>
					{/if}

					<!-- Delete (for mistakes) -->
					{#if onDelete}
						<button type="button" class="menu-item danger" onclick={handleDelete} role="menuitem">
							<svg
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
								width="16"
								height="16"
							>
								<path
									d="M3 6h18M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2m3 0v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6h14z"
								/>
								<path d="M10 11v6M14 11v6" />
							</svg>
							Delete
						</button>
					{/if}
				</div>
			{/if}
		</div>
	{/if}
</div>

<!-- Delete Confirmation Modal -->
<ConfirmationModal
	isOpen={showDeleteModal}
	title="Delete Position"
	message="Are you sure you want to delete {position.ticker}? This action cannot be undone."
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmDelete}
	onCancel={() => (showDeleteModal = false)}
/>

<style>
	/* ═══════════════════════════════════════════════════════════════════════
	   ACTIVE POSITION CARD - Netflix-grade compact trading card
	   Professional design with status-based left border accent
	   ═══════════════════════════════════════════════════════════════════════ */
	.card {
		position: relative;
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-default);
		border-left: 3px solid var(--color-border-strong);
		border-radius: 8px;
		padding: 10px 12px;
		font-size: var(--text-sm);
		transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
		will-change: transform, box-shadow;
	}

	.card:hover {
		transform: translateY(-2px);
		box-shadow: var(--shadow-md);
		border-color: var(--color-border-strong);
	}

	.card.profit {
		border-left-color: var(--color-profit);
		background: linear-gradient(
			135deg,
			var(--color-bg-card) 0%,
			var(--color-profit-bg-subtle) 100%
		);
	}
	.card.loss {
		border-left-color: var(--color-loss);
		background: linear-gradient(135deg, var(--color-bg-card) 0%, var(--color-loss-bg-subtle) 100%);
	}
	.card.watching {
		border-left-color: var(--color-watching);
		background: linear-gradient(135deg, var(--color-bg-card) 0%, rgba(245, 158, 11, 0.04) 100%);
	}
	.card.entry {
		border-left-color: var(--color-entry);
		background: linear-gradient(135deg, var(--color-bg-card) 0%, rgba(20, 184, 166, 0.04) 100%);
	}

	/* Updated Badge - Animated pulse effect */
	.updated-badge {
		position: absolute;
		top: -6px;
		left: 10px;
		background: linear-gradient(135deg, var(--color-brand-primary) 0%, var(--color-update) 100%);
		color: white;
		font-size: 8px;
		font-weight: 700;
		padding: 2px 6px;
		border-radius: 4px;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		box-shadow: 0 2px 6px rgba(59, 130, 246, 0.3);
		animation: pulse-badge 2s ease-in-out infinite;
	}

	@keyframes pulse-badge {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.8;
		}
	}

	/* Row 1 - Main ticker info */
	.row-main {
		display: flex;
		align-items: center;
		gap: 6px;
		margin-bottom: 6px;
		padding-right: 28px;
	}

	.ticker {
		font-weight: 800;
		font-size: 0.9375rem;
		color: var(--color-text-primary);
		line-height: 1;
		letter-spacing: -0.01em;
	}

	.status {
		font-size: 9px;
		font-weight: 700;
		text-transform: uppercase;
		padding: 2px 6px;
		border-radius: 4px;
		background: var(--color-bg-subtle);
		color: var(--color-text-tertiary);
		letter-spacing: 0.03em;
	}

	.pnl {
		margin-left: auto;
		font-weight: 800;
		font-size: 14px;
		font-variant-numeric: tabular-nums;
		color: var(--color-text-muted);
		transition: transform 0.15s ease;
	}

	.card:hover .pnl {
		transform: scale(1.05);
	}

	.pnl.profit {
		color: var(--color-profit);
	}

	.pnl.loss {
		color: var(--color-loss);
	}

	/* Row 2 - Price levels */
	.row-prices {
		display: flex;
		align-items: center;
		gap: 6px;
		font-size: 11px;
		color: var(--color-text-secondary);
		font-variant-numeric: tabular-nums;
		margin-bottom: 6px;
		flex-wrap: wrap;
	}

	.sep {
		color: var(--color-border-strong);
		opacity: 0.5;
	}

	.stop {
		color: var(--color-loss);
		font-weight: 600;
	}

	.target {
		color: var(--color-profit);
		font-weight: 600;
	}

	/* Row 3 - Progress bar with animation */
	.row-progress {
		display: flex;
		align-items: center;
		gap: 8px;
		margin-top: 6px;
	}

	.bar {
		flex: 1;
		height: 4px;
		background: var(--color-bg-muted);
		border-radius: 4px;
		overflow: hidden;
	}

	.fill {
		height: 100%;
		background: linear-gradient(90deg, var(--color-loss) 0%, var(--color-loss-light) 100%);
		border-radius: 4px;
		transition: width 0.5s cubic-bezier(0.16, 1, 0.3, 1);
	}

	.fill.profit {
		background: linear-gradient(90deg, var(--color-profit) 0%, var(--color-profit-light) 100%);
	}

	.pct {
		font-size: 11px;
		font-weight: 700;
		color: var(--color-text-muted);
		font-variant-numeric: tabular-nums;
		min-width: 32px;
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
		transition:
			background 0.15s,
			color 0.15s;
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
		z-index: 100;
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
		transition:
			background 0.15s,
			color 0.15s;
	}
	.menu-item:hover {
		background: var(--color-bg-subtle);
		color: var(--color-text-primary);
	}
	.menu-item:focus-visible {
		outline: 2px solid var(--color-brand-primary);
		outline-offset: -2px;
	}
	/* Warning style (Invalidate) */
	.menu-item.warning {
		color: var(--color-warning, #f59e0b);
	}
	.menu-item.warning:hover {
		background: rgba(245, 158, 11, 0.1);
		color: var(--color-warning, #f59e0b);
	}

	/* Danger style (Delete) */
	.menu-item.danger {
		color: var(--color-loss);
	}
	.menu-item.danger:hover {
		background: var(--color-loss-bg);
		color: var(--color-loss);
	}

	/* Menu divider */
	.menu-divider {
		height: 1px;
		background: var(--color-border-default);
		margin: 4px 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   RESPONSIVE BREAKPOINTS - Mobile-first (min-width)
	   ═══════════════════════════════════════════════════════════════════════ */

	/* Menu trigger touch target */
	.menu-trigger {
		min-width: var(--touch-target-min);
		min-height: var(--touch-target-min);
	}
</style>
