<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { cubicOut } from 'svelte/easing';
	import {
		Icon,
		IconActivity,
		IconArrowUpRight,
		IconBuilding,
		IconTerminal,
		IconTrendingUp
	} from '$lib/icons';
	// --- Data Configuration ---
	const products = [
		{
			id: 'day',
			type: 'candles', // UPDATED: Triggers the Live Candle animation
			label: 'INTRADAY',
			title: 'Day Trading Desk',
			metric: '9:30 AM EST',
			description:
				'High-velocity execution environment. Real-time order flow analysis and level 2 data interpretation.',
			features: ['Live Execution', 'Risk Parameters', 'Gap Strategy'],
			href: '/live-trading-rooms/day-trading',
			icon: IconActivity,
			accent: 'blue',
			cta: 'Launch Terminal'
		},
		{
			id: 'swing',
			type: 'wave',
			label: 'POSITIONAL',
			title: 'Swing Strategy',
			metric: 'Weekly Cycle',
			description:
				'Institutional timeframe alignment. Capture multi-day moves driven by macro-economic flows.',
			features: ['Macro Analysis', 'Flow Tracking', 'Overnight Risk'],
			href: '/live-trading-rooms/swing-trading',
			icon: IconTrendingUp,
			accent: 'emerald',
			cta: 'View Strategy'
		},
		{
			id: 'foundation',
			type: 'step',
			label: 'FOUNDATION',
			title: 'Capital Builder',
			metric: '< $25k Accts',
			description:
				'Strict discipline protocol for emerging capital. Focus on risk-adjusted returns and drawdown control.',
			features: ['Risk Protocol', 'Capital Preservation', 'Scaling Logic'],
			href: '/live-trading-rooms/small-accounts',
			icon: IconBuilding,
			accent: 'indigo',
			cta: 'Start Building'
		}
	];

	// ICT11+ Fix: IntersectionObserver triggers animations when section scrolls into view
	let isVisible = $state(false);
	let containerRef = $state<HTMLElement | null>(null);

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
	let observer: IntersectionObserver | null = null;

	onMount(() => {
		if (!browser) {
			isVisible = true;
			return;
		}

		// Use queueMicrotask to ensure bind:this has completed
		queueMicrotask(() => {
			if (!containerRef) {
				isVisible = true;
				return;
			}

			observer = new IntersectionObserver(
				(entries) => {
					if (entries[0]?.isIntersecting) {
						isVisible = true;
						observer?.disconnect();
					}
				},
				{ threshold: 0.1, rootMargin: '50px' }
			);

			observer.observe(containerRef);
		});

		return () => observer?.disconnect();
	});
</script>

<section bind:this={containerRef} class="tr-section">
	<div class="tr-bg">
		<div class="tr-grid-lines"></div>
	</div>

	<div class="tr-container">
		<div class="tr-header">
			{#if isVisible}
				<div in:heavySlide={{ delay: 0, duration: 1000 }} class="tr-live-badge">
					<span class="tr-ping-wrap">
						<span class="tr-ping"></span>
						<span class="tr-ping-dot"></span>
					</span>
					<span class="tr-live-label">Market Access Open</span>
				</div>

				<h2 in:heavySlide={{ delay: 100 }} class="tr-title">Professional Trading Environments</h2>

				<p in:heavySlide={{ delay: 200 }} class="tr-subtitle">
					Select an environment tailored to your liquidity requirements.
					<span class="tr-subtitle-light">Hover over a desk to preview data feeds.</span>
				</p>
			{/if}
		</div>

		<div class="tr-cards">
			{#each products as item, i}
				{@const iconStr = item.icon}
				{#if isVisible}
					<div in:heavySlide={{ delay: 300 + i * 100 }} class="tr-card" data-accent={item.accent}>
						{#if item.type === 'candles'}
							<div class="tr-anim-overlay">
								<div class="tr-anim-tint" data-accent="blue"></div>
								<svg class="tr-candle-svg" viewBox="0 0 400 200" preserveAspectRatio="none">
									<line
										x1="0"
										y1="50"
										x2="400"
										y2="50"
										stroke="oklch(0.3 0.15 260)"
										stroke-width="1"
										stroke-dasharray="4 4"
										opacity="0.3"
									/>
									<line
										x1="0"
										y1="100"
										x2="400"
										y2="100"
										stroke="oklch(0.3 0.15 260)"
										stroke-width="1"
										stroke-dasharray="4 4"
										opacity="0.3"
									/>
									<line
										x1="0"
										y1="150"
										x2="400"
										y2="150"
										stroke="oklch(0.3 0.15 260)"
										stroke-width="1"
										stroke-dasharray="4 4"
										opacity="0.3"
									/>
									<line
										x1="40"
										y1="120"
										x2="40"
										y2="180"
										stroke="oklch(0.6 0.18 260)"
										stroke-width="1"
									/>
									<rect
										x="30"
										y="140"
										width="20"
										height="30"
										fill="oklch(0.6 0.18 260)"
										opacity="0.8"
									/>
									<line
										x1="90"
										y1="130"
										x2="90"
										y2="150"
										stroke="oklch(0.3 0.15 260)"
										stroke-width="1"
									/>
									<rect
										x="80"
										y="135"
										width="20"
										height="10"
										fill="none"
										stroke="oklch(0.6 0.18 260)"
										stroke-width="2"
									/>
									<line
										x1="140"
										y1="80"
										x2="140"
										y2="140"
										stroke="oklch(0.6 0.18 260)"
										stroke-width="1"
									/>
									<rect
										x="130"
										y="90"
										width="20"
										height="40"
										fill="oklch(0.6 0.18 260)"
										opacity="0.9"
									/>
									<line
										x1="190"
										y1="85"
										x2="190"
										y2="105"
										stroke="oklch(0.6 0.18 260)"
										stroke-width="1"
									/>
									<rect
										x="180"
										y="90"
										width="20"
										height="5"
										fill="oklch(0.6 0.18 260)"
										opacity="0.8"
									/>
									<line
										x1="240"
										y1="80"
										x2="240"
										y2="120"
										stroke="oklch(0.3 0.15 260)"
										stroke-width="1"
									/>
									<rect
										x="230"
										y="95"
										width="20"
										height="15"
										fill="none"
										stroke="oklch(0.6 0.18 260)"
										stroke-width="2"
									/>
									<line
										x1="290"
										y1="60"
										x2="290"
										y2="110"
										stroke="oklch(0.6 0.18 260)"
										stroke-width="1"
									/>
									<rect
										x="280"
										y="70"
										width="20"
										height="35"
										fill="oklch(0.6 0.18 260)"
										opacity="0.9"
									/>
									<g class="tr-live-candle">
										<line
											x1="340"
											y1="20"
											x2="340"
											y2="80"
											stroke="oklch(0.7 0.15 260)"
											stroke-width="1"
										/>
										<rect x="330" y="40" width="20" height="30" fill="oklch(0.7 0.15 260)" />
									</g>
								</svg>
							</div>
						{/if}

						{#if item.type === 'wave'}
							<div class="tr-anim-overlay">
								<div class="tr-anim-tint" data-accent="emerald"></div>
								<svg class="tr-wave-svg" viewBox="0 0 1000 300" preserveAspectRatio="none">
									<path
										d="M0,150 C150,50 350,250 500,150 C650,50 850,250 1000,150 V300 H0 Z"
										fill="currentColor"
									/>
									<path
										d="M0,160 C140,60 360,260 500,160 C640,60 860,260 1000,160"
										fill="none"
										stroke="currentColor"
										stroke-width="2"
										class="tr-wave-line"
									/>
								</svg>
							</div>
						{/if}

						{#if item.type === 'step'}
							<div class="tr-anim-overlay">
								<div class="tr-anim-tint" data-accent="indigo"></div>
								<div class="tr-step-bars">
									{#each Array(8) as _, k}
										<div class="tr-step-bar" style="--h: {k * 10}%"></div>
									{/each}
								</div>
							</div>
						{/if}

						<div class="tr-card-gradient"></div>

						<div class="tr-card-top">
							<div class="tr-icon-box">
								<Icon icon={iconStr} size={24} stroke={1.25} />
							</div>
							<span class="tr-label-badge">{item.label}</span>
						</div>

						<div class="tr-card-body">
							<h3 class="tr-card-title">{item.title}</h3>

							<div class="tr-metric-row">
								<Icon icon={IconTerminal} size={12} />
								<span>{item.metric}</span>
							</div>

							<div class="tr-desc-wrap">
								<div class="tr-desc-fade">
									<p class="tr-card-desc">{item.description}</p>
									<div class="tr-features">
										{#each item.features as feat}
											<div class="tr-feature">{feat}</div>
										{/each}
									</div>
								</div>
							</div>
						</div>

						<div class="tr-card-footer">
							<a href={item.href} class="tr-cta-link">
								<span class="tr-cta-text">{item.cta}</span>
								<div class="tr-cta-arrow">
									<Icon icon={IconArrowUpRight} size={16} />
								</div>
							</a>
						</div>
					</div>
				{/if}
			{/each}
		</div>
	</div>
</section>

<style>
	/* ─── Section ─── */
	.tr-section {
		position: relative;
		padding-block: 8rem;
		padding-inline: 1rem;
		background-color: oklch(0.12 0.005 285);
		overflow: hidden;
		border-block-start: 1px solid oklch(0.2 0.005 285);

		@media (min-width: 640px) {
			padding-inline: 1.5rem;
		}
		@media (min-width: 1024px) {
			padding-inline: 2rem;
		}
	}

	/* ─── Background ─── */
	.tr-bg {
		position: absolute;
		inset: 0;
		pointer-events: none;
	}

	.tr-grid-lines {
		position: absolute;
		inset: 0;
		background-image:
			linear-gradient(to right, oklch(0.22 0.005 285) 1px, transparent 1px),
			linear-gradient(to bottom, oklch(0.22 0.005 285) 1px, transparent 1px);
		background-size: 4rem 4rem;
		mask-image: radial-gradient(ellipse 60% 50% at 50% 0%, oklch(0 0 0) 70%, transparent 100%);
		opacity: 0.4;
	}

	/* ─── Container ─── */
	.tr-container {
		position: relative;
		max-inline-size: 80rem;
		margin-inline: auto;
		z-index: 10;
	}

	.tr-header {
		max-inline-size: 48rem;
		margin-inline: auto;
		text-align: center;
		margin-block-end: 5rem;
	}

	.tr-live-badge {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		margin-block-end: 1.5rem;
	}

	.tr-ping-wrap {
		position: relative;
		display: flex;
		block-size: 0.5rem;
		inline-size: 0.5rem;
	}
	.tr-ping {
		position: absolute;
		display: inline-flex;
		block-size: 100%;
		inline-size: 100%;
		border-radius: 50%;
		background-color: oklch(0.7 0.17 160);
		opacity: 0.75;
		animation: ping 1s cubic-bezier(0, 0, 0.2, 1) infinite;
	}
	.tr-ping-dot {
		position: relative;
		display: inline-flex;
		border-radius: 50%;
		block-size: 0.5rem;
		inline-size: 0.5rem;
		background-color: oklch(0.7 0.17 160);
	}

	.tr-live-label {
		font-size: var(--text-xs);
		font-family: var(--font-mono, monospace);
		text-transform: uppercase;
		letter-spacing: 0.1em;
		color: oklch(0.7 0.17 160 / 0.8);
	}

	.tr-title {
		font-size: clamp(1.875rem, 4vw, 3rem);
		font-weight: var(--weight-medium);
		letter-spacing: -0.02em;
		color: oklch(1 0 0);
		margin-block-end: 1.5rem;
	}

	.tr-subtitle {
		font-size: clamp(1rem, 2vw, var(--text-lg));
		color: oklch(0.45 0.005 285);
		line-height: 1.7;
		font-weight: 300;
		max-inline-size: 32rem;
		margin-inline: auto;
	}

	.tr-subtitle-light {
		color: oklch(0.55 0.01 265);
	}

	/* ─── Cards Grid ─── */
	.tr-cards {
		display: grid;
		gap: 1px;
		background-color: oklch(0.3 0.005 285);
		border: 1px solid oklch(0.3 0.005 285);
		border-radius: var(--radius-lg);
		overflow: hidden;
		box-shadow: 0 25px 50px oklch(0 0 0 / 0.5);

		@media (min-width: 768px) {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	/* ─── Card ─── */
	.tr-card {
		position: relative;
		background-color: oklch(0.12 0.005 285);
		padding: 2rem;
		display: flex;
		flex-direction: column;
		block-size: 26.25rem;
		overflow: hidden;

		@media (min-width: 1024px) {
			padding: 2.5rem;
		}

		&:hover .tr-anim-overlay {
			opacity: 1;
		}
		&:hover .tr-card-gradient {
			opacity: 0.4;
		}
		&:hover .tr-icon-box {
			color: oklch(1 0 0);
			border-color: oklch(0.4 0.005 285);
		}
		&:hover .tr-card-title {
			transform: scaleX(1.05);
		}
		&:hover .tr-desc-fade {
			opacity: 0;
			transform: translateY(-0.5rem);
			filter: blur(4px);
		}
		&:hover .tr-cta-link {
			color: oklch(1 0 0);
		}
		&:hover .tr-cta-text {
			transform: translateX(0.25rem);
		}
		&:hover .tr-cta-arrow {
			color: oklch(1 0 0);
		}
		&:hover .tr-step-bar {
			block-size: calc(20% + var(--h));
		}
	}

	.tr-card[data-accent='blue'] {
		&:hover .tr-card-footer {
			border-color: oklch(0.6 0.18 260 / 0.3);
		}
	}
	.tr-card[data-accent='blue'] {
		&:hover .tr-cta-arrow {
			background-color: oklch(0.6 0.18 260);
		}
	}
	.tr-card[data-accent='emerald'] {
		&:hover .tr-card-footer {
			border-color: oklch(0.7 0.17 160 / 0.3);
		}
	}
	.tr-card[data-accent='emerald'] {
		&:hover .tr-cta-arrow {
			background-color: oklch(0.7 0.17 160);
		}
	}
	.tr-card[data-accent='indigo'] {
		&:hover .tr-card-footer {
			border-color: oklch(0.6 0.2 280 / 0.3);
		}
	}
	.tr-card[data-accent='indigo'] {
		&:hover .tr-cta-arrow {
			background-color: oklch(0.6 0.2 280);
		}
	}

	/* ─── Animation Overlays ─── */
	.tr-anim-overlay {
		position: absolute;
		inset: 0;
		opacity: 0;
		transition: opacity 500ms;
		pointer-events: none;
	}

	.tr-anim-tint {
		position: absolute;
		inset: 0;
	}
	.tr-anim-tint[data-accent='blue'] {
		background-color: oklch(0.2 0.1 260 / 0.1);
	}
	.tr-anim-tint[data-accent='emerald'] {
		background-color: oklch(0.2 0.1 160 / 0.1);
	}
	.tr-anim-tint[data-accent='indigo'] {
		background-color: oklch(0.2 0.1 280 / 0.1);
	}

	.tr-candle-svg {
		position: absolute;
		inset-block-end: 2.5rem;
		inset-inline-start: 0;
		inline-size: 100%;
		block-size: 12rem;
	}

	.tr-live-candle {
		animation: live-candle 2s ease-in-out infinite;
		transform-box: fill-box;
		transform-origin: bottom;
	}

	.tr-wave-svg {
		position: absolute;
		inset-block-end: 0;
		inset-inline-start: 0;
		inline-size: 200%;
		block-size: 16rem;
		color: oklch(0.7 0.17 160 / 0.2);
		animation: wave-slide 10s linear infinite;
	}

	.tr-wave-line {
		color: oklch(0.7 0.17 160 / 0.4);
	}

	.tr-step-bars {
		position: absolute;
		inset-block-end: 0;
		inset-inline-start: 0;
		inset-inline-end: 0;
		block-size: 16rem;
		display: flex;
		align-items: flex-end;
		justify-content: space-around;
		padding-inline: 1rem;
	}

	.tr-step-bar {
		inline-size: 2rem;
		block-size: 0;
		background-color: oklch(0.6 0.2 280 / 0.2);
		border-block-start: 1px solid oklch(0.65 0.18 280 / 0.3);
		transition: block-size 700ms ease-out;
	}

	/* ─── Card Gradient ─── */
	.tr-card-gradient {
		position: absolute;
		inset: 0;
		background: linear-gradient(
			to top,
			oklch(0.12 0.005 285),
			oklch(0.12 0.005 285 / 0.8),
			transparent
		);
		opacity: 0.9;
		transition: opacity 500ms;
		pointer-events: none;
	}

	/* ─── Card Top ─── */
	.tr-card-top {
		position: relative;
		z-index: 10;
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-block-end: 2rem;
	}

	.tr-icon-box {
		padding: 0.75rem;
		background-color: oklch(0.15 0.005 285 / 0.8);
		backdrop-filter: blur(8px);
		border: 1px solid oklch(0.3 0.005 285);
		border-radius: var(--radius-md);
		color: oklch(0.55 0.01 265);
		transition:
			color 300ms,
			border-color 300ms;
	}

	.tr-label-badge {
		font-family: var(--font-mono, monospace);
		font-size: 0.625rem;
		letter-spacing: 0.1em;
		color: oklch(0.4 0.005 285);
		text-transform: uppercase;
		border: 1px solid oklch(0.3 0.005 285);
		background-color: oklch(0.12 0.005 285 / 0.5);
		backdrop-filter: blur(8px);
		padding-inline: 0.5rem;
		padding-block: 0.25rem;
		border-radius: var(--radius-sm);
	}

	/* ─── Card Body ─── */
	.tr-card-body {
		position: relative;
		z-index: 10;
		flex-grow: 1;
	}

	.tr-card-title {
		font-size: var(--text-xl);
		font-weight: var(--weight-medium);
		color: oklch(1 0 0);
		margin-block-end: 0.5rem;
		transition: transform 500ms;
		transform-origin: left;
	}

	.tr-metric-row {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-block-end: 1.5rem;
		font-size: var(--text-xs);
		font-family: var(--font-mono, monospace);
	}

	.tr-card[data-accent='blue'] .tr-metric-row {
		color: oklch(0.65 0.15 260 / 0.8);
	}
	.tr-card[data-accent='emerald'] .tr-metric-row {
		color: oklch(0.7 0.17 160 / 0.8);
	}
	.tr-card[data-accent='indigo'] .tr-metric-row {
		color: oklch(0.6 0.18 280 / 0.8);
	}

	.tr-desc-wrap {
		position: relative;
	}

	.tr-desc-fade {
		transition:
			opacity 500ms ease-in-out,
			transform 500ms ease-in-out,
			filter 500ms ease-in-out;
	}

	.tr-card-desc {
		font-size: var(--text-sm);
		color: oklch(0.55 0.01 265);
		line-height: 1.7;
		font-weight: 300;
		margin-block-end: 2rem;
	}

	.tr-features {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		border-inline-start: 1px solid oklch(0.3 0.005 285);
		padding-inline-start: 1rem;
	}

	.tr-feature {
		font-size: var(--text-xs);
		color: oklch(0.45 0.005 285);
	}

	/* ─── Card Footer ─── */
	.tr-card-footer {
		position: relative;
		z-index: 20;
		margin-block-start: auto;
		padding-block-start: 1.5rem;
		border-block-start: 1px solid oklch(0.2 0.005 285);
		transition: border-color 300ms;
		background-color: oklch(0.12 0.005 285 / 0.2);
		backdrop-filter: blur(4px);
	}

	.tr-cta-link {
		display: flex;
		align-items: center;
		justify-content: space-between;
		inline-size: 100%;
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
		color: oklch(0.8 0.01 265);
		transition: color 300ms;
		text-decoration: none;
	}

	.tr-cta-text {
		transition: transform 300ms;
	}

	.tr-cta-arrow {
		padding: 0.375rem;
		border-radius: var(--radius-sm);
		background-color: oklch(0.15 0.005 285);
		transition:
			background-color 300ms,
			color 300ms;
	}

	/* ─── Keyframes ─── */
	@keyframes live-candle {
		0% {
			transform: scaleY(1) translateY(0);
		}
		50% {
			transform: scaleY(1.4) translateY(-10px);
		}
		70% {
			transform: scaleY(0.9) translateY(5px);
		}
		100% {
			transform: scaleY(1) translateY(0);
		}
	}

	@keyframes wave-slide {
		0% {
			transform: translateX(0);
		}
		100% {
			transform: translateX(-50%);
		}
	}

	@keyframes ping {
		75%,
		100% {
			transform: scale(2);
			opacity: 0;
		}
	}
</style>
