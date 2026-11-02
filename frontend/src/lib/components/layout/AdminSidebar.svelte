<script lang="ts">
	/**
	 * AdminSidebar - Sidebar for admin dashboard
	 * Extracted from admin layout for reusability
	 *
	 * @version 2.0.0
	 * @author Revolution Trading Pros
	 */
	import { page } from '$app/state';
	import { MediaQuery } from 'svelte/reactivity';
	import IconLayoutSidebarLeftCollapse from '@tabler/icons-svelte-runes/icons/layout-sidebar-left-collapse';
	import IconLayoutSidebarLeftExpand from '@tabler/icons-svelte-runes/icons/layout-sidebar-left-expand';
	import { goto } from '$app/navigation';
	import { user } from '#lib/stores/auth.svelte.js';
	import { logout } from '#lib/api/auth.js';
	import { isSuperadmin } from '#lib/config/roles.js';
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
		IconTrendingUp,
		IconLayoutKanban,
		IconBookmark,
		IconClock,
		IconShieldLock,
		IconGauge,
		IconPackage,
		IconCrown
	} from '#lib/icons/index.js';
	interface Props {
		isOpen?: boolean;
		/** Desktop preference: collapse to the vertical icon rail. */
		collapsed?: boolean;
		onclose?: () => void;
		ontoggle?: () => void;
	}

	let props: Props = $props();

	// Breakpoints follow the canonical scale in design-tokens.css (lg 1024 / xl 1280).
	const isDesktop = new MediaQuery('min-width: 1024px');
	const isWide = new MediaQuery('min-width: 1280px');

	// Rail = icon-only vertical bar. Forced on mid-range desktops (1024–1279px),
	// optional on wide desktops via the toggle. Never applies to the mobile drawer.
	const rail = $derived(isDesktop.current && (props.collapsed || !isWide.current));

	// Tooltip for the rail. Rendered as a fixed element outside the scrolling
	// <nav>, so `overflow` can never clip it.
	let tip = $state<{ label: string; top: number; left: number } | null>(null);

	function showTip(event: Event, label: string) {
		if (!rail) return;
		const rect = (event.currentTarget as HTMLElement).getBoundingClientRect();
		tip = { label, top: rect.top + rect.height / 2, left: rect.right + 12 };
	}

	function hideTip() {
		tip = null;
	}

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
				{ icon: IconTicket, label: 'Coupons', href: '/admin/coupons' },
				{ icon: IconCrown, label: 'Memberships', href: '/admin/memberships' },
				{ icon: IconPackage, label: 'Orders', href: '/admin/orders' },
				{ icon: IconShoppingCart, label: 'Abandoned Carts', href: '/admin/cart/abandoned' }
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
				{ icon: IconForms, label: 'Forms', href: '/admin/forms' },
				{ icon: IconBookmark, label: 'Watchlist', href: '/admin/watchlist' },
				{ icon: IconLayoutKanban, label: 'Boards', href: '/admin/boards' }
			]
		},
		{
			title: 'Marketing',
			items: [
				{ icon: IconSend, label: 'Campaigns', href: '/admin/email/campaigns' },
				{ icon: IconMail, label: 'Email Templates', href: '/admin/email/templates' },
				{ icon: IconMail, label: 'Email Settings', href: '/admin/email/smtp' },
				{ icon: IconUsers, label: 'Subscribers', href: '/admin/email/subscribers' },
				{ icon: IconSeo, label: 'SEO', href: '/admin/seo' }
			]
		},
		{
			title: 'Analytics',
			items: [
				{ icon: IconChartBar, label: 'Dashboard', href: '/admin/analytics' },
				{ icon: IconEye, label: 'Behavior', href: '/admin/behavior' },
				{ icon: IconUsers, label: 'CRM', href: '/admin/crm' },
				{ icon: IconGauge, label: 'Performance', href: '/admin/performance' }
			]
		},
		{
			title: 'System',
			items: [
				{ icon: IconHeartbeat, label: 'Site Health', href: '/admin/site-health' },
				{ icon: IconPlugConnected, label: 'Connections', href: '/admin/connections' },
				{ icon: IconUsers, label: 'Admin Users', href: '/admin/users' },
				{ icon: IconClock, label: 'Schedules', href: '/admin/schedules' },
				{ icon: IconShieldLock, label: 'Consent', href: '/admin/consent' },
				{ icon: IconSettings, label: 'Settings', href: '/admin/settings' }
			]
		}
	];

	function closeSidebar() {
		props.onclose?.();
	}

	let currentPath = $derived(page.url.pathname);

	// Best-active-href: longest matching prefix.
	// Fixes nested-route highlighting — `/admin/members/123` should highlight
	// `/admin/members`, and `/admin/members/segments` should pick `/segments`
	// (not `/members`) because it's a longer match.
	let bestActiveHref = $derived.by(() => {
		const allHrefs = menuSections.flatMap((s) => s.items.map((i) => i.href));
		let best = '';
		for (const href of allHrefs) {
			if (currentPath === href || currentPath.startsWith(href + '/')) {
				if (href.length > best.length) best = href;
			}
		}
		return best;
	});

	// FIX-2026-04-26: replaced legacy `$user` autosubscribe with `user.current`
	// rune-getter. The previous `$user` reference inside `$derived.by` registered
	// a `legacy_pre_subscribe(user)` at component init, whose synchronous
	// `fn(getSnapshot())` reads 7 $state runes inside a tracked reactive context
	// and writes the synthetic mirror rune — that's the read-and-write
	// self-loop that fires `effect_update_depth_exceeded` on the post-login
	// `goto('/admin', { refreshAll: true })` flush. `user.current` is a
	// plain getter, no fan-out, no synthetic rune writes.
	// Old: const u = $user;
	let displayRole = $derived.by(() => {
		const u = user.current;
		if (!u) return 'User';
		if (isSuperadmin(u)) return 'Super Admin';
		const role = (u as { role?: string }).role;
		if (typeof role === 'string' && role.length > 0) {
			return role
				.split(/[_-]/)
				.map((part) => part.charAt(0).toUpperCase() + part.slice(1).toLowerCase())
				.join(' ');
		}
		return 'Member';
	});

	// Real sign-out — clears the auth session, then routes home.
	// Replaces the old <a href="/"> which only navigated and left tokens intact.
	async function handleSignOut() {
		try {
			await logout();
		} catch (err) {
			// Even if the API logout 500s, drop client state and move on.
			console.warn('[AdminSidebar] logout API failed; signing out client-side anyway', err);
		}
		closeSidebar();
		await goto('/');
	}

	// FIX-2026-04-26: Esc-to-close on mobile drawer.
	// Audit P3 nit: a user on mobile-with-keyboard couldn't dismiss the drawer
	// with Escape. The layout's keyboard.init() escape action only closes the
	// command palette / notification center / shortcuts help / connection-health
	// panel — not the sidebar drawer itself. Listening on window keeps focus
	// requirements minimal (drawer doesn't need to own focus to receive Esc).
	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape' && props.isOpen) {
			closeSidebar();
		}
	}
</script>

<svelte:window onkeydown={handleKeydown} />

<aside
	class={{ 'admin-sidebar': true, open: props.isOpen, rail }}
	aria-label="Admin sidebar"
	inert={!isDesktop.current && !props.isOpen}
>
	<!-- Header -->
	<div class="sidebar-header">
		<a href="/admin" class="sidebar-logo" aria-label="Revolution Trading Pros admin home">
			<img src="/revolution-trading-pros-mark.png" alt="" width="256" height="256" />
		</a>
		{#if isWide.current}
			<button
				type="button"
				class="header-btn collapse-btn"
				onclick={() => props.ontoggle?.()}
				onmouseenter={(e) => showTip(e, props.collapsed ? 'Expand sidebar' : 'Collapse sidebar')}
				onmouseleave={hideTip}
				onfocus={(e) => showTip(e, props.collapsed ? 'Expand sidebar' : 'Collapse sidebar')}
				onblur={hideTip}
				aria-label={props.collapsed ? 'Expand sidebar' : 'Collapse sidebar'}
				aria-pressed={props.collapsed}
			>
				{#if props.collapsed}
					<IconLayoutSidebarLeftExpand size={18} aria-hidden="true" />
				{:else}
					<IconLayoutSidebarLeftCollapse size={18} aria-hidden="true" />
				{/if}
			</button>
		{/if}
		<button
			type="button"
			class="header-btn close-btn"
			onclick={closeSidebar}
			aria-label="Close sidebar"
		>
			<IconX size={20} aria-hidden="true" />
		</button>
	</div>

	<!-- Navigation -->
	<nav class="sidebar-nav" aria-label="Admin primary navigation" onscroll={hideTip}>
		{#each menuSections as section (section.title ?? '__top__')}
			{#if section.title}
				<div class="nav-section-title" aria-hidden={rail}>{section.title}</div>
			{/if}
			{#each section.items as item (item.href)}
				{@const Icon = item.icon}
				{@const active = item.href === bestActiveHref}
				<a
					href={item.href}
					class={{ 'nav-item': true, active }}
					aria-current={active ? 'page' : undefined}
					aria-label={rail ? item.label : undefined}
					onclick={() => {
						hideTip();
						closeSidebar();
					}}
					onmouseenter={(e) => showTip(e, item.label)}
					onmouseleave={hideTip}
					onfocus={(e) => showTip(e, item.label)}
					onblur={hideTip}
				>
					<Icon size={20} aria-hidden="true" />
					<span>{item.label}</span>
				</a>
			{/each}
		{/each}
	</nav>

	<!-- User Section -->
	<div class="sidebar-footer">
		<div class="user-info">
			<div class="user-avatar" aria-hidden="true">
				{user.current?.name?.[0]?.toUpperCase() || 'A'}
			</div>
			<div class="user-details">
				<span class="user-name">{user.current?.name || 'Admin'}</span>
				<span class="user-role">{displayRole}</span>
			</div>
		</div>
		<button
			type="button"
			class="exit-btn"
			onclick={handleSignOut}
			onmouseenter={(e) => showTip(e, 'Sign Out')}
			onmouseleave={hideTip}
			onfocus={(e) => showTip(e, 'Sign Out')}
			onblur={hideTip}
			aria-label={rail ? 'Sign Out' : undefined}
		>
			<IconLogout size={18} aria-hidden="true" />
			<span>Sign Out</span>
		</button>
	</div>
</aside>

{#if tip && rail}
	<div class="rail-tooltip" role="tooltip" style:top="{tip.top}px" style:left="{tip.left}px">
		{tip.label}
	</div>
{/if}

<!-- Mobile Overlay -->
{#if props.isOpen}
	<button type="button" class="sidebar-overlay" onclick={closeSidebar} aria-label="Close sidebar"
	></button>
{/if}

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * ADMIN SIDEBAR
	 * Three modes (breakpoints from design-tokens.css):
	 *   • >= 1280px   full sidebar (240px) — can be collapsed to the rail by the user
	 *   • 1024–1279px icon rail (72px)
	 *   • <  1024px   off-canvas drawer
	 * Colours come only from the admin token layer (colors.css / admin-surfaces.css).
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
		height: 100dvh;
		padding-top: env(safe-area-inset-top, 0);
		padding-bottom: env(safe-area-inset-bottom, 0);
		padding-left: env(safe-area-inset-left, 0);
		z-index: var(--z-modal, 400);
		transition: width 0.2s ease;
	}

	.admin-sidebar.rail {
		width: 72px;
	}

	/* Header: logo mark + controls */
	.sidebar-header {
		height: var(--admin-header-height, 70px);
		padding: 0 1rem;
		border-bottom: 1px solid var(--admin-border-light);
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 0.5rem;
		flex-shrink: 0;
	}

	.sidebar-logo {
		display: inline-flex;
		align-items: center;
		border-radius: var(--radius-md, 6px);
	}

	.sidebar-logo img {
		width: 36px;
		height: 36px;
		object-fit: contain;
	}

	.sidebar-logo:focus-visible,
	.nav-item:focus-visible,
	.header-btn:focus-visible,
	.exit-btn:focus-visible {
		box-shadow: var(--admin-focus-ring);
		outline: none;
	}

	.header-btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		padding: 0;
		background: transparent;
		border: none;
		border-radius: var(--radius-sm, 4px);
		color: var(--admin-text-muted);
		cursor: pointer;
		transition: background-color 0.2s ease, color 0.2s ease;
	}

	.header-btn:hover:not(:disabled) {
		background: var(--admin-btn-bg-hover);
		color: var(--admin-text-primary);
	}

	.header-btn:disabled {
		cursor: default;
	}

	/*
	 * Collapse toggle: blends into the sidebar. Near-invisible at rest, it comes back to
	 * full strength when the sidebar is hovered, the button is hovered, or it has keyboard
	 * focus, so it stays discoverable and keyboard-reachable.
	 */
	.collapse-btn {
		color: var(--admin-border);
		transition:
			color 0.2s ease,
			background-color 0.2s ease;
	}

	.admin-sidebar:hover .collapse-btn,
	.collapse-btn:hover,
	.collapse-btn:focus-visible {
		color: var(--admin-text-muted);
	}

	.collapse-btn:hover:not(:disabled) {
		color: var(--admin-text-primary);
	}

	@media (forced-colors: active), (pointer: coarse) {
		.collapse-btn {
			color: var(--admin-text-muted);
		}
	}

	/* The drawer shows a close button; the desktop shows the collapse toggle. */
	.close-btn {
		display: none;
	}

	/* Navigation */
	.sidebar-nav {
		flex: 1;
		padding: 0.5rem 1rem;
		overflow-y: auto;
		overflow-x: hidden;
	}

	.nav-section-title {
		font-family: var(--font-heading);
		font-size: 0.75rem;
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

	.nav-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		min-height: 44px;
		padding: 0.5rem 0.75rem;
		color: var(--admin-nav-text);
		text-decoration: none;
		border-radius: var(--radius-md, 6px);
		font-family: var(--font-body);
		font-size: 0.875rem;
		font-weight: 500;
		letter-spacing: 0.01em;
		margin-bottom: 0.25rem;
		position: relative;
		transition: background-color 0.2s ease, color 0.2s ease;
	}

	.nav-item::before {
		content: '';
		position: absolute;
		left: 0;
		top: 50%;
		transform: translateY(-50%);
		width: 3px;
		height: 0;
		background: var(--admin-accent-primary);
		border-radius: 0 2px 2px 0;
		transition: height 0.2s ease;
	}

	.nav-item:hover {
		background: var(--admin-nav-bg-hover);
		color: var(--admin-nav-text-hover);
	}

	.nav-item:hover::before {
		height: 50%;
	}

	.nav-item.active {
		background: var(--sidebar-active);
		color: var(--admin-accent-primary);
		font-weight: 600;
	}

	.nav-item.active::before {
		height: 60%;
	}

	/* Footer */
	.sidebar-footer {
		padding: 1rem;
		border-top: 1px solid var(--admin-border-light);
		flex-shrink: 0;
	}

	.user-info {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem;
		background: var(--admin-surface-sunken);
		border: 1px solid var(--admin-border-light);
		border-radius: var(--radius-lg, 8px);
		margin-bottom: 0.75rem;
	}

	.user-avatar {
		width: 36px;
		height: 36px;
		border-radius: var(--radius-md, 6px);
		background: var(--admin-accent-bg);
		border: 1px solid var(--sidebar-active);
		display: flex;
		align-items: center;
		justify-content: center;
		font-weight: 700;
		font-size: 1rem;
		color: var(--admin-accent-primary);
		flex-shrink: 0;
	}

	.user-details {
		flex: 1;
		min-width: 0;
		display: flex;
		flex-direction: column;
	}

	.user-name {
		font-family: var(--font-body);
		font-weight: 500;
		font-size: 0.875rem;
		color: var(--admin-text-primary);
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.user-role {
		font-family: var(--font-body);
		font-size: 0.75rem;
		font-weight: 400;
		color: var(--admin-text-muted);
	}

	.exit-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
		width: 100%;
		min-height: 40px;
		padding: 0 1rem;
		background: transparent;
		border: 1px solid var(--admin-border);
		border-radius: var(--radius-md, 6px);
		color: var(--admin-text-secondary);
		font-family: var(--font-body);
		font-weight: 500;
		font-size: 0.875rem;
		letter-spacing: 0.01em;
		cursor: pointer;
		transition: background-color 0.2s ease, color 0.2s ease, border-color 0.2s ease;
	}

	.exit-btn:hover {
		background: var(--admin-error-bg);
		color: var(--admin-error);
		border-color: var(--admin-error-border);
	}

	/* Rail mode: icons only, section titles collapse to hairline dividers */
	.admin-sidebar.rail .sidebar-header {
		flex-direction: column;
		justify-content: center;
		height: auto;
		min-height: var(--admin-header-height, 70px);
		padding: 0.75rem 0;
		gap: 0.5rem;
	}

	.admin-sidebar.rail .sidebar-nav {
		padding: 0.5rem;
	}

	.admin-sidebar.rail .nav-section-title {
		font-size: 0;
		height: 0;
		padding: 0;
		margin: 0.5rem 0.25rem;
		border-top: 1px solid var(--admin-border-light);
	}

	.admin-sidebar.rail .nav-item {
		justify-content: center;
		gap: 0;
		padding: 0.5rem 0;
	}

	.admin-sidebar.rail .nav-item span,
	.admin-sidebar.rail .exit-btn span,
	.admin-sidebar.rail .user-details {
		display: none;
	}

	.admin-sidebar.rail .sidebar-footer {
		padding: 0.75rem 0.5rem;
	}

	.admin-sidebar.rail .user-info {
		justify-content: center;
		padding: 0.5rem 0;
	}

	.admin-sidebar.rail .exit-btn {
		padding: 0;
	}

	/* Rail tooltip: fixed so the scrolling <nav> can never clip it */
	.rail-tooltip {
		position: fixed;
		transform: translateY(-50%);
		padding: 0.375rem 0.625rem;
		background: var(--admin-tooltip-bg);
		color: var(--admin-tooltip-text);
		border: 1px solid var(--admin-tooltip-border);
		border-radius: var(--radius-md, 6px);
		font-family: var(--font-body);
		font-size: 0.8125rem;
		font-weight: 500;
		white-space: nowrap;
		pointer-events: none;
		z-index: var(--z-tooltip, 700);
		box-shadow: var(--shadow-lg, 0 8px 24px rgba(0, 0, 0, 0.35));
	}

	.sidebar-overlay {
		display: none;
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.6);
		z-index: calc(var(--z-modal, 400) - 1);
		border: none;
		cursor: pointer;
	}

	/* Mobile / tablet drawer */
	@media (max-width: 1023.98px) {
		.admin-sidebar {
			transform: translateX(-100%);
			transition: transform 0.3s cubic-bezier(0.16, 1, 0.3, 1);
		}

		.admin-sidebar.open {
			transform: translateX(0);
		}

		.collapse-btn {
			display: none;
		}

		.close-btn {
			display: inline-flex;
			width: 44px;
			height: 44px;
		}

		.sidebar-overlay {
			display: block;
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.admin-sidebar,
		.nav-item,
		.nav-item::before,
		.header-btn,
		.exit-btn {
			transition: none;
		}
	}
</style>
