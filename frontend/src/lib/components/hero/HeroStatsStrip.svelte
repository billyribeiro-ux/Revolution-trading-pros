<script lang="ts">
	/**
	 * HeroStatsStrip - Netflix L11+ Live Stats Ticker
	 * ══════════════════════════════════════════════════════════════════════════════
	 * Bottom ticker strip with:
	 * - Live market indicators
	 * - Animated stat counters
	 * - Subtle scroll marquee
	 * - Trading session status
	 * ══════════════════════════════════════════════════════════════════════════════
	 */
	import { onMount, onDestroy } from 'svelte';
	import { browser } from '$app/environment';

	// ============================================================================
	// PROPS
	// ============================================================================
	interface Props {
		isVisible?: boolean;
	}

	let { isVisible = true }: Props = $props();

	// ============================================================================
	// STATE
	// ============================================================================
	let stripRef = $state<HTMLElement | null>(null);
	let currentTime = $state('');
	let marketStatus = $state<'pre' | 'open' | 'closed'>('closed');
	let timeInterval: ReturnType<typeof setInterval> | null = null;

	// Live market data (simulated - would connect to real API)
	const marketIndicators = [
		{ symbol: 'SPY', price: 475.32, change: 0.85, changePercent: 0.18 },
		{ symbol: 'QQQ', price: 402.18, change: 1.24, changePercent: 0.31 },
		{ symbol: 'VIX', price: 13.42, change: -0.38, changePercent: -2.75 },
		{ symbol: 'ES', price: 4782.50, change: 12.25, changePercent: 0.26 }
	];

	// Session stats
	const sessionStats = [
		{ label: 'Alerts Today', value: 12 },
		{ label: 'Win Rate', value: '87%' },
		{ label: 'Avg R:R', value: '2.4:1' }
	];

	// ============================================================================
	// TIME & MARKET STATUS
	// ============================================================================
	function updateTime(): void {
		const now = new Date();
		const etTime = new Date(now.toLocaleString('en-US', { timeZone: 'America/New_York' }));

		currentTime = etTime.toLocaleTimeString('en-US', {
			hour: '2-digit',
			minute: '2-digit',
			second: '2-digit',
			hour12: true
		});

		// Determine market status (simplified)
		const hours = etTime.getHours();
		const minutes = etTime.getMinutes();
		const dayOfWeek = etTime.getDay();

		if (dayOfWeek === 0 || dayOfWeek === 6) {
			marketStatus = 'closed';
		} else if (hours < 9 || (hours === 9 && minutes < 30)) {
			marketStatus = 'pre';
		} else if (hours < 16) {
			marketStatus = 'open';
		} else {
			marketStatus = 'closed';
		}
	}

	// ============================================================================
	// LIFECYCLE
	// ============================================================================
	onMount(() => {
		if (browser) {
			updateTime();
			timeInterval = setInterval(updateTime, 1000);
		}
	});

	onDestroy(() => {
		if (timeInterval) clearInterval(timeInterval);
	});
</script>

<div
	bind:this={stripRef}
	class="stats-strip"
	class:stats-strip--visible={isVisible}
	aria-label="Live trading statistics"
>
	<!-- Market Status -->
	<div class="strip-section strip-section--status">
		<div class="market-status market-status--{marketStatus}">
			<span class="market-status__dot"></span>
			<span class="market-status__label">
				{#if marketStatus === 'open'}
					Market Open
				{:else if marketStatus === 'pre'}
					Pre-Market
				{:else}
					Market Closed
				{/if}
			</span>
		</div>
		<span class="strip-time">{currentTime} ET</span>
	</div>

	<!-- Divider -->
	<span class="strip-divider"></span>

	<!-- Market Tickers -->
	<div class="strip-section strip-section--tickers">
		{#each marketIndicators as ticker, i (ticker.symbol)}
			<div class="ticker">
				<span class="ticker__symbol">{ticker.symbol}</span>
				<span class="ticker__price">{ticker.price.toLocaleString('en-US', { minimumFractionDigits: 2 })}</span>
				<span
					class="ticker__change"
					class:positive={ticker.change >= 0}
					class:negative={ticker.change < 0}
				>
					{ticker.change >= 0 ? '+' : ''}{ticker.changePercent.toFixed(2)}%
				</span>
			</div>
			{#if i < marketIndicators.length - 1}
				<span class="ticker-separator">•</span>
			{/if}
		{/each}
	</div>

	<!-- Divider -->
	<span class="strip-divider strip-divider--desktop"></span>

	<!-- Session Stats -->
	<div class="strip-section strip-section--session">
		{#each sessionStats as stat, i (stat.label)}
			<div class="session-stat">
				<span class="session-stat__value">{stat.value}</span>
				<span class="session-stat__label">{stat.label}</span>
			</div>
			{#if i < sessionStats.length - 1}
				<span class="stat-separator"></span>
			{/if}
		{/each}
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Stats Strip Container
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.stats-strip {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 1.5rem;
		padding: 0.75rem 1.5rem;
		background: rgba(15, 23, 42, 0.8);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 1rem;
		backdrop-filter: blur(12px);
		-webkit-backdrop-filter: blur(12px);
		opacity: 0;
		transform: translateY(20px);
		transition: opacity 0.6s ease, transform 0.6s ease;
		overflow-x: auto;
		scrollbar-width: none;
		-ms-overflow-style: none;
	}

	.stats-strip::-webkit-scrollbar {
		display: none;
	}

	.stats-strip--visible {
		opacity: 1;
		transform: translateY(0);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Strip Sections
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.strip-section {
		display: flex;
		align-items: center;
		gap: 1rem;
		flex-shrink: 0;
	}

	.strip-divider {
		width: 1px;
		height: 1.5rem;
		background: rgba(255, 255, 255, 0.1);
		flex-shrink: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Market Status
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.market-status {
		display: flex;
		align-items: center;
		gap: 0.4rem;
		padding: 0.25rem 0.6rem;
		border-radius: 9999px;
		font-family: var(--font-heading, system-ui);
		font-size: 0.7rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.03em;
	}

	.market-status--open {
		background: rgba(52, 211, 153, 0.15);
		color: rgb(52, 211, 153);
	}

	.market-status--pre {
		background: rgba(250, 204, 21, 0.15);
		color: rgb(250, 204, 21);
	}

	.market-status--closed {
		background: rgba(156, 163, 175, 0.15);
		color: rgb(156, 163, 175);
	}

	.market-status__dot {
		width: 6px;
		height: 6px;
		border-radius: 50%;
		background: currentColor;
	}

	.market-status--open .market-status__dot {
		animation: pulse 2s ease-in-out infinite;
	}

	@keyframes pulse {
		0%, 100% { opacity: 1; }
		50% { opacity: 0.5; }
	}

	.strip-time {
		font-family: var(--font-mono, ui-monospace);
		font-size: 0.75rem;
		font-weight: 500;
		color: rgba(255, 255, 255, 0.6);
		white-space: nowrap;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Market Tickers
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.strip-section--tickers {
		gap: 0.75rem;
	}

	.ticker {
		display: flex;
		align-items: center;
		gap: 0.4rem;
	}

	.ticker__symbol {
		font-family: var(--font-heading, system-ui);
		font-size: 0.7rem;
		font-weight: 700;
		color: rgba(255, 255, 255, 0.9);
		letter-spacing: 0.02em;
	}

	.ticker__price {
		font-family: var(--font-mono, ui-monospace);
		font-size: 0.7rem;
		font-weight: 500;
		color: rgba(255, 255, 255, 0.7);
	}

	.ticker__change {
		font-family: var(--font-mono, ui-monospace);
		font-size: 0.65rem;
		font-weight: 600;
		padding: 0.1rem 0.3rem;
		border-radius: 0.25rem;
	}

	.ticker__change.positive {
		background: rgba(52, 211, 153, 0.15);
		color: rgb(52, 211, 153);
	}

	.ticker__change.negative {
		background: rgba(239, 68, 68, 0.15);
		color: rgb(248, 113, 113);
	}

	.ticker-separator {
		color: rgba(255, 255, 255, 0.2);
		font-size: 0.5rem;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Session Stats
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.strip-section--session {
		gap: 0.75rem;
	}

	.session-stat {
		display: flex;
		align-items: baseline;
		gap: 0.35rem;
	}

	.session-stat__value {
		font-family: var(--font-heading, system-ui);
		font-size: 0.85rem;
		font-weight: 700;
		color: white;
	}

	.session-stat__label {
		font-family: var(--font-body, system-ui);
		font-size: 0.65rem;
		color: rgba(255, 255, 255, 0.5);
		text-transform: uppercase;
		letter-spacing: 0.03em;
	}

	.stat-separator {
		width: 1px;
		height: 1rem;
		background: rgba(255, 255, 255, 0.1);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Responsive
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	@media (max-width: 1024px) {
		.stats-strip {
			gap: 1rem;
			padding: 0.6rem 1rem;
			justify-content: flex-start;
		}

		.strip-divider--desktop {
			display: none;
		}

		.strip-section--session {
			display: none;
		}
	}

	@media (max-width: 768px) {
		.stats-strip {
			border-radius: 0.75rem;
		}

		.strip-section--status {
			gap: 0.5rem;
		}

		.strip-section--tickers {
			gap: 0.5rem;
		}

		.ticker__price {
			display: none;
		}

		.strip-time {
			font-size: 0.7rem;
		}
	}

	@media (max-width: 480px) {
		.stats-strip {
			padding: 0.5rem 0.75rem;
			gap: 0.75rem;
		}

		.market-status {
			padding: 0.2rem 0.5rem;
			font-size: 0.65rem;
		}

		.ticker__symbol {
			font-size: 0.65rem;
		}

		.ticker__change {
			font-size: 0.6rem;
		}

		.ticker-separator:nth-child(n+6) {
			display: none;
		}

		.ticker:nth-child(n+4) {
			display: none;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Reduced Motion
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	@media (prefers-reduced-motion: reduce) {
		.stats-strip {
			transition: none;
		}

		.market-status__dot {
			animation: none;
		}
	}
</style>
