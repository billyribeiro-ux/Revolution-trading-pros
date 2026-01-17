<script lang="ts">
	/**
	 * ActionsDropdown - Row Actions Menu
	 * Revolution Trading Pros - Apple ICT 11+ Principal Engineer Grade
	 *
	 * Reusable dropdown menu for table row actions.
	 * Supports icons, dividers, and danger actions.
	 */
	import { IconDotsVertical } from '$lib/icons';

	interface Action {
		id: string;
		label: string;
		icon?: any;
		variant?: 'default' | 'danger' | 'warning' | 'success';
		dividerBefore?: boolean;
		disabled?: boolean;
	}

	interface Props {
		actions: Action[];
		onAction: (actionId: string) => void;
		position?: 'left' | 'right';
		size?: 'sm' | 'md';
	}

	let { actions, onAction, position = 'right', size = 'md' }: Props = $props();

	let isOpen = $state(false);
	let triggerRef = $state<HTMLButtonElement | null>(null);
	let menuRef = $state<HTMLDivElement | null>(null);

	function toggle(e: MouseEvent) {
		e.stopPropagation();
		isOpen = !isOpen;
	}

	function handleAction(actionId: string, disabled?: boolean) {
		if (disabled) return;
		onAction(actionId);
		isOpen = false;
	}

	function handleClickOutside(e: MouseEvent) {
		if (
			isOpen &&
			triggerRef &&
			menuRef &&
			!triggerRef.contains(e.target as Node) &&
			!menuRef.contains(e.target as Node)
		) {
			isOpen = false;
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			isOpen = false;
		}
	}

	$effect(() => {
		if (isOpen) {
			document.addEventListener('click', handleClickOutside);
			document.addEventListener('keydown', handleKeydown);
		}
		return () => {
			document.removeEventListener('click', handleClickOutside);
			document.removeEventListener('keydown', handleKeydown);
		};
	});
</script>

<div class="actions-dropdown" class:size-sm={size === 'sm'}>
	<button
		type="button"
		class="trigger-btn"
		bind:this={triggerRef}
		onclick={toggle}
		aria-haspopup="true"
		aria-expanded={isOpen}
	>
		<IconDotsVertical size={size === 'sm' ? 16 : 20} />
	</button>

	{#if isOpen}
		<div
			class="dropdown-menu"
			class:position-left={position === 'left'}
			class:position-right={position === 'right'}
			bind:this={menuRef}
			role="menu"
		>
			{#each actions as action (action.id)}
				{#if action.dividerBefore}
					<div class="menu-divider"></div>
				{/if}
				<button
					type="button"
					class="menu-item"
					class:danger={action.variant === 'danger'}
					class:warning={action.variant === 'warning'}
					class:success={action.variant === 'success'}
					class:disabled={action.disabled}
					role="menuitem"
					onclick={() => handleAction(action.id, action.disabled)}
					disabled={action.disabled}
				>
					{#if action.icon}
						{@const Icon = action.icon}
						<Icon size={16} />
					{/if}
					<span>{action.label}</span>
				</button>
			{/each}
		</div>
	{/if}
</div>

<style>
	.actions-dropdown {
		position: relative;
		display: inline-flex;
	}

	.trigger-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		background: transparent;
		border: none;
		color: var(--admin-text-muted);
		border-radius: var(--radius-md, 0.5rem);
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.size-sm .trigger-btn {
		width: 28px;
		height: 28px;
	}

	.trigger-btn:hover {
		background: var(--admin-surface-hover);
		color: var(--admin-text-secondary);
	}

	.trigger-btn:focus-visible {
		box-shadow: var(--admin-focus-ring);
		outline: none;
	}

	.dropdown-menu {
		position: absolute;
		top: 100%;
		z-index: 50;
		min-width: 160px;
		background: var(--admin-surface-primary);
		border: 1px solid var(--admin-border-subtle);
		border-radius: var(--radius-md, 0.5rem);
		padding: 0.375rem;
		box-shadow: 0 10px 25px -5px rgba(0, 0, 0, 0.3);
		animation: dropdownIn 0.15s ease;
	}

	@keyframes dropdownIn {
		from {
			opacity: 0;
			transform: translateY(-8px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.dropdown-menu.position-right {
		right: 0;
	}

	.dropdown-menu.position-left {
		left: 0;
	}

	.menu-divider {
		height: 1px;
		background: var(--admin-border-subtle);
		margin: 0.375rem 0;
	}

	.menu-item {
		display: flex;
		align-items: center;
		gap: 0.625rem;
		width: 100%;
		padding: 0.5rem 0.75rem;
		background: transparent;
		border: none;
		color: var(--admin-text-secondary);
		font-family: var(--font-body), 'Roboto', sans-serif;
		font-size: 0.8125rem;
		text-align: left;
		border-radius: var(--radius-sm, 0.25rem);
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.menu-item:hover:not(:disabled) {
		background: var(--admin-surface-hover);
		color: var(--admin-text-primary);
	}

	.menu-item:focus-visible {
		box-shadow: var(--admin-focus-ring);
		outline: none;
	}

	.menu-item.danger {
		color: var(--admin-error);
	}

	.menu-item.danger:hover:not(:disabled) {
		background: var(--admin-error-bg);
	}

	.menu-item.warning {
		color: var(--admin-warning);
	}

	.menu-item.warning:hover:not(:disabled) {
		background: var(--admin-warning-bg);
	}

	.menu-item.success {
		color: var(--admin-success);
	}

	.menu-item.success:hover:not(:disabled) {
		background: var(--admin-success-bg);
	}

	.menu-item.disabled,
	.menu-item:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}
</style>
