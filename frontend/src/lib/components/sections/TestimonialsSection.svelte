<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { cubicOut } from 'svelte/easing';
	import {
		Icon,
		IconActivity,
		IconChartDots,
		IconCircleCheckFilled,
		IconQuote,
		IconShieldCheck,
		IconTrendingUp,
		IconUserCircle
	} from '$lib/icons';
	// --- Institutional Client Data ---
	const reviews = [
		{
			id: 'CL-8821',
			name: 'Marcus T.',
			role: 'Prop Desk Manager',
			location: 'London, UK',
			quote:
				"The risk protocols alone saved our desk six figures during the last CPI volatility. This isn't a course; it's an operating system for capital preservation.",
			metric: 'Drawdown Reduced',
			value: '-18.5%',
			trend: 'positive'
		},
		{
			id: 'CL-4920',
			name: 'Sarah J.',
			role: 'Independent Fund',
			location: 'New York, NY',
			quote:
				"I spent 3 years in retail 'discord' hell. One month in the institutional audit reset my entire view of liquidity. My execution drag is now virtually zero.",
			metric: 'Execution Alpha',
			value: '+12bps',
			trend: 'positive'
		},
		{
			id: 'CL-1049',
			name: 'David K.',
			role: 'Swing Trader',
			location: 'Singapore',
			quote:
				'The dark pool logic is frighteningly accurate. We are front-running moves that I used to chase. The mental clarity of having a structured invalidated level is priceless.',
			metric: 'Win Rate',
			value: '68.2%',
			trend: 'positive'
		},
		{
			id: 'CL-3392',
			name: 'Elena R.',
			role: 'Futures Scalper',
			location: 'Chicago, IL',
			quote:
				'Strictly business. No hype, no noise. Just level 2 analysis and probability mapping. If you treat trading as a career, this is the only room that matters.',
			metric: 'Profit Factor',
			value: '2.45',
			trend: 'positive'
		},
		{
			id: 'CL-5591',
			name: 'James H.',
			role: 'Options Desk',
			location: 'Toronto, CA',
			quote:
				"The 0DTE structure is surgical. We aren't gambling on gamma squeezes anymore; we are capturing defined volatility expansion. Professional grade.",
			metric: 'Sharpe Ratio',
			value: '1.92',
			trend: 'positive'
		},
		{
			id: 'CL-9920',
			name: 'Robert B.',
			role: 'Capital Growth',
			location: 'Sydney, AU',
			quote:
				"Scaled my account from mid-five to six figures in Q3 using the 'Compound Protocol.' The psychological sizing models removed the fear of pulling the trigger.",
			metric: 'YTD Return',
			value: '+41.8%',
			trend: 'positive'
		}
	];

	// --- Animation Logic ---
	let containerRef = $state<HTMLElement | null>(null);
	let mouse = $state({ x: 0, y: 0 });
	// ICT11+ Fix: Start false, set true in onMount to trigger in: transitions
	let isVisible = $state(false);

	const handleMouseMove = (e: MouseEvent) => {
		if (!containerRef) return;
		const rect = containerRef.getBoundingClientRect();
		mouse.x = e.clientX - rect.left;
		mouse.y = e.clientY - rect.top;
	};

	function heavySlide(_node: Element, { delay = 0, duration = 1000 }) {
		return {
			delay,
			duration,
			css: (t: number) => {
				const eased = cubicOut(t);
				return `opacity: ${eased}; transform: translateY(${(1 - eased) * 30}px);`;
			}
		};
	}

	// Trigger entrance animations when section scrolls into viewport
	onMount(() => {
		if (!browser) {
			isVisible = true;
			return;
		}

		queueMicrotask(() => {
			if (!containerRef) {
				isVisible = true;
				return;
			}

			const visibilityObserver = new IntersectionObserver(
				(entries) => {
					if (entries[0]?.isIntersecting) {
						isVisible = true;
						visibilityObserver.disconnect();
					}
				},
				{ threshold: 0.1, rootMargin: '50px' }
			);

			visibilityObserver.observe(containerRef);
		});
	});

	// Ticker Tape Data
	const tickerItems = Array(20)
		.fill(0)
		.map((_, i) => ({
			symbol: ['ES_F', 'NQ_F', 'SPX', 'VIX', 'AAPL', 'NVDA'][i % 6],
			price: (4000 + Math.random() * 1000).toFixed(2),
			change: (Math.random() * 2 - 0.5).toFixed(2) + '%'
		}));
</script>

<section
	bind:this={containerRef}
	onmousemove={handleMouseMove}
	class="ts-section"
	aria-label="Client Performance Ledger"
>
	<div class="ts-ticker-bg">
		<div class="ts-ticker-row ts-ticker-left">
			{#each [...tickerItems, ...tickerItems, ...tickerItems] as item}
				<div class="ts-ticker-item">
					<span class="ts-ticker-symbol">{item.symbol}</span>
					<span>{item.price}</span>
					<span
						class="ts-ticker-change"
						data-direction={item.change.startsWith('-') ? 'down' : 'up'}>{item.change}</span
					>
				</div>
			{/each}
		</div>
		<div class="ts-ticker-row ts-ticker-right">
			{#each [...tickerItems, ...tickerItems, ...tickerItems] as item}
				<div class="ts-ticker-item">
					<span class="ts-ticker-symbol">{item.symbol}</span>
					<span>{item.price}</span>
					<span
						class="ts-ticker-change"
						data-direction={item.change.startsWith('-') ? 'down' : 'up'}>{item.change}</span
					>
				</div>
			{/each}
		</div>
	</div>

	<div class="ts-fade-overlay"></div>
	<div
		class="ts-spotlight"
		style="background: radial-gradient(1000px circle at var(--x) var(--y), oklch(0.7 0.17 160 / 0.03), transparent 50%);"
	></div>

	<div class="ts-container">
		<div class="ts-header">
			{#if isVisible}
				<div in:heavySlide={{ delay: 0, duration: 1000 }} class="ts-badge">
					<Icon icon={IconShieldCheck} size={14} />
					Verified Performance
				</div>

				<h2 in:heavySlide={{ delay: 100 }} class="ts-title">
					Performance <span class="ts-title-muted">Attribution.</span>
				</h2>

				<p in:heavySlide={{ delay: 200 }} class="ts-subtitle">
					We don't rely on marketing claims. We rely on the PnL of our desk. Verified feedback from
					funded traders, prop managers, and institutional clients.
				</p>
			{/if}
		</div>

		<div class="ts-masonry">
			{#each reviews as review, i}
				{#if isVisible}
					<div in:heavySlide={{ delay: 300 + i * 100 }} class="ts-card">
						<div class="ts-card-top">
							<div class="ts-avatar-row">
								<div class="ts-avatar">
									<Icon icon={IconUserCircle} size={20} />
								</div>
								<div>
									<div class="ts-name">{review.name}</div>
									<div class="ts-role">{review.role}</div>
								</div>
							</div>
							<div class="ts-id-badge">{review.id}</div>
						</div>

						<div class="ts-quote-wrap">
							<Icon icon={IconQuote} size={24} class="ts-quote-icon" />
							<p class="ts-quote">"{review.quote}"</p>
						</div>

						<div class="ts-card-bottom">
							<div class="ts-metric">
								<span class="ts-metric-label">{review.metric}</span>
								<div class="ts-metric-row">
									<Icon icon={IconTrendingUp} size={14} class="ts-emerald-icon" />
									<span class="ts-metric-value">{review.value}</span>
								</div>
							</div>
							<div class="ts-verified">
								<Icon icon={IconCircleCheckFilled} size={14} />
								Verified
							</div>
						</div>

						<div class="ts-corner">
							<div class="ts-corner-dot"></div>
							<svg class="ts-corner-svg" viewBox="0 0 24 24">
								<path d="M24 0 L0 0" stroke="currentColor" stroke-width="1" />
								<path d="M24 0 L24 24" stroke="currentColor" stroke-width="1" />
							</svg>
						</div>
					</div>
				{/if}
			{/each}
		</div>

		{#if isVisible}
			<div in:heavySlide={{ delay: 600 }} class="ts-trust-bar">
				<div class="ts-trust-item">
					<Icon icon={IconChartDots} size={24} />
					<span class="ts-trust-label">Audited by NinjaTrader</span>
				</div>
				<div class="ts-trust-item">
					<Icon icon={IconShieldCheck} size={24} />
					<span class="ts-trust-label">MyFxBook Verified</span>
				</div>
				<div class="ts-trust-item">
					<Icon icon={IconActivity} size={24} />
					<span class="ts-trust-label">Institutional Data Feed</span>
				</div>
			</div>
		{/if}
	</div>
</section>

<style>
	/* ─── Section ─── */
	.ts-section {
		position: relative;
		padding-block: 8rem;
		padding-inline: 1.5rem;
		background-color: oklch(0.05 0 0);
		overflow: hidden;
		border-block-end: 1px solid oklch(1 0 0 / 0.05);
	}

	/* ─── Ticker Background ─── */
	.ts-ticker-bg {
		position: absolute;
		inset: 0;
		pointer-events: none;
		opacity: 0.04;
		overflow: hidden;
		display: flex;
		flex-direction: column;
		justify-content: center;
		gap: 6rem;
		transform: rotate(-6deg) scale(1.1);
	}

	.ts-ticker-row {
		display: flex;
		gap: 3rem;
		white-space: nowrap;
		font-size: var(--text-xs);
		font-family: var(--font-mono, monospace);
		color: oklch(1 0 0);
	}

	.ts-ticker-left {
		animation: marquee-left 40s linear infinite;
	}
	.ts-ticker-right {
		animation: marquee-right 40s linear infinite;
	}

	.ts-ticker-item {
		display: flex;
		gap: 1rem;
	}
	.ts-ticker-symbol {
		font-weight: var(--weight-bold);
	}
	.ts-ticker-change[data-direction='up'] {
		color: oklch(0.7 0.17 160);
	}
	.ts-ticker-change[data-direction='down'] {
		color: oklch(0.6 0.2 25);
	}

	/* ─── Overlays ─── */
	.ts-fade-overlay {
		position: absolute;
		inset: 0;
		pointer-events: none;
		background: linear-gradient(to bottom, oklch(0.05 0 0), transparent, oklch(0.05 0 0));
	}

	.ts-spotlight {
		position: absolute;
		inset: 0;
		pointer-events: none;
		opacity: 0.4;
		transition: opacity 300ms;
	}

	/* ─── Container ─── */
	.ts-container {
		position: relative;
		max-inline-size: 100rem;
		margin-inline: auto;
		z-index: 10;
	}

	.ts-header {
		max-inline-size: 56rem;
		margin-inline: auto;
		text-align: center;
		margin-block-end: 6rem;
	}

	.ts-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.75rem;
		padding-inline: 1rem;
		padding-block: 0.375rem;
		border: 1px solid oklch(0.3 0.1 160 / 0.3);
		background-color: oklch(0.15 0.05 160 / 0.1);
		color: oklch(0.7 0.17 160);
		font-size: 0.625rem;
		font-weight: var(--weight-bold);
		letter-spacing: 0.3em;
		text-transform: uppercase;
		margin-block-end: 2rem;
		border-radius: 2px;
	}

	.ts-title {
		font-size: clamp(3rem, 6vw, 4.5rem);
		font-family: var(--font-serif, serif);
		color: oklch(1 0 0);
		margin-block-end: 2rem;
		letter-spacing: -0.02em;
	}

	.ts-title-muted {
		color: oklch(0.35 0.01 265);
	}

	.ts-subtitle {
		font-size: var(--text-lg);
		color: oklch(0.55 0.01 265);
		font-weight: 300;
		line-height: 1.7;
		max-inline-size: 32rem;
		margin-inline: auto;
	}

	/* ─── Masonry ─── */
	.ts-masonry {
		columns: 1;
		gap: 1.5rem;

		@media (min-width: 768px) {
			columns: 2;
		}
		@media (min-width: 1024px) {
			columns: 3;
		}

		& > * {
			margin-block-end: 1.5rem;
		}
	}

	/* ─── Card ─── */
	.ts-card {
		break-inside: avoid;
		position: relative;
		background-color: oklch(0.08 0 0);
		border: 1px solid oklch(1 0 0 / 0.1);
		padding: 2rem;
		transition:
			background-color 500ms,
			border-color 500ms;

		&:hover {
			background-color: oklch(0.1 0 0);
			border-color: oklch(0.7 0.17 160 / 0.3);
		}

		&:hover .ts-verified {
			opacity: 1;
		}
		&:hover .ts-card-bottom {
			background-color: oklch(0.15 0.05 160 / 0.05);
		}
		&:hover .ts-corner-dot {
			background-color: oklch(0.7 0.17 160 / 0.5);
		}
		&:hover .ts-corner-svg {
			opacity: 1;
		}
	}

	.ts-card-top {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-block-end: 1.5rem;
		padding-block-end: 1.5rem;
		border-block-end: 1px solid oklch(1 0 0 / 0.05);
	}

	.ts-avatar-row {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.ts-avatar {
		inline-size: 2rem;
		block-size: 2rem;
		border-radius: 50%;
		background-color: oklch(1 0 0 / 0.05);
		display: flex;
		align-items: center;
		justify-content: center;
		color: oklch(0.55 0.01 265);
	}

	.ts-name {
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
		color: oklch(1 0 0);
	}
	.ts-role {
		font-size: 0.625rem;
		font-family: var(--font-mono, monospace);
		text-transform: uppercase;
		color: oklch(0.55 0.01 265);
		letter-spacing: 0.05em;
	}

	.ts-id-badge {
		font-size: 0.625rem;
		font-family: var(--font-mono, monospace);
		color: oklch(0.7 0.17 160 / 0.8);
		border: 1px solid oklch(0.3 0.1 160 / 0.3);
		padding-inline: 0.5rem;
		padding-block: 0.25rem;
		background-color: oklch(0.15 0.05 160 / 0.1);
		border-radius: var(--radius-sm);
	}

	/* ─── Quote ─── */
	.ts-quote-wrap {
		position: relative;
		margin-block-end: 2rem;
	}
	:global(.ts-quote-icon) {
		position: absolute;
		inset-block-start: -0.5rem;
		inset-inline-start: -0.5rem;
		color: oklch(1 0 0 / 0.05);
		transform: scaleX(-1);
	}

	.ts-quote {
		position: relative;
		z-index: 10;
		color: oklch(0.8 0.01 265);
		font-weight: 300;
		line-height: 1.7;
		font-size: var(--text-sm);
	}

	/* ─── Card Bottom ─── */
	.ts-card-bottom {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-inline: -2rem;
		margin-block-end: -2rem;
		padding-inline: 2rem;
		padding-block: 1rem;
		border-block-start: 1px solid oklch(1 0 0 / 0.05);
		background-color: oklch(1 0 0 / 0.02);
		transition: background-color 500ms;
	}

	.ts-metric {
		display: flex;
		flex-direction: column;
	}

	.ts-metric-label {
		font-size: 0.625rem;
		font-family: var(--font-mono, monospace);
		text-transform: uppercase;
		color: oklch(0.55 0.01 265);
		letter-spacing: 0.1em;
		margin-block-end: 0.25rem;
	}

	.ts-metric-row {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}
	:global(.ts-emerald-icon) {
		color: oklch(0.7 0.17 160);
	}
	.ts-metric-value {
		font-size: var(--text-lg);
		font-family: var(--font-serif, serif);
		color: oklch(1 0 0);
	}

	.ts-verified {
		opacity: 0;
		transition: opacity 500ms;
		display: flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.625rem;
		font-weight: var(--weight-bold);
		color: oklch(0.7 0.17 160);
		text-transform: uppercase;
		letter-spacing: 0.1em;
	}

	/* ─── Corner Accent ─── */
	.ts-corner {
		position: absolute;
		inset-block-start: 0;
		inset-inline-end: 0;
		inline-size: 2rem;
		block-size: 2rem;
		pointer-events: none;
		overflow: hidden;
	}

	.ts-corner-dot {
		position: absolute;
		inset-block-start: 0;
		inset-inline-end: 0;
		inline-size: 0.5rem;
		block-size: 0.5rem;
		background-color: oklch(0.7 0.17 160 / 0);
		transition: background-color 500ms;
	}

	.ts-corner-svg {
		position: absolute;
		inset-block-start: 0;
		inset-inline-end: 0;
		inline-size: 100%;
		block-size: 100%;
		color: oklch(0.7 0.17 160 / 0.2);
		opacity: 0;
		transition: opacity 500ms;
	}

	/* ─── Trust Bar ─── */
	.ts-trust-bar {
		margin-block-start: 5rem;
		padding-block-start: 2.5rem;
		border-block-start: 1px solid oklch(1 0 0 / 0.05);
		display: flex;
		flex-wrap: wrap;
		justify-content: center;
		gap: 3rem;
		opacity: 0.5;
		filter: grayscale(1);
		transition:
			filter 500ms,
			opacity 500ms;

		@media (min-width: 1024px) {
			gap: 6rem;
		}

		&:hover {
			filter: grayscale(0);
			opacity: 1;
		}
	}

	.ts-trust-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		color: oklch(1 0 0 / 0.6);
	}

	.ts-trust-label {
		font-size: var(--text-xs);
		font-family: var(--font-mono, monospace);
		text-transform: uppercase;
		letter-spacing: 0.1em;
	}

	/* ─── Keyframes ─── */
	@keyframes marquee-left {
		0% {
			transform: translateX(0);
		}
		100% {
			transform: translateX(-50%);
		}
	}
	@keyframes marquee-right {
		0% {
			transform: translateX(-50%);
		}
		100% {
			transform: translateX(0);
		}
	}
</style>
