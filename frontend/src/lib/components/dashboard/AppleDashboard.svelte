<script lang="ts">
	import { onMount } from 'svelte';
	import { fly, fade, scale } from 'svelte/transition';
	import { cubicOut, elasticOut, backOut } from 'svelte/easing';
	import { tweened, spring } from 'svelte/motion';
	import { scrollReveal, tilt3d, magnetic, parallax } from '$lib/animations/appleAnimations';

	// Title available for dashboard customization
	export const title = 'Dashboard';
	export let subtitle = 'Welcome back';

	let mounted = false;
	let scrollY = 0;
	let innerHeight = 0;

	// Animated values
	const headerOpacity = tweened(0, { duration: 800, easing: cubicOut });
	const headerScale = tweened(0.95, { duration: 800, easing: cubicOut });
	const greeting = tweened(0, { duration: 1200, easing: cubicOut });

	// Spring values for smooth interactions
	const cursorX = spring(0, { stiffness: 0.02, damping: 0.15 });
	const cursorY = spring(0, { stiffness: 0.02, damping: 0.15 });

	onMount(() => {
		mounted = true;
		headerOpacity.set(1);
		headerScale.set(1);
		greeting.set(1);

		// Track cursor for gradient effect
		const handleMouseMove = (e: MouseEvent) => {
			cursorX.set(e.clientX);
			cursorY.set(e.clientY);
		};

		window.addEventListener('mousemove', handleMouseMove);
		return () => window.removeEventListener('mousemove', handleMouseMove);
	});

	function getTimeGreeting(): string {
		const hour = new Date().getHours();
		if (hour < 12) return 'Good morning';
		if (hour < 17) return 'Good afternoon';
		return 'Good evening';
	}

	$: parallaxOffset = Math.min(scrollY * 0.3, 100);
	$: headerBlur = Math.min(scrollY / 10, 20);
</script>

<svelte:window bind:scrollY bind:innerHeight />

<div class="apple-dashboard">
	<!-- Animated Background Gradient -->
	<div
		class="background-gradient"
		style="--cursor-x: {$cursorX}px; --cursor-y: {$cursorY}px;"
	></div>

	<!-- Hero Header Section -->
	<header
		class="dashboard-header"
		style="
      opacity: {$headerOpacity};
      transform: scale({$headerScale}) translateY({-parallaxOffset}px);
      backdrop-filter: blur({headerBlur}px);
    "
	>
		<div class="header-content">
			<div class="greeting-section">
				{#if mounted}
					<h1 class="greeting-text" in:fly={{ y: 30, duration: 800, delay: 200, easing: cubicOut }}>
						{getTimeGreeting()}
					</h1>
					<p class="subtitle" in:fly={{ y: 20, duration: 600, delay: 400, easing: cubicOut }}>
						{subtitle}
					</p>
				{/if}
			</div>

			<div class="header-actions">
				{#if mounted}
					<button
						class="action-button primary"
						use:magnetic={{ strength: 0.2 }}
						in:scale={{ start: 0.8, duration: 500, delay: 600, easing: backOut }}
					>
						<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<line x1="12" y1="5" x2="12" y2="19" />
							<line x1="5" y1="12" x2="19" y2="12" />
						</svg>
						Add Widget
					</button>
					<button
						class="action-button secondary"
						use:magnetic={{ strength: 0.2 }}
						in:scale={{ start: 0.8, duration: 500, delay: 700, easing: backOut }}
					>
						<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<circle cx="12" cy="12" r="3" />
							<path
								d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"
							/>
						</svg>
						Settings
					</button>
				{/if}
			</div>
		</div>

		<!-- Animated Stat Highlights -->
		<div class="quick-stats">
			{#if mounted}
				{#each [{ label: 'Total Revenue', value: '$124,592', change: '+12.5%', positive: true }, { label: 'Active Users', value: '8,492', change: '+23.1%', positive: true }, { label: 'Conversion Rate', value: '3.24%', change: '+0.5%', positive: true }, { label: 'Avg. Session', value: '4m 32s', change: '-8.2%', positive: false }] as stat, i}
					<div
						class="stat-card"
						use:tilt3d={{ intensity: 8, scale: 1.02, glare: true }}
						use:scrollReveal={{ type: 'scale-up', delay: 200 + i * 100 }}
					>
						<span class="stat-label">{stat.label}</span>
						<span class="stat-value">{stat.value}</span>
						<span class="stat-change" class:positive={stat.positive}>
							{stat.change}
						</span>
					</div>
				{/each}
			{/if}
		</div>
	</header>

	<!-- Main Dashboard Grid -->
	<main class="dashboard-main">
		<slot />
	</main>
</div>

<style>
	.apple-dashboard {
		min-height: 100vh;
		position: relative;
		overflow-x: hidden;
	}

	/* Dynamic Gradient Background - Apple Style */
	.background-gradient {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: linear-gradient(135deg, #f5f5f7 0%, #fafafa 50%, #f0f0f2 100%);
		z-index: -2;
	}

	.background-gradient::before {
		content: '';
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: radial-gradient(
			800px circle at var(--cursor-x, 50%) var(--cursor-y, 50%),
			rgba(79, 70, 229, 0.06) 0%,
			transparent 40%
		);
		transition: opacity 0.3s ease;
		pointer-events: none;
	}

	/* Header */
	.dashboard-header {
		position: sticky;
		top: 0;
		z-index: 100;
		padding: 2rem 2.5rem;
		background: rgba(255, 255, 255, 0.8);
		border-bottom: 1px solid rgba(0, 0, 0, 0.06);
		will-change: transform, opacity;
	}

	.header-content {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 2rem;
	}

	.greeting-section {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.greeting-text {
		font-size: 2.5rem;
		font-weight: 700;
		background: linear-gradient(135deg, #1d1d1f 0%, #424245 100%);
		-webkit-background-clip: text;
		background-clip: text;
		-webkit-text-fill-color: transparent;
		letter-spacing: -0.02em;
		line-height: 1.1;
	}

	.subtitle {
		font-size: 1.125rem;
		color: #86868b;
		font-weight: 400;
	}

	.header-actions {
		display: flex;
		gap: 0.75rem;
	}

	.action-button {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		border-radius: 980px;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.3s cubic-bezier(0.25, 0.1, 0.25, 1);
		border: none;
	}

	.action-button svg {
		width: 16px;
		height: 16px;
	}

	.action-button.primary {
		background: linear-gradient(135deg, #0071e3 0%, #42a5f5 100%);
		color: white;
		box-shadow: 0 4px 14px rgba(0, 113, 227, 0.3);
	}

	.action-button.primary:hover {
		box-shadow: 0 6px 20px rgba(0, 113, 227, 0.4);
		transform: translateY(-2px);
	}

	.action-button.secondary {
		background: rgba(0, 0, 0, 0.06);
		color: #1d1d1f;
	}

	.action-button.secondary:hover {
		background: rgba(0, 0, 0, 0.1);
	}

	/* Quick Stats - Apple Product Card Style */
	.quick-stats {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
	}

	.stat-card {
		background: white;
		border-radius: 20px;
		padding: 1.5rem;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		box-shadow:
			0 2px 8px rgba(0, 0, 0, 0.04),
			0 12px 40px rgba(0, 0, 0, 0.06);
		cursor: pointer;
		transform-style: preserve-3d;
		will-change: transform;
	}

	.stat-label {
		font-size: 0.75rem;
		color: #86868b;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		font-weight: 600;
	}

	.stat-value {
		font-size: 1.75rem;
		font-weight: 700;
		color: #1d1d1f;
		letter-spacing: -0.02em;
	}

	.stat-change {
		font-size: 0.875rem;
		font-weight: 600;
		color: #ff3b30;
	}

	.stat-change.positive {
		color: #34c759;
	}

	/* Main Content */
	.dashboard-main {
		padding: 2rem 2.5rem;
		min-height: calc(100vh - 300px);
	}

	/* Responsive */
	@media (max-width: 1024px) {
		.quick-stats {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 768px) {
		.dashboard-header {
			padding: 1.5rem;
		}

		.header-content {
			flex-direction: column;
			gap: 1.5rem;
		}

		.greeting-text {
			font-size: 1.75rem;
		}

		.quick-stats {
			grid-template-columns: 1fr;
		}

		.dashboard-main {
			padding: 1.5rem;
		}
	}

	/* Dark mode support */
	@media (prefers-color-scheme: dark) {
		.background-gradient {
			background: linear-gradient(135deg, #1d1d1f 0%, #2c2c2e 50%, #1d1d1f 100%);
		}

		.dashboard-header {
			background: rgba(29, 29, 31, 0.8);
			border-bottom-color: rgba(255, 255, 255, 0.1);
		}

		.greeting-text {
			background: linear-gradient(135deg, #ffffff 0%, #a1a1a6 100%);
			-webkit-background-clip: text;
			background-clip: text;
		}

		.stat-card {
			background: #2c2c2e;
			box-shadow: 0 12px 40px rgba(0, 0, 0, 0.3);
		}

		.stat-value {
			color: #f5f5f7;
		}

		.action-button.secondary {
			background: rgba(255, 255, 255, 0.1);
			color: #f5f5f7;
		}
	}
</style>
