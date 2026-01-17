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
</script>

<aside class="admin-sidebar">
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
				<a href={item.href} class="nav-item" class:active={isActive(item.href)}>
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
	.admin-sidebar {
		width: 16rem;
		height: 100vh;
		display: flex;
		flex-direction: column;
		background: var(--admin-sidebar-bg);
		border-right: 1px solid var(--admin-sidebar-border);
		color: var(--admin-text-primary);
	}

	/* Header */
	.sidebar-header {
		padding: 1.5rem;
		border-bottom: 1px solid var(--admin-border-light);
	}

	.sidebar-title {
		font-size: 1.25rem;
		font-weight: 700;
		color: var(--admin-text-primary);
	}

	.sidebar-subtitle {
		font-size: 0.75rem;
		color: var(--admin-text-muted);
		margin-top: 0.25rem;
	}

	/* Navigation */
	.sidebar-nav {
		flex: 1;
		overflow-y: auto;
		padding: 1rem;
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.nav-group {
		display: flex;
		flex-direction: column;
	}

	.nav-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.625rem 1rem;
		border-radius: 0.5rem;
		color: var(--admin-nav-text);
		text-decoration: none;
		transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
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
		font-weight: 500;
	}

	/* Child Navigation */
	.nav-children {
		margin-left: 2rem;
		margin-top: 0.25rem;
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.nav-child {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		color: var(--admin-nav-text);
		text-decoration: none;
		transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
	}

	.nav-child:hover {
		color: var(--admin-nav-text-hover);
	}

	.nav-child.active {
		color: var(--admin-accent-primary);
		font-weight: 500;
	}

	/* Footer */
	.sidebar-footer {
		padding: 1rem;
		border-top: 1px solid var(--admin-border-light);
	}

	.user-info {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 0.75rem;
	}

	.user-avatar {
		width: 2.5rem;
		height: 2.5rem;
		border-radius: 9999px;
		background: var(--admin-accent-primary);
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
		font-size: 0.875rem;
		font-weight: 600;
	}

	.user-details {
		flex: 1;
		min-width: 0;
	}

	.user-name {
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--admin-text-primary);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.user-email {
		font-size: 0.75rem;
		color: var(--admin-text-muted);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.logout-btn {
		width: 100%;
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		font-size: 0.875rem;
		color: var(--admin-text-muted);
		background: transparent;
		border: none;
		border-radius: 0.5rem;
		cursor: pointer;
		transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
	}

	.logout-btn:hover {
		color: var(--admin-text-primary);
		background: var(--admin-nav-bg-hover);
	}
</style>
