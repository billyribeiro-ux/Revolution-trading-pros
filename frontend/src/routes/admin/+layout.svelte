<script lang="ts">
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { user, isAuthenticated } from '$lib/stores/auth';
	import {
		IconDashboard,
		IconReceipt,
		IconTicket,
		IconUsers,
		IconSettings,
		IconLogout,
		IconMenu2,
		IconX,
		IconBellRinging,
		IconForms,
		IconSeo,
		IconNews,
		IconMail
	} from '@tabler/icons-svelte';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	let isSidebarOpen = false;

	// Check if user is admin (you'll need to add role check)
	$: if (browser && !$isAuthenticated) {
		goto('/login?redirect=/admin');
	}

	function toggleSidebar() {
		isSidebarOpen = !isSidebarOpen;
	}

	const menuItems = [
		{ icon: IconDashboard, label: 'Overview', href: '/admin' },
		{ icon: IconReceipt, label: 'Subscriptions', href: '/admin/subscriptions' },
		{ icon: IconTicket, label: 'Coupons', href: '/admin/coupons' },
		{ icon: IconBellRinging, label: 'Popups', href: '/admin/popups' },
		{ icon: IconForms, label: 'Forms', href: '/admin/forms' },
		{ icon: IconNews, label: 'Blog', href: '/admin/blog' },
		{ icon: IconMail, label: 'Email Templates', href: '/admin/email/templates' },
		{ icon: IconMail, label: 'Email Settings', href: '/admin/email/smtp' },
		{ icon: IconSeo, label: 'SEO', href: '/admin/seo' },
		{ icon: IconUsers, label: 'Users', href: '/admin/users' },
		{ icon: IconSettings, label: 'Settings', href: '/admin/settings' }
	];
</script>

<svelte:head>
	<title>Admin Dashboard | Revolution Trading Pros</title>
</svelte:head>

<div class="admin-layout">
	<!-- Sidebar -->
	<aside class="admin-sidebar" class:open={isSidebarOpen}>
		<!-- Logo Section -->
		<div class="sidebar-header">
			<div class="logo-container">
				<img src="/revolution-trading-pros.png" alt="RTP" class="admin-logo" />
				<span class="admin-badge">ADMIN</span>
			</div>
			<button class="close-btn" on:click={toggleSidebar}>
				<IconX size={24} />
			</button>
		</div>

		<!-- Navigation -->
		<nav class="sidebar-nav">
			{#each menuItems as item}
				<a
					href={item.href}
					class="nav-item"
					class:active={$page.url.pathname === item.href}
					on:click={() => (isSidebarOpen = false)}
				>
					<svelte:component this={item.icon} size={20} />
					<span>{item.label}</span>
				</a>
			{/each}
		</nav>

		<!-- User Section -->
		<div class="sidebar-footer">
			<div class="user-info">
				<div class="user-avatar">
					{$user?.name?.charAt(0).toUpperCase() || 'A'}
				</div>
				<div class="user-details">
					<div class="user-name">{$user?.name || 'Admin'}</div>
					<div class="user-role">Administrator</div>
				</div>
			</div>
			<a href="/" class="logout-btn">
				<IconLogout size={18} />
				<span>Exit Admin</span>
			</a>
		</div>
	</aside>

	<!-- Main Content -->
	<div class="admin-main">
		<!-- Top Bar -->
		<header class="admin-header">
			<button class="mobile-menu-btn" on:click={toggleSidebar}>
				<IconMenu2 size={24} />
			</button>
			<div class="header-title">
				<h1>{$page.url.pathname.split('/').pop() || 'Dashboard'}</h1>
			</div>
			<div class="header-actions">
				<a href="/" class="view-site-btn">View Site</a>
			</div>
		</header>

		<!-- Page Content -->
		<main class="admin-content">
			<slot />
		</main>
	</div>

	<!-- Mobile Overlay -->
	{#if isSidebarOpen}
		<div
			class="mobile-overlay"
			role="button"
			tabindex="0"
			on:click={toggleSidebar}
			on:keydown={(e) => e.key === 'Escape' && toggleSidebar()}
		></div>
	{/if}
</div>

<style>
	.admin-layout {
		display: flex;
		min-height: 100vh;
		background: #0f172a;
		color: #e2e8f0;
	}

	/* Sidebar */
	.admin-sidebar {
		width: 280px;
		background: linear-gradient(180deg, #1e293b 0%, #0f172a 100%);
		border-right: 1px solid rgba(99, 102, 241, 0.1);
		display: flex;
		flex-direction: column;
		position: fixed;
		top: 0;
		left: 0;
		bottom: 0;
		z-index: 40;
		transition: transform 0.3s ease;
	}

	.sidebar-header {
		padding: 1.5rem;
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.logo-container {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.admin-logo {
		height: 40px;
		width: auto;
	}

	.admin-badge {
		padding: 0.25rem 0.5rem;
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		color: white;
		font-size: 0.7rem;
		font-weight: 700;
		border-radius: 6px;
		letter-spacing: 0.5px;
	}

	.close-btn {
		display: none;
		background: none;
		border: none;
		color: #94a3b8;
		cursor: pointer;
		padding: 0.5rem;
		border-radius: 8px;
		transition: all 0.2s;
	}

	.close-btn:hover {
		background: rgba(99, 102, 241, 0.1);
		color: #818cf8;
	}

	/* Navigation */
	.sidebar-nav {
		flex: 1;
		padding: 1rem;
		overflow-y: auto;
	}

	.nav-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.875rem 1rem;
		color: #94a3b8;
		text-decoration: none;
		border-radius: 10px;
		margin-bottom: 0.5rem;
		transition: all 0.2s;
		font-weight: 500;
	}

	.nav-item:hover {
		background: rgba(99, 102, 241, 0.1);
		color: #a5b4fc;
	}

	.nav-item.active {
		background: linear-gradient(135deg, rgba(99, 102, 241, 0.2), rgba(139, 92, 246, 0.2));
		color: #a5b4fc;
		border: 1px solid rgba(99, 102, 241, 0.3);
	}

	/* Sidebar Footer */
	.sidebar-footer {
		padding: 1rem;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
	}

	.user-info {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem;
		background: rgba(99, 102, 241, 0.05);
		border-radius: 10px;
		margin-bottom: 0.75rem;
	}

	.user-avatar {
		width: 40px;
		height: 40px;
		border-radius: 10px;
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		display: flex;
		align-items: center;
		justify-content: center;
		font-weight: 700;
		font-size: 1.125rem;
		color: white;
	}

	.user-details {
		flex: 1;
		min-width: 0;
	}

	.user-name {
		font-weight: 600;
		font-size: 0.9375rem;
		color: #e2e8f0;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.user-role {
		font-size: 0.8125rem;
		color: #64748b;
	}

	.logout-btn {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		color: #94a3b8;
		text-decoration: none;
		border-radius: 10px;
		transition: all 0.2s;
		font-weight: 500;
		width: 100%;
		justify-content: center;
		border: 1px solid rgba(239, 68, 68, 0.2);
	}

	.logout-btn:hover {
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
		border-color: rgba(239, 68, 68, 0.3);
	}

	/* Main Content */
	.admin-main {
		flex: 1;
		margin-left: 280px;
		display: flex;
		flex-direction: column;
	}

	.admin-header {
		height: 70px;
		background: #1e293b;
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
		display: flex;
		align-items: center;
		padding: 0 2rem;
		gap: 1rem;
		position: sticky;
		top: 0;
		z-index: 30;
	}

	.mobile-menu-btn {
		display: none;
		background: none;
		border: none;
		color: #94a3b8;
		cursor: pointer;
		padding: 0.5rem;
		border-radius: 8px;
		transition: all 0.2s;
	}

	.mobile-menu-btn:hover {
		background: rgba(99, 102, 241, 0.1);
		color: #818cf8;
	}

	.header-title h1 {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
		text-transform: capitalize;
		margin: 0;
	}

	.header-actions {
		margin-left: auto;
	}

	.view-site-btn {
		padding: 0.625rem 1.25rem;
		background: rgba(99, 102, 241, 0.1);
		color: #a5b4fc;
		text-decoration: none;
		border-radius: 8px;
		font-weight: 600;
		font-size: 0.9375rem;
		border: 1px solid rgba(99, 102, 241, 0.2);
		transition: all 0.2s;
	}

	.view-site-btn:hover {
		background: rgba(99, 102, 241, 0.2);
		border-color: rgba(99, 102, 241, 0.4);
	}

	.admin-content {
		flex: 1;
		padding: 2rem;
		overflow-y: auto;
	}

	.mobile-overlay {
		display: none;
	}

	/* Responsive */
	@media (max-width: 1024px) {
		.admin-sidebar {
			transform: translateX(-100%);
		}

		.admin-sidebar.open {
			transform: translateX(0);
		}

		.close-btn {
			display: block;
		}

		.admin-main {
			margin-left: 0;
		}

		.mobile-menu-btn {
			display: block;
		}

		.mobile-overlay {
			display: block;
			position: fixed;
			inset: 0;
			background: rgba(0, 0, 0, 0.5);
			z-index: 35;
		}

		.admin-content {
			padding: 1.5rem;
		}
	}

	@media (max-width: 640px) {
		.admin-header {
			padding: 0 1rem;
		}

		.header-title h1 {
			font-size: 1.25rem;
		}

		.admin-content {
			padding: 1rem;
		}
	}
</style>
