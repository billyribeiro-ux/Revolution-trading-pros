<script lang="ts">
	/**
	 * Secondary Navigation Component - WordPress Exact Implementation
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * The lighter-colored submenu panel that appears when:
	 * - User clicks on their profile/avatar (Account section)
	 * - User navigates to a membership dashboard (e.g., Mastering the Trade)
	 *
	 * WordPress structure: .dashboard__nav-secondary
	 *
	 * @version 1.0.0 (December 2025)
	 */

	import { page } from '$app/stores';
	import type { UserMembership } from '$lib/api/user-memberships';

	// Tabler Icons
	import IconDashboard from '@tabler/icons-svelte/icons/layout-dashboard';
	import IconVideo from '@tabler/icons-svelte/icons/video';
	import IconBook from '@tabler/icons-svelte/icons/book';
	import IconArchive from '@tabler/icons-svelte/icons/archive';
	import IconUsers from '@tabler/icons-svelte/icons/users';
	import IconChevronDown from '@tabler/icons-svelte/icons/chevron-down';

	// ═══════════════════════════════════════════════════════════════════════════
	// TYPES
	// ═══════════════════════════════════════════════════════════════════════════

	interface SecondaryNavItem {
		href: string;
		label: string;
		icon?: typeof IconDashboard;
		noIcon?: boolean;
		children?: SecondaryNavItem[];
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

	// Account navigation items (WordPress WooCommerce exact)
	const accountNavItems: SecondaryNavItem[] = [
		{ href: '/dashboard/orders', label: 'My Orders', noIcon: true },
		{ href: '/dashboard/subscriptions', label: 'My Subscriptions', noIcon: true },
		{ href: '/dashboard/coupons', label: 'Coupons', noIcon: true },
		{ href: '/dashboard/addresses', label: 'Billing Address', noIcon: true },
		{ href: '/dashboard/payment-methods', label: 'Payment Methods', noIcon: true },
		{ href: '/dashboard/account', label: 'Account Details', noIcon: true },
		{ href: '/logout', label: 'Log out', noIcon: true }
	];

	// Generate membership navigation items dynamically
	const membershipNavItems = $derived.by(() => {
		if (!membershipSlug) return [];

		const basePath = `/dashboard/${membershipSlug}`;

		// Default items for most memberships
		const items: SecondaryNavItem[] = [
			{
				href: basePath,
				label: `${membershipName || 'Membership'} Dashboard`,
				icon: IconDashboard
			},
			{
				href: `${basePath}/daily-videos`,
				label: 'Premium Daily Videos',
				icon: IconVideo
			},
			{
				href: `${basePath}/learning-center`,
				label: 'Learning Center',
				icon: IconBook
			},
			{
				href: `${basePath}/archive`,
				label: 'Trading Room Archives',
				icon: IconArchive
			},
			{
				href: '#',
				label: 'Meet the Traders',
				icon: IconUsers,
				children: [
					{ href: `${basePath}/traders/trader-1`, label: 'Trader 1', noIcon: true },
					{ href: `${basePath}/traders/trader-2`, label: 'Trader 2', noIcon: true }
				]
			}
		];

		return items;
	});

	// Get the current nav items based on section
	const navItems = $derived(section === 'account' ? accountNavItems : membershipNavItems);

	// ═══════════════════════════════════════════════════════════════════════════
	// HELPER FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	function isActive(href: string): boolean {
		if (href === '#') return false;
		if (href === `/dashboard/${membershipSlug}`) {
			return currentPath === href;
		}
		return currentPath === href || currentPath.startsWith(href + '/');
	}

	function toggleSubmenu(label: string): void {
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
     TEMPLATE - WordPress Exact Structure
     ═══════════════════════════════════════════════════════════════════════════ -->

<nav class="dashboard__nav-secondary" aria-label="Secondary navigation">
	<ul>
		{#each navItems as item (item.href + item.label)}
			<li
				class:is-active={isActive(item.href)}
				class:has-submenu={item.children && item.children.length > 0}
				class:is-expanded={expandedMenus.has(item.label) || hasActiveChild(item)}
			>
				{#if item.children && item.children.length > 0}
					<!-- Parent with submenu -->
					<button
						class="submenu-toggle"
						class:no-icon={item.noIcon}
						onclick={() => toggleSubmenu(item.label)}
						aria-expanded={expandedMenus.has(item.label) || hasActiveChild(item)}
					>
						{#if item.icon && !item.noIcon}
							<span class="dashboard__nav-item-icon">
								<svelte:component this={item.icon} size={20} />
							</span>
						{/if}
						<span class="dashboard__nav-item-text">{item.label}</span>
						<span class="submenu-arrow">
							<IconChevronDown size={16} />
						</span>
					</button>

					<!-- Submenu items -->
					{#if expandedMenus.has(item.label) || hasActiveChild(item)}
						<ul class="dashboard__nav-submenu">
							{#each item.children as child (child.href)}
								<li class:is-active={isActive(child.href)}>
									<a
										href={child.href}
										class:no-icon={child.noIcon}
									>
										<span class="dashboard__nav-item-text">{child.label}</span>
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
							<span class="dashboard__nav-item-icon">
								<svelte:component this={item.icon} size={20} />
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
     STYLES - WordPress Exact CSS
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   SECONDARY NAVIGATION (WordPress EXACT: .dashboard__nav-secondary)
	   Lighter background color than primary nav
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-secondary {
		width: 220px;
		min-height: 100vh;
		background-color: #1a3a4f; /* Lighter than primary (#0f2d41) */
		padding: 20px 0;
		font-size: 14px;
		overflow-y: auto;
		overflow-x: hidden;
		-webkit-overflow-scrolling: touch;
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
	   LINK STYLES
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-secondary a,
	.dashboard__nav-secondary .submenu-toggle {
		display: flex;
		align-items: center;
		position: relative;
		min-height: 44px;
		padding: 10px 20px 10px 50px;
		color: hsla(0, 0%, 100%, 0.6);
		text-decoration: none;
		font-weight: 400;
		font-size: 14px;
		line-height: 1.4;
		transition: all 0.15s ease-in-out;
		background: none;
		border: none;
		width: 100%;
		text-align: left;
		cursor: pointer;
	}

	/* No icon variant (WooCommerce account links) */
	.dashboard__nav-secondary a.no-icon,
	.dashboard__nav-secondary .submenu-toggle.no-icon {
		padding-left: 20px;
	}

	.dashboard__nav-secondary a:hover,
	.dashboard__nav-secondary .submenu-toggle:hover {
		color: #fff;
		background: rgba(255, 255, 255, 0.05);
	}

	/* Active state */
	.dashboard__nav-secondary li.is-active > a,
	.dashboard__nav-secondary li.is-active > .submenu-toggle {
		color: #fff;
		background: rgba(9, 132, 174, 0.2);
	}

	/* Active indicator bar */
	.dashboard__nav-secondary li.is-active > a::after {
		content: '';
		position: absolute;
		top: 0;
		right: 0;
		bottom: 0;
		width: 4px;
		background-color: #0984ae;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ICONS
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
		color: #8796A0;
		transition: color 0.15s ease-in-out;
	}

	.dashboard__nav-item-icon :global(svg) {
		width: 20px;
		height: 20px;
		stroke: currentColor;
		stroke-width: 1.5;
	}

	.dashboard__nav-secondary a:hover .dashboard__nav-item-icon,
	.dashboard__nav-secondary li.is-active .dashboard__nav-item-icon {
		color: #fff;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SUBMENU (WordPress EXACT: .dashboard__nav-submenu)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.submenu-arrow {
		position: absolute;
		right: 16px;
		top: 50%;
		transform: translateY(-50%);
		color: hsla(0, 0%, 100%, 0.4);
		transition: transform 0.2s ease;
		display: flex;
		align-items: center;
	}

	.has-submenu.is-expanded .submenu-arrow {
		transform: translateY(-50%) rotate(180deg);
	}

	.dashboard__nav-submenu {
		background: rgba(0, 0, 0, 0.15);
		padding: 8px 0;
	}

	.dashboard__nav-submenu a {
		padding-left: 36px;
		min-height: 38px;
		font-size: 13px;
	}

	.dashboard__nav-submenu li.is-active > a {
		color: #fff;
		background: rgba(9, 132, 174, 0.15);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media screen and (max-width: 1279px) {
		.dashboard__nav-secondary {
			position: fixed;
			left: 60px; /* After collapsed primary nav */
			top: 0;
			bottom: 50px; /* Above mobile toggle */
			z-index: 100010;
			opacity: 0;
			visibility: hidden;
			transition: all 0.3s ease-in-out;
		}

		:global(.dashboard--menu-open) .dashboard__nav-secondary {
			opacity: 1;
			visibility: visible;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ACCESSIBILITY
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-secondary a:focus-visible,
	.dashboard__nav-secondary .submenu-toggle:focus-visible {
		outline: 2px solid #0984ae;
		outline-offset: -2px;
	}

	@media (prefers-reduced-motion: reduce) {
		.dashboard__nav-secondary a,
		.dashboard__nav-secondary .submenu-toggle,
		.submenu-arrow {
			transition: none;
		}
	}
</style>
