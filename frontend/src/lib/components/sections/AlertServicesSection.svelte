<script lang="ts">
	import { browser } from '$app/environment';
	import { cubicOut } from 'svelte/easing';
	import {
		Icon,
		IconActivity,
		IconAntenna,
		IconArrowUpRight,
		IconBolt,
		IconClock,
		IconTarget,
		IconTrendingUp
	} from '$lib/icons';
	// --- Data Configuration ---
	const signals = [
		{
			id: 'spx',
			type: 'intraday',
			badge: '0DTE STRUCTURE',
			title: 'SPX Profit Pulse',
			price: '$97/mo',
			description:
				'Context-rich intraday alerts. We isolate high-probability inflection points on the S&P 500 with defined invalidation levels.',
			metrics: [
				{ label: 'Avg Duration', value: '45m' },
				{ label: 'Risk/Reward', value: '1:3' },
				{ label: 'Frequency', value: 'High' }
			],
			href: '/alert-services/spx-profit-pulse',
			icon: IconBolt,
			accent: 'amber', // Tailwind color
			chartColor: '#fbbf24' // Hex for SVG
		},
		{
			id: 'swing',
			type: 'swing',
			badge: 'MULTI-DAY FLOW',
			title: 'Explosive Swings',
			price: '$127/mo',
			description:
				'Institutional accumulation setups in high-beta equities. We track dark pool positioning to catch breakouts before the crowd.',
			metrics: [
				{ label: 'Hold Time', value: '2-5d' },
				{ label: 'Risk/Reward', value: '1:5' },
				{ label: 'Volatility', value: 'Med' }
			],
			href: '/alert-services/explosive-swings',
			icon: IconTrendingUp,
			accent: 'orange',
			chartColor: '#fb923c'
		}
	];

	// --- Interaction Logic ---
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
				return `opacity: ${eased}; transform: translateY(${(1 - eased) * 20}px);`;
			}
		};
	}

	// Trigger entrance animations when section scrolls into viewport
	$effect(() => {
		if (!browser || !containerRef) {
			isVisible = true;
			return;
		}

		const observer = new IntersectionObserver(
			(entries) => {
				if (entries[0]?.isIntersecting) {
					isVisible = true;
					observer.disconnect();
				}
			},
			{ threshold: 0.1, rootMargin: '50px' }
		);

		observer.observe(containerRef);

		return () => observer.disconnect();
	});
</script>

<section
	bind:this={containerRef}
	onmousemove={handleMouseMove}
	role="group"
	aria-label="Alert Services"
	class="relative py-24 lg:py-32 3xl:py-40 5xl:py-48 px-4 sm:px-6 lg:px-8 3xl:px-12 5xl:px-16 6xl:px-20 bg-zinc-950 overflow-hidden border-t border-zinc-900"
>
	<div class="as-grid-bg">
		<div class="as-grid-lines"></div>
	</div>

	<div
		class="relative max-w-7xl 3xl:max-w-[1800px] 4xl:max-w-[2200px] 5xl:max-w-[2600px] 6xl:max-w-[3200px] mx-auto z-10"
	>
		<div
			class="max-w-4xl 3xl:max-w-[1200px] 4xl:max-w-[1600px] 5xl:max-w-[2000px] 6xl:max-w-[2400px] mx-auto text-center mb-24 3xl:mb-32 5xl:mb-40"
		>
			{#if isVisible}
				<div in:heavySlide={{ delay: 0, duration: 1000 }} class="as-badge">
					<Icon icon={IconBolt} size={14} />
					Signal Intelligence
				</div>

				<h2
					in:heavySlide={{ delay: 100 }}
					class="text-4xl xs:text-5xl sm:text-5xl md:text-7xl 3xl:text-8xl 4xl:text-9xl 5xl:text-[10rem] font-serif text-white mb-8 tracking-tight"
				>
					Alert <span class="text-slate-700">Systems.</span>
				</h2>

				<p
					in:heavySlide={{ delay: 200 }}
					class="text-lg 3xl:text-xl 5xl:text-2xl text-slate-400 font-light leading-relaxed max-w-2xl 3xl:max-w-3xl 5xl:max-w-4xl mx-auto"
				>
					We don't send generic alerts. We deliver institutional-grade signal intelligence. Verified
					by quantitative analysts and professional traders worldwide.
				</p>
			{/if}
		</div>

		<div
			class="group/grid grid md:grid-cols-2 gap-8 3xl:gap-12 5xl:gap-16 max-w-5xl 3xl:max-w-[1400px] 4xl:max-w-[1800px] 5xl:max-w-[2200px] 6xl:max-w-[2600px] mx-auto"
			style="--x: {mouse.x}px; --y: {mouse.y}px;"
		>
			{#each signals as item, i (item.title ?? i)}
				{#if isVisible}
					<div in:heavySlide={{ delay: 300 + i * 150 }} class="as-card" data-accent={item.accent}>
						<div
							class="as-card-spotlight"
							style="background: radial-gradient(600px circle at var(--x) var(--y), oklch(1 0 0 / 0.04), transparent 40%);"
						></div>

						<div class="as-chart-area">
							<div class="as-chart-grid"></div>

							<div class="as-chart-inner">
								{#if item.type === 'intraday'}
									<svg class="as-chart-svg" viewBox="0 0 300 100" preserveAspectRatio="none">
										<defs>
											<linearGradient id="grad-spx" x1="0%" y1="0%" x2="100%" y2="0%">
												<stop offset="0%" stop-color={item.chartColor} stop-opacity="0.1" />
												<stop offset="100%" stop-color={item.chartColor} stop-opacity="1" />
											</linearGradient>
										</defs>
										<path
											d="M0,50 L20,45 L40,55 L60,30 L80,60 L100,40 L120,50 L140,20 L160,40 L180,30 L200,80 L220,60 L240,70 L260,20 L300,10"
											fill="none"
											stroke="url(#grad-spx)"
											stroke-width="2"
											stroke-linecap="round"
											stroke-linejoin="round"
											class="as-chart-path"
										/>
										<circle cx="300" cy="10" r="3" fill={item.chartColor}>
											<animate attributeName="r" values="3;6;3" dur="2s" repeatCount="indefinite" />
											<animate
												attributeName="opacity"
												values="1;0.5;1"
												dur="2s"
												repeatCount="indefinite"
											/>
										</circle>
										<rect
											x="240"
											y="15"
											width="60"
											height="60"
											fill={item.chartColor}
											fill-opacity="0.05"
											stroke={item.chartColor}
											stroke-opacity="0.2"
											stroke-dasharray="4 2"
										/>
										<text
											x="245"
											y="85"
											font-family="monospace"
											font-size="8"
											fill={item.chartColor}
											opacity="0.8">ENTRY ZONE</text
										>
									</svg>
								{:else}
									<svg class="as-chart-svg" viewBox="0 0 300 100" preserveAspectRatio="none">
										<defs>
											<linearGradient id="grad-swing" x1="0%" y1="0%" x2="100%" y2="0%">
												<stop offset="0%" stop-color={item.chartColor} stop-opacity="0.1" />
												<stop offset="100%" stop-color={item.chartColor} stop-opacity="1" />
											</linearGradient>
										</defs>
										<path
											d="M0,20 Q50,20 70,50 Q100,90 150,90 Q200,90 230,50 L250,60 L280,30 L300,5"
											fill="none"
											stroke="url(#grad-swing)"
											stroke-width="2"
											stroke-linecap="round"
											class="as-chart-path"
										/>
										<line
											x1="200"
											y1="50"
											x2="300"
											y2="50"
											stroke="white"
											stroke-opacity="0.1"
											stroke-dasharray="4"
										/>
										<circle cx="280" cy="30" r="3" fill={item.chartColor} />
										<text
											x="220"
											y="40"
											font-family="monospace"
											font-size="8"
											fill="white"
											opacity="0.5">BREAKOUT LEVEL</text
										>
									</svg>
								{/if}
							</div>
						</div>

						<div class="as-card-body">
							<div class="as-card-top">
								<div class="as-card-info">
									<div class="as-icon-box">
										<Icon icon={item.icon} size={20} />
									</div>
									<div>
										<h3 class="as-card-title">{item.title}</h3>
										<div class="as-badge-row">
											<span class="as-pulse-dot"></span>
											<span class="as-badge-label">{item.badge}</span>
										</div>
									</div>
								</div>
								<div class="as-price-col">
									<div class="as-price">{item.price}</div>
									<div class="as-price-sub">per month</div>
								</div>
							</div>

							<p class="as-desc">{item.description}</p>

							<div
								class="grid grid-cols-3 gap-px bg-zinc-800 border border-zinc-800 rounded-lg overflow-hidden mb-8"
							>
								{#each item.metrics as metric (metric.label)}
									<div
										class="bg-zinc-900/50 p-3 text-center group-hover/card:bg-zinc-900 transition-colors"
									>
										<div class="text-[10px] uppercase text-zinc-500 font-mono mb-1">
											{metric.label}
										</div>
										<div class="text-sm font-medium text-zinc-300">{metric.value}</div>
									</div>
								{/each}
							</div>

							<a href={item.href} class="as-cta-link">
								<span class="as-cta-start">
									<Icon icon={IconAntenna} size={16} class="as-cta-icon" />
									<span>Subscribe to Feed</span>
								</span>
								<Icon icon={IconArrowUpRight} size={16} class="as-cta-arrow" />
							</a>
						</div>
					</div>
				{/if}
			{/each}
		</div>

		{#if isVisible}
			<div in:heavySlide={{ delay: 600 }} class="as-footer">
				<div class="as-footer-items">
					<span class="as-footer-item">
						<Icon icon={IconTarget} size={14} />
						<span>STRICT INVALIDATION LEVELS</span>
					</span>
					<span class="as-footer-sep"></span>
					<span class="as-footer-item">
						<Icon icon={IconClock} size={14} />
						<span>REAL-TIME PUSH NOTIFICATIONS</span>
					</span>
					<span class="as-footer-sep"></span>
					<span class="as-footer-item">
						<Icon icon={IconActivity} size={14} />
						<span>FULL TRADE MANAGEMENT</span>
					</span>
				</div>
			</div>
		{/if}
	</div>
</section>

<style>
	/* ─── Section ─── */
	.as-section {
		position: relative;
		padding-block: 6rem;
		padding-inline: 1rem;
		background-color: oklch(0.12 0.005 285);
		overflow: hidden;
		border-block-start: 1px solid oklch(0.2 0.005 285);

		@media (min-width: 640px) {
			padding-inline: 1.5rem;
		}
		@media (min-width: 1024px) {
			padding-block: 8rem;
			padding-inline: 2rem;
		}
	}

	/* ─── Grid Background ─── */
	.as-grid-bg {
		position: absolute;
		inset: 0;
		pointer-events: none;
	}

	.as-grid-lines {
		position: absolute;
		inset: 0;
		background-image:
			linear-gradient(to right, oklch(0.22 0.005 285) 1px, transparent 1px),
			linear-gradient(to bottom, oklch(0.22 0.005 285) 1px, transparent 1px);
		background-size: 3rem 3rem;
		mask-image: radial-gradient(ellipse 60% 50% at 50% 0%, oklch(0 0 0) 70%, transparent 100%);
		opacity: 0.3;
	}

	/* ─── Container ─── */
	.as-container {
		position: relative;
		max-inline-size: 80rem;
		margin-inline: auto;
		z-index: 10;
	}

	.as-header {
		max-inline-size: 56rem;
		margin-inline: auto;
		text-align: center;
		margin-block-end: 6rem;
	}

	.as-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.75rem;
		padding-inline: 1rem;
		padding-block: 0.375rem;
		border: 1px solid oklch(0.45 0.12 70 / 0.3);
		background-color: oklch(0.2 0.06 70 / 0.1);
		color: oklch(0.75 0.15 85);
		font-size: 0.625rem;
		font-weight: var(--weight-bold);
		letter-spacing: 0.3em;
		text-transform: uppercase;
		margin-block-end: 2rem;
		border-radius: 2px;
	}

	.as-title {
		font-size: clamp(3rem, 6vw, 4.5rem);
		font-family: var(--font-serif, serif);
		color: oklch(1 0 0);
		margin-block-end: 2rem;
		letter-spacing: -0.02em;
	}

	.as-title-muted {
		color: oklch(0.35 0.01 265);
	}

	.as-subtitle {
		font-size: var(--text-lg);
		color: oklch(0.55 0.01 265);
		font-weight: 300;
		line-height: 1.7;
		max-inline-size: 32rem;
		margin-inline: auto;
	}

	/* ─── Cards Grid ─── */
	.as-cards {
		display: grid;
		gap: 2rem;
		max-inline-size: 70rem;
		margin-inline: auto;

		@media (min-width: 768px) {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	/* ─── Card ─── */
	.as-card {
		position: relative;
		background-color: oklch(0.12 0.005 285);
		border: 1px solid oklch(0.3 0.005 285);
		border-radius: var(--radius-xl);
		overflow: hidden;
		transition: border-color 500ms;

		&:hover {
			border-color: oklch(0.4 0.005 285);
		}
		&:hover .as-metric-cell {
			background-color: oklch(0.15 0.005 285);
		}
	}

	.as-card-spotlight {
		position: absolute;
		inset: 0;
		z-index: 0;
		opacity: 0;
		transition: opacity 500ms;
		pointer-events: none;
	}

	.as-cards:hover .as-card-spotlight {
		opacity: 1;
	}

	/* ─── Chart Area ─── */
	.as-chart-area {
		position: relative;
		block-size: 12rem;
		inline-size: 100%;
		background-color: oklch(0.15 0.005 285 / 0.3);
		border-block-end: 1px solid oklch(0.3 0.005 285);
		overflow: hidden;
	}

	.as-chart-grid {
		position: absolute;
		inset: 0;
		background-image:
			linear-gradient(to right, oklch(0.25 0.005 285) 1px, transparent 1px),
			linear-gradient(to bottom, oklch(0.25 0.005 285) 1px, transparent 1px);
		background-size: 1.25rem 1.25rem;
	}

	.as-chart-inner {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 2rem;
	}

	.as-chart-svg {
		inline-size: 100%;
		block-size: 100%;
		overflow: visible;
	}

	.as-chart-path {
		filter: drop-shadow(0 0 10px oklch(0.75 0.15 85 / 0.5));
	}

	/* ─── Card Body ─── */
	.as-card-body {
		position: relative;
		z-index: 10;
		padding: 2rem;
	}

	.as-card-top {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-block-end: 1.5rem;
	}

	.as-card-info {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.as-icon-box {
		padding: 0.5rem;
		border-radius: var(--radius-sm);
		background-color: oklch(0.15 0.005 285);
		border: 1px solid oklch(0.3 0.005 285);
	}

	.as-card[data-accent='amber'] .as-icon-box {
		color: oklch(0.75 0.15 85);
	}
	.as-card[data-accent='orange'] .as-icon-box {
		color: oklch(0.7 0.16 55);
	}

	.as-card-title {
		font-size: var(--text-xl);
		font-weight: var(--weight-medium);
		color: oklch(1 0 0);
	}

	.as-badge-row {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-block-start: 0.25rem;
	}

	.as-pulse-dot {
		inline-size: 0.375rem;
		block-size: 0.375rem;
		border-radius: 50%;
		animation: pulse 1s ease-in-out infinite;
	}

	.as-card[data-accent='amber'] .as-pulse-dot {
		background-color: oklch(0.75 0.15 85);
	}
	.as-card[data-accent='orange'] .as-pulse-dot {
		background-color: oklch(0.7 0.16 55);
	}

	.as-badge-label {
		font-size: 0.625rem;
		font-family: var(--font-mono, monospace);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.as-card[data-accent='amber'] .as-badge-label {
		color: oklch(0.75 0.15 85 / 0.8);
	}
	.as-card[data-accent='orange'] .as-badge-label {
		color: oklch(0.7 0.16 55 / 0.8);
	}

	.as-price-col {
		text-align: end;
	}
	.as-price {
		font-size: var(--text-lg);
		font-weight: var(--weight-medium);
		color: oklch(1 0 0);
	}
	.as-price-sub {
		font-size: var(--text-xs);
		color: oklch(0.45 0.005 285);
	}

	.as-desc {
		font-size: var(--text-sm);
		color: oklch(0.55 0.01 265);
		line-height: 1.7;
		margin-block-end: 2rem;
		block-size: 3rem;
	}

	/* ─── Metrics ─── */
	.as-metrics {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 1px;
		background-color: oklch(0.3 0.005 285);
		border: 1px solid oklch(0.3 0.005 285);
		border-radius: var(--radius-lg);
		overflow: hidden;
		margin-block-end: 2rem;
	}

	.as-metric-cell {
		background-color: oklch(0.15 0.005 285 / 0.5);
		padding: 0.75rem;
		text-align: center;
		transition: background-color 200ms;
	}

	.as-metric-label {
		font-size: 0.625rem;
		text-transform: uppercase;
		color: oklch(0.45 0.005 285);
		font-family: var(--font-mono, monospace);
		margin-block-end: 0.25rem;
	}

	.as-metric-value {
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
		color: oklch(0.8 0.01 265);
	}

	/* ─── CTA Link ─── */
	.as-cta-link {
		position: relative;
		inline-size: 100%;
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding-inline: 1rem;
		padding-block: 0.75rem;
		background-color: oklch(0.15 0.005 285);
		border: 1px solid oklch(0.3 0.005 285);
		border-radius: var(--radius-md);
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
		color: oklch(0.8 0.01 265);
		transition:
			color 300ms,
			border-color 300ms,
			background-color 300ms;
		text-decoration: none;

		&:hover {
			color: oklch(1 0 0);
		}
	}

	.as-card[data-accent='amber'] .as-cta-link:hover {
		border-color: oklch(0.75 0.15 85 / 0.5);
		background-color: oklch(0.75 0.15 85 / 0.1);
	}
	.as-card[data-accent='orange'] .as-cta-link:hover {
		border-color: oklch(0.7 0.16 55 / 0.5);
		background-color: oklch(0.7 0.16 55 / 0.1);
	}

	.as-cta-start {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}
	:global(.as-cta-icon) {
		color: oklch(0.75 0.15 85);
	}
	.as-card[data-accent='orange'] :global(.as-cta-icon) {
		color: oklch(0.7 0.16 55);
	}

	:global(.as-cta-arrow) {
		color: oklch(0.45 0.005 285);
		transition:
			transform 300ms,
			color 300ms;
	}
	.as-cta-link:hover :global(.as-cta-arrow) {
		transform: translate(2px, -2px);
		color: oklch(1 0 0);
	}

	/* ─── Footer ─── */
	.as-footer {
		margin-block-start: 3rem;
		text-align: center;
		border-block-start: 1px solid oklch(0.2 0.005 285);
		padding-block-start: 2rem;
	}

	.as-footer-items {
		display: inline-flex;
		align-items: center;
		gap: 1.5rem;
		font-size: var(--text-xs);
		color: oklch(0.45 0.005 285);
		font-family: var(--font-mono, monospace);
	}

	.as-footer-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.as-footer-sep {
		inline-size: 1px;
		block-size: 0.75rem;
		background-color: oklch(0.3 0.005 285);
		display: none;

		@media (min-width: 640px) {
			display: inline;
		}
	}

	/* ─── Keyframes ─── */
	@keyframes pulse {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.4;
		}
	}
</style>
