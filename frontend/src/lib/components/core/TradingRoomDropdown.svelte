<!--
═══════════════════════════════════════════════════════════════════════════════════
TRADING ROOM DROPDOWN COMPONENT - Svelte 5 / SvelteKit (Nov/Dec 2025)
═══════════════════════════════════════════════════════════════════════════════════

PURPOSE:
This component renders the "Enter a Trading Room" dropdown button found in the
dashboard header. It displays all available trading rooms with SSO access links.
The dropdown closes when clicking outside (click-away pattern).

SVELTE 5 PATTERNS USED:
- $props() rune: Type-safe prop declaration
- $state() rune: Local reactive state management
- $effect() rune: Side effects and cleanup
- onclick handlers: Modern event syntax
- bind:this: Element references for DOM access

ACCESSIBILITY:
- Proper ARIA attributes (aria-expanded, aria-labelledby)
- Keyboard navigation support
- Focus management

USAGE:
<TradingRoomDropdown
  rooms={tradingRooms}
  isOpen={dropdownOpen}
  onToggle={() => dropdownOpen = !dropdownOpen}
  onSelect={(room) => window.open(room.ssoUrl, '_blank')}
/>

@version 3.0.0 - Svelte 5 Runes
@updated December 2025
═══════════════════════════════════════════════════════════════════════════════════
-->
<script lang="ts">
	/**
	 * ─────────────────────────────────────────────────────────────────────────────
	 * IMPORTS
	 * ─────────────────────────────────────────────────────────────────────────────
	 */
	import type { TradingRoom, TradingRoomDropdownProps } from './types';

	/**
	 * ─────────────────────────────────────────────────────────────────────────────
	 * PROPS - Svelte 5 $props() Rune
	 * ─────────────────────────────────────────────────────────────────────────────
	 *
	 * Props are destructured with default values using $props().
	 * This provides type safety and cleaner code than `export let`.
	 */
	let {
		// Required: Array of trading rooms to display
		rooms,
		// Optional: Control open/closed state from parent
		isOpen = false,
		// Optional: Callback when toggle button is clicked
		onToggle = undefined,
		// Optional: Callback when a room is selected
		onSelect = undefined
	}: TradingRoomDropdownProps = $props();

	/**
	 * ─────────────────────────────────────────────────────────────────────────────
	 * LOCAL STATE - Svelte 5 $state() Rune
	 * ─────────────────────────────────────────────────────────────────────────────
	 *
	 * The $state() rune creates reactive state variables that trigger
	 * re-renders when their values change. It replaces `let variable = value`.
	 *
	 * @see https://svelte.dev/docs/svelte/$state
	 */

	// Internal open state (used when not controlled by parent)
	let internalOpen = $state(false);

	// Reference to the dropdown container for click-outside detection
	let dropdownRef: HTMLDivElement | null = $state(null);

	/**
	 * ─────────────────────────────────────────────────────────────────────────────
	 * DERIVED STATE - Computed from props or internal state
	 * ─────────────────────────────────────────────────────────────────────────────
	 */

	// Use parent-controlled state if onToggle is provided, otherwise use internal
	let open = $derived(onToggle ? isOpen : internalOpen);

	/**
	 * ─────────────────────────────────────────────────────────────────────────────
	 * EFFECTS - Svelte 5 $effect() Rune
	 * ─────────────────────────────────────────────────────────────────────────────
	 *
	 * The $effect() rune runs code after the component mounts and re-runs
	 * when its dependencies change. It replaces `onMount` and reactive statements
	 * for side effects.
	 *
	 * IMPORTANT: Return a cleanup function to remove event listeners!
	 *
	 * @see https://svelte.dev/docs/svelte/$effect
	 */

	// Click-outside handler to close dropdown
	$effect(() => {
		// Only add listener when dropdown is open
		if (!open) return;

		/**
		 * Handle clicks outside the dropdown to close it
		 * Uses event.composedPath() for shadow DOM compatibility
		 */
		function handleClickOutside(event: MouseEvent) {
			if (!dropdownRef) return;

			// Check if click was inside the dropdown
			const path = event.composedPath();
			const clickedInside = path.includes(dropdownRef);

			if (!clickedInside) {
				closeDropdown();
			}
		}

		// Add listener with slight delay to prevent immediate close
		const timeoutId = setTimeout(() => {
			document.addEventListener('click', handleClickOutside);
		}, 10);

		// Cleanup function - runs when effect re-runs or component unmounts
		return () => {
			clearTimeout(timeoutId);
			document.removeEventListener('click', handleClickOutside);
		};
	});

	/**
	 * ─────────────────────────────────────────────────────────────────────────────
	 * EVENT HANDLERS
	 * ─────────────────────────────────────────────────────────────────────────────
	 */

	/**
	 * Toggle dropdown open/closed state
	 * Uses parent callback if provided, otherwise manages internally
	 */
	function handleToggle(event: MouseEvent): void {
		event.preventDefault();
		event.stopPropagation();

		if (onToggle) {
			// Parent controls the state
			onToggle();
		} else {
			// Manage state internally
			internalOpen = !internalOpen;
		}
	}

	/**
	 * Close the dropdown
	 */
	function closeDropdown(): void {
		if (onToggle) {
			// Let parent know to close
			// Note: Parent should toggle to false
			onToggle();
		} else {
			internalOpen = false;
		}
	}

	/**
	 * Handle keyboard navigation in the dropdown
	 * Escape key closes the dropdown
	 */
	function handleKeydown(event: KeyboardEvent): void {
		if (event.key === 'Escape') {
			closeDropdown();
		}
	}

	/**
	 * Handle room selection
	 * Calls the optional callback if provided
	 */
	function handleRoomClick(room: TradingRoom, event: MouseEvent): void {
		if (onSelect) {
			onSelect(room);
		}
		// Don't close dropdown - let the link navigate
	}
</script>

<!--
═══════════════════════════════════════════════════════════════════════════════════
TEMPLATE - Dropdown Structure
═══════════════════════════════════════════════════════════════════════════════════

Matches the WordPress structure from core file:
<div class="dropdown display-inline-block">
  <a class="btn btn-xs btn-orange btn-tradingroom dropdown-toggle">...</a>
  <nav class="dropdown-menu dropdown-menu--full-width">...</nav>
</div>
-->

<!--
svelte:window - Global event listener
Handles Escape key to close dropdown from anywhere
-->
<svelte:window onkeydown={handleKeydown} />

<!--
Dropdown Container
bind:this binds the element reference for click-outside detection
-->
<div
	bind:this={dropdownRef}
	class="dropdown display-inline-block"
	class:is-open={open}
>
	<!--
	─────────────────────────────────────────────────────────────────────────────
	TOGGLE BUTTON - Opens/closes the dropdown
	─────────────────────────────────────────────────────────────────────────────
	-->
	<button
		type="button"
		class="btn btn-xs btn-orange btn-tradingroom dropdown-toggle"
		id="tradingRoomDropdownLabel"
		onclick={handleToggle}
		aria-expanded={open}
		aria-haspopup="true"
	>
		<strong>Enter a Trading Room</strong>
		<!-- Dropdown arrow indicator -->
		<span class="dropdown-arrow"></span>
	</button>

	<!--
	─────────────────────────────────────────────────────────────────────────────
	DROPDOWN MENU - List of trading rooms
	─────────────────────────────────────────────────────────────────────────────
	Conditional rendering with {#if} for performance
	-->
	{#if open}
		<nav
			class="dropdown-menu dropdown-menu--full-width"
			aria-labelledby="tradingRoomDropdownLabel"
			role="menu"
		>
			<ul class="dropdown-menu__menu" role="none">
				<!--
				Iterate over trading rooms using {#each} block
				(room.id) provides a unique key for efficient DOM updates
				-->
				{#each rooms as room (room.id)}
					<li role="none">
						<a
							href={room.ssoUrl}
							target="_blank"
							rel="nofollow noopener"
							role="menuitem"
							onclick={(e) => handleRoomClick(room, e)}
						>
							<!-- Room icon using custom font -->
							<span class="st-icon-{room.slug} icon icon--md"></span>
							<!-- Room label (custom or derived from name) -->
							{room.roomLabel || room.name}
						</a>
					</li>
				{/each}
			</ul>
		</nav>
	{/if}
</div>

<!--
═══════════════════════════════════════════════════════════════════════════════════
STYLES - Dropdown Component CSS
═══════════════════════════════════════════════════════════════════════════════════
-->
<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   DROPDOWN CONTAINER
	   ═══════════════════════════════════════════════════════════════════════════ */

	.display-inline-block {
		display: inline-block !important;
	}

	.dropdown {
		position: relative;
		display: inline-block;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   TOGGLE BUTTON - Orange "Enter a Trading Room" button
	   Matches WordPress exactly (core file lines 2865-2875)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.btn-orange,
	.btn-tradingroom {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		background: #f69532;
		border: none;
		color: #fff;
		padding: 10px 20px;
		border-radius: 5px;
		font-size: 14px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		cursor: pointer;
		transition: all 0.2s ease-in-out;
		box-shadow: 0 2px 5px rgba(0, 0, 0, 0.16);
		line-height: 1.4;
	}

	.btn-orange:hover,
	.btn-tradingroom:hover {
		background: #dc7309;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
	}

	.btn-orange:focus,
	.btn-tradingroom:focus {
		outline: 2px solid #0984ae;
		outline-offset: 2px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DROPDOWN ARROW - CSS triangle indicator
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dropdown-arrow {
		display: inline-block;
		margin-left: 0.255em;
		vertical-align: 0.255em;
		border-top: 0.3em solid;
		border-right: 0.3em solid transparent;
		border-bottom: 0;
		border-left: 0.3em solid transparent;
		transition: transform 0.2s ease;
	}

	/* Rotate arrow when dropdown is open */
	.dropdown.is-open .dropdown-arrow {
		transform: rotate(180deg);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DROPDOWN MENU - Flyout menu container
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dropdown-menu {
		position: absolute;
		top: 100%;
		right: 0;
		margin-top: 5px;
		background: #fff;
		border: none;
		border-radius: 5px;
		box-shadow: 0 10px 30px rgba(0, 0, 0, 0.15);
		min-width: 260px;
		padding: 10px;
		z-index: 1000;
		font-size: 14px;
		/* Animation for smooth open */
		animation: dropdownFadeIn 0.15s ease-out;
	}

	/* Dropdown open animation */
	@keyframes dropdownFadeIn {
		from {
			opacity: 0;
			transform: translateY(-10px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.dropdown-menu--full-width {
		/* Allows menu to be wider than button */
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MENU LIST - Room items
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dropdown-menu__menu {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.dropdown-menu__menu li a {
		display: flex;
		align-items: center;
		padding: 10px 15px;
		color: #666;
		text-decoration: none;
		font-size: 14px;
		font-family: 'Open Sans', sans-serif;
		border-radius: 5px;
		white-space: nowrap;
		text-overflow: ellipsis;
		transition: all 0.15s ease-in-out;
	}

	.dropdown-menu__menu li a:hover {
		background-color: #f4f4f4;
		color: #0984ae;
	}

	.dropdown-menu__menu li a:focus {
		outline: 2px solid #0984ae;
		outline-offset: -2px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ICON STYLING
	   ═══════════════════════════════════════════════════════════════════════════ */

	.icon {
		display: inline-block;
		vertical-align: middle;
	}

	.icon--md {
		width: 24px;
		height: 24px;
		font-size: 24px;
		line-height: 24px;
		margin-right: 10px;
		color: #999;
		transition: color 0.15s ease;
	}

	.dropdown-menu__menu li a:hover .icon--md {
		color: #0984ae;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   BUTTON SIZE VARIANT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.btn-xs {
		padding: 8px 16px;
		font-size: 13px;
		line-height: 1.5;
		border-radius: 4px;
	}
</style>
