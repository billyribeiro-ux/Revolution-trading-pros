<script lang="ts">
	/**
	 * AppSidebar Component
	 * Main application sidebar navigation
	 *
	 * @version 1.0.0
	 * @author Revolution Trading Pros
	 */
	import type { Snippet } from 'svelte';

	interface NavItem {
		label: string;
		href: string;
		icon?: string;
		badge?: string | number;
		children?: NavItem[];
	}

	interface Props {
		items?: NavItem[];
		collapsed?: boolean;
		header?: Snippet;
		footer?: Snippet;
		onToggle?: (collapsed: boolean) => void;
	}

	const { items = [], collapsed = false, header, footer, onToggle }: Props = $props();

	let isCollapsed = $state(false);

	$effect(() => {
		isCollapsed = collapsed;
	});

	function toggleSidebar() {
		isCollapsed = !isCollapsed;
		onToggle?.(isCollapsed);
	}
</script>

<aside class="app-sidebar" class:collapsed={isCollapsed}>
	{#if header}
		<div class="sidebar-header">
			{@render header()}
		</div>
	{/if}

	<nav class="sidebar-nav">
		<ul>
			{#each items as item}
				<li>
					<a href={item.href} class="nav-item">
						{#if item.icon}
							<span class="nav-icon">{item.icon}</span>
						{/if}
						{#if !isCollapsed}
							<span class="nav-label">{item.label}</span>
							{#if item.badge}
								<span class="nav-badge">{item.badge}</span>
							{/if}
						{/if}
					</a>
				</li>
			{/each}
		</ul>
	</nav>

	<button type="button" class="sidebar-toggle" onclick={toggleSidebar} aria-label="Toggle sidebar">
		{isCollapsed ? '→' : '←'}
	</button>

	{#if footer}
		<div class="sidebar-footer">
			{@render footer()}
		</div>
	{/if}
</aside>

<style>
	.app-sidebar {
		display: flex;
		flex-direction: column;
		width: 260px;
		height: 100vh;
		background: var(--color-bg-sidebar, #1f2937);
		color: var(--color-text-sidebar, #f9fafb);
		transition: width 0.2s ease;
		position: relative;
	}

	.app-sidebar.collapsed {
		width: 64px;
	}

	.sidebar-header {
		padding: 1rem;
		border-bottom: 1px solid var(--color-border-sidebar, #374151);
	}

	.sidebar-nav {
		flex: 1;
		overflow-y: auto;
		padding: 0.5rem;
	}

	.sidebar-nav ul {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.nav-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		color: inherit;
		text-decoration: none;
		border-radius: var(--radius-md, 0.375rem);
		transition: background 0.15s;
		min-height: 44px;
	}

	.nav-item:hover {
		background: var(--color-bg-sidebar-hover, #374151);
	}

	.nav-icon {
		flex-shrink: 0;
		width: 20px;
		text-align: center;
	}

	.nav-label {
		flex: 1;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.nav-badge {
		padding: 0.125rem 0.5rem;
		background: var(--color-primary, #6366f1);
		border-radius: var(--radius-full, 9999px);
		font-size: 0.75rem;
		font-weight: 600;
	}

	.sidebar-toggle {
		position: absolute;
		right: -12px;
		top: 50%;
		transform: translateY(-50%);
		width: 24px;
		height: 24px;
		background: var(--color-bg-sidebar, #1f2937);
		border: 1px solid var(--color-border-sidebar, #374151);
		border-radius: 50%;
		color: inherit;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 0.75rem;
	}

	.sidebar-footer {
		padding: 1rem;
		border-top: 1px solid var(--color-border-sidebar, #374151);
	}

	/* Mobile */
	@media (max-width: 768px) {
		.app-sidebar {
			position: fixed;
			left: 0;
			top: 0;
			z-index: 1000;
			transform: translateX(-100%);
		}

		.app-sidebar:not(.collapsed) {
			transform: translateX(0);
		}
	}
</style>
