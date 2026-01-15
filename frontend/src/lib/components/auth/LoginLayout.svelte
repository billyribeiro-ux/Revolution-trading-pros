<script lang="ts">
	/**
	 * LoginLayout - Premium Split-Panel Auth Layout
	 * Netflix L11+ Principal Engineer Grade
	 *
	 * Features:
	 * - Responsive split-panel design (hero + form)
	 * - Desktop: 60/40 split with trading hero
	 * - Tablet: 50/50 split
	 * - Mobile: Full-width form with simplified background
	 * - Theme-aware (light/dark)
	 *
	 * @version 1.0.0
	 */
	import type { Snippet } from 'svelte';
	import { browser } from '$app/environment';
	import { onMount } from 'svelte';
	import MobileBackground from './MobileBackground.svelte';

	interface Props {
		children: Snippet;
		heroContent?: Snippet;
	}

	let { children, heroContent }: Props = $props();

	// Mouse position for spotlight effect
	let mouseX = $state(50);
	let mouseY = $state(50);
	let isMobile = $state(false);

	onMount(() => {
		if (!browser) return;

		// Check for mobile viewport
		const checkMobile = () => {
			isMobile = window.innerWidth < 1024;
		};
		checkMobile();
		window.addEventListener('resize', checkMobile);

		const handleMouseMove = (e: MouseEvent) => {
			mouseX = (e.clientX / window.innerWidth) * 100;
			mouseY = (e.clientY / window.innerHeight) * 100;
		};

		window.addEventListener('mousemove', handleMouseMove);
		return () => {
			window.removeEventListener('mousemove', handleMouseMove);
			window.removeEventListener('resize', checkMobile);
		};
	});
</script>

<div
	class="login-layout"
	style="--mouse-x: {mouseX}%; --mouse-y: {mouseY}%;"
>
	<!-- Mobile Background (shown on mobile only) -->
	{#if isMobile}
		<MobileBackground />
	{/if}

	<!-- Background Grid -->
	<div class="grid-overlay" aria-hidden="true"></div>

	<!-- Spotlight Effect -->
	<div class="spotlight" aria-hidden="true"></div>

	<!-- Hero Panel (Desktop/Tablet) -->
	<div class="hero-panel">
		{#if heroContent}
			{@render heroContent()}
		{/if}
	</div>

	<!-- Form Panel -->
	<div class="form-panel">
		<div class="form-container">
			{@render children()}
		</div>
	</div>
</div>

<style>
	/* Main Layout - ICT11+ Animation-First Architecture */
	.login-layout {
		position: relative;
		min-height: 100vh;
		display: grid;
		grid-template-columns: 1fr;
		/* ICT11+ Pattern: Allow spotlight glow to extend beyond bounds */
		overflow-x: hidden;
		overflow-y: auto;
		background: var(--auth-bg);
	}

	/* Desktop: Split panel layout */
	@media (min-width: 1024px) {
		.login-layout {
			grid-template-columns: 1.2fr 1fr;
		}
	}

	/* Grid Background */
	.grid-overlay {
		position: fixed;
		inset: 0;
		background-image:
			linear-gradient(var(--auth-grid-line) 1px, transparent 1px),
			linear-gradient(90deg, var(--auth-grid-line) 1px, transparent 1px);
		background-size: 60px 60px;
		opacity: 0.4;
		pointer-events: none;
		z-index: 1;
	}

	/* Spotlight Effect */
	.spotlight {
		position: fixed;
		inset: 0;
		background: radial-gradient(
			circle 800px at var(--mouse-x) var(--mouse-y),
			var(--auth-spotlight),
			transparent 70%
		);
		pointer-events: none;
		z-index: 2;
		transition: background 0.3s ease;
	}

	/* Hero Panel */
	.hero-panel {
		position: relative;
		display: none;
		background: var(--auth-hero-bg);
		overflow: hidden;
		z-index: 3;
	}

	@media (min-width: 1024px) {
		.hero-panel {
			display: flex;
			align-items: center;
			justify-content: center;
		}
	}

	/* Form Panel */
	.form-panel {
		position: relative;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 1.5rem;
		z-index: 10;
		background: var(--auth-form-bg);
	}

	@media (min-width: 640px) {
		.form-panel {
			padding: 2rem;
		}
	}

	@media (min-width: 1024px) {
		.form-panel {
			padding: 3rem;
		}
	}

	/* Form Container */
	.form-container {
		width: 100%;
		max-width: 440px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * CSS Custom Properties - Auth Theme Tokens
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Dark Theme (Default) */
	.login-layout {
		--auth-bg: oklch(0.10 0.02 250);
		--auth-form-bg: oklch(0.12 0.02 250);
		--auth-hero-bg: linear-gradient(135deg, oklch(0.08 0.02 250) 0%, oklch(0.12 0.03 270) 100%);
		--auth-grid-line: rgba(230, 184, 0, 0.04);
		--auth-spotlight: rgba(230, 184, 0, 0.08);

		/* Surface Colors */
		--auth-card-bg: rgba(15, 23, 42, 0.6);
		--auth-card-border: rgba(230, 184, 0, 0.15);
		--auth-card-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);

		/* Input Colors */
		--auth-input-bg: rgba(15, 23, 42, 0.5);
		--auth-input-border: rgba(100, 116, 139, 0.3);
		--auth-input-border-focus: #E6B800;
		--auth-input-text: #f1f5f9;
		--auth-input-placeholder: #475569;

		/* Text Colors */
		--auth-heading: linear-gradient(135deg, #FFD11A, #E6B800, #B38F00);
		--auth-subheading: #94a3b8;
		--auth-text: #e2e8f0;
		--auth-muted: #64748b;
		--auth-link: #FFD11A;
		--auth-link-hover: #E6B800;

		/* Button Colors */
		--auth-btn-primary-bg: linear-gradient(135deg, #E6B800 0%, #B38F00 100%);
		--auth-btn-primary-text: #0D1117;
		--auth-btn-primary-shadow: 0 10px 25px -5px rgba(230, 184, 0, 0.4);
		--auth-btn-primary-shadow-hover: 0 20px 40px -12px rgba(230, 184, 0, 0.6);

		/* Semantic Colors */
		--auth-error: #f87171;
		--auth-error-bg: rgba(239, 68, 68, 0.1);
		--auth-error-border: rgba(239, 68, 68, 0.3);
		--auth-success: #4ade80;
		--auth-success-bg: rgba(74, 222, 128, 0.1);

		/* Trading Colors */
		--auth-bull: #22c55e;
		--auth-bull-soft: rgba(34, 197, 94, 0.15);
		--auth-bear: #ef4444;
		--auth-bear-soft: rgba(239, 68, 68, 0.15);

		/* Glow Effects */
		--auth-glow-primary: rgba(230, 184, 0, 0.3);
		--auth-glow-success: rgba(34, 197, 94, 0.3);
	}

	/* Light Theme */
	:global(html.light) .login-layout,
	:global(body.light) .login-layout {
		--auth-bg: #f5f5f7;
		--auth-form-bg: #ffffff;
		--auth-hero-bg: linear-gradient(135deg, #f8fafc 0%, #FFF8E1 100%);
		--auth-grid-line: rgba(230, 184, 0, 0.06);
		--auth-spotlight: rgba(230, 184, 0, 0.06);

		/* Surface Colors */
		--auth-card-bg: #ffffff;
		--auth-card-border: rgba(230, 184, 0, 0.15);
		--auth-card-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.1);

		/* Input Colors */
		--auth-input-bg: #ffffff;
		--auth-input-border: #d2d2d7;
		--auth-input-border-focus: #B38F00;
		--auth-input-text: #1d1d1f;
		--auth-input-placeholder: #86868b;

		/* Text Colors */
		--auth-heading: linear-gradient(135deg, #B38F00, #E6B800, #B38F00);
		--auth-subheading: #6e6e73;
		--auth-text: #424245;
		--auth-muted: #86868b;
		--auth-link: #B38F00;
		--auth-link-hover: #8A6F00;

		/* Button Colors */
		--auth-btn-primary-bg: linear-gradient(135deg, #E6B800 0%, #B38F00 100%);
		--auth-btn-primary-text: #0D1117;
		--auth-btn-primary-shadow: 0 10px 25px -5px rgba(230, 184, 0, 0.3);
		--auth-btn-primary-shadow-hover: 0 20px 40px -12px rgba(230, 184, 0, 0.4);

		/* Semantic Colors */
		--auth-error: #dc2626;
		--auth-error-bg: rgba(220, 38, 38, 0.08);
		--auth-error-border: rgba(220, 38, 38, 0.2);
		--auth-success: #059669;
		--auth-success-bg: rgba(5, 150, 105, 0.08);

		/* Trading Colors */
		--auth-bull: #059669;
		--auth-bull-soft: rgba(5, 150, 105, 0.1);
		--auth-bear: #dc2626;
		--auth-bear-soft: rgba(220, 38, 38, 0.1);

		/* Glow Effects */
		--auth-glow-primary: rgba(230, 184, 0, 0.2);
		--auth-glow-success: rgba(5, 150, 105, 0.2);
	}
</style>
