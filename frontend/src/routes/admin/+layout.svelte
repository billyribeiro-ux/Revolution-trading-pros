<script lang="ts">
	/**
	 * Admin Layout - Dashboard shell for admin area
	 *
	 * @version 4.1.0 - Svelte 5 Runes
	 * @author Revolution Trading Pros
	 */
	import '$lib/styles/main.css';
	import '$lib/styles/admin-responsive.css';

	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';
	import { isAuthenticated, user, isInitializing } from '$lib/stores/auth.svelte';
	import { isAdmin as checkIsAdmin } from '$lib/config/roles';
	import { getUnreadCount } from '$lib/stores/notifications.svelte';
	import { keyboard } from '$lib/stores/keyboard.svelte';

	import IconMenu2 from '@tabler/icons-svelte-runes/icons/menu-2';
	import IconBell from '@tabler/icons-svelte-runes/icons/bell';
	import IconSearch from '@tabler/icons-svelte-runes/icons/search';
	import IconPlugConnected from '@tabler/icons-svelte-runes/icons/plug-connected';
	import IconCommand from '@tabler/icons-svelte-runes/icons/command';

	// PRINCIPAL-2026-04-26: onMount imported for auth-guard conversion (see line ~52).
	import { onMount } from 'svelte';
	import { AdminSidebar } from '$lib/components/layout';
	import Toast from '$lib/components/Toast.svelte';
	import CommandPalette from '$lib/components/CommandPalette.svelte';
	import NotificationCenter from '$lib/components/NotificationCenter.svelte';
	import KeyboardShortcutsHelp from '$lib/components/KeyboardShortcutsHelp.svelte';
	import RateLimitIndicator from '$lib/components/RateLimitIndicator.svelte';
	import ConnectionHealthPanel from '$lib/components/ConnectionHealthPanel.svelte';
	import OfflineIndicator from '$lib/components/OfflineIndicator.svelte';

	import type { Snippet } from 'svelte';

	// Props (Svelte 5 - no destructuring)
	interface Props {
		children: Snippet;
	}
	let props: Props = $props();

	// Local derived from getters
	const unreadCount = $derived(getUnreadCount());

	// State
	let isSidebarOpen = $state(false);
	let isCommandPaletteOpen = $state(false);
	let isNotificationCenterOpen = $state(false);
	let isKeyboardHelpOpen = $state(false);
	let isConnectionHealthOpen = $state(false);

	// PRINCIPAL-2026-04-26: auth-guard converted from $effect to onMount.
	// The previous $effect read $isAuthenticated, which under Svelte 5 compiles
	// to a `legacy_pre_subscribe` call that synchronously fires fn(getSnapshot())
	// — and getSnapshot() reads the _user $state rune. That synchronous write
	// to the compiler-generated subscriber rune INSIDE a running $effect is the
	// classic write-while-reading-tracked-dep pattern that trips
	// effect_update_depth_exceeded on the post-login goto('/admin',
	// { invalidateAll: true }) flush. Identical fix to dashboard/+layout.svelte:
	// the dashboard layout converted its equivalent guard to onMount with the
	// same reasoning. Admin was missed in that sweep (commit 01c4eecfe).
	// See https://svelte.dev/docs/svelte/$effect#When-not-to-use-$effect.
	//
	// Old code (kept for one revision per PRINCIPAL-2026-04-26 marker — delete in follow-up):
	// $effect(() => {
	// 	if (browser && !$isAuthenticated) {
	// 		goto('/login?redirect=/admin');
	// 	}
	// });
	// FIX-2026-04-26: replaced legacy `$isAuthenticated` autosubscribe with
	// `isAuthenticated.current` rune-getter. Even though the read sits in
	// onMount (untracked), the Svelte compiler hoists `legacy_pre_subscribe`
	// to component init, which adds a long-lived subscriber on
	// `authStore.subscribers` that participates in every post-login fan-out.
	// Removing the autosubscribe shrinks the fan-out volume.
	// Old: if (!$isAuthenticated) { goto('/login?redirect=/admin'); }
	//
	// FIX P0-2 (audits/admin-2026-04-26/01-shell-and-dashboard.md): role-based
	// guard added. Previously the shell only checked "any authenticated session"
	// and relied on the Rust API to 403 on each admin endpoint. We now also gate
	// at the route level: if the user is not admin/superadmin, send them to /
	// (route them to /admin/+error 403 via the home redirect message). We wait
	// for auth initialization to complete so we don't bounce a real admin whose
	// session is still being hydrated from the cookie.
	onMount(() => {
		if (!browser) return;

		// If auth is still initializing, defer the role check; subscribe once and
		// re-evaluate when initialization settles. The subscribe call here is a
		// one-shot guard — it runs outside any reactive scope (onMount, not
		// $effect), so there is no write-while-reading hazard.
		const evaluate = () => {
			if (isInitializing.current) return false;
			if (!isAuthenticated.current) {
				goto('/login?redirect=/admin');
				return true;
			}
			if (!checkIsAdmin(user.current)) {
				// Not an admin → kick out. Use replaceState so back button doesn't
				// trap the user in a redirect loop on /admin.
				goto('/?error=admin_required', { replaceState: true });
				return true;
			}
			return true;
		};

		if (evaluate()) return;

		// Wait for auth init to complete, then evaluate exactly once.
		const unsub = isInitializing.subscribe(() => {
			if (evaluate()) {
				queueMicrotask(() => unsub());
			}
		});
		return () => unsub();
	});

	// FIX-2026-04-26: keyboard-shortcut bootstrap converted from $effect to onMount.
	// The previous $effect called keyboard.init(), whose body executes
	// `storeState = { ...storeState, shortcuts }` — that's a READ+WRITE of the
	// same $state rune inside the effect's reactive scope. The cleanup return
	// `() => keyboard.destroy()` unconditionally sets `isInitialized = false`,
	// defeating the gate that was supposed to make init() idempotent — so each
	// scheduled re-run re-enters init, re-writes storeState, schedules another
	// re-run … and Svelte's depth guard throws after ~16 frames. onMount has
	// no dependency tracking, so the read inside the spread is harmless.
	// See https://svelte.dev/docs/svelte/$effect#When-not-to-use-$effect.
	//
	// Old code (kept for one revision per FIX-2026-04-26 marker — delete in follow-up):
	// $effect(() => {
	// 	if (!browser) return;
	// 	keyboard.init({
	// 		search: () => (isCommandPaletteOpen = true),
	// 		'search-alt': () => (isCommandPaletteOpen = true),
	// 		help: () => (isKeyboardHelpOpen = true),
	// 		'goto-dashboard': () => goto('/admin'),
	// 		'goto-analytics': () => goto('/admin/analytics'),
	// 		'goto-blog': () => goto('/admin/blog'),
	// 		'goto-settings': () => goto('/admin/settings'),
	// 		escape: () => {
	// 			isCommandPaletteOpen = false;
	// 			isNotificationCenterOpen = false;
	// 			isKeyboardHelpOpen = false;
	// 			isConnectionHealthOpen = false;
	// 		}
	// 	});
	// 	return () => keyboard.destroy();
	// });
	onMount(() => {
		if (!browser) return;

		keyboard.init({
			search: () => (isCommandPaletteOpen = true),
			'search-alt': () => (isCommandPaletteOpen = true),
			help: () => (isKeyboardHelpOpen = true),
			'goto-dashboard': () => goto('/admin'),
			'goto-analytics': () => goto('/admin/analytics'),
			'goto-blog': () => goto('/admin/blog'),
			'goto-settings': () => goto('/admin/settings'),
			escape: () => {
				isCommandPaletteOpen = false;
				isNotificationCenterOpen = false;
				isKeyboardHelpOpen = false;
				isConnectionHealthOpen = false;
			}
		});

		return () => keyboard.destroy();
	});

	// Functions
	function toggleSidebar() {
		isSidebarOpen = !isSidebarOpen;
	}

	function closeSidebar() {
		isSidebarOpen = false;
	}

	function formatPageTitle(pathname: string): string {
		const segments = pathname.split('/').filter(Boolean);
		if (segments.length <= 1) return 'Dashboard';

		const lastSegment = segments[segments.length - 1];

		const titleMap: Record<string, string> = {
			blog: 'Blog Posts',
			categories: 'Categories',
			media: 'Media Library',
			members: 'Members',
			segments: 'Segments',
			subscriptions: 'Subscriptions',
			products: 'Products',
			coupons: 'Coupons',
			crm: 'CRM',
			campaigns: 'Email Campaigns',
			templates: 'Email Templates',
			smtp: 'Email Settings',
			seo: 'SEO Settings',
			analytics: 'Analytics',
			behavior: 'Behavior Tracking',
			users: 'Admin Users',
			settings: 'Settings',
			popups: 'Popups',
			forms: 'Forms',
			videos: 'Videos',
			resources: 'Resources',
			indicators: 'Indicators',
			'site-health': 'Site Health',
			connections: 'API Connections'
		};

		return (
			titleMap[lastSegment] ||
			lastSegment.charAt(0).toUpperCase() + lastSegment.slice(1).replace(/-/g, ' ')
		);
	}
</script>

<svelte:head>
	<title>{formatPageTitle(page.url.pathname)} · Admin · Revolution Trading Pros</title>
</svelte:head>

<div class="admin-layout admin">
	<!-- Sidebar -->
	<AdminSidebar isOpen={isSidebarOpen} onclose={closeSidebar} />

	<!-- Main Content -->
	<div class="admin-main">
		<!-- Header -->
		<header class="admin-header">
			<button
				type="button"
				class="mobile-menu-btn"
				onclick={toggleSidebar}
				aria-label="Toggle sidebar"
			>
				<IconMenu2 size={24} />
			</button>

			<div class="header-title">
				<h1>{formatPageTitle(page.url.pathname)}</h1>
			</div>

			<div class="header-actions">
				<!-- Search — Linear/Vercel pattern: looks like a placeholder input,
				     opens the command palette. Wide click target on desktop, icon-only on mobile. -->
				<button
					type="button"
					class="search-trigger"
					onclick={() => (isCommandPaletteOpen = true)}
					title="Search (⌘K)"
					aria-label="Open search"
				>
					<IconSearch size={16} aria-hidden="true" />
					<span class="search-trigger__label desktop-only">Search…</span>
					<kbd class="kbd desktop-only">⌘K</kbd>
				</button>

				<!-- Icon-only utility cluster (notifications · connection · rate · shortcuts).
				     Grouped together visually so the row reads as one toolbar, not 5 stand-alone buttons. -->
				<div class="header-utility-group" role="group" aria-label="Admin utilities">
					<!-- Notifications -->
					<button
						type="button"
						class="icon-btn notification-btn"
						onclick={() => (isNotificationCenterOpen = true)}
						title="Notifications"
						aria-label="Open notifications"
					>
						<IconBell size={18} aria-hidden="true" />
						{#if unreadCount > 0}
							<span class="notification-badge">{unreadCount > 9 ? '9+' : unreadCount}</span>
						{/if}
					</button>

					<!-- Connection Health -->
					<button
						type="button"
						class="icon-btn"
						onclick={() => (isConnectionHealthOpen = true)}
						title="API Connection Status"
						aria-label="View API connection status"
					>
						<IconPlugConnected size={18} aria-hidden="true" />
					</button>

					<!-- Rate Limit -->
					<RateLimitIndicator />

					<!-- Keyboard Shortcuts -->
					<button
						type="button"
						class="icon-btn desktop-only"
						onclick={() => (isKeyboardHelpOpen = true)}
						title="Keyboard Shortcuts (?)"
						aria-label="View keyboard shortcuts"
					>
						<IconCommand size={18} aria-hidden="true" />
					</button>
				</div>

				<a href="/" class="view-site-btn" title="View main website">View Site</a>
			</div>
		</header>

		<!-- Content -->
		<main id="main-content" class="admin-content">
			{@render props.children()}
		</main>
	</div>
</div>

<!-- Global Components -->
<Toast />
<CommandPalette bind:isOpen={isCommandPaletteOpen} />
<NotificationCenter bind:isOpen={isNotificationCenterOpen} />
<KeyboardShortcutsHelp bind:isOpen={isKeyboardHelpOpen} />
<ConnectionHealthPanel bind:isOpen={isConnectionHealthOpen} />
<OfflineIndicator />

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   ADMIN LAYOUT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.admin-layout {
		/* Sidebar width defaults to 240px (full mode); collapses at the
		   compact-rail breakpoint and to 0 on mobile (drawer overlays). */
		--admin-sidebar-width: 240px;
		display: flex;
		background: var(--admin-bg);
		color: var(--admin-text-primary);
		transition:
			background-color 0.3s ease,
			color 0.3s ease;
	}

	.admin-main {
		flex: 1;
		margin-left: var(--admin-sidebar-width);
		display: flex;
		flex-direction: column;
		transition: margin-left 0.3s cubic-bezier(0.16, 1, 0.3, 1);
	}

	/* Compact rail breakpoint — sidebar shrinks to icon-only at 1024–1279px. */
	@media (min-width: 1024px) and (max-width: 1279px) {
		.admin-layout {
			--admin-sidebar-width: 72px;
		}
	}

	/* Mobile drawer — sidebar overlays content, no left offset. */
	@media (max-width: 1023px) {
		.admin-layout {
			--admin-sidebar-width: 0px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   HEADER
	   ═══════════════════════════════════════════════════════════════════════════ */

	.admin-header {
		height: 70px;
		background: var(--admin-header-bg);
		border-bottom: 1px solid var(--admin-header-border);
		box-shadow: var(--admin-header-shadow);
		display: flex;
		align-items: center;
		padding: 0 var(--space-8);
		gap: var(--space-4);
		position: sticky;
		top: 0;
		z-index: var(--z-sticky);
		transition:
			var(--transition-colors),
			box-shadow var(--duration-normal) var(--ease-default);
	}

	.mobile-menu-btn {
		display: none;
		background: transparent;
		border: none;
		color: var(--admin-text-muted);
		cursor: pointer;
		padding: var(--space-2);
		border-radius: var(--radius-md);
		transition: var(--transition-all);
	}

	.mobile-menu-btn:hover {
		background: var(--admin-btn-bg-hover);
		color: var(--admin-accent-primary);
	}

	.mobile-menu-btn:focus-visible {
		box-shadow: var(--admin-focus-ring);
		outline: none;
	}

	.header-title h1 {
		font-family: var(--font-display);
		font-size: var(--text-xl);
		font-weight: var(--font-semibold);
		color: var(--admin-text-primary);
		text-transform: capitalize;
		margin: 0;
		letter-spacing: var(--tracking-tight);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   HEADER ACTIONS
	   ═══════════════════════════════════════════════════════════════════════════ */

	.header-actions {
		margin-left: auto;
		display: flex;
		align-items: center;
		gap: var(--space-3);
	}

	/* Search trigger — Linear/Vercel pattern: looks like a placeholder input,
	   opens the command palette. ~280px wide so it visually anchors the right
	   side of the header without competing with the icon-only utility cluster. */
	.search-trigger {
		display: inline-flex;
		align-items: center;
		gap: var(--space-2);
		min-height: 36px;
		padding: 0 var(--space-3);
		background: var(--admin-surface-sunken, var(--admin-btn-bg));
		border: 1px solid var(--admin-btn-border);
		border-radius: var(--radius-md);
		color: var(--admin-text-muted);
		cursor: text;
		transition: var(--transition-all);
	}

	.search-trigger:hover {
		border-color: var(--admin-btn-border-hover);
		color: var(--admin-text-primary);
	}

	.search-trigger:focus-visible {
		box-shadow: var(--admin-focus-ring);
		outline: none;
	}

	.search-trigger__label {
		flex: 1;
		min-width: 200px;
		text-align: left;
		font-family: var(--font-sans);
		font-size: var(--text-sm);
		font-weight: var(--font-normal);
		letter-spacing: var(--tracking-normal);
	}

	.kbd {
		padding: var(--space-1) var(--space-2);
		background: var(--admin-kbd-bg);
		border: 1px solid var(--admin-kbd-border);
		border-radius: var(--radius-sm);
		font-size: var(--text-xs);
		font-weight: var(--font-semibold);
		color: var(--admin-kbd-text);
		font-family: var(--font-mono);
	}

	/* Utility cluster — notifications, connection, rate limit, shortcuts.
	   Visually grouped via a shared sunken surface so the row reads as one
	   toolbar instead of five orphan buttons. */
	.header-utility-group {
		display: inline-flex;
		align-items: center;
		gap: var(--space-1);
		padding: 4px;
		background: var(--admin-surface-sunken, transparent);
		border: 1px solid var(--admin-btn-border);
		border-radius: var(--radius-md);
	}

	/* RateLimitIndicator ships its own pill styling (green/amber/red bg +
	   1px border) so it can stand alone elsewhere. Inside this toolbar we
	   neutralize the pill chrome so it visually conforms to the surrounding
	   .icon-btn row — color is preserved (still semantic) but background,
	   border, and padding match the flat icon-btn shape. */
	.header-utility-group :global(.rate-limit-btn) {
		background: transparent;
		border: none;
		padding: 0 var(--space-2);
		min-height: 32px;
		border-radius: var(--radius-sm);
		gap: 4px;
	}

	.header-utility-group :global(.rate-limit-btn:hover) {
		background: var(--admin-btn-bg-hover);
	}

	.header-utility-group :global(.rate-limit-btn .limit-percentage) {
		font-size: var(--text-xs);
		font-weight: var(--font-semibold);
	}

	.icon-btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		padding: 0;
		background: transparent;
		border: none;
		border-radius: var(--radius-sm);
		color: var(--admin-text-muted);
		cursor: pointer;
		transition: var(--transition-all);
		position: relative;
	}

	.icon-btn:hover {
		background: var(--admin-btn-bg-hover);
		color: var(--admin-text-primary);
	}

	.icon-btn:focus-visible {
		box-shadow: var(--admin-focus-ring);
		outline: none;
	}

	.icon-btn:active {
		transform: scale(0.96);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   NOTIFICATION BADGE
	   ═══════════════════════════════════════════════════════════════════════════ */

	.notification-btn {
		position: relative;
	}

	.notification-badge {
		position: absolute;
		top: -4px;
		right: -4px;
		min-width: 18px;
		height: 18px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--color-loss);
		border-radius: var(--radius-full);
		font-size: var(--text-xs);
		font-weight: var(--font-bold);
		color: var(--color-bg-card);
		padding: 0 var(--space-1);
		box-shadow: var(--shadow-loss);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   VIEW SITE BUTTON
	   ═══════════════════════════════════════════════════════════════════════════ */

	.view-site-btn {
		padding: var(--space-2) var(--space-4);
		background: var(--color-watching-bg);
		color: var(--color-watching-text);
		text-decoration: none;
		border-radius: var(--radius-md);
		font-family: var(--font-sans);
		font-weight: var(--font-medium);
		font-size: var(--text-sm);
		letter-spacing: var(--tracking-normal);
		border: 1px solid var(--color-watching-border);
		transition: var(--transition-all);
	}

	.view-site-btn:hover {
		background: var(--color-watching);
		border-color: var(--color-watching);
		color: var(--color-bg-card);
		transform: translateY(-1px);
	}

	.view-site-btn:focus-visible {
		box-shadow: 0 0 0 3px var(--color-watching-bg);
		outline: none;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CONTENT AREA
	   ═══════════════════════════════════════════════════════════════════════════ */

	.admin-content {
		flex: 1;
		padding: var(--space-8);
		overflow-y: auto;
		background: var(--admin-bg);
	}

	.desktop-only {
		display: inline-flex;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE - Tablet (< lg: 1024px)
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 1023px) {
		.admin-main {
			margin-left: 0;
		}

		.mobile-menu-btn {
			display: block;
		}

		.admin-content {
			padding: var(--space-6);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE - Mobile Landscape (< md: 768px)
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 767px) {
		:where(.desktop-only) {
			display: none;
		}

		/* Search collapses to icon-only on small screens — the placeholder text
		   and ⌘K hint hide via .desktop-only above. */
		.search-trigger {
			min-width: 0;
			padding: 0 var(--space-2);
			min-height: 36px;
			width: 36px;
			justify-content: center;
		}

		.view-site-btn {
			padding: var(--space-2) var(--space-3);
			font-size: var(--text-sm);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE - Mobile Portrait (< sm: 640px)
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 639px) {
		.admin-header {
			padding: 0 var(--space-4);
			height: 60px;
		}

		.header-title h1 {
			font-size: var(--text-lg);
		}

		.admin-content {
			padding: var(--space-4);
		}

		.header-actions {
			gap: var(--space-2);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE - Extra Small Mobile
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 380px) {
		.admin-header {
			padding: 0 var(--space-3);
			height: 56px;
		}

		.header-title h1 {
			font-size: var(--text-base);
		}

		.header-actions {
			gap: var(--space-1);
		}

		.view-site-btn {
			padding: var(--space-2) var(--space-3);
			font-size: var(--text-xs);
		}

		.admin-content {
			padding: var(--space-3);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   TOUCH DEVICES - 44pt minimum touch targets
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (hover: none) and (pointer: coarse) {
		.search-trigger,
		.icon-btn,
		.mobile-menu-btn,
		.view-site-btn {
			min-height: 44px;
			min-width: 44px;
		}

		.icon-btn {
			width: 44px;
			height: 44px;
		}

		.view-site-btn {
			padding: var(--space-3) var(--space-4);
		}

		.header-actions {
			gap: var(--space-2);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ACCESSIBILITY
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		.admin-layout,
		.admin-header,
		.search-trigger,
		.icon-btn,
		.mobile-menu-btn,
		.view-site-btn {
			transition: none;
		}

		.view-site-btn:hover,
		.mobile-menu-btn:hover {
			transform: none;
		}
	}

	@media (prefers-contrast: high) {
		.admin-header {
			border-bottom-width: 2px;
		}

		.search-trigger,
		.header-utility-group,
		.view-site-btn {
			border-width: 2px;
		}

		.header-title h1 {
			font-weight: var(--font-extrabold);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   PRINT
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media print {
		.admin-header,
		.mobile-menu-btn {
			display: none;
		}

		.admin-main {
			margin-left: 0;
		}

		.admin-content {
			padding: 0;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   LANDSCAPE - Short Viewport
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-height: 500px) and (orientation: landscape) {
		.admin-header {
			height: 50px;
			padding: 0 var(--space-4);
		}

		.header-title h1 {
			font-size: var(--text-base);
		}

		.admin-content {
			padding: var(--space-3) var(--space-4);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   GLOBAL TAB PANEL UTILITIES - Layout Shift Prevention
	   ICT 7 Principal Engineer Grade - Svelte 5 Best Practices
	   ═══════════════════════════════════════════════════════════════════════════ */

	:global(.admin-tab-container) {
		position: relative;
		min-height: 400px;
		contain: layout style;
		isolation: isolate;
	}

	:global(.admin-tab-panel) {
		position: absolute;
		inset: 0;
		width: 100%;
		contain: content;
		opacity: 0;
		visibility: hidden;
		transform: translateY(8px);
		transition:
			opacity 0.2s ease,
			visibility 0.2s ease,
			transform 0.2s ease;
		z-index: 0;
		pointer-events: none;
	}

	:global(.admin-tab-panel.active) {
		position: relative;
		opacity: 1;
		visibility: visible;
		transform: translateY(0);
		z-index: 1;
		pointer-events: auto;
	}

	@media (prefers-reduced-motion: reduce) {
		:global(.admin-tab-panel) {
			transition: none;
			transform: none;
		}
		:global(.admin-tab-panel:not(.active)) {
			display: none;
		}
	}
</style>
