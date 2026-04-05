<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { cubicOut } from 'svelte/easing';
	import { Icon, IconArrowRight, IconDatabase, IconShieldLock, IconSitemap } from '$lib/icons';
	// Using the direct path imports as requested in your snippet
				
	const features = [
		{
			id: 'SYS-01',
			title: 'Execution Framework',
			subtitle: 'STRUCTURE',
			description:
				'We replace discretionary guessing with a repeatable, probabilistic execution model used by proprietary desks.',
			icon: IconSitemap,
			status: 'Operational',
			type: 'grid'
		},
		{
			id: 'SYS-02',
			title: 'Variance Control',
			subtitle: 'SURVIVAL',
			description:
				'Capital preservation is the mandate. Every alert includes hard invalidation points and fat-tail risk sizing logic.',
			icon: IconShieldLock,
			status: 'Active',
			type: 'radar'
		},
		{
			id: 'SYS-03',
			title: 'Data Sovereignty',
			subtitle: 'INTELLIGENCE',
			description:
				'Direct access to institutional flows. We track Dark Pool volume, GEX levels, and Vanna flows in real-time.',
			icon: IconDatabase,
			status: 'Live Feed',
			type: 'circuit'
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
	class="ms-section"
>
	<div class="ms-bg">
		<div class="ms-grid-lines"></div>
	</div>

	<div class="ms-container">
		<div class="ms-header">
			{#if isVisible}
				<div in:heavySlide={{ delay: 0, duration: 1000 }} class="ms-badge">
					<span class="ms-ping-wrap">
						<span class="ms-ping"></span>
						<span class="ms-ping-dot"></span>
					</span>
					System Architecture
				</div>

				<h2 in:heavySlide={{ delay: 100 }} class="ms-title">
					Core <span class="ms-title-muted">Infrastructure.</span>
				</h2>

				<p in:heavySlide={{ delay: 200 }} class="ms-subtitle">
					We replaced marketing hype with financial engineering. Our ecosystem combines structured
					execution frameworks with institutional data tools.
				</p>
			{/if}
		</div>

		<div class="ms-cards" style="--x: {mouse.x}px; --y: {mouse.y}px;">
			{#each features as feature, i}
				{@const iconStr = feature.icon}
				{#if isVisible}
					<div in:heavySlide={{ delay: 300 + i * 150 }} class="ms-card">
						<div
							class="ms-card-spotlight"
							style="background: radial-gradient(800px circle at var(--x) var(--y), oklch(1 0 0 / 0.03), transparent 40%);"
						></div>

						<div class="ms-card-svg-bg">
							{#if feature.type === 'grid'}
								<svg viewBox="0 0 100 100" fill="none" stroke="currentColor" stroke-width="0.5">
									<path d="M10 10 H90 M10 30 H90 M10 50 H90 M10 70 H90 M10 90 H90 M10 10 V90 M30 10 V90 M50 10 V90 M70 10 V90 M90 10 V90" />
								</svg>
							{:else if feature.type === 'radar'}
								<svg viewBox="0 0 100 100" fill="none" stroke="currentColor" stroke-width="0.5">
									<circle cx="50" cy="50" r="20" />
									<circle cx="50" cy="50" r="35" />
									<circle cx="50" cy="50" r="45" opacity="0.5" />
									<line x1="50" y1="50" x2="95" y2="50" />
								</svg>
							{:else}
								<svg viewBox="0 0 100 100" fill="none" stroke="currentColor" stroke-width="0.5">
									<rect x="20" y="20" width="60" height="60" rx="4" />
									<path d="M50 20 V10 M50 90 V80 M20 50 H10 M90 50 H80" />
									<rect x="35" y="35" width="30" height="30" />
								</svg>
							{/if}
						</div>

						<div class="ms-card-top">
							<div class="ms-icon-box">
								<Icon icon={iconStr} size={24} stroke={1.5} />
							</div>
							<span class="ms-card-id">{feature.id}</span>
						</div>

						<div class="ms-card-content">
							<div class="ms-card-subtitle">{feature.subtitle}</div>
							<h3 class="ms-card-title">{feature.title}</h3>
							<p class="ms-card-desc">{feature.description}</p>

							<div class="ms-card-footer">
								<div class="ms-status">
									<span class="ms-status-dot"></span>
									<span class="ms-status-label">{feature.status}</span>
								</div>
								<div class="ms-arrow-wrap">
									<Icon icon={IconArrowRight} size={16} class="ms-arrow-icon" />
								</div>
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
	.ms-section {
		position: relative;
		padding-block: 8rem;
		padding-inline: 1.5rem;
		background-color: oklch(0.08 0 0);
		overflow: hidden;
		border-block-end: 1px solid oklch(1 0 0 / 0.05);
	}

	/* ─── Background ─── */
	.ms-bg { position: absolute; inset: 0; pointer-events: none; }

	.ms-grid-lines {
		position: absolute;
		inset: 0;
		background-image:
			linear-gradient(oklch(1 0 0 / 0.03) 1px, transparent 1px),
			linear-gradient(90deg, oklch(1 0 0 / 0.03) 1px, transparent 1px);
		background-size: 4rem 4rem;
		mask-image: radial-gradient(ellipse 60% 50% at 50% 0%, oklch(0 0 0) 70%, transparent 100%);
	}

	/* ─── Container ─── */
	.ms-container {
		position: relative;
		max-inline-size: 100rem;
		margin-inline: auto;
		z-index: 10;
	}

	.ms-header { max-inline-size: 48rem; margin-block-end: 6rem; }

	.ms-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.75rem;
		padding-inline: 0.75rem;
		padding-block: 0.25rem;
		border: 1px solid oklch(0.45 0.12 70 / 0.3);
		background-color: oklch(0.45 0.12 70 / 0.1);
		color: oklch(0.75 0.15 85);
		font-size: 0.625rem;
		font-weight: var(--weight-bold);
		letter-spacing: 0.3em;
		text-transform: uppercase;
		margin-block-end: 2rem;
	}

	.ms-ping-wrap { position: relative; display: flex; block-size: 0.5rem; inline-size: 0.5rem; }
	.ms-ping { position: absolute; display: inline-flex; block-size: 100%; inline-size: 100%; border-radius: 50%; background-color: oklch(0.75 0.15 85); opacity: 0.75; animation: ping 1s cubic-bezier(0, 0, 0.2, 1) infinite; }
	.ms-ping-dot { position: relative; display: inline-flex; border-radius: 50%; block-size: 0.5rem; inline-size: 0.5rem; background-color: oklch(0.75 0.15 85); }

	.ms-title {
		font-size: clamp(2.25rem, 5vw, 3.75rem);
		font-family: var(--font-serif, serif);
		color: oklch(1 0 0);
		margin-block-end: 2rem;
		letter-spacing: -0.02em;
	}

	.ms-title-muted { color: oklch(0.35 0.01 265); }

	.ms-subtitle {
		font-size: var(--text-lg);
		color: oklch(0.55 0.01 265);
		font-weight: 300;
		line-height: 1.7;
		max-inline-size: 32rem;
		border-inline-start: 2px solid oklch(1 0 0 / 0.1);
		padding-inline-start: 1.5rem;
	}

	/* ─── Cards ─── */
	.ms-cards {
		display: grid;
		gap: 2rem;

		@media (min-width: 768px) { grid-template-columns: repeat(3, 1fr); }
	}

	.ms-card {
		position: relative;
		background-color: oklch(0.1 0 0);
		border: 1px solid oklch(1 0 0 / 0.1);
		padding: 2.5rem;
		overflow: hidden;
		transition: border-color 500ms;

		&:hover { border-color: oklch(0.45 0.12 70 / 0.5); }
		&:hover .ms-card-spotlight { opacity: 1; }
		&:hover .ms-card-svg-bg { opacity: 0.1; }
		&:hover .ms-icon-box { background-color: oklch(0.75 0.15 85); color: oklch(0 0 0); border-color: oklch(0.75 0.15 85); }
		&:hover .ms-status-label { color: oklch(1 0 0); }
		&:hover .ms-arrow-wrap { opacity: 1; transform: translateX(0); }
	}

	.ms-card-spotlight {
		position: absolute;
		inset: 0;
		opacity: 0;
		transition: opacity 500ms;
		pointer-events: none;
	}

	.ms-card-svg-bg {
		position: absolute;
		inset-block-start: 0;
		inset-inline-end: 0;
		inline-size: 10rem;
		block-size: 10rem;
		opacity: 0.02;
		transition: opacity 500ms;
		pointer-events: none;
		color: oklch(1 0 0);
	}

	/* ─── Card Top ─── */
	.ms-card-top {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-block-end: 3rem;
		position: relative;
		z-index: 10;
	}

	.ms-icon-box {
		inline-size: 3rem;
		block-size: 3rem;
		display: flex;
		align-items: center;
		justify-content: center;
		background-color: oklch(1 0 0 / 0.05);
		border: 1px solid oklch(1 0 0 / 0.1);
		color: oklch(1 0 0);
		transition: background-color 300ms, color 300ms, border-color 300ms;
	}

	.ms-card-id {
		font-family: var(--font-mono, monospace);
		font-size: 0.625rem;
		color: oklch(0.4 0.01 265);
		text-transform: uppercase;
		letter-spacing: 0.1em;
	}

	/* ─── Card Content ─── */
	.ms-card-content { position: relative; z-index: 10; }

	.ms-card-subtitle {
		font-size: 0.625rem;
		font-family: var(--font-mono, monospace);
		text-transform: uppercase;
		letter-spacing: 0.1em;
		color: oklch(0.6 0.15 70);
		margin-block-end: 0.75rem;
	}

	.ms-card-title {
		font-size: 1.5rem;
		font-family: var(--font-serif, serif);
		color: oklch(1 0 0);
		margin-block-end: 1.5rem;
		transition: color 200ms;
	}

	.ms-card-desc {
		font-size: var(--text-sm);
		color: oklch(0.55 0.01 265);
		line-height: 1.7;
		font-weight: 300;
		margin-block-end: 3rem;
		block-size: 4rem;
	}

	.ms-card-footer {
		display: flex;
		align-items: center;
		justify-content: space-between;
		border-block-start: 1px solid oklch(1 0 0 / 0.1);
		padding-block-start: 1.5rem;
	}

	.ms-status { display: flex; align-items: center; gap: 0.5rem; }

	.ms-status-dot {
		inline-size: 0.375rem;
		block-size: 0.375rem;
		border-radius: 50%;
		background-color: oklch(0.7 0.17 160);
	}

	.ms-status-label {
		font-size: 0.625rem;
		font-family: var(--font-mono, monospace);
		color: oklch(0.55 0.01 265);
		text-transform: uppercase;
		letter-spacing: 0.1em;
		transition: color 200ms;
	}

	.ms-arrow-wrap {
		opacity: 0;
		transform: translateX(-0.5rem);
		transition: opacity 300ms, transform 300ms;
	}

	:global(.ms-arrow-icon) { color: oklch(1 0 0); }

	/* ─── Keyframes ─── */
	@keyframes ping {
		75%, 100% { transform: scale(2); opacity: 0; }
	}
</style>
