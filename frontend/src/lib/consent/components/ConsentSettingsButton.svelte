<script lang="ts">
	/**
	 * Floating Consent Settings Button - Svelte 5
	 *
	 * A persistent, accessible button that allows users to revisit
	 * their cookie preferences at any time.
	 *
	 * Updated: December 2025 - Migrated to Svelte 5 runes ($props, $state, $effect)
	 * @component
	 */

	import { fade, fly } from 'svelte/transition';
	import { browser } from '$app/environment';
	import { consentStore, showPreferencesModal, openPreferencesModal } from '../store';

	// Svelte 5: Props using $props() rune
	interface Props {
		/** Position of the button */
		position?: 'bottom-left' | 'bottom-right';
		/** Show only after consent has been given */
		showAfterConsent?: boolean;
		/** Delay before showing the button (ms) */
		showDelay?: number;
		/** Custom class for styling */
		class?: string;
	}

	let { position = 'bottom-left', showAfterConsent = true, showDelay = 1000, class: className = '' }: Props = $props();

	// Svelte 5: Reactive state using $state() rune
	let visible = $state(false);
	let expanded = $state(false);

	// Svelte 5: Side effects using $effect() rune - visibility logic
	$effect(() => {
		if (browser && $consentStore.hasInteracted && showAfterConsent) {
			const timeout = setTimeout(() => {
				visible = true;
			}, showDelay);
			return () => clearTimeout(timeout);
		}
		return undefined;
	});

	// Svelte 5: Side effect for modal visibility
	$effect(() => {
		if ($showPreferencesModal) {
			visible = false;
		} else if (browser && $consentStore.hasInteracted && showAfterConsent) {
			visible = true;
		}
	});

	function handleClick() {
		openPreferencesModal();
	}

	function handleMouseEnter() {
		expanded = true;
	}

	function handleMouseLeave() {
		expanded = false;
	}

	function handleKeyDown(event: KeyboardEvent) {
		if (event.key === 'Enter' || event.key === ' ') {
			event.preventDefault();
			handleClick();
		}
	}

	// Svelte 5: Derived value for reduced motion preference
	let prefersReducedMotion = $derived(
		browser ? window.matchMedia('(prefers-reduced-motion: reduce)').matches : false
	);
</script>

{#if visible}
	<button
		type="button"
		class="consent-settings-btn {position} {className}"
		class:expanded
		onclick={handleClick}
		onmouseenter={handleMouseEnter}
		onmouseleave={handleMouseLeave}
		onfocus={handleMouseEnter}
		onblur={handleMouseLeave}
		onkeydown={handleKeyDown}
		aria-label="Cookie settings"
		title="Manage cookie preferences"
		transition:fly={{ x: position.includes('left') ? -20 : 20, duration: prefersReducedMotion ? 0 : 300 }}
	>
		<span class="btn-icon">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="20"
				height="20"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<!-- Cookie icon -->
				<circle cx="12" cy="12" r="10" />
				<circle cx="8" cy="9" r="1" fill="currentColor" />
				<circle cx="15" cy="8" r="1" fill="currentColor" />
				<circle cx="10" cy="14" r="1" fill="currentColor" />
				<circle cx="16" cy="13" r="1" fill="currentColor" />
				<circle cx="13" cy="17" r="1" fill="currentColor" />
			</svg>
		</span>
		{#if expanded}
			<span
				class="btn-label"
				transition:fade={{ duration: prefersReducedMotion ? 0 : 150 }}
			>
				Cookie Settings
			</span>
		{/if}
	</button>
{/if}

<style>
	.consent-settings-btn {
		position: fixed;
		z-index: 9998;
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem;
		background: linear-gradient(135deg, #1a1f2e 0%, #0d1117 100%);
		border: 1px solid rgba(255, 255, 255, 0.15);
		border-radius: 2rem;
		color: rgba(255, 255, 255, 0.9);
		cursor: pointer;
		transition: all 0.2s ease;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
	}

	.consent-settings-btn:hover {
		background: linear-gradient(135deg, #242a3d 0%, #151b24 100%);
		border-color: rgba(255, 255, 255, 0.25);
		box-shadow: 0 6px 16px rgba(0, 0, 0, 0.4);
	}

	.consent-settings-btn:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	.consent-settings-btn.expanded {
		padding-right: 1rem;
	}

	/* Positions */
	.bottom-left {
		bottom: 1.5rem;
		left: 1.5rem;
	}

	.bottom-right {
		bottom: 1.5rem;
		right: 1.5rem;
	}

	.btn-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.btn-label {
		font-size: 0.875rem;
		font-weight: 500;
		white-space: nowrap;
	}

	/* Mobile adjustments */
	@media (max-width: 640px) {
		.consent-settings-btn {
			padding: 0.625rem;
		}

		.bottom-left {
			bottom: 1rem;
			left: 1rem;
		}

		.bottom-right {
			bottom: 1rem;
			right: 1rem;
		}

		/* Always show label on mobile when expanded */
		.btn-label {
			font-size: 0.75rem;
		}
	}

	/* Reduced motion */
	@media (prefers-reduced-motion: reduce) {
		.consent-settings-btn {
			transition: none;
		}
	}
</style>
