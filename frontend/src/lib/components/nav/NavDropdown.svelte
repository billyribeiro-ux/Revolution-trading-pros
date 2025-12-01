<script lang="ts">
	/**
	 * NavDropdown - Accessible dropdown menu component
	 * 
	 * Features:
	 * - Click to toggle (not hover per SSOT spec)
	 * - Keyboard navigation (Enter/Space/Escape/Arrow keys)
	 * - ARIA attributes for accessibility
	 * - Edge-aware positioning
	 * - Focus management
	 * 
	 * @version 1.0.0
	 */
	import { onMount } from 'svelte';
	import { IconChevronDown } from '@tabler/icons-svelte';
	import type { SubMenuItem } from './types';

	interface Props {
		id: string;
		label: string;
		items: SubMenuItem[];
		isActive?: boolean;
		isOpen?: boolean;
		onToggle?: (id: string) => void;
		onClose?: () => void;
		onNavigate?: () => void;
	}

	let {
		id,
		label,
		items,
		isActive = false,
		isOpen = false,
		onToggle,
		onClose,
		onNavigate
	}: Props = $props();

	let triggerRef = $state<HTMLButtonElement | null>(null);
	let menuRef = $state<HTMLDivElement | null>(null);
	let menuItems: HTMLAnchorElement[] = [];
	let focusedIndex = $state(-1);
	let alignRight = $state(false);

	// Check if dropdown would overflow viewport
	function checkEdgePosition() {
		if (!triggerRef) return;
		const rect = triggerRef.getBoundingClientRect();
		const menuWidth = 240; // min-width of dropdown
		alignRight = rect.left + menuWidth > window.innerWidth - 20;
	}

	function handleTriggerClick() {
		checkEdgePosition();
		onToggle?.(id);
	}

	function handleKeyDown(event: KeyboardEvent) {
		if (!isOpen) {
			if (event.key === 'Enter' || event.key === ' ') {
				event.preventDefault();
				handleTriggerClick();
			}
			return;
		}

		switch (event.key) {
			case 'Escape':
				event.preventDefault();
				onClose?.();
				triggerRef?.focus();
				break;
			case 'ArrowDown':
				event.preventDefault();
				focusedIndex = Math.min(focusedIndex + 1, items.length - 1);
				menuItems[focusedIndex]?.focus();
				break;
			case 'ArrowUp':
				event.preventDefault();
				focusedIndex = Math.max(focusedIndex - 1, 0);
				menuItems[focusedIndex]?.focus();
				break;
			case 'Tab':
				// Allow natural tab behavior but close menu
				onClose?.();
				break;
		}
	}

	function handleItemClick() {
		onClose?.();
		onNavigate?.();
	}

	$effect(() => {
		if (isOpen) {
			focusedIndex = -1;
		}
	});
</script>

<div class="nav-dropdown" data-dropdown={id}>
	<button
		bind:this={triggerRef}
		class="nav-dropdown-trigger"
		class:active={isActive}
		class:open={isOpen}
		aria-haspopup="true"
		aria-expanded={isOpen}
		aria-controls="dropdown-{id}"
		onclick={handleTriggerClick}
		onkeydown={handleKeyDown}
	>
		<span>{label}</span>
		<IconChevronDown 
			size={14} 
			class="chevron {isOpen ? 'rotate' : ''}"
		/>
	</button>

	{#if isOpen}
		<div
			bind:this={menuRef}
			id="dropdown-{id}"
			class="nav-dropdown-menu"
			class:align-right={alignRight}
			role="menu"
			aria-labelledby="dropdown-trigger-{id}"
		>
			{#each items as item, index (item.href)}
				<a
					bind:this={menuItems[index]}
					href={item.href}
					class="nav-dropdown-item"
					role="menuitem"
					tabindex={focusedIndex === index ? 0 : -1}
					onclick={handleItemClick}
					onkeydown={handleKeyDown}
				>
					<span class="item-label">{item.label}</span>
					{#if item.description}
						<span class="item-description">{item.description}</span>
					{/if}
				</a>
			{/each}
		</div>
	{/if}
</div>

<style>
	.nav-dropdown {
		position: relative;
	}

	.nav-dropdown-trigger {
		display: flex;
		align-items: center;
		gap: 4px;
		padding: 10px 14px;
		color: rgba(255, 255, 255, 0.75);
		font-weight: 500;
		font-size: 14px;
		font-family: inherit;
		background: transparent;
		border: none;
		border-radius: 8px;
		cursor: pointer;
		transition: color 0.15s ease, background-color 0.15s ease;
	}

	.nav-dropdown-trigger:hover,
	.nav-dropdown-trigger.active,
	.nav-dropdown-trigger.open {
		color: #facc15;
		background: rgba(255, 255, 255, 0.05);
	}

	.nav-dropdown-trigger:focus-visible {
		outline: none;
		box-shadow: 0 0 0 2px rgba(250, 204, 21, 0.5);
	}

	.nav-dropdown-trigger :global(.chevron) {
		transition: transform 0.2s ease;
	}

	.nav-dropdown-trigger :global(.chevron.rotate) {
		transform: rotate(180deg);
	}

	.nav-dropdown-menu {
		position: absolute;
		top: calc(100% + 8px);
		left: 0;
		min-width: 240px;
		padding: 8px;
		background: #1a2538;
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		box-shadow: 
			0 4px 6px -1px rgba(0, 0, 0, 0.3),
			0 10px 20px -5px rgba(0, 0, 0, 0.4);
		z-index: 100;
	}

	.nav-dropdown-menu.align-right {
		left: auto;
		right: 0;
	}

	.nav-dropdown-item {
		display: flex;
		flex-direction: column;
		gap: 2px;
		padding: 12px 14px;
		color: rgba(255, 255, 255, 0.8);
		text-decoration: none;
		border-radius: 8px;
		transition: background-color 0.15s ease, color 0.15s ease;
	}

	.nav-dropdown-item:hover,
	.nav-dropdown-item:focus {
		background: rgba(250, 204, 21, 0.1);
		color: #facc15;
		outline: none;
	}

	.nav-dropdown-item:focus-visible {
		box-shadow: inset 0 0 0 2px rgba(250, 204, 21, 0.5);
	}

	.item-label {
		font-size: 14px;
		font-weight: 500;
	}

	.item-description {
		font-size: 12px;
		color: rgba(255, 255, 255, 0.5);
	}
</style>
