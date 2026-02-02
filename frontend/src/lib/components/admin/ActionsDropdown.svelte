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

	let props: Props = $props();

	// Destructure with defaults for internal use
	const actions = $derived(props.actions);
	const onAction = $derived(props.onAction);
	const position = $derived(props.position ?? 'right');
	const size = $derived(props.size ?? 'md');

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
		min-width: 180px;
		background: #1e293b;
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 0.75rem;
		padding: 0.5rem;
		box-shadow: 0 20px 40px -10px rgba(0, 0, 0, 0.6);
		animation: dropdownIn 0.15s ease;
		backdrop-filter: blur(20px);
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
		background: rgba(148, 163, 184, 0.15);
		margin: 0.5rem 0;
	}

	.menu-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		width: 100%;
		padding: 0.625rem 0.875rem;
		background: transparent;
		border: none;
		color: #e2e8f0;
		font-family: inherit;
		font-size: 0.875rem;
		font-weight: 500;
		text-align: left;
		border-radius: 0.5rem;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.menu-item:hover:not(:disabled) {
		background: rgba(230, 184, 0, 0.15);
		color: #f1f5f9;
	}

	.menu-item:focus-visible {
		outline: 2px solid #e6b800;
		outline-offset: -2px;
	}

	.menu-item.danger {
		color: #f87171;
	}

	.menu-item.danger:hover:not(:disabled) {
		background: rgba(239, 68, 68, 0.15);
		color: #fca5a5;
	}

	.menu-item.warning {
		color: #fbbf24;
	}

	.menu-item.warning:hover:not(:disabled) {
		background: rgba(251, 191, 36, 0.15);
		color: #fcd34d;
	}

	.menu-item.success {
		color: #34d399;
	}

	.menu-item.success:hover:not(:disabled) {
		background: rgba(16, 185, 129, 0.15);
		color: #6ee7b7;
	}

	.menu-item.disabled,
	.menu-item:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}
</style>
