<script lang="ts">
	/**
	 * Secondary Navigation Component - WordPress EXACT Implementation
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * The lighter-colored submenu panel that appears when:
	 * - User clicks on their profile/avatar (Account section - WooCommerce style)
	 * - User navigates to a membership dashboard (e.g., Mastering the Trade)
	 *
	 * WordPress structure: .dashboard__nav-secondary
	 * Account uses: woocommerce-MyAccount-navigation-link classes with no-icon
	 * Membership uses: st-icon-* classes mapped to Tabler icons
	 *
	 * Font: "Open Sans", sans-serif
	 * Font size: 14px (main), 13px (submenu)
	 * Font weight: 400
	 *
	 * @version 2.0.0 (WordPress EXACT / December 2025)
	 */

	import { page } from '$app/stores';
	import type { UserMembership } from '$lib/api/user-memberships';

	// Tabler Icons - Mapped to WordPress st-icon-* classes
	import IconLayoutDashboard from '@tabler/icons-svelte/icons/layout-dashboard'; // st-icon-dashboard
	import IconVideo from '@tabler/icons-svelte/icons/video'; // st-icon-daily-videos
	import IconBook from '@tabler/icons-svelte/icons/book'; // st-icon-learning-center
	import IconArchive from '@tabler/icons-svelte/icons/archive'; // st-icon-chatroom-archive
	import IconUsers from '@tabler/icons-svelte/icons/users'; // st-icon-forum (Meet the Traders, Trader Store)
	import IconBuildingStore from '@tabler/icons-svelte/icons/building-store'; // st-icon-training-room (Simpler Showcase)
	import IconChevronDown from '@tabler/icons-svelte/icons/chevron-down';

	// ═══════════════════════════════════════════════════════════════════════════
	// TYPES
	// ═══════════════════════════════════════════════════════════════════════════

	interface SecondaryNavItem {
		href: string;
		label: string;
		icon?: typeof IconLayoutDashboard;
		noIcon?: boolean;
		cursorDefault?: boolean;
		children?: SecondaryNavItem[];
		wooClass?: string; // WooCommerce class for account items
	}

	interface Props {
		section: 'account' | 'membership';
		membershipSlug?: string;
		membershipName?: string;
		memberships?: UserMembership[];
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// PROPS & STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let {
		section,
		membershipSlug = '',
		membershipName = '',
		memberships = []
	}: Props = $props();

	// Track which submenus are expanded
	let expandedMenus = $state<Set<string>>(new Set());

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	const currentPath = $derived($page.url.pathname);

	// Account navigation items (WordPress WooCommerce EXACT - no icons, just text)
	const accountNavItems: SecondaryNavItem[] = [
		{
			href: '/dashboard/orders',
			label: 'My Orders',
			noIcon: true,
			wooClass: 'woocommerce-MyAccount-navigation-link--orders'
		},
		{
			href: '/dashboard/account/subscriptions',
			label: 'My Subscriptions',
			noIcon: true,
			wooClass: 'woocommerce-MyAccount-navigation-link--subscriptions'
		},
		{
			href: '/dashboard/account/coupons',
			label: 'Coupons',
			noIcon: true,
			wooClass: 'woocommerce-MyAccount-navigation-link--wc-smart-coupons'
		},
		{
			href: '/dashboard/account/edit-address',
			label: 'Billing Address',
			noIcon: true,
			wooClass: 'woocommerce-MyAccount-navigation-link--edit-address'
		},
		{
			href: '/dashboard/account/payment-methods',
			label: 'Payment Methods',
			noIcon: true,
			wooClass: 'woocommerce-MyAccount-navigation-link--payment-methods'
		},
		{
			href: '/dashboard/account',
			label: 'Account Details',
			noIcon: true,
			wooClass: 'woocommerce-MyAccount-navigation-link--edit-account'
		},
		{
			href: '/logout',
			label: 'Log out',
			noIcon: true,
			wooClass: 'woocommerce-MyAccount-navigation-link--customer-logout'
		}
	];

	// Generate membership navigation items dynamically (WordPress EXACT structure)
	const membershipNavItems = $derived.by(() => {
		if (!membershipSlug) return [];

		const basePath = `/dashboard/${membershipSlug}`;
		const displayName = membershipName || 'Membership';

		// WordPress EXACT items for Mastering the Trade style membership
		const items: SecondaryNavItem[] = [
			{
				href: basePath,
				label: `${displayName} Dashboard`,
				icon: IconLayoutDashboard // st-icon-dashboard
			},
			{
				href: `${basePath}/daily-videos`,
				label: 'Premium Daily Videos',
				icon: IconVideo // st-icon-daily-videos
			},
			{
				href: `${basePath}/learning-center`,
				label: 'Learning Center',
				icon: IconBook // st-icon-learning-center
			},
			{
				href: `${basePath}/trading-room-archive`,
				label: 'Trading Room Archives',
				icon: IconArchive // st-icon-chatroom-archive
			},
			{
				href: '#',
				label: 'Meet the Traders',
				icon: IconUsers, // st-icon-forum
				cursorDefault: true,
				children: [
					{ href: `${basePath}/john-carter`, label: 'John Carter', noIcon: true },
					{ href: `${basePath}/henry-gambell`, label: 'Henry Gambell', noIcon: true },
					{ href: `${basePath}/taylor-horton`, label: 'Taylor Horton', noIcon: true },
					{ href: `${basePath}/bruce-marshall`, label: 'Bruce Marshall', noIcon: true },
					{ href: `${basePath}/danielle-shay`, label: 'Danielle Shay', noIcon: true },
					{ href: `${basePath}/allison-ostrander`, label: 'Allison Ostrander', noIcon: true },
					{ href: `${basePath}/sam-shames`, label: 'Sam Shames', noIcon: true },
					{ href: `${basePath}/kody-ashmore`, label: 'Kody Ashmore', noIcon: true },
					{ href: `${basePath}/raghee-horner`, label: 'Raghee Horner', noIcon: true }
				]
			},
			{
				href: '#',
				label: 'Trader Store',
				icon: IconUsers, // st-icon-forum (WordPress uses same icon as Meet the Traders)
				cursorDefault: true,
				children: [
					// WordPress EXACT URLs: /trader-name/trader-name-trader-store/
					{ href: `${basePath}/john-carter/john-carter-trader-store`, label: 'John Carter', noIcon: true },
					{ href: `${basePath}/henry-gambell/trader-store`, label: 'Henry Gambell', noIcon: true },
					{ href: `${basePath}/taylor-horton/taylor-horton-trader-store`, label: 'Taylor Horton', noIcon: true },
					{ href: `${basePath}/bruce-marshall/bruce-marshall-trader-store`, label: 'Bruce Marshall', noIcon: true },
					{ href: `${basePath}/danielle-shay/danielle-shay-trader-store`, label: 'Danielle Shay', noIcon: true },
					{ href: `${basePath}/allison-ostrander/allison-ostrander-trader-store`, label: 'Allison Ostrander', noIcon: true },
					{ href: `${basePath}/sam-shames/sam-shames-trader-store`, label: 'Sam Shames', noIcon: true },
					{ href: `${basePath}/kody-ashmore/kody-ashmore-trader-store`, label: 'Kody Ashmore', noIcon: true },
					{ href: `${basePath}/raghee-horner/raghee-horner-trader-store`, label: 'Raghee Horner', noIcon: true }
				]
			}
		];

		// Simpler Trading EXACT: Add Simpler Showcase link for mastering-the-trade membership
		if (membershipSlug === 'mastering-the-trade') {
			items.push({
				href: '/dashboard/simpler-showcase',
				label: 'Simpler Showcase',
				icon: IconBuildingStore // st-icon-training-room
			});
		}

		return items;
	});

	// Get the current nav items based on section
	const navItems = $derived(section === 'account' ? accountNavItems : membershipNavItems);

	// ═══════════════════════════════════════════════════════════════════════════
	// HELPER FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	function isActive(href: string): boolean {
		if (href === '#') return false;
		// Exact match for dashboard root
		if (href === `/dashboard/${membershipSlug}`) {
			return currentPath === href;
		}
		// Prefix match for sub-pages
		return currentPath === href || currentPath.startsWith(href + '/');
	}

	function toggleSubmenu(label: string, event: MouseEvent): void {
		event.preventDefault();
		if (expandedMenus.has(label)) {
			expandedMenus.delete(label);
		} else {
			expandedMenus.add(label);
		}
		expandedMenus = new Set(expandedMenus);
	}

	function hasActiveChild(item: SecondaryNavItem): boolean {
		if (!item.children) return false;
		return item.children.some(child => isActive(child.href));
	}
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     TEMPLATE - WordPress EXACT Structure
     ═══════════════════════════════════════════════════════════════════════════ -->

<nav class="dashboard__nav-secondary" aria-label="Secondary navigation">
	<ul>
		{#each navItems as item (item.href + item.label)}
			<li
				class={item.wooClass || ''}
				class:is-active={isActive(item.href)}
				class:has-submenu={item.children && item.children.length > 0}
				class:is-expanded={expandedMenus.has(item.label) || hasActiveChild(item)}
			>
				{#if item.children && item.children.length > 0}
					<!-- Parent with submenu - WordPress uses onclick="return false;" -->
					<a
						href={item.href}
						class:no-icon={item.noIcon}
						style={item.cursorDefault ? 'cursor: default;' : ''}
						onclick={(e: MouseEvent) => toggleSubmenu(item.label, e)}
						aria-expanded={expandedMenus.has(item.label) || hasActiveChild(item)}
					>
						{#if item.icon && !item.noIcon}
							{@const Icon = item.icon}
							<span class="dashboard__nav-item-icon">
								<Icon size={18} stroke={1.5} />
							</span>
						{/if}
						<span class="dashboard__nav-item-text">{item.label}</span>
					</a>

					<!-- Submenu items (WordPress: .dashboard__nav-submenu) -->
					{#if expandedMenus.has(item.label) || hasActiveChild(item)}
						<ul class="dashboard__nav-submenu">
							{#each item.children as child (child.href)}
								<li class:is-active={isActive(child.href)}>
									<a href={child.href}>
										{child.label}
									</a>
								</li>
							{/each}
						</ul>
					{/if}
				{:else}
					<!-- Regular link -->
					<a
						href={item.href}
						class:no-icon={item.noIcon}
						aria-current={isActive(item.href) ? 'page' : undefined}
					>
						{#if item.icon && !item.noIcon}
							{@const Icon = item.icon}
							<span class="dashboard__nav-item-icon">
								<Icon size={18} stroke={1.5} />
							</span>
						{/if}
						<span class="dashboard__nav-item-text">{item.label}</span>
					</a>
				{/if}
			</li>
		{/each}
			</ul>
		</nav>

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES - WordPress EXACT CSS
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   CSS CUSTOM PROPERTIES (WordPress EXACT values)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-secondary {
		--secondary-bg: #1a3a4f;
		--secondary-text: hsla(0, 0%, 100%, 0.5);
		--secondary-text-hover: #fff;
		--secondary-icon: #8796A0;
		--secondary-accent: #0984ae;
		--secondary-hover-bg: rgba(255, 255, 255, 0.05);
		--secondary-active-bg: rgba(9, 132, 174, 0.2);
		--secondary-submenu-bg: rgba(0, 0, 0, 0.15);
		--font-family: "Open Sans", sans-serif;
		--font-size: 14px;
		--font-size-submenu: 13px;
		--font-weight: 400;
		--transition: all 0.15s ease-in-out;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SECONDARY NAVIGATION (WordPress EXACT: .dashboard__nav-secondary)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-secondary {
		width: 220px;
		background-color: var(--secondary-bg);
		padding: 20px 0;
		font-family: var(--font-family);
		font-size: var(--font-size);
		font-weight: var(--font-weight);
		overflow-y: auto;
		overflow-x: hidden;
		-webkit-overflow-scrolling: touch;
		scrollbar-width: thin;
		scrollbar-color: rgba(255,255,255,0.2) transparent;
		/* WordPress EXACT: Visible on desktop */
		display: block;
		opacity: 1;
		visibility: visible;
		flex-shrink: 0;
		position: sticky;
		top: 0;
		max-height: 100vh;
		/* Sidebar sizes to content, ends where footer starts */
	}

	.dashboard__nav-secondary::-webkit-scrollbar {
		width: 6px;
	}

	.dashboard__nav-secondary::-webkit-scrollbar-track {
		background: transparent;
	}

	.dashboard__nav-secondary::-webkit-scrollbar-thumb {
		background: rgba(255,255,255,0.2);
		border-radius: 3px;
	}

	.dashboard__nav-secondary ul {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.dashboard__nav-secondary li {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   LINK STYLES (WordPress EXACT)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-secondary > ul > li > a {
		display: flex;
		align-items: center;
		position: relative;
		height: 50px;
		padding: 0 20px 0 50px;
		color: var(--secondary-text);
		text-decoration: none;
		font-weight: var(--font-weight);
		font-size: var(--font-size);
		line-height: 1.4;
		transition: var(--transition);
		background: none;
		border: none;
		width: 100%;
		text-align: left;
	}

	/* No icon variant (WooCommerce account links - WordPress uses class="no-icon") */
	.dashboard__nav-secondary > ul > li > a.no-icon {
		padding-left: 20px;
	}

	/* Hover state */
	.dashboard__nav-secondary > ul > li > a:hover {
		color: var(--secondary-text-hover);
		background: var(--secondary-hover-bg);
	}

	/* Active state */
	.dashboard__nav-secondary > ul > li.is-active > a {
		color: var(--secondary-text-hover);
		background: var(--secondary-active-bg);
	}

	/* Active indicator bar (Simpler Trading EXACT - LEFT border) */
	.dashboard__nav-secondary > ul > li.is-active > a::before {
		content: '';
		position: absolute;
		top: 0;
		left: 0;
		bottom: 0;
		width: 4px;
		background-color: var(--secondary-accent);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ICONS (WordPress: .dashboard__nav-item-icon with st-icon-* classes)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-item-icon {
		position: absolute;
		left: 16px;
		top: 50%;
		transform: translateY(-50%);
		width: 20px;
		height: 20px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--secondary-icon);
		transition: color 0.15s ease-in-out;
	}

	.dashboard__nav-item-icon :global(svg) {
		width: 18px;
		height: 18px;
		stroke: currentColor;
	}

	/* Icon hover/active states */
	.dashboard__nav-secondary > ul > li > a:hover .dashboard__nav-item-icon,
	.dashboard__nav-secondary > ul > li.is-active .dashboard__nav-item-icon {
		color: var(--secondary-text-hover);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   NAV ITEM TEXT (WordPress: .dashboard__nav-item-text)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-item-text {
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SUBMENU (WordPress EXACT: .dashboard__nav-submenu)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-submenu {
		background: var(--secondary-submenu-bg);
		padding: 8px 0;
		z-index: 110;
	}

	.dashboard__nav-submenu li {
		margin: 0;
		padding: 0;
	}

	.dashboard__nav-submenu a {
		display: block;
		padding: 8px 20px 8px 36px;
		color: var(--secondary-text);
		text-decoration: none;
		font-size: var(--font-size-submenu);
		font-weight: var(--font-weight);
		line-height: 1.4;
		transition: var(--transition);
	}

	.dashboard__nav-submenu a:hover {
		color: var(--secondary-text-hover);
		background: var(--secondary-hover-bg);
	}

	.dashboard__nav-submenu li.is-active > a {
		color: var(--secondary-text-hover);
		background: rgba(9, 132, 174, 0.15);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE (WordPress EXACT breakpoints)
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media screen and (min-width: 1280px) {
		.dashboard__nav-secondary {
			/* WordPress EXACT: Desktop - visible, positioned in flex layout */
			display: block;
			position: relative;
			opacity: 1;
			visibility: visible;
			width: 220px;
			flex-shrink: 0;
			/* Sidebar sizes to content, ends where footer starts */
		}
	}

	@media screen and (max-width: 1279px) {
		.dashboard__nav-secondary {
			position: fixed;
			left: 60px; /* After collapsed primary nav (60px) */
			top: 0;
			bottom: 50px; /* Above mobile toggle */
			z-index: 100010;
			opacity: 0;
			visibility: hidden;
			transition: all 0.3s ease-in-out;
		}
	}

	/* WordPress EXACT: Show secondary nav when mobile menu is open (mobile only) */
	/* Using :global() to match parent component's class */
	@media screen and (max-width: 1279px) {
		:global(.dashboard--menu-open) .dashboard__nav-secondary {
			opacity: 1;
			visibility: visible;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ACCESSIBILITY
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-secondary a:focus-visible {
		outline: 2px solid var(--secondary-accent);
		outline-offset: -2px;
	}

	@media (prefers-reduced-motion: reduce) {
		.dashboard__nav-secondary,
		.dashboard__nav-secondary a,
		.dashboard__nav-item-icon {
			transition: none;
		}
	}
</style>
