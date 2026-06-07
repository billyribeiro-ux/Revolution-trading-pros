<script lang="ts">
	/**
	 * AdminSidebar - Sidebar for admin dashboard
	 * Extracted from admin layout for reusability
	 *
	 * @version 2.0.0
	 * @author Revolution Trading Pros
	 */
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { user } from '$lib/stores/auth.svelte';
	import { logout } from '$lib/api/auth';
	import { isSuperadmin } from '$lib/config/roles';
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
		IconPackage
	} from '$lib/icons';
	interface Props {
		isOpen?: boolean;
		onclose?: () => void;
	}

	let props: Props = $props();

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
	// `goto('/admin', { invalidateAll: true })` flush. `user.current` is a
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

<aside class={{ 'admin-sidebar': true, open: props.isOpen }}>
	<!-- Header -->
	<div class="sidebar-header">
		<a href="/admin" class="sidebar-logo">
			<img src="/revolution-trading-pros.png" alt="RTP Admin" width="3163" height="835" />
		</a>
		<button class="close-btn" onclick={closeSidebar}>
			<IconX size={24} />
		</button>
	</div>

	<!-- Navigation -->
	<nav class="sidebar-nav" aria-label="Admin primary navigation">
		{#each menuSections as section (section.title ?? '__top__')}
			{#if section.title}
				<div class="nav-section-title">{section.title}</div>
			{/if}
			{#each section.items as item (item.href)}
				{@const Icon = item.icon}
				{@const active = item.href === bestActiveHref}
				<a
					href={item.href}
					class={{ 'nav-item': true, active }}
					aria-current={active ? 'page' : undefined}
					data-tooltip={item.label}
					onclick={closeSidebar}
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
		<button type="button" class="exit-btn" onclick={handleSignOut}>
			<IconLogout size={18} aria-hidden="true" />
			<span>Sign Out</span>
		</button>
	</div>
</aside>

<!-- Mobile Overlay -->
{#if props.isOpen}
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
		/* Use dynamic viewport height for better mobile browser support */
		height: 100vh;
		height: 100dvh;
		/* Safe area insets for notched devices (2026 mobile standards) */
		padding-top: env(safe-area-inset-top, 0);
		padding-bottom: env(safe-area-inset-bottom, 0);
		padding-left: env(safe-area-inset-left, 0);
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
		/* Touch target optimization - minimum 44px (2026 WCAG standards) */
		min-width: 44px;
		min-height: 44px;
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
		font-size: 0.8125rem;
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
		/* Touch target optimization - minimum 44px height (2026 WCAG standards) */
		min-height: 44px;
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

	/* FIX P3 (audits/admin-2026-04-26/01-shell-and-dashboard.md):
	   replaced 5 hardcoded RTP-gold hex values with `--admin-accent-primary`
	   (defined at the admin scope; resolves to gold today). Future re-themes
	   that change `--admin-accent-primary` will now flow through the
	   sidebar without missing this file. The "Gold text" / "Bright gold for
	   active" pair (#ffd11a) doesn't have a brighter sibling variable, so
	   we keep `var(--admin-accent-primary)` for those too — close enough
	   that the file no longer needs a per-shade override. */
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
		transition: height 0.3s ease-in-out;
	}

	.nav-item:hover {
		background: var(--admin-nav-bg-hover, rgba(37, 43, 51, 0.8));
		color: var(--admin-nav-text-hover, #f0f6fc);
	}

	.nav-item:hover::before {
		height: 50%;
		background: var(--admin-accent-primary);
	}

	/* Smooth exit - uses same transition from ::before */

	.nav-item:focus-visible {
		box-shadow: 0 0 0 3px rgba(230, 184, 0, 0.35); /* Gold focus ring (alpha-blended hex; var doesn't expose RGB triplet) */
		outline: none;
	}

	.nav-item.active {
		background: rgba(
			230,
			184,
			0,
			0.12
		); /* Gold soft background (alpha-blended hex; var doesn't expose RGB triplet) */
		color: var(--admin-accent-primary);
		font-weight: 600;
	}

	.nav-item.active::before {
		height: 60%;
		background: var(--admin-accent-primary);
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
	 *
	 * Three layout modes:
	 *   • >= 1280px       full sidebar (240px, labels visible)
	 *   • 1024–1279px     compact rail (72px, icons only — Linear/Vercel pattern)
	 *   • <  1024px       off-canvas drawer (slides in via .open)
	 *
	 * The compact rail keeps the sidebar visible at midrange widths instead of
	 * forcing a drawer pattern, so admins on 13–14" laptops still get persistent
	 * navigation. `.admin-main` reads `--admin-sidebar-width` for its left
	 * offset, so flipping the variable is enough to keep the layout in sync.
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Compact rail — labels collapse to icons only. CSS-only tooltips appear
	   on hover/focus to expose the missing label; group titles hide entirely.
	   No JS / no portal — same pattern Linear, Vercel, and GitHub use for icon
	   rails. The tooltip only renders inside this breakpoint, so the full
	   sidebar (>=1280px) and the mobile drawer (<1024px) never show it. */
	@media (min-width: 1024px) and (max-width: 1279px) {
		.admin-sidebar {
			width: 72px;
		}

		.sidebar-logo img {
			height: 28px;
		}

		.sidebar-nav {
			padding: 0.5rem 0.5rem;
		}

		.nav-section-title {
			display: none;
		}

		.nav-item {
			justify-content: center;
			gap: 0;
			padding: 0.6rem 0;
			/* Allow the absolutely-positioned tooltip pseudo-element to escape
			   the 72px rail. The .admin-sidebar itself has overflow-y: auto on
			   .sidebar-nav, so the tooltip lives on .nav-item's stacking
			   context but is positioned via `left: calc(100% + …)`. */
			position: relative;
		}

		.nav-item span {
			display: none;
		}

		/* CSS tooltip — shows the data-tooltip attribute on hover/focus.
		   Positioned to the right of the rail, with a small arrow. Hidden by
		   default; opacity transition for a subtle fade-in. */
		.nav-item[data-tooltip]::after {
			content: attr(data-tooltip);
			position: absolute;
			left: calc(100% + 0.75rem);
			top: 50%;
			transform: translateY(-50%);
			padding: 0.4rem 0.625rem;
			background: var(--admin-tooltip-bg, #0f1419);
			color: var(--admin-tooltip-text, #f0f6fc);
			border: 1px solid var(--admin-tooltip-border, rgba(230, 184, 0, 0.25));
			border-radius: 0.375rem;
			font-family: var(--font-body), 'Roboto', sans-serif;
			font-size: 0.8125rem;
			font-weight: 500;
			letter-spacing: 0.01em;
			white-space: nowrap;
			pointer-events: none;
			opacity: 0;
			visibility: hidden;
			transition:
				opacity 0.15s ease-out,
				visibility 0.15s ease-out;
			z-index: 1000;
			box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
		}

		.nav-item[data-tooltip]:hover::after,
		.nav-item[data-tooltip]:focus-visible::after {
			opacity: 1;
			visibility: visible;
		}

		.user-info {
			justify-content: center;
			padding: 0.5rem;
		}

		.user-details {
			display: none;
		}

		.exit-btn {
			padding: 0.625rem 0;
			gap: 0;
			position: relative;
		}

		.exit-btn span {
			display: none;
		}

		/* Mirror the tooltip pattern for the sign-out button. */
		.exit-btn::after {
			content: 'Sign Out';
			position: absolute;
			left: calc(100% + 0.75rem);
			top: 50%;
			transform: translateY(-50%);
			padding: 0.4rem 0.625rem;
			background: var(--admin-tooltip-bg, #0f1419);
			color: var(--admin-tooltip-text, #f0f6fc);
			border: 1px solid var(--admin-tooltip-border, rgba(230, 184, 0, 0.25));
			border-radius: 0.375rem;
			font-family: var(--font-body), 'Roboto', sans-serif;
			font-size: 0.8125rem;
			font-weight: 500;
			white-space: nowrap;
			pointer-events: none;
			opacity: 0;
			visibility: hidden;
			transition:
				opacity 0.15s ease-out,
				visibility 0.15s ease-out;
			z-index: 1000;
			box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
		}

		.exit-btn:hover::after,
		.exit-btn:focus-visible::after {
			opacity: 1;
			visibility: visible;
		}
	}

	/* Honour reduced-motion: skip the fade so tooltips appear instantly. */
	@media (prefers-reduced-motion: reduce) {
		.nav-item[data-tooltip]::after,
		.exit-btn::after {
			transition: none;
		}
	}

	/* Mobile / tablet — off-canvas drawer, requires explicit open toggle. */
	@media (max-width: 1023px) {
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
