<script lang="ts">
	/**
	 * AdminSidebar - Sidebar for admin dashboard
	 * Extracted from admin layout for reusability
	 * 
	 * @version 2.0.0
	 * @author Revolution Trading Pros
	 */
	import { page } from '$app/stores';
	import { user } from '$lib/stores/auth';
	import {
		IconDashboard,
		IconReceipt,
		IconTicket,
		IconUsers,
		IconUserCircle,
		IconSettings,
		IconLogout,
		IconX,
		IconForms,
		IconSeo,
		IconNews,
		IconMail,
		IconChartBar,
		IconSend,
		IconFilter,
		IconEye,
		IconPhoto,
		IconTag,
		IconVideo,
		IconShoppingCart,
		IconBellRinging
	} from '@tabler/icons-svelte';
	interface Props {
		isOpen?: boolean;
		onclose?: () => void;
	}

	let { isOpen = false, onclose }: Props = $props();

	const menuSections = [
		{
			title: null,
			items: [
				{ icon: IconDashboard, label: 'Overview', href: '/admin' }
			]
		},
		{
			title: 'Members',
			items: [
				{ icon: IconUserCircle, label: 'All Members', href: '/admin/members' },
				{ icon: IconFilter, label: 'Segments', href: '/admin/members/segments' },
				{ icon: IconReceipt, label: 'Subscriptions', href: '/admin/subscriptions' },
				{ icon: IconShoppingCart, label: 'Products', href: '/admin/products' },
				{ icon: IconTicket, label: 'Coupons', href: '/admin/coupons' }
			]
		},
		{
			title: 'Content',
			items: [
				{ icon: IconNews, label: 'Blog Posts', href: '/admin/blog' },
				{ icon: IconTag, label: 'Categories', href: '/admin/blog/categories' },
				{ icon: IconPhoto, label: 'Media Library', href: '/admin/media' },
				{ icon: IconVideo, label: 'Videos', href: '/admin/videos' },
				{ icon: IconBellRinging, label: 'Popups', href: '/admin/popups' },
				{ icon: IconForms, label: 'Forms', href: '/admin/forms' }
			]
		},
		{
			title: 'Marketing',
			items: [
				{ icon: IconSend, label: 'Campaigns', href: '/admin/email/campaigns' },
				{ icon: IconMail, label: 'Email Templates', href: '/admin/email/templates' },
				{ icon: IconMail, label: 'Email Settings', href: '/admin/email/smtp' },
				{ icon: IconSeo, label: 'SEO', href: '/admin/seo' }
			]
		},
		{
			title: 'Analytics',
			items: [
				{ icon: IconChartBar, label: 'Dashboard', href: '/admin/analytics' },
				{ icon: IconEye, label: 'Behavior', href: '/admin/behavior' },
				{ icon: IconUsers, label: 'CRM', href: '/admin/crm' }
			]
		},
		{
			title: 'System',
			items: [
				{ icon: IconUsers, label: 'Admin Users', href: '/admin/users' },
				{ icon: IconSettings, label: 'Settings', href: '/admin/settings' }
			]
		}
	];

	function closeSidebar() {
		onclose?.();
	}

	let currentPath = $derived($page.url.pathname);
</script>

<aside class="admin-sidebar" class:open={isOpen}>
	<!-- Header -->
	<div class="sidebar-header">
		<a href="/admin" class="sidebar-logo">
			<img src="/revolution-trading-pros.png" alt="RTP Admin" />
		</a>
		<button class="close-btn" onclick={closeSidebar}>
			<IconX size={24} />
		</button>
	</div>

	<!-- Navigation -->
	<nav class="sidebar-nav">
		{#each menuSections as section}
			{#if section.title}
				<div class="nav-section-title">{section.title}</div>
			{/if}
			{#each section.items as item}
				{@const Icon = item.icon}
				<a
					href={item.href}
					class="nav-item"
					class:active={currentPath === item.href}
					onclick={closeSidebar}
				>
					<Icon size={20} />
					<span>{item.label}</span>
				</a>
			{/each}
		{/each}
	</nav>

	<!-- User Section -->
	<div class="sidebar-footer">
		<div class="user-info">
			<div class="user-avatar">
				{$user?.name?.charAt(0).toUpperCase() || 'A'}
			</div>
			<div class="user-details">
				<span class="user-name">{$user?.name || 'Admin'}</span>
				<span class="user-role">Administrator</span>
			</div>
		</div>
		<a href="/" class="exit-btn">
			<IconLogout size={18} />
			<span>Exit Admin</span>
		</a>
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
	.admin-sidebar {
		width: 240px;
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
		padding: 0.5rem 1rem;
		overflow-y: auto;
	}

	.nav-section-title {
		font-size: 0.7rem;
		font-weight: 600;
		color: var(--color-rtp-muted, #64748b);
		text-transform: uppercase;
		letter-spacing: 0.5px;
		padding: 1rem 0.5rem 0.5rem;
		margin-top: 0.5rem;
	}

	.nav-section-title:first-child {
		margin-top: 0;
		padding-top: 0.5rem;
	}

	.nav-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.6rem 0.75rem;
		color: var(--color-rtp-muted, #94a3b8);
		text-decoration: none;
		border-radius: var(--radius-md, 0.5rem);
		font-size: 0.9rem;
		font-weight: 500;
		margin-bottom: 0.25rem;
		transition: all 0.2s;
	}

	.nav-item:hover {
		background: rgba(99, 102, 241, 0.1);
		color: var(--color-rtp-text, #a5b4fc);
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
		font-size: 1.125rem;
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

	.user-role {
		font-size: 0.8125rem;
		color: var(--color-rtp-muted, #64748b);
	}

	.exit-btn {
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
		text-decoration: none;
		font-weight: 500;
		transition: all 0.2s;
	}

	.exit-btn:hover {
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
		.admin-sidebar {
			transform: translateX(-100%);
		}

		.admin-sidebar.open {
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
