<script lang="ts">
	/**
	 * ActionsDropdown - Row Actions Menu
	 * Revolution Trading Pros - Apple ICT 11+ Principal Engineer Grade
	 *
	 * Reusable dropdown menu for table row actions.
	 * Supports icons, dividers, and danger actions.
	 */
	import type { Attachment } from 'svelte/attachments';
	import { IconDotsVertical, type IconComponent } from '#lib/icons/index.js';

	interface Action {
		id: string;
		label: string;
		icon?: IconComponent;
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
	let triggerRef: HTMLButtonElement | null = null;
	let menuRef: HTMLDivElement | null = null;

	const trackTrigger: Attachment<HTMLButtonElement> = (element) => {
		triggerRef = element;

		return () => {
			if (triggerRef === element) {
				triggerRef = null;
			}
		};
	};

	const trackMenu: Attachment<HTMLDivElement> = (element) => {
		menuRef = element;

		return () => {
			if (menuRef === element) {
				menuRef = null;
			}
		};
	};

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
</script>

<svelte:document onclick={handleClickOutside} onkeydown={handleKeydown} />

<div class={['actions-dropdown', { 'size-sm': size === 'sm' }]}>
	<button
		type="button"
		class="trigger-btn"
		{@attach trackTrigger}
		onclick={toggle}
		aria-haspopup="true"
		aria-expanded={isOpen}
	>
		<IconDotsVertical size={size === 'sm' ? 16 : 20} />
	</button>

	{#if isOpen}
		<div
			class={[
				'dropdown-menu',
				{
					'position-left': position === 'left',
					'position-right': position === 'right'
				}
			]}
			{@attach trackMenu}
			role="menu"
		>
			{#each actions as action (action.id)}
				{#if action.dividerBefore}
					<div class="menu-divider"></div>
				{/if}
				<button
					type="button"
					class={[
						'menu-item',
						{
							danger: action.variant === 'danger',
							warning: action.variant === 'warning',
							success: action.variant === 'success',
							disabled: action.disabled
						}
					]}
					role="menuitem"
					onclick={() => handleAction(action.id, action.disabled)}
					disabled={action.disabled}
				>
					{#if action.icon}
						{const Icon = action.icon}
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
		z-index: var(--z-dropdown);
		min-width: 180px;
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 0.75rem;
		padding: 0.5rem;
		box-shadow: 0 20px 40px -10px rgba(0, 0, 0, 0.6);
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
		background: var(--bg-hover, #252b33);
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
		color: var(--text-primary, #f0f6fc);
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
		color: var(--text-primary, #f0f6fc);
	}

	.menu-item:focus-visible {
		outline: 2px solid #e6b800;
		outline-offset: -2px;
	}

	.menu-item.danger {
		color: var(--error-emphasis, #f85149);
	}

	.menu-item.danger:hover:not(:disabled) {
		background: rgba(248, 81, 73, 0.15);
		color: var(--error-emphasis, #f85149);
	}

	.menu-item.warning {
		color: var(--warning-emphasis, #d29922);
	}

	.menu-item.warning:hover:not(:disabled) {
		background: rgba(210, 153, 34, 0.15);
		color: #fcd34d;
	}

	.menu-item.success {
		color: var(--success-emphasis, #3fb950);
	}

	.menu-item.success:hover:not(:disabled) {
		background: rgba(63, 185, 80, 0.15);
		color: #6ee7b7;
	}

	.menu-item.disabled,
	.menu-item:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}
</style>
