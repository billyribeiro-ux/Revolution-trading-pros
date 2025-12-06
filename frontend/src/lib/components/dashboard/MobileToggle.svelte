<script lang="ts">
	/**
	 * Mobile Toggle Button Component - Svelte 5 Implementation
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Fixed mobile navigation toggle button matching WordPress reference.
	 * Features:
	 * - Animated hamburger icon
	 * - Accessible (ARIA attributes)
	 * - Svelte 5 runes
	 *
	 * @version 3.0.0 (Svelte 5 / December 2025)
	 */

	import { IconMenu2, IconX } from '@tabler/icons-svelte';

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
	<span class="dashboard__toggle-button" aria-hidden="true">
		{#if isOpen}
			<IconX size={24} />
		{:else}
			<IconMenu2 size={24} />
		{/if}
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
		--toggle-icon-size: 24px;
		--toggle-shadow: 0 4px 14px rgba(9, 132, 174, 0.4);
		--toggle-transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   TOGGLE BUTTON
	   ═══════════════════════════════════════════════════════════════════════════ */

	.csdashboard__toggle {
		display: none;
		position: fixed;
		bottom: 20px;
		left: 20px;
		z-index: 1001;
		width: var(--toggle-size);
		height: var(--toggle-size);
		border-radius: 50%;
		background: var(--toggle-bg);
		border: none;
		cursor: pointer;
		box-shadow: var(--toggle-shadow);
		transition: var(--toggle-transition);
		padding: 0;
		align-items: center;
		justify-content: center;
	}

	.csdashboard__toggle.position-right {
		left: auto;
		right: 20px;
	}

	.csdashboard__toggle:hover {
		background: var(--toggle-bg-open);
		transform: scale(1.05);
	}

	.csdashboard__toggle:active {
		transform: scale(0.95);
	}

	.csdashboard__toggle:focus-visible {
		outline: 3px solid white;
		outline-offset: 2px;
	}

	.csdashboard__toggle.is-open {
		background: var(--toggle-bg-open);
		transform: rotate(180deg);
	}

	.csdashboard__toggle.is-open:hover {
		transform: rotate(180deg) scale(1.05);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   TOGGLE ICON
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__toggle-button {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 100%;
		height: 100%;
		color: white;
		transition: transform 0.3s ease;
	}

	.is-open .dashboard__toggle-button {
		transform: rotate(-180deg);
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
		.dashboard__toggle-button {
			transition: none;
		}

		.csdashboard__toggle.is-open {
			transform: none;
		}

		.is-open .dashboard__toggle-button {
			transform: none;
		}
	}
</style>
