<script lang="ts">
	/**
	 * Admin Layout - Dashboard shell for admin area
	 *
	 * @version 4.2.0 - Dedicated Isolated admin.css
	 * @author Revolution Trading Pros
	 */
	import '#lib/styles/admin.css';

	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { isAuthenticated, isInitializing } from '#lib/stores/auth.svelte.js';
	import { useCrmAccess } from '#lib/crm/access.svelte.js';
	import { getUnreadCount } from '#lib/stores/notifications.svelte.js';
	import { keyboard } from '#lib/stores/keyboard.svelte.js';

	import IconMenu2 from '@tabler/icons-svelte-runes/icons/menu-2';
	import IconBell from '@tabler/icons-svelte-runes/icons/bell';
	import IconSearch from '@tabler/icons-svelte-runes/icons/search';
	import IconPlugConnected from '@tabler/icons-svelte-runes/icons/plug-connected';
	import IconCommand from '@tabler/icons-svelte-runes/icons/command';
	import IconWorld from '@tabler/icons-svelte-runes/icons/world';

	// PRINCIPAL-2026-04-26: onMount imported for auth-guard conversion (see line ~52).
	import { onMount } from 'svelte';
	import { AdminSidebar } from '#lib/components/layout/index.js';
	import Toast from '#lib/components/Toast.svelte';
	import CommandPalette from '#lib/components/CommandPalette.svelte';
	import NotificationCenter from '#lib/components/NotificationCenter.svelte';
	import KeyboardShortcutsHelp from '#lib/components/KeyboardShortcutsHelp.svelte';
	import RateLimitIndicator from '#lib/components/RateLimitIndicator.svelte';
	import ConnectionHealthPanel from '#lib/components/ConnectionHealthPanel.svelte';
	import OfflineIndicator from '#lib/components/OfflineIndicator.svelte';

	import type { Snippet } from 'svelte';

	// Props (Svelte 5 - no destructuring)
	interface Props {
		children: Snippet;
	}
	let props: Props = $props();
	const access = useCrmAccess();

	// Local derived from getters
	const unreadCount = $derived(getUnreadCount());

	// State
	let isSidebarOpen = $state(false);
	let isCommandPaletteOpen = $state(false);
	let isNotificationCenterOpen = $state(false);
	let isKeyboardHelpOpen = $state(false);
	let isConnectionHealthOpen = $state(false);

	// Desktop preference: collapse the sidebar to the vertical icon rail.
	const SIDEBAR_COLLAPSED_KEY = 'rtp-admin-sidebar-collapsed';
	// Read synchronously so the first paint already has the right width. Restoring it in
	// onMount shifted the whole shell (sidebar + content) for every user who collapsed it.
	// The admin area is ssr = false, so localStorage is available at init.
	function readCollapsedPreference(): boolean {
		if (typeof localStorage === 'undefined') return false;
		try {
			return localStorage.getItem(SIDEBAR_COLLAPSED_KEY) === '1';
		} catch {
			return false;
		}
	}
	let isSidebarCollapsed = $state(readCollapsedPreference());

	function toggleSidebarCollapsed() {
		isSidebarCollapsed = !isSidebarCollapsed;
		try {
			localStorage.setItem(SIDEBAR_COLLAPSED_KEY, isSidebarCollapsed ? '1' : '0');
		} catch (err) {
			// Storage can be blocked (private mode); the preference simply won't persist.
			console.warn('[admin layout] could not persist sidebar preference', err);
		}
	}

	// PRINCIPAL-2026-04-26: auth-guard converted from $effect to onMount.
	// The previous $effect read $isAuthenticated, which under Svelte 5 compiles
	// to a `legacy_pre_subscribe` call that synchronously fires fn(getSnapshot())
	// — and getSnapshot() reads the _user $state rune. That synchronous write
	// to the compiler-generated subscriber rune INSIDE a running $effect is the
	// classic write-while-reading-tracked-dep pattern that trips
	// effect_update_depth_exceeded on the post-login goto('/admin',
	// { refreshAll: true }) flush. Identical fix to dashboard/+layout.svelte:
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
			if (!access.global && !access.page(page.url.pathname)) {
				// Not an admin → kick out. Use replaceState so back button doesn't
				// trap the user in a redirect loop on /admin.
				goto('/?error=admin_required', { replace: true });
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
	// 		'goto-dashboard': () => goto(access.global ? '/admin' : '/admin/crm'),
	// 		'goto-analytics': () => { if (access.page('/admin/analytics')) void goto('/admin/analytics'); },
	// 		'goto-blog': () => { if (access.page('/admin/blog')) void goto('/admin/blog'); },
	// 		'goto-settings': () => { if (access.page('/admin/settings')) void goto('/admin/settings'); },
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
		keyboard.init({
			search: () => (isCommandPaletteOpen = true),
			'search-alt': () => (isCommandPaletteOpen = true),
			help: () => (isKeyboardHelpOpen = true),
			'goto-dashboard': () => goto(access.global ? '/admin' : '/admin/crm'),
			'goto-analytics': () => {
				if (access.page('/admin/analytics')) void goto('/admin/analytics');
			},
			'goto-blog': () => {
				if (access.page('/admin/blog')) void goto('/admin/blog');
			},
			'goto-settings': () => {
				if (access.page('/admin/settings')) void goto('/admin/settings');
			},
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

<div class={{ 'admin-layout': true, admin: true, 'sidebar-collapsed': isSidebarCollapsed }}>
	<!-- Sidebar -->
	<AdminSidebar
		isOpen={isSidebarOpen}
		collapsed={isSidebarCollapsed}
		onclose={closeSidebar}
		ontoggle={toggleSidebarCollapsed}
	/>

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
					<IconSearch size={18} aria-hidden="true" />
					<span class="search-trigger__label desktop-only">Search…</span>
					<kbd class="kbd desktop-only">⌘K</kbd>
				</button>

				<!-- Icon-only utility cluster (notifications · connection · rate · shortcuts).
				     Grouped together visually so the row reads as one toolbar, not 5 stand-alone buttons. -->
				{#if access.global}<div
						class="header-utility-group"
						role="group"
						aria-label="Admin utilities"
					>
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
					</div>{/if}

				<a href="/" class="view-site-btn" title="View main website" aria-label="View Site">
					<IconWorld size={18} aria-hidden="true" class="view-site-icon" />
					<span class="view-site-label">View Site</span>
				</a>
			</div>
		</header>

		<!-- Content -->
		<main id="main-content" class="admin-content">
			{#if access.error}<p role="alert">{access.error}</p>
				<button onclick={() => void access.refresh().catch(() => undefined)}
					>Refresh CRM permissions</button
				>{/if}
			{#if access.page(page.url.pathname)}{@render props.children()}{:else}<p role="alert">
					Your current CRM access does not include this page.
				</p>{/if}
		</main>
	</div>
</div>

<!-- Global Components. `.admin-tokens` gives these overlays the same --admin-* /
     palette variables as the shell (they render outside `.admin-layout`). -->
<div class="admin-tokens">
	<Toast />
	<CommandPalette bind:isOpen={isCommandPaletteOpen} />
	{#if access.global}<NotificationCenter bind:isOpen={isNotificationCenterOpen} />
		<KeyboardShortcutsHelp bind:isOpen={isKeyboardHelpOpen} />
		<ConnectionHealthPanel bind:isOpen={isConnectionHealthOpen} />{/if}
	<OfflineIndicator />
</div>
