<script lang="ts">
	/**
	 * AdminSidebar - Sidebar for admin dashboard
	 * Extracted from admin layout for reusability
	 *
	 * @version 2.0.0
	 * @author Revolution Trading Pros
	 */
	import { page } from '$app/state';
	import { user } from '$lib/stores/auth.svelte';
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
		IconBellRinging,
		IconHeartbeat,
		IconPlugConnected,
		IconBook,
		IconFileText,
		IconTrendingUp
	} from '$lib/icons';
	interface Props {
		isOpen?: boolean;
		onclose?: () => void;
	}

	let { isOpen = false, onclose }: Props = $props();

	const menuSections = [
		{
			title: null,
			items: [{ icon: IconDashboard, label: 'Overview', href: '/admin' }]
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
				{ icon: IconBook, label: 'Courses', href: '/admin/courses' },
				{ icon: IconTrendingUp, label: 'Indicators', href: '/admin/indicators' },
				{ icon: IconChartBar, label: 'Trading Rooms', href: '/admin/trading-rooms' },
				{ icon: IconFileText, label: 'Resources', href: '/admin/resources' },
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
				{ icon: IconHeartbeat, label: 'Site Health', href: '/admin/site-health' },
				{ icon: IconPlugConnected, label: 'Connections', href: '/admin/connections' },
				{ icon: IconUsers, label: 'Admin Users', href: '/admin/users' },
				{ icon: IconSettings, label: 'Settings', href: '/admin/settings' }
			]
		}
	];

	function closeSidebar() {
		onclose?.();
	}

	let currentPath = $derived(page.url.pathname);
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
				{$user?.name?.[0]?.toUpperCase() || 'A'}
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
	<button class="sidebar-overlay" onclick={closeSidebar} aria-label="Close sidebar"></button>
{/if}

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * ADMIN SIDEBAR - Netflix L11+ Principal Engineer Grade
	 * Uses CSS custom properties for bulletproof light/dark theme support
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.admin-sidebar {
		width: 240px;
		background: var(--admin-sidebar-bg);
		border-right: 1px solid var(--admin-sidebar-border);
		display: flex;
		flex-direction: column;
		position: fixed;
		top: 0;
		left: 0;
		height: 100vh;
		z-index: var(--z-modal, 500);
		transition:
			transform 0.3s cubic-bezier(0.16, 1, 0.3, 1),
			background 0.3s ease,
			border-color 0.3s ease;
	}

	/* Sidebar Header */
	.sidebar-header {
		padding: 1.5rem;
		border-bottom: 1px solid var(--admin-border-light);
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.sidebar-logo img {
		height: 36px;
		width: auto;
	}

	/* Close Button (Mobile) */
	.close-btn {
		display: none;
		background: transparent;
		border: none;
		color: var(--admin-text-muted);
		cursor: pointer;
		padding: 0.5rem;
		border-radius: var(--radius-md, 0.5rem);
		transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
	}

	.close-btn:hover {
		background: var(--admin-btn-bg-hover);
		color: var(--admin-accent-primary);
	}

	.close-btn:focus-visible {
		box-shadow: var(--admin-focus-ring);
		outline: none;
	}

	/* Navigation Container */
	.sidebar-nav {
		flex: 1;
		padding: 0.5rem 1rem;
		overflow-y: auto;
	}

	/* Section Title - Montserrat 600 for overlines/small headings */
	.nav-section-title {
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		font-size: 0.6875rem;
		font-weight: 600;
		color: var(--admin-nav-section-title);
		text-transform: uppercase;
		letter-spacing: 0.08em;
		padding: 1rem 0.5rem 0.5rem;
		margin-top: 0.5rem;
	}

	.nav-section-title:first-child {
		margin-top: 0;
		padding-top: 0.5rem;
	}

	/* Navigation Item - Roboto 500 for nav items */
	.nav-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.6rem 0.75rem;
		color: var(--admin-nav-text);
		text-decoration: none;
		border-radius: var(--radius-md, 0.5rem);
		font-family: var(--font-body), 'Roboto', sans-serif;
		font-size: 0.875rem;
		font-weight: 500;
		letter-spacing: 0.01em;
		margin-bottom: 0.25rem;
		position: relative;
		transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
	}

	/* Gold left border indicator for active/hover states - RTP Brand Gold */
	.nav-item::before {
		content: '';
		position: absolute;
		left: 0;
		top: 50%;
		transform: translateY(-50%);
		width: 3px;
		height: 0;
		background: #e6b800; /* RTP Gold - hardcoded for reliability */
		border-radius: 0 2px 2px 0;
		transition: height 0.15s cubic-bezier(0.16, 1, 0.3, 1);
	}

	.nav-item:hover {
		background: var(--admin-nav-bg-hover, rgba(37, 43, 51, 0.8));
		color: var(--admin-nav-text-hover, #f0f6fc);
	}

	.nav-item:hover::before {
		height: 50%;
		background: #e6b800; /* RTP Gold */
	}

	/* Clear hover state immediately when mouse leaves */
	.nav-item:not(:hover)::before {
		height: 0;
		transition: height 0.1s cubic-bezier(0.16, 1, 0.3, 1);
	}

	.nav-item:focus-visible {
		box-shadow: 0 0 0 3px rgba(230, 184, 0, 0.35); /* Gold focus ring */
		outline: none;
	}

	.nav-item.active {
		background: rgba(230, 184, 0, 0.12); /* Gold soft background */
		color: #ffd11a; /* Gold text */
		font-weight: 600;
	}

	.nav-item.active::before {
		height: 60%;
		background: #ffd11a; /* Bright gold for active */
	}

	/* Ensure active state is not affected by hover transition leak */
	.nav-item.active:not(:hover)::before {
		height: 60%;
	}

	/* Sidebar Footer */
	.sidebar-footer {
		padding: 1rem;
		border-top: 1px solid var(--admin-border-light);
	}

	/* User Info Card */
	.user-info {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem;
		background: var(--admin-surface-sunken);
		border-radius: var(--radius-lg, 0.75rem);
		margin-bottom: 0.75rem;
		transition: background-color 0.3s ease;
	}

	.user-avatar {
		width: 40px;
		height: 40px;
		border-radius: var(--radius-md, 0.5rem);
		background: linear-gradient(
			135deg,
			var(--admin-accent-primary),
			var(--admin-widget-purple-icon)
		);
		display: flex;
		align-items: center;
		justify-content: center;
		font-weight: 700;
		font-size: 1.125rem;
		color: white;
		flex-shrink: 0;
	}

	.user-details {
		flex: 1;
		min-width: 0;
		display: flex;
		flex-direction: column;
	}

	.user-name {
		font-family: var(--font-body), 'Roboto', sans-serif;
		font-weight: 500;
		font-size: 0.875rem;
		color: var(--admin-text-primary);
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.user-role {
		font-family: var(--font-body), 'Roboto', sans-serif;
		font-size: 0.75rem;
		font-weight: 400;
		color: var(--admin-text-muted);
	}

	/* Exit Button - Roboto 500 */
	.exit-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
		width: 100%;
		padding: 0.625rem 1rem;
		background: transparent;
		border: 1px solid var(--admin-error-border);
		border-radius: var(--radius-md, 0.5rem);
		color: var(--admin-text-muted);
		text-decoration: none;
		font-family: var(--font-body), 'Roboto', sans-serif;
		font-weight: 500;
		font-size: 0.875rem;
		letter-spacing: 0.01em;
		transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
	}

	.exit-btn:hover {
		background: var(--admin-error-bg);
		color: var(--admin-error);
		border-color: var(--admin-error-border);
	}

	.exit-btn:focus-visible {
		box-shadow: var(--admin-focus-ring);
		outline: none;
	}

	/* Mobile Overlay */
	.sidebar-overlay {
		display: none;
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.5);
		z-index: calc(var(--z-modal, 500) - 1);
		border: none;
		cursor: pointer;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * RESPONSIVE BREAKPOINTS
	 * ═══════════════════════════════════════════════════════════════════════════ */

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
