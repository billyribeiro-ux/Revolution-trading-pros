<script lang="ts">
	/**
	 * Mobile Toggle Button Component - Svelte 5 Implementation
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Fixed mobile navigation toggle button matching WordPress reference.
	 * Uses animated hamburger icon with 3 spans (WordPress style).
	 *
	 * Features:
	 * - Animated hamburger to X transition
	 * - Accessible (ARIA attributes)
	 * - Svelte 5 runes
	 *
	 * @version 3.0.0 (Svelte 5 / December 2025)
	 */

	// ═══════════════════════════════════════════════════════════════════════════
	// PROPS
	// ═══════════════════════════════════════════════════════════════════════════

	interface Props {
		isOpen?: boolean;
		onToggle?: () => void;
		position?: 'left' | 'right';
	}

	let {
		isOpen = $bindable(false),
		onToggle,
		position = 'left'
	}: Props = $props();

	// ═══════════════════════════════════════════════════════════════════════════
	// HANDLERS
	// ═══════════════════════════════════════════════════════════════════════════

	function handleClick(): void {
		isOpen = !isOpen;
		onToggle?.();
	}

	function handleKeyDown(event: KeyboardEvent): void {
		if (event.key === 'Enter' || event.key === ' ') {
			event.preventDefault();
			handleClick();
		}
	}
</script>

<button
	class="csdashboard__toggle"
	class:is-open={isOpen}
	class:position-right={position === 'right'}
	onclick={handleClick}
	onkeydown={handleKeyDown}
	aria-label={isOpen ? 'Close navigation menu' : 'Open navigation menu'}
	aria-expanded={isOpen}
	aria-controls="dashboard-sidebar"
	type="button"
>
	<!-- WordPress-style animated hamburger icon with 3 spans -->
	<span class="dashboard__toggle-button" aria-hidden="true">
		<span class="dashboard__toggle-button-icon">
			<span></span>
			<span></span>
			<span></span>
		</span>
	</span>
</button>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   CSS CUSTOM PROPERTIES
	   ═══════════════════════════════════════════════════════════════════════════ */

	:root {
		--toggle-bg: #0984ae;
		--toggle-bg-open: #076787;
		--toggle-size: 50px;
		--toggle-shadow: 0 4px 14px rgba(9, 132, 174, 0.4);
		--toggle-transition: all 0.15s ease-in-out;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   TOGGLE BUTTON (WordPress Reference)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.csdashboard__toggle {
		display: none;
		position: fixed;
		bottom: 20px;
		left: 20px;
		z-index: 100010;
		width: var(--toggle-size);
		height: var(--toggle-size);
		line-height: var(--toggle-size);
		border-radius: 10px;
		background: var(--toggle-bg);
		border: 1px solid rgba(255, 255, 255, 0.2);
		cursor: pointer;
		box-shadow: var(--toggle-shadow);
		transition: var(--toggle-transition);
		padding: 0;
		align-items: center;
		justify-content: center;
		-webkit-appearance: none;
		-moz-appearance: none;
		appearance: none;
		overflow: hidden;
	}

	.csdashboard__toggle.position-right {
		left: auto;
		right: 20px;
	}

	.csdashboard__toggle:hover {
		background: var(--toggle-bg-open);
	}

	.csdashboard__toggle:focus,
	.csdashboard__toggle:hover {
		outline: none;
	}

	.csdashboard__toggle:focus-visible {
		outline: 3px solid white;
		outline-offset: 2px;
	}

	.csdashboard__toggle.is-open {
		background: var(--toggle-bg-open);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   TOGGLE BUTTON CONTAINER
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__toggle-button {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 100%;
		height: 100%;
		color: white;
		position: relative;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   HAMBURGER ICON (WordPress 3-span pattern)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__toggle-button-icon {
		height: var(--toggle-size);
		position: relative;
		width: 20px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.dashboard__toggle-button-icon span {
		background-color: #fff;
		border-radius: 0;
		display: block;
		height: 2px;
		left: 0;
		opacity: 1;
		position: absolute;
		transform: rotate(0);
		transform-origin: left center;
		transition: var(--toggle-transition);
		width: 20px;
	}

	.dashboard__toggle-button-icon span:first-child {
		top: calc(50% - 6px);
	}

	.dashboard__toggle-button-icon span:nth-child(2) {
		top: 50%;
		transform: translateY(-50%);
	}

	.dashboard__toggle-button-icon span:nth-child(3) {
		top: calc(50% + 4px);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   OPEN STATE - X Animation (WordPress Reference)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.is-open .dashboard__toggle-button-icon span:first-child {
		transform: rotate(45deg);
		top: calc(50% - 1px);
		left: 3px;
	}

	.is-open .dashboard__toggle-button-icon span:nth-child(2) {
		opacity: 0;
		width: 0;
	}

	.is-open .dashboard__toggle-button-icon span:nth-child(3) {
		transform: rotate(-45deg);
		top: calc(50% - 1px);
		left: 3px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media screen and (max-width: 980px) {
		.csdashboard__toggle {
			display: flex;
		}
	}

	/* Safe area for mobile devices */
	@supports (padding-bottom: env(safe-area-inset-bottom)) {
		.csdashboard__toggle {
			bottom: calc(20px + env(safe-area-inset-bottom));
		}

		.csdashboard__toggle.position-right {
			right: calc(20px + env(safe-area-inset-right));
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   REDUCED MOTION
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		.csdashboard__toggle,
		.dashboard__toggle-button-icon span {
			transition: none;
		}
	}
</style>
