<script lang="ts">
	import { page } from '$app/state';
	import {
		IconDashboard,
		IconForms,
		IconUsers,
		IconSeo,
		IconLink,
		IconError404,
		IconSettings,
		IconLogout
	} from '$lib/icons';
	import { authStore } from '$lib/stores/auth.svelte';
	import { goto } from '$app/navigation';

	// Props
	interface Props {
		isOpen: boolean;
		onclose: () => void;
	}

	let props: Props = $props();

	// Destructure with defaults for internal use
	const isOpen = $derived(props.isOpen ?? false);
	const onclose = $derived(props.onclose);

	interface NavItem {
		label: string;
		href: string;
		icon: any;
		children?: NavItem[];
	}

	const navigation: NavItem[] = [
		{
			label: 'Dashboard',
			href: '/admin',
			icon: IconDashboard
		},
		{
			label: 'Forms',
			href: '/admin/forms',
			icon: IconForms,
			children: [
				{ label: 'All Forms', href: '/admin/forms', icon: IconForms },
				{ label: 'Entries', href: '/admin/forms/entries', icon: IconForms },
				{ label: 'Contacts', href: '/admin/contacts', icon: IconUsers }
			]
		},
		{
			label: 'SEO',
			href: '/admin/seo',
			icon: IconSeo,
			children: [
				{ label: 'Analysis', href: '/admin/seo/analysis', icon: IconSeo },
				{ label: 'Redirects', href: '/admin/seo/redirects', icon: IconLink },
				{ label: '404 Errors', href: '/admin/seo/404s', icon: IconError404 },
				{ label: 'Settings', href: '/admin/seo/settings', icon: IconSettings }
			]
		}
	];

	function handleLogout() {
		authStore.logout();
		goto('/login');
	}

	function isActive(href: string): boolean {
		if (href === '/admin') {
			return page.url.pathname === '/admin';
		}
		return page.url.pathname.startsWith(href);
	}

	function handleNavClick() {
		// Close sidebar on mobile after navigation
		if (window.innerWidth < 1024) {
			onclose();
		}
	}
</script>

<!-- Backdrop for mobile -->
{#if isOpen}
	<button class="sidebar-backdrop" onclick={onclose} aria-label="Close sidebar"></button>
{/if}

<aside class="admin-sidebar" class:open={isOpen}>
	<!-- Logo -->
	<div class="sidebar-header">
		<h1 class="sidebar-title">Revolution Admin</h1>
		<p class="sidebar-subtitle">Fluent Revo & SEO Pro</p>
	</div>

	<!-- Navigation -->
	<nav class="sidebar-nav">
		{#each navigation as item}
			{@const IconComponent = item.icon}
			<div class="nav-group">
				<a
					href={item.href}
					class="nav-item"
					class:active={isActive(item.href)}
					onclick={handleNavClick}
				>
					<IconComponent size={20} />
					<span class="nav-label">{item.label}</span>
				</a>

				{#if item.children && isActive(item.href)}
					<div class="nav-children">
						{#each item.children as child}
							<a
								href={child.href}
								class="nav-child"
								class:active={page.url.pathname === child.href}
								onclick={handleNavClick}
							>
								{child.label}
							</a>
						{/each}
					</div>
				{/if}
			</div>
		{/each}
	</nav>

	<!-- User Section -->
	<div class="sidebar-footer">
		<div class="user-info">
			<div class="user-avatar">
				<span>
					{$authStore.user?.name?.[0]?.toUpperCase() || 'A'}
				</span>
			</div>
			<div class="user-details">
				<p class="user-name">{$authStore.user?.name || 'Admin'}</p>
				<p class="user-email">{$authStore.user?.email || ''}</p>
			</div>
		</div>
		<button onclick={handleLogout} class="logout-btn">
			<IconLogout size={18} />
			<span>Logout</span>
		</button>
	</div>
</aside>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   SIDEBAR - Fixed Desktop, Drawer Mobile
	   ═══════════════════════════════════════════════════════════════════════════ */

	.admin-sidebar {
		width: var(--admin-sidebar-width, 240px);
		height: 100vh;
		display: flex;
		flex-direction: column;
		background: var(--admin-sidebar-bg);
		border-right: 1px solid var(--admin-sidebar-border);
		color: var(--admin-text-primary);
		position: fixed;
		left: 0;
		top: 0;
		z-index: var(--z-modal);
		transition: transform var(--duration-normal) var(--ease-out);
	}

	/* Backdrop for mobile */
	.sidebar-backdrop {
		display: none;
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.5);
		z-index: calc(var(--z-modal) - 1);
		cursor: pointer;
		border: none;
		padding: 0;
	}

	/* Header */
	.sidebar-header {
		padding: var(--space-6);
		border-bottom: 1px solid var(--admin-border-light);
	}

	.sidebar-title {
		font-size: var(--text-lg);
		font-weight: var(--font-bold);
		color: var(--admin-text-primary);
		margin: 0;
	}

	.sidebar-subtitle {
		font-size: var(--text-xs);
		color: var(--admin-text-muted);
		margin-top: var(--space-1);
		margin-bottom: 0;
	}

	/* Navigation */
	.sidebar-nav {
		flex: 1;
		overflow-y: auto;
		padding: var(--space-4);
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
	}

	.nav-group {
		display: flex;
		flex-direction: column;
	}

	.nav-item {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		padding: var(--space-3) var(--space-4);
		border-radius: var(--radius-md);
		color: var(--admin-nav-text);
		text-decoration: none;
		transition: var(--transition-all);
		min-height: 44px;
	}

	.nav-item:hover {
		background: var(--admin-nav-bg-hover);
		color: var(--admin-nav-text-hover);
	}

	.nav-item.active {
		background: var(--admin-nav-bg-active);
		color: var(--admin-nav-text-active);
	}

	.nav-label {
		font-weight: var(--font-medium);
		font-size: var(--text-sm);
	}

	/* Child Navigation */
	.nav-children {
		margin-left: var(--space-8);
		margin-top: var(--space-1);
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
	}

	.nav-child {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-2) var(--space-3);
		border-radius: var(--radius-sm);
		font-size: var(--text-sm);
		color: var(--admin-nav-text);
		text-decoration: none;
		transition: var(--transition-all);
		min-height: 40px;
	}

	.nav-child:hover {
		color: var(--admin-nav-text-hover);
	}

	.nav-child.active {
		color: var(--admin-accent-primary);
		font-weight: var(--font-medium);
	}

	/* Footer */
	.sidebar-footer {
		padding: var(--space-4);
		border-top: 1px solid var(--admin-border-light);
	}

	.user-info {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		margin-bottom: var(--space-3);
	}

	.user-avatar {
		width: 2.5rem;
		height: 2.5rem;
		border-radius: var(--radius-full);
		background: var(--admin-accent-primary);
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--color-bg-card);
		font-size: var(--text-sm);
		font-weight: var(--font-semibold);
		flex-shrink: 0;
	}

	.user-details {
		flex: 1;
		min-width: 0;
	}

	.user-name {
		font-size: var(--text-sm);
		font-weight: var(--font-medium);
		color: var(--admin-text-primary);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.user-email {
		font-size: var(--text-xs);
		color: var(--admin-text-muted);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.logout-btn {
		width: 100%;
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-2) var(--space-4);
		font-size: var(--text-sm);
		color: var(--admin-text-muted);
		background: transparent;
		border: none;
		border-radius: var(--radius-md);
		cursor: pointer;
		transition: var(--transition-all);
		min-height: 44px;
	}

	.logout-btn:hover {
		color: var(--admin-text-primary);
		background: var(--admin-nav-bg-hover);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE - Mobile Drawer (< lg: 1024px)
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: calc(var(--breakpoint-lg) - 1px)) {
		.admin-sidebar {
			transform: translateX(-100%);
		}

		.admin-sidebar.open {
			transform: translateX(0);
		}

		.sidebar-backdrop {
			display: block;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ACCESSIBILITY
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		.admin-sidebar {
			transition: none;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   PRINT
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media print {
		.admin-sidebar {
			display: none;
		}
	}
</style>
