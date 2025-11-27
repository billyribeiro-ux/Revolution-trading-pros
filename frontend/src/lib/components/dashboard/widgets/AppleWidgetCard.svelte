<script lang="ts">
	import { onMount } from 'svelte';
	import { fly, scale } from 'svelte/transition';
	import { cubicOut, backOut } from 'svelte/easing';
	import { spring } from 'svelte/motion';
	import { tilt3d } from '$lib/animations/appleAnimations';

	export let title = 'Widget';
	export let subtitle = '';
	export let icon: 'chart' | 'users' | 'revenue' | 'globe' | 'search' | 'analytics' | 'custom' =
		'chart';
	export let accentColor = '#0071e3';
	export let size: 'small' | 'medium' | 'large' = 'medium';
	export let loading = false;
	export let interactive = true;
	export let animationDelay = 0;

	let mounted = false;
	let isHovered = false;

	const hoverScale = spring(1, { stiffness: 0.1, damping: 0.4 });

	onMount(() => {
		mounted = true;
	});

	function handleMouseEnter() {
		isHovered = true;
		if (interactive) hoverScale.set(1.02);
	}

	function handleMouseLeave() {
		isHovered = false;
		if (interactive) hoverScale.set(1);
	}

	const icons = {
		chart: `<path d="M18 20V10" /><path d="M12 20V4" /><path d="M6 20v-6" />`,
		users: `<path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" /><circle cx="9" cy="7" r="4" /><path d="M22 21v-2a4 4 0 0 0-3-3.87" /><path d="M16 3.13a4 4 0 0 1 0 7.75" />`,
		revenue: `<line x1="12" y1="1" x2="12" y2="23" /><path d="M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6" />`,
		globe: `<circle cx="12" cy="12" r="10" /><line x1="2" y1="12" x2="22" y2="12" /><path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z" />`,
		search: `<circle cx="11" cy="11" r="8" /><line x1="21" y1="21" x2="16.65" y2="16.65" />`,
		analytics: `<path d="M21 21H4.6c-.6 0-1.1-.4-1.1-1V3" /><path d="m14 15 3-3 4 4" /><path d="m10 18-2.5-2.5L3 20" /><path d="M21 11V6l-4 4" />`,
		custom: ``
	};
</script>

{#if mounted}
	<article
		class="apple-widget-card {size}"
		class:loading
		class:interactive
		class:hovered={isHovered}
		style="
      --accent-color: {accentColor};
      --accent-glow: {accentColor}33;
      transform: scale({$hoverScale});
    "
		on:mouseenter={handleMouseEnter}
		on:mouseleave={handleMouseLeave}
		use:tilt3d={{ intensity: interactive ? 6 : 0, scale: 1, glare: interactive }}
		in:fly={{ y: 40, duration: 600, delay: animationDelay, easing: cubicOut }}
	>
		<!-- Loading Overlay -->
		{#if loading}
			<div class="loading-overlay" in:scale={{ start: 0.9, duration: 300 }}>
				<div class="loading-spinner"></div>
			</div>
		{/if}

		<!-- Header -->
		<header class="widget-header">
			<div class="header-left">
				{#if icon !== 'custom'}
					<div class="icon-container" style="background: {accentColor}15;">
						<svg
							viewBox="0 0 24 24"
							fill="none"
							stroke={accentColor}
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
						>
							{@html icons[icon]}
						</svg>
					</div>
				{/if}
				<div class="title-group">
					<h3 class="widget-title">{title}</h3>
					{#if subtitle}
						<span class="widget-subtitle">{subtitle}</span>
					{/if}
				</div>
			</div>

			<div class="header-right">
				<slot name="header-actions">
					<button class="more-button" aria-label="More options">
						<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<circle cx="12" cy="12" r="1" />
							<circle cx="12" cy="5" r="1" />
							<circle cx="12" cy="19" r="1" />
						</svg>
					</button>
				</slot>
			</div>
		</header>

		<!-- Content -->
		<div class="widget-content">
			<slot />
		</div>

		<!-- Footer (optional) -->
		{#if $$slots.footer}
			<footer class="widget-footer">
				<slot name="footer" />
			</footer>
		{/if}

		<!-- Accent glow effect on hover -->
		<div class="accent-glow" class:visible={isHovered}></div>
	</article>
{/if}

<style>
	.apple-widget-card {
		position: relative;
		background: white;
		border-radius: 24px;
		overflow: hidden;
		display: flex;
		flex-direction: column;
		box-shadow:
			0 2px 8px rgba(0, 0, 0, 0.04),
			0 12px 40px rgba(0, 0, 0, 0.06);
		transition:
			box-shadow 0.4s cubic-bezier(0.25, 0.1, 0.25, 1),
			border-color 0.4s ease;
		border: 1px solid rgba(0, 0, 0, 0.04);
		transform-style: preserve-3d;
		will-change: transform, box-shadow;
	}

	.apple-widget-card.interactive:hover {
		box-shadow:
			0 4px 16px rgba(0, 0, 0, 0.06),
			0 20px 60px rgba(0, 0, 0, 0.1);
		border-color: var(--accent-color);
	}

	/* Size variants */
	.apple-widget-card.small {
		min-height: 200px;
	}

	.apple-widget-card.medium {
		min-height: 320px;
	}

	.apple-widget-card.large {
		min-height: 480px;
	}

	/* Loading state */
	.loading-overlay {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: rgba(255, 255, 255, 0.9);
		backdrop-filter: blur(4px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 10;
	}

	.loading-spinner {
		width: 32px;
		height: 32px;
		border: 3px solid #f3f4f6;
		border-top-color: var(--accent-color);
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Header */
	.widget-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		padding: 1.25rem 1.5rem;
		border-bottom: 1px solid rgba(0, 0, 0, 0.04);
	}

	.header-left {
		display: flex;
		align-items: center;
		gap: 0.875rem;
	}

	.icon-container {
		width: 40px;
		height: 40px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.icon-container svg {
		width: 20px;
		height: 20px;
	}

	.title-group {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.widget-title {
		font-size: 1rem;
		font-weight: 600;
		color: #1d1d1f;
		letter-spacing: -0.01em;
		margin: 0;
	}

	.widget-subtitle {
		font-size: 0.75rem;
		color: #86868b;
	}

	.header-right {
		display: flex;
		gap: 0.5rem;
	}

	.more-button {
		width: 32px;
		height: 32px;
		border-radius: 8px;
		background: transparent;
		border: none;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #86868b;
		transition: all 0.2s ease;
	}

	.more-button:hover {
		background: rgba(0, 0, 0, 0.05);
		color: #1d1d1f;
	}

	.more-button svg {
		width: 16px;
		height: 16px;
	}

	/* Content */
	.widget-content {
		flex: 1;
		padding: 1.25rem 1.5rem;
		overflow: hidden;
	}

	/* Footer */
	.widget-footer {
		padding: 1rem 1.5rem;
		border-top: 1px solid rgba(0, 0, 0, 0.04);
		background: rgba(0, 0, 0, 0.01);
	}

	/* Accent glow effect */
	.accent-glow {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		height: 3px;
		background: linear-gradient(90deg, transparent, var(--accent-color), transparent);
		opacity: 0;
		transition: opacity 0.4s ease;
	}

	.accent-glow.visible {
		opacity: 1;
	}

	/* Dark mode */
	@media (prefers-color-scheme: dark) {
		.apple-widget-card {
			background: #1c1c1e;
			border-color: rgba(255, 255, 255, 0.08);
		}

		.apple-widget-card.interactive:hover {
			box-shadow:
				0 4px 16px rgba(0, 0, 0, 0.2),
				0 20px 60px rgba(0, 0, 0, 0.4);
		}

		.widget-header {
			border-bottom-color: rgba(255, 255, 255, 0.08);
		}

		.widget-title {
			color: #f5f5f7;
		}

		.loading-overlay {
			background: rgba(28, 28, 30, 0.9);
		}

		.more-button:hover {
			background: rgba(255, 255, 255, 0.1);
			color: #f5f5f7;
		}

		.widget-footer {
			background: rgba(255, 255, 255, 0.02);
			border-top-color: rgba(255, 255, 255, 0.08);
		}
	}
</style>
