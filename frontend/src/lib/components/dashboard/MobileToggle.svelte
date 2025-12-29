<script lang="ts">
	/**
	 * MobileToggle - Simpler Trading Style
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Svelte 5 component for the mobile dashboard menu toggle:
	 * - Fixed bottom bar (#0d2532)
	 * - Hamburger to X animation
	 * - Controls sidebar visibility
	 *
	 * @version 1.0.0 - Svelte 5 with $props()
	 */
	import { browser } from '$app/environment';

	interface Props {
		/** Whether menu is open */
		isOpen?: boolean;
		/** Label text */
		label?: string;
		/** Callback when toggled */
		onToggle?: () => void;
	}

	let {
		isOpen = false,
		label = 'Dashboard Menu',
		onToggle
	}: Props = $props();

	function handleToggle() {
		if (browser) {
			document.documentElement.classList.toggle('html--dashboard-menu-open', !isOpen);
		}
		onToggle?.();
	}

	function handleOverlayClick() {
		if (browser) {
			document.documentElement.classList.remove('html--dashboard-menu-open');
		}
		onToggle?.();
	}
</script>

<!-- Toggle Bar -->
<footer class="mobile-toggle">
	<button
		type="button"
		class="mobile-toggle__button"
		onclick={handleToggle}
		aria-expanded={isOpen}
		aria-label={isOpen ? 'Close menu' : 'Open menu'}
	>
		<div class="mobile-toggle__icon" class:is-open={isOpen}>
			<span></span>
			<span></span>
			<span></span>
		</div>
		<span class="mobile-toggle__label">{label}</span>
	</button>
</footer>

<!-- Overlay -->
{#if isOpen}
	<div
		class="mobile-toggle__overlay"
		onclick={handleOverlayClick}
		onkeydown={(e) => e.key === 'Escape' && handleOverlayClick()}
		role="button"
		tabindex="-1"
		aria-label="Close menu"
	></div>
{/if}

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   MOBILE TOGGLE - Exact Simpler Trading Match
	   ═══════════════════════════════════════════════════════════════════════════ */
	.mobile-toggle {
		display: none;
		position: fixed;
		bottom: 0;
		left: 0;
		right: 0;
		height: 50px;
		background-color: #0d2532;
		z-index: 100010;
	}

	@media (max-width: 1279px) {
		.mobile-toggle {
			display: block;
		}
	}

	.mobile-toggle__button {
		display: flex;
		align-items: center;
		width: 100%;
		height: 50px;
		padding: 0 20px 0 60px;
		background: none;
		border: none;
		color: #fff;
		cursor: pointer;
		position: relative;
		text-align: left;
	}

	.mobile-toggle__button:focus {
		outline: none;
	}

	/* Hamburger Icon */
	.mobile-toggle__icon {
		position: absolute;
		left: 20px;
		top: 50%;
		margin-top: -7px;
		width: 20px;
		height: 14px;
	}

	.mobile-toggle__icon span {
		position: absolute;
		left: 0;
		width: 20px;
		height: 2px;
		background-color: #fff;
		transition: all 0.2s ease-in-out;
		transform-origin: left center;
	}

	.mobile-toggle__icon span:nth-child(1) { top: 0; }
	.mobile-toggle__icon span:nth-child(2) { top: 6px; }
	.mobile-toggle__icon span:nth-child(3) { top: 12px; }

	/* Hamburger → X animation */
	.mobile-toggle__icon.is-open span:nth-child(1) {
		top: -1px;
		left: 3px;
		transform: rotate(45deg);
	}

	.mobile-toggle__icon.is-open span:nth-child(2) {
		opacity: 0;
		width: 0;
	}

	.mobile-toggle__icon.is-open span:nth-child(3) {
		top: 13px;
		left: 3px;
		transform: rotate(-45deg);
	}

	/* Label */
	.mobile-toggle__label {
		font-size: 12px;
		font-family: 'Open Sans', sans-serif;
		text-transform: uppercase;
	}

	/* Overlay */
	.mobile-toggle__overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background-color: rgba(0, 0, 0, 0.65);
		z-index: 100009;
	}
</style>
