<script lang="ts">
	/**
	 * AppSidebar - Sidebar for authenticated member area
	 * 
	 * @version 2.0.0
	 * @author Revolution Trading Pros
	 */
	import { page } from '$app/stores';
	import { user } from '$lib/stores/auth';
	import {
		IconDashboard,
		IconSchool,
		IconChartLine,
		IconBell,
		IconSettings,
		IconUser,
		IconLogout,
		IconX
	} from '@tabler/icons-svelte';
	interface Props {
		isOpen?: boolean;
		onclose?: () => void;
	}

	let { isOpen = false, onclose }: Props = $props();

	const menuItems = [
		{ icon: IconDashboard, label: 'Dashboard', href: '/dashboard' },
		{ icon: IconSchool, label: 'My Courses', href: '/my-courses' },
		{ icon: IconChartLine, label: 'Trading Rooms', href: '/live-trading-rooms' },
		{ icon: IconBell, label: 'Alerts', href: '/my-alerts' },
		{ icon: IconUser, label: 'Account', href: '/account' },
		{ icon: IconSettings, label: 'Settings', href: '/settings' }
	];

	function closeSidebar() {
		onclose?.();
	}

	let currentPath = $derived($page.url.pathname);
</script>

<aside class="app-sidebar" class:open={isOpen}>
	<!-- Header -->
	<div class="sidebar-header">
		<a href="/" class="sidebar-logo">
			<img src="/revolution-trading-pros.png" alt="RTP" />
		</a>
		<button class="close-btn" onclick={closeSidebar}>
			<IconX size={24} />
		</button>
	</div>

	<!-- Navigation -->
	<nav class="sidebar-nav">
		{#each menuItems as item}
			{@const Icon = item.icon}
			<a
				href={item.href}
				class="nav-item"
				class:active={currentPath === item.href || currentPath.startsWith(item.href + '/')}
				onclick={closeSidebar}
			>
				<Icon size={20} />
				<span>{item.label}</span>
			</a>
		{/each}
	</nav>

	<!-- User Section -->
	<div class="sidebar-footer">
		<div class="user-info">
			<div class="user-avatar">
				{$user?.name?.charAt(0).toUpperCase() || 'U'}
			</div>
			<div class="user-details">
				<span class="user-name">{$user?.name || 'User'}</span>
				<span class="user-email">{$user?.email || ''}</span>
			</div>
		</div>
		<button class="logout-btn">
			<IconLogout size={18} />
			<span>Sign Out</span>
		</button>
	</div>
</aside>

<!-- Mobile Overlay -->
{#if isOpen}
	<button 
		class="sidebar-overlay" 
		onclick={closeSidebar}
		aria-label="Close sidebar"
	></button>
{/if}

<style>
	.app-sidebar {
		width: 260px;
		background: linear-gradient(180deg, var(--color-rtp-surface, #1e293b) 0%, var(--color-rtp-bg, #0f172a) 100%);
		border-right: 1px solid rgba(99, 102, 241, 0.1);
		display: flex;
		flex-direction: column;
		position: fixed;
		top: 0;
		left: 0;
		height: 100vh;
		z-index: var(--z-modal, 500);
		transition: transform 0.3s ease;
	}

	.sidebar-header {
		padding: 1.5rem;
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.sidebar-logo img {
		height: 36px;
		width: auto;
	}

	.close-btn {
		display: none;
		background: none;
		border: none;
		color: var(--color-rtp-muted, #94a3b8);
		cursor: pointer;
		padding: 0.5rem;
		border-radius: var(--radius-md, 0.5rem);
		transition: all 0.2s;
	}

	.close-btn:hover {
		background: rgba(99, 102, 241, 0.1);
		color: var(--color-rtp-primary, #818cf8);
	}

	.sidebar-nav {
		flex: 1;
		padding: 1rem;
		overflow-y: auto;
	}

	.nav-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		color: var(--color-rtp-muted, #94a3b8);
		text-decoration: none;
		border-radius: var(--radius-md, 0.5rem);
		font-size: 0.9375rem;
		font-weight: 500;
		margin-bottom: 0.25rem;
		transition: all 0.2s;
	}

	.nav-item:hover {
		background: rgba(99, 102, 241, 0.1);
		color: var(--color-rtp-text, #f1f5f9);
	}

	.nav-item.active {
		background: linear-gradient(135deg, rgba(99, 102, 241, 0.2), rgba(139, 92, 246, 0.2));
		color: var(--color-rtp-primary, #a5b4fc);
		border: 1px solid rgba(99, 102, 241, 0.3);
	}

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
		border-radius: var(--radius-lg, 0.75rem);
		margin-bottom: 0.75rem;
	}

	.user-avatar {
		width: 40px;
		height: 40px;
		border-radius: var(--radius-md, 0.5rem);
		background: linear-gradient(135deg, var(--color-rtp-primary, #6366f1), var(--color-rtp-indigo, #8b5cf6));
		display: flex;
		align-items: center;
		justify-content: center;
		font-weight: 700;
		font-size: 1rem;
		color: white;
	}

	.user-details {
		flex: 1;
		min-width: 0;
		display: flex;
		flex-direction: column;
	}

	.user-name {
		font-weight: 600;
		font-size: 0.9375rem;
		color: var(--color-rtp-text, #e2e8f0);
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.user-email {
		font-size: 0.75rem;
		color: var(--color-rtp-muted, #64748b);
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.logout-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
		width: 100%;
		padding: 0.75rem 1rem;
		background: none;
		border: 1px solid rgba(239, 68, 68, 0.2);
		border-radius: var(--radius-md, 0.5rem);
		color: var(--color-rtp-muted, #94a3b8);
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.logout-btn:hover {
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
		border-color: rgba(239, 68, 68, 0.3);
	}

	.sidebar-overlay {
		display: none;
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.5);
		z-index: calc(var(--z-modal, 500) - 1);
		border: none;
	}

	/* Responsive */
	@media (max-width: 1024px) {
		.app-sidebar {
			transform: translateX(-100%);
		}

		.app-sidebar.open {
			transform: translateX(0);
		}

		.close-btn {
			display: block;
		}

		.sidebar-overlay {
			display: block;
		}
	}
</style>
