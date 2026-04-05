<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { cubicOut } from 'svelte/easing';
	import { Icon, IconBuilding, IconCheck, IconCpu, IconShield, IconSitemap } from '$lib/icons';
	const features = [
		{
			title: 'Structured Curriculum',
			subtitle: 'FRAMEWORK',
			description:
				'Move beyond random setups. We teach a repeatable, probabilistic execution model used by proprietary desks.',
			icon: IconSitemap,
			accent: 'cyan',
			type: 'grid' // Triggers grid background
		},
		{
			title: 'Risk Protocols',
			subtitle: 'SURVIVAL',
			description:
				'Capital preservation is the primary objective. Every trade alert includes hard invalidation points and sizing logic.',
			icon: IconShield,
			accent: 'emerald',
			type: 'radar' // Triggers radar background
		},
		{
			title: 'Proprietary Analytics',
			subtitle: 'INFRASTRUCTURE',
			description:
				'Access institutional-grade indicators that track dark pool volume, gamma exposure, and volatility flows.',
			icon: IconCpu,
			accent: 'indigo',
			type: 'circuit' // Triggers circuit background
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
	let observer: IntersectionObserver | null = null;

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

<section
	bind:this={containerRef}
	onmousemove={handleMouseMove}
	role="group"
	aria-label="Core Infrastructure Features"
	class="ws-section"
>
	<div class="ws-bg">
		<div class="ws-grid-lines"></div>
	</div>

	<div class="ws-container">
		<div class="ws-header">
			{#if isVisible}
				<div in:heavySlide={{ delay: 0, duration: 1000 }} class="ws-badge">
					<Icon icon={IconBuilding} size={14} />
					System Design
				</div>

				<h2 in:heavySlide={{ delay: 100 }} class="ws-title">
					Trading <span class="ws-title-muted">Framework.</span>
				</h2>

				<p in:heavySlide={{ delay: 200 }} class="ws-subtitle">
					We don't build retail platforms. We engineer institutional trading systems. Verified by
					quantitative funds and proprietary trading desks.
				</p>
			{/if}
		</div>

		<div class="ws-cards" style="--x: {mouse.x}px; --y: {mouse.y}px;">
			{#each features as feature, i}
				{@const iconStr = feature.icon}
				{#if isVisible}
					<div
						in:heavySlide={{ delay: 300 + i * 150 }}
						class="ws-card"
						data-accent={feature.accent}
					>
						<div
							class="ws-card-spotlight"
							style="background: radial-gradient(600px circle at var(--x) var(--y), oklch(1 0 0 / 0.03), transparent 40%);"
						></div>

						<div class="ws-card-svg-bg">
							{#if feature.type === 'grid'}
								<svg viewBox="0 0 100 100" fill="none" stroke="currentColor" stroke-width="1">
									<path d="M10 10 H90 M10 30 H90 M10 50 H90 M10 70 H90 M10 90 H90 M10 10 V90 M30 10 V90 M50 10 V90 M70 10 V90 M90 10 V90" />
								</svg>
							{:else if feature.type === 'radar'}
								<svg viewBox="0 0 100 100" fill="none" stroke="currentColor" stroke-width="1">
									<circle cx="50" cy="50" r="20" />
									<circle cx="50" cy="50" r="35" />
									<circle cx="50" cy="50" r="45" opacity="0.5" />
									<line x1="50" y1="50" x2="95" y2="50" />
								</svg>
							{:else}
								<svg viewBox="0 0 100 100" fill="none" stroke="currentColor" stroke-width="1">
									<rect x="20" y="20" width="60" height="60" rx="4" />
									<path d="M50 20 V10 M50 90 V80 M20 50 H10 M90 50 H80" />
									<rect x="35" y="35" width="30" height="30" />
								</svg>
							{/if}
						</div>

						<div class="ws-icon-wrap">
							<div class="ws-icon-box">
								<Icon icon={iconStr} size={28} stroke={1.5} />
							</div>
							<div class="ws-icon-line"></div>
						</div>

						<div class="ws-card-content">
							<div class="ws-tag-row">
								<span class="ws-tag">{feature.subtitle}</span>
							</div>

							<h3 class="ws-card-title">{feature.title}</h3>
							<p class="ws-card-desc">{feature.description}</p>

							<div class="ws-card-footer">
								<Icon icon={IconCheck} size={12} class="ws-check-icon" />
								<span>MODULE ACTIVE</span>
							</div>
						</div>
					</div>
				{/if}
			{/each}
		</div>
	</div>
</section>

<style>
	/* ─── Section ─── */
	.ws-section {
		position: relative;
		padding-block: 6rem;
		padding-inline: 1rem;
		background-color: oklch(0.12 0.005 285);
		overflow: hidden;
		border-block-start: 1px solid oklch(0.2 0.005 285);

		@media (min-width: 640px) { padding-inline: 1.5rem; }
		@media (min-width: 1024px) { padding-block: 8rem; padding-inline: 2rem; }
	}

	/* ─── Background ─── */
	.ws-bg { position: absolute; inset: 0; pointer-events: none; }

	.ws-grid-lines {
		position: absolute;
		inset: 0;
		background-image:
			linear-gradient(to right, oklch(0.22 0.005 285) 1px, transparent 1px),
			linear-gradient(to bottom, oklch(0.22 0.005 285) 1px, transparent 1px);
		background-size: 2rem 2rem;
		mask-image: radial-gradient(ellipse 60% 50% at 50% 0%, oklch(0 0 0) 70%, transparent 100%);
		opacity: 0.2;
	}

	/* ─── Container ─── */
	.ws-container {
		position: relative;
		max-inline-size: 80rem;
		margin-inline: auto;
		z-index: 10;
	}

	.ws-header {
		max-inline-size: 56rem;
		margin-inline: auto;
		text-align: center;
		margin-block-end: 6rem;
	}

	.ws-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.75rem;
		padding-inline: 1rem;
		padding-block: 0.375rem;
		border: 1px solid oklch(0.3 0.005 285 / 0.3);
		background-color: oklch(0.15 0.005 285 / 0.1);
		color: oklch(0.6 0.01 265);
		font-size: 0.625rem;
		font-weight: var(--weight-bold);
		letter-spacing: 0.3em;
		text-transform: uppercase;
		margin-block-end: 2rem;
		border-radius: 2px;
	}

	.ws-title {
		font-size: clamp(3rem, 6vw, 4.5rem);
		font-family: var(--font-serif, serif);
		color: oklch(1 0 0);
		margin-block-end: 2rem;
		letter-spacing: -0.02em;
	}

	.ws-title-muted { color: oklch(0.35 0.01 265); }

	.ws-subtitle {
		font-size: var(--text-lg);
		color: oklch(0.55 0.01 265);
		font-weight: 300;
		line-height: 1.7;
		max-inline-size: 32rem;
		margin-inline: auto;
	}

	/* ─── Cards ─── */
	.ws-cards {
		display: grid;
		gap: 2rem;

		@media (min-width: 768px) { grid-template-columns: repeat(3, 1fr); }
	}

	.ws-card {
		position: relative;
		background-color: oklch(0.12 0.005 285 / 0.5);
		border: 1px solid oklch(0.3 0.005 285);
		border-radius: var(--radius-xl);
		padding: 2rem;
		overflow: hidden;
		transition: background-color 500ms, border-color 500ms;

		&:hover { background-color: oklch(0.15 0.005 285 / 0.3); }
		&:hover .ws-card-spotlight { opacity: 1; }
		&:hover .ws-card-svg-bg { opacity: 0.1; }
		&:hover .ws-icon-box { box-shadow: 0 0 20px oklch(0 0 0 / 0.3); transform: translateY(-0.25rem); }
		&:hover .ws-card-footer { color: oklch(0.55 0.005 285); }
	}

	/* accent: cyan */
	.ws-card[data-accent='cyan'] { &:hover .ws-icon-box { color: oklch(0.7 0.15 195); border-color: oklch(0.7 0.15 195 / 0.3); } }
	.ws-card[data-accent='cyan'] { &:hover .ws-icon-line { background-color: oklch(0.7 0.15 195 / 0.5); } }
	.ws-card[data-accent='cyan'] { &:hover .ws-card-title { color: oklch(0.7 0.15 195); } }
	.ws-card[data-accent='cyan'] .ws-card-svg-bg { color: oklch(0.7 0.15 195); }
	.ws-card[data-accent='cyan'] .ws-icon-box { color: oklch(0.7 0.15 195); }
	.ws-card[data-accent='cyan'] :global(.ws-check-icon) { color: oklch(0.7 0.15 195); }

	/* accent: emerald */
	.ws-card[data-accent='emerald'] { &:hover .ws-icon-box { color: oklch(0.7 0.17 160); border-color: oklch(0.7 0.17 160 / 0.3); } }
	.ws-card[data-accent='emerald'] { &:hover .ws-icon-line { background-color: oklch(0.7 0.17 160 / 0.5); } }
	.ws-card[data-accent='emerald'] { &:hover .ws-card-title { color: oklch(0.7 0.17 160); } }
	.ws-card[data-accent='emerald'] .ws-card-svg-bg { color: oklch(0.7 0.17 160); }
	.ws-card[data-accent='emerald'] .ws-icon-box { color: oklch(0.7 0.17 160); }
	.ws-card[data-accent='emerald'] :global(.ws-check-icon) { color: oklch(0.7 0.17 160); }

	/* accent: indigo */
	.ws-card[data-accent='indigo'] { &:hover .ws-icon-box { color: oklch(0.6 0.2 280); border-color: oklch(0.6 0.2 280 / 0.3); } }
	.ws-card[data-accent='indigo'] { &:hover .ws-icon-line { background-color: oklch(0.6 0.2 280 / 0.5); } }
	.ws-card[data-accent='indigo'] { &:hover .ws-card-title { color: oklch(0.6 0.2 280); } }
	.ws-card[data-accent='indigo'] .ws-card-svg-bg { color: oklch(0.6 0.2 280); }
	.ws-card[data-accent='indigo'] .ws-icon-box { color: oklch(0.6 0.2 280); }
	.ws-card[data-accent='indigo'] :global(.ws-check-icon) { color: oklch(0.6 0.2 280); }

	.ws-card-spotlight {
		position: absolute;
		inset: 0;
		opacity: 0;
		transition: opacity 500ms;
		pointer-events: none;
	}

	.ws-card-svg-bg {
		position: absolute;
		inset-block-start: 0;
		inset-inline-end: 0;
		inline-size: 8rem;
		block-size: 8rem;
		opacity: 0.03;
		transition: opacity 500ms;
		pointer-events: none;
	}

	/* ─── Icon ─── */
	.ws-icon-wrap {
		position: relative;
		z-index: 10;
		margin-block-end: 2rem;
		display: inline-block;
	}

	.ws-icon-box {
		padding: 0.75rem;
		background-color: oklch(0.15 0.005 285);
		border: 1px solid oklch(0.3 0.005 285);
		border-radius: var(--radius-lg);
		transition: box-shadow 300ms, transform 300ms, color 300ms, border-color 300ms;
	}

	.ws-icon-line {
		position: absolute;
		inset-inline-start: 50%;
		inset-block-end: 0;
		inline-size: 1px;
		block-size: 2rem;
		background-color: oklch(0.3 0.005 285);
		transform: translateY(100%) translateX(-50%);
		z-index: -1;
		transition: background-color 200ms;
	}

	/* ─── Content ─── */
	.ws-card-content { position: relative; z-index: 10; margin-block-start: 1rem; }

	.ws-tag-row { display: flex; align-items: center; gap: 0.75rem; margin-block-end: 0.75rem; }

	.ws-tag {
		font-size: 0.625rem;
		font-family: var(--font-mono, monospace);
		text-transform: uppercase;
		letter-spacing: 0.1em;
		color: oklch(0.45 0.005 285);
		border: 1px solid oklch(0.3 0.005 285);
		padding-inline: 0.5rem;
		padding-block: 0.125rem;
		border-radius: var(--radius-sm);
	}

	.ws-card-title {
		font-size: var(--text-xl);
		font-weight: var(--weight-medium);
		color: oklch(1 0 0);
		margin-block-end: 1rem;
		transition: color 200ms;
	}

	.ws-card-desc {
		font-size: var(--text-sm);
		color: oklch(0.55 0.01 265);
		line-height: 1.7;
		font-weight: 300;
		margin-block-end: 2rem;
	}

	.ws-card-footer {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.625rem;
		font-family: var(--font-mono, monospace);
		color: oklch(0.4 0.005 285);
		border-block-start: 1px solid oklch(0.2 0.005 285);
		padding-block-start: 1rem;
		transition: color 200ms;
	}
</style>
