<script lang="ts">
	/**
	 * Dashboard Sidebar Component - WordPress Exact Implementation
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Exact match of WordPress Revolution Trading sidebar structure:
	 * - .dashboard__nav-primary (primary navigation)
	 * - .dashboard__profile-nav-item (profile section)
	 * - .dash_main_links (main navigation links)
	 * - .dashboard__nav-category (category headers)
	 * - .dashboard__nav-item-icon / .dashboard__nav-item-text
	 *
	 * Routes to local services instead of WordPress endpoints.
	 *
	 * @version 5.0.0 (Tabler Icons / December 2025)
	 */

	import { page } from '$app/stores';
	import { browser } from '$app/environment';
	import type { UserMembership } from '$lib/api/user-memberships';

	// Tabler Icons - Using proper SVG icons
	import IconHome from '@tabler/icons-svelte/icons/home';
	import IconVideo from '@tabler/icons-svelte/icons/video';
	import IconChartCandle from '@tabler/icons-svelte/icons/chart-candle';
	import IconChartLine from '@tabler/icons-svelte/icons/chart-line';
	import IconSettings from '@tabler/icons-svelte/icons/settings';
	import IconLogout from '@tabler/icons-svelte/icons/logout';
	import IconHelp from '@tabler/icons-svelte/icons/help-circle';
	import IconUser from '@tabler/icons-svelte/icons/user';
	import IconReceipt from '@tabler/icons-svelte/icons/receipt';
	import IconCreditCard from '@tabler/icons-svelte/icons/credit-card';
	import IconDiscount from '@tabler/icons-svelte/icons/discount';
	import IconMapPin from '@tabler/icons-svelte/icons/map-pin';
	import IconCalendar from '@tabler/icons-svelte/icons/calendar';
	import IconTrendingUp from '@tabler/icons-svelte/icons/trending-up';
	import IconFlame from '@tabler/icons-svelte/icons/flame';
	import IconTarget from '@tabler/icons-svelte/icons/target';
	import IconBolt from '@tabler/icons-svelte/icons/bolt';
	import IconBook from '@tabler/icons-svelte/icons/book';
	import IconListCheck from '@tabler/icons-svelte/icons/list-check';
	import IconHeadset from '@tabler/icons-svelte/icons/headset';
	import IconBell from '@tabler/icons-svelte/icons/bell';
	import IconUsers from '@tabler/icons-svelte/icons/users';

	// ═══════════════════════════════════════════════════════════════════════════
	// TYPES
	// ═══════════════════════════════════════════════════════════════════════════

	interface SubscriptionInfo {
		nextChargeDate?: string;
		nextChargeAmount?: number;
		paymentMethod?: string;
	}

	interface Props {
		memberships?: UserMembership[];
		isMobileOpen?: boolean;
		onCloseMobile?: () => void;
		userName?: string;
		userEmail?: string;
		userAvatar?: string;
		subscriptionInfo?: SubscriptionInfo;
		isCollapsed?: boolean;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// PROPS & STATE (Svelte 5 Runes)
	// ═══════════════════════════════════════════════════════════════════════════

	let {
		memberships = [],
		isMobileOpen = $bindable(false),
		onCloseMobile,
		userName = 'My Account',
		userEmail = '',
		userAvatar = '',
		subscriptionInfo,
		isCollapsed = false
	}: Props = $props();

	// Generate gravatar URL from email if no avatar provided
	function getGravatarUrl(email: string): string {
		if (!email) return '';
		// Simple hash for gravatar - in production use proper MD5
		const hash = email.toLowerCase().trim();
		return `https://secure.gravatar.com/avatar/${hash}?s=32&d=mm&r=g`;
	}

	const profilePhotoUrl = $derived(userAvatar || (userEmail ? getGravatarUrl(userEmail) : ''));

	// ═══════════════════════════════════════════════════════════════════════════
	// HELPER FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	function formatDate(dateStr?: string): string {
		if (!dateStr) return '';
		try {
			const date = new Date(dateStr);
			return date.toLocaleDateString('en-US', {
				month: 'short',
				day: 'numeric',
				year: 'numeric'
			});
		} catch {
			return '';
		}
	}

	function formatCurrency(amount?: number): string {
		if (amount === undefined || amount === null) return '';
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD'
		}).format(amount);
	}

	const hasSubscriptionInfo = $derived(
		subscriptionInfo?.nextChargeDate || subscriptionInfo?.nextChargeAmount
	);

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	// Group memberships by type
	const tradingRooms = $derived(memberships.filter(m => m.type === 'trading-room'));
	const alertServices = $derived(memberships.filter(m => m.type === 'alert-service'));
	const courses = $derived(memberships.filter(m => m.type === 'course'));
	const indicators = $derived(memberships.filter(m => m.type === 'indicator'));

	// Check if any memberships exist
	const hasTradingRooms = $derived(tradingRooms.length > 0);
	const hasAlertServices = $derived(alertServices.length > 0);
	const hasCourses = $derived(courses.length > 0);
	const hasIndicators = $derived(indicators.length > 0);

	// Current path for active state
	const currentPath = $derived($page.url.pathname);

	// Check if we're on any account page (for highlighting profile section)
	const isAccountSection = $derived(
		currentPath.startsWith('/dashboard/account') ||
		currentPath.startsWith('/dashboard/orders') ||
		currentPath.startsWith('/dashboard/subscriptions') ||
		currentPath.startsWith('/dashboard/coupons') ||
		currentPath.startsWith('/dashboard/addresses') ||
		currentPath.startsWith('/dashboard/payment-methods')
	);

	// ═══════════════════════════════════════════════════════════════════════════
	// HELPER FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	function isActive(href: string): boolean {
		if (href === '/dashboard') {
			return currentPath === '/dashboard';
		}
		return currentPath === href || currentPath.startsWith(href + '/');
	}

	// Close mobile menu on route change
	$effect(() => {
		if (browser && isMobileOpen && currentPath) {
			onCloseMobile?.();
		}
	});

	// Handle escape key
	$effect(() => {
		if (!browser || !isMobileOpen) return;

		function handleEscape(event: KeyboardEvent): void {
			if (event.key === 'Escape') {
				onCloseMobile?.();
			}
		}

		document.addEventListener('keydown', handleEscape);
		return () => document.removeEventListener('keydown', handleEscape);
	});
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     TEMPLATE - WordPress Exact Structure
     ═══════════════════════════════════════════════════════════════════════════ -->

<!-- WordPress: .dashboard__nav-primary / .dashboard__nav-primary.is-collapsed -->
<nav
	class="dashboard__nav-primary"
	class:is-collapsed={isCollapsed}
	aria-label="Dashboard navigation"
>

	<!-- ═══════════════════════════════════════════════════════════════════════
	     PROFILE SECTION (WordPress: .dashboard__profile-nav-item)
	     Simpler Trading EXACT structure with photo and name
	     ═══════════════════════════════════════════════════════════════════════ -->
	<a
		href="/dashboard/account"
		class="dashboard__profile-nav-item"
		class:is-account-active={isAccountSection}
		aria-current={isAccountSection ? 'page' : undefined}
	>
		{#if profilePhotoUrl}
			<span class="dashboard__profile-photo" style="background-image: url({profilePhotoUrl});"></span>
		{/if}
		<span class="dashboard__profile-name">{userName}</span>
	</a>

	<!-- ═══════════════════════════════════════════════════════════════════════
	     MEMBER SUBSCRIPTION INFO (Simpler Trading style) - Hidden when collapsed
	     ═══════════════════════════════════════════════════════════════════════ -->
	{#if hasSubscriptionInfo && !isCollapsed}
			<div class="subscription-info-box">
				<p class="subscription-info-title">Member Subscription Info</p>
				{#if subscriptionInfo?.nextChargeDate && subscriptionInfo?.nextChargeAmount}
					<p class="subscription-info-text">
						Next charge: {formatCurrency(subscriptionInfo.nextChargeAmount)} on {formatDate(subscriptionInfo.nextChargeDate)}
					</p>
				{:else if subscriptionInfo?.nextChargeDate}
					<p class="subscription-info-text">
						Next charge: {formatDate(subscriptionInfo.nextChargeDate)}
					</p>
				{:else if subscriptionInfo?.nextChargeAmount}
					<p class="subscription-info-text">
						Amount: {formatCurrency(subscriptionInfo.nextChargeAmount)}
					</p>
				{/if}
				{#if subscriptionInfo?.paymentMethod}
					<p class="subscription-info-payment">via {subscriptionInfo.paymentMethod}</p>
				{/if}
			</div>
		{/if}

		<!-- ═══════════════════════════════════════════════════════════════════════
		     MAIN LINKS (WordPress: .dash_main_links)
		     ═══════════════════════════════════════════════════════════════════════ -->
		<ul class="dash_main_links">
			<li class:is-active={isActive('/dashboard')}>
				<a href="/dashboard">
					<span class="dashboard__nav-item-icon"><IconHome size={24} /></span>
					<span class="dashboard__nav-item-text">Member Dashboard</span>
				</a>
			</li>
			<li class:is-active={isActive('/dashboard/courses')}>
				<a href="/dashboard/courses">
					<span class="dashboard__nav-item-icon"><IconVideo size={24} /></span>
					<span class="dashboard__nav-item-text" style="font-weight:bold;color:white;">My Classes</span>
				</a>
			</li>
			<li class:is-active={isActive('/dashboard/indicators')}>
				<a href="/dashboard/indicators">
					<span class="dashboard__nav-item-icon"><IconChartCandle size={24} /></span>
					<span class="dashboard__nav-item-text" style="font-weight:bold;color:white;">My Indicators</span>
				</a>
			</li>
		</ul>

		<!-- ═══════════════════════════════════════════════════════════════════════
		     MEMBERSHIPS SECTION (WordPress: .dashboard__nav-category)
		     ═══════════════════════════════════════════════════════════════════════ -->
		{#if hasTradingRooms || hasAlertServices}
			<ul>
				<li><p class="dashboard__nav-category">memberships</p></li>
			</ul>
			<ul class="dash_main_links">
				{#each tradingRooms as room (room.id)}
					<li
						class="{room.slug}-mp"
						class:is-active={isActive(`/dashboard/${room.slug}`)}
					>
						<a href="/dashboard/{room.slug}">
							<span class="dashboard__nav-item-icon">
								{#if room.icon}
									<img src={room.icon} alt="" class="service-icon-img" loading="lazy" />
								{:else}
									<IconUsers size={24} />
								{/if}
							</span>
							<span class="dashboard__nav-item-text">{room.name}</span>
						</a>
					</li>
				{/each}
				{#each alertServices as alert (alert.id)}
					<li
						class="{alert.slug}-mp"
						class:is-active={isActive(`/dashboard/${alert.slug}`)}
					>
						<a href="/dashboard/{alert.slug}">
							<span class="dashboard__nav-item-icon">
								{#if alert.icon}
									<img src={alert.icon} alt="" class="service-icon-img" loading="lazy" />
								{:else}
									<IconBell size={24} />
								{/if}
							</span>
							<span class="dashboard__nav-item-text">{alert.name}</span>
						</a>
					</li>
				{/each}
			</ul>
		{/if}

		<!-- ═══════════════════════════════════════════════════════════════════════
		     MASTERY SECTION (WordPress EXACT - Only show when courses exist)
		     ═══════════════════════════════════════════════════════════════════════ -->
		{#if hasCourses}
			<ul>
				<li><p class="dashboard__nav-category">mastery</p></li>
			</ul>
			<ul class="dash_main_links">
				{#each courses as course (course.id)}
					<li
						class="{course.slug}-mp"
						class:is-active={isActive(`/dashboard/${course.slug}`)}
					>
						<a href="/dashboard/{course.slug}">
							<span class="dashboard__nav-item-icon">
								{#if course.icon}
									<img src={course.icon} alt="" class="service-icon-img" loading="lazy" />
								{:else}
									<IconBook size={24} />
								{/if}
							</span>
							<span class="dashboard__nav-item-text">{course.name}</span>
						</a>
					</li>
				{/each}
			</ul>
		{/if}

		<!-- ═══════════════════════════════════════════════════════════════════════
		     PREMIUM REPORTS SECTION (WordPress EXACT - Category always shown, empty if no reports)
		     ═══════════════════════════════════════════════════════════════════════ -->
		<ul>
			<li><p class="dashboard__nav-category">premium reports</p></li>
		</ul>
		<ul class="dash_main_links">
			<!-- Premium reports will be dynamically added here when user has subscriptions -->
		</ul>

		<!-- ═══════════════════════════════════════════════════════════════════════
		     TOOLS SECTION (WordPress EXACT - Always shown)
		     ═══════════════════════════════════════════════════════════════════════ -->
		<ul>
			<li><p class="dashboard__nav-category">tools</p></li>
		</ul>
		<ul class="dash_main_links">
			<li class:is-active={isActive('/dashboard/ww')}>
				<a href="/dashboard/ww">
					<span class="dashboard__nav-item-icon"><IconListCheck size={24} /></span>
					<span class="dashboard__nav-item-text">Weekly Watchlist</span>
				</a>
			</li>
			<li class:is-active={isActive('/dashboard/support')}>
				<a href="https://intercom.help/simpler-trading/en/" target="_blank">
					<span class="dashboard__nav-item-icon"><IconHeadset size={24} /></span>
					<span class="dashboard__nav-item-text">Support</span>
				</a>
			</li>
		</ul>

		<!-- ═══════════════════════════════════════════════════════════════════════
		     ACCOUNT SECTION (Simpler Trading EXACT - just My Account)
		     ═══════════════════════════════════════════════════════════════════════ -->
		<ul>
			<li><p class="dashboard__nav-category">account</p></li>
		</ul>
		<ul class="dash_main_links">
			<li class:is-active={isActive('/dashboard/account')}>
				<a href="/dashboard/account">
					<span class="dashboard__nav-item-icon"><IconSettings size={24} /></span>
					<span class="dashboard__nav-item-text">My Account</span>
				</a>
			</li>
		</ul>
</nav>

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES - WordPress Exact CSS
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   CSS CUSTOM PROPERTIES (WordPress EXACT)
	   ═══════════════════════════════════════════════════════════════════════════ */

	:root {
		--sidebar-bg-primary: #0f2d41;
		--sidebar-toggle-bg: #0d2532;
		--sidebar-bg-hover: rgba(255, 255, 255, 0.05);
		--sidebar-text: hsla(0, 0%, 100%, 0.5);
		--sidebar-text-active: #fff;
		--sidebar-accent: #0984ae;
		--sidebar-icon-color: #8796A0;
		--sidebar-transition: all 0.15s ease-in-out;
		--sidebar-width: 280px;
		--toggle-height: 50px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   PRIMARY NAVIGATION (WordPress EXACT: .dashboard__nav-primary)
	   On mobile: fixed position, ends where toggle footer starts (bottom: 50px)
	   On desktop: static position, full height
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-primary {
		width: var(--sidebar-width);
		padding-bottom: 30px;
		font-size: 16px;
		background-color: var(--sidebar-bg-primary);
		position: fixed;
		bottom: var(--toggle-height); /* Ends where the full-width footer starts */
		left: 0;
		top: 0;
		opacity: 0;
		visibility: hidden;
		overflow-x: hidden;
		overflow-y: auto;
		z-index: 100010;
		transition: all 0.3s ease-in-out;
		-webkit-overflow-scrolling: touch;
		scrollbar-width: thin;
		scrollbar-color: rgba(255,255,255,0.2) transparent;
	}

	/* WordPress EXACT: Custom scrollbar for webkit browsers */
	.dashboard__nav-primary::-webkit-scrollbar {
		width: 6px;
	}

	.dashboard__nav-primary::-webkit-scrollbar-track {
		background: transparent;
	}

	.dashboard__nav-primary::-webkit-scrollbar-thumb {
		background: rgba(255,255,255,0.2);
		border-radius: 3px;
	}

	.dashboard__nav-primary::-webkit-scrollbar-thumb:hover {
		background: rgba(255,255,255,0.3);
	}

	/* WordPress EXACT: Desktop state (min-width 1280px) - no footer, full height */
	@media screen and (min-width: 1280px) {
		.dashboard__nav-primary {
			display: block;
			position: static;
			opacity: 1;
			visibility: visible;
			bottom: 0; /* Full height on desktop - no footer */
			min-height: 100vh;
		}
	}

	/* WordPress EXACT: Mobile open state */
	:global(.dashboard--menu-open) .dashboard__nav-primary {
		opacity: 1;
		visibility: visible;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   COLLAPSED STATE (WordPress EXACT: .dashboard__nav-primary.is-collapsed)
	   When viewing account or membership pages - shows icons only, narrow width
	   HOVER: Expands to full width and shows text labels
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-primary.is-collapsed {
		width: 60px;
		transition: width 0.2s ease-in-out;
	}

	/* WordPress EXACT: Expand on hover to show names */
	@media screen and (min-width: 1280px) {
		.dashboard__nav-primary.is-collapsed:hover {
			width: var(--sidebar-width);
			box-shadow: 4px 0 20px rgba(0, 0, 0, 0.3);
			z-index: 100015;
		}

		/* Show profile name on hover */
		.dashboard__nav-primary.is-collapsed:hover .dashboard__profile-name {
			display: block;
		}

		/* Restore profile layout on hover */
		.dashboard__nav-primary.is-collapsed:hover .dashboard__profile-nav-item {
			padding-left: 80px;
			padding-right: 20px;
			justify-content: flex-start;
		}

		/* Restore profile photo position on hover */
		.dashboard__nav-primary.is-collapsed:hover .dashboard__profile-photo {
			position: absolute;
			left: 30px;
			top: 50%;
			margin-top: -17px;
		}

		/* Show nav text on hover */
		.dashboard__nav-primary.is-collapsed:hover .dashboard__nav-item-text {
			display: block;
		}

		/* Show category headers on hover */
		.dashboard__nav-primary.is-collapsed:hover .dashboard__nav-category {
			display: block;
		}

		/* Restore link padding on hover */
		.dashboard__nav-primary.is-collapsed:hover .dash_main_links a {
			padding: 0 20px 0 80px;
			justify-content: flex-start;
		}

		/* Restore icon position on hover */
		.dashboard__nav-primary.is-collapsed:hover .dashboard__nav-item-icon {
			position: absolute;
			left: 30px;
			top: 50%;
			margin-top: -12px;
		}
	}

	/* Hide profile name when collapsed (default) */
	.dashboard__nav-primary.is-collapsed .dashboard__profile-name {
		display: none;
	}

	/* Adjust profile section padding when collapsed */
	.dashboard__nav-primary.is-collapsed .dashboard__profile-nav-item {
		padding-left: 0;
		padding-right: 0;
		display: flex;
		justify-content: center;
	}

	/* Center the profile photo when collapsed */
	.dashboard__nav-primary.is-collapsed .dashboard__profile-photo {
		position: relative;
		left: auto;
		top: auto;
		margin-top: 0;
	}

	/* Hide nav text when collapsed */
	.dashboard__nav-primary.is-collapsed .dashboard__nav-item-text {
		display: none;
	}

	/* Hide category headers when collapsed */
	.dashboard__nav-primary.is-collapsed .dashboard__nav-category {
		display: none;
	}

	/* Hide subscription info when collapsed */
	.dashboard__nav-primary.is-collapsed .subscription-info-box {
		display: none;
	}

	/* Adjust link padding to center icons when collapsed */
	.dashboard__nav-primary.is-collapsed .dash_main_links a {
		padding: 0;
		justify-content: center;
		height: 50px;
	}

	/* Center icons when collapsed */
	.dashboard__nav-primary.is-collapsed .dashboard__nav-item-icon {
		position: relative;
		left: auto;
		top: auto;
		margin-top: 0;
	}

	/* WordPress EXACT: Text color in collapsed state */
	.dashboard__nav-primary.is-collapsed .dash_main_links .dashboard__nav-item-text {
		color: var(--sidebar-accent) !important;
	}

	/* Active indicator still shows on right */
	.dashboard__nav-primary.is-collapsed .dash_main_links li.is-active a::after {
		background-color: var(--sidebar-accent);
	}

	.dashboard__nav-primary ul {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.dashboard__nav-primary li {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   PROFILE SECTION (WordPress EXACT: .dashboard__profile-nav-item)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__profile-nav-item {
		display: block;
		position: relative;
		height: auto;
		line-height: 1.4;
		padding-top: 32px;
		padding-bottom: 28px;
		padding-left: 80px;
		padding-right: 20px;
		text-decoration: none;
		transition: var(--sidebar-transition);
	}

	.dashboard__profile-nav-item:hover {
		background: var(--sidebar-bg-hover);
	}

	.dashboard__profile-nav-item:hover .dashboard__profile-name {
		color: var(--sidebar-text-active);
	}

	/* WordPress EXACT: .dashboard__profile-photo */
	.dashboard__profile-photo {
		position: absolute;
		top: 50%;
		left: 30px;
		margin-top: -17px;
		width: 34px;
		height: 34px;
		border: 2px solid #fff;
		border-radius: 50%;
		background-size: 32px;
		background-position: center;
		background-repeat: no-repeat;
		transition: var(--sidebar-transition);
	}

	/* WordPress EXACT: Profile photo hover - border changes to cyan */
	.dashboard__profile-nav-item:hover .dashboard__profile-photo {
		border-color: var(--sidebar-accent);
	}

	/* Lighter blue highlight when on account section */
	.dashboard__profile-nav-item.is-account-active {
		background: rgba(9, 132, 174, 0.15);
	}

	.dashboard__profile-nav-item.is-account-active .dashboard__profile-name {
		color: #5bc0de;
	}

	.dashboard__profile-nav-item.is-account-active .dashboard__profile-photo {
		border-color: var(--sidebar-accent);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SUBSCRIPTION INFO BOX (Simpler Trading style)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.subscription-info-box {
		background: rgba(9, 132, 174, 0.1);
		border-radius: 8px;
		padding: 14px 16px;
		margin-bottom: 20px;
	}

	.subscription-info-title {
		color: var(--sidebar-text-active);
		font-weight: 700;
		font-size: 11px;
		letter-spacing: 0.5px;
		text-transform: uppercase;
		margin: 0 0 8px 0;
		opacity: 0.7;
	}

	.subscription-info-text {
		color: var(--sidebar-text-active);
		font-size: 13px;
		margin: 0 0 4px 0;
		line-height: 1.4;
	}

	.subscription-info-payment {
		color: var(--sidebar-text);
		font-size: 12px;
		margin: 0;
	}

	.dashboard__profile-name {
		display: block;
		color: var(--sidebar-text-active);
		font-weight: 600;
		font-size: 16px;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MAIN LINKS (WordPress EXACT: .dash_main_links)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dash_main_links {
		margin: 0;
		padding: 0;
	}

	.dash_main_links li {
		margin: 0;
		padding: 0;
	}

	/* WordPress EXACT: Link styles - height 50px, flex, padding 0 20px 0 80px */
	.dash_main_links a {
		display: flex;
		align-items: center;
		position: relative;
		height: 50px;
		padding: 0 20px 0 80px;
		color: var(--sidebar-text);
		text-decoration: none;
		font-weight: 300;
		font-size: 14px;
		transition: var(--sidebar-transition);
	}

	/* WordPress EXACT: ::after pseudo-element for active indicator */
	.dash_main_links a::after {
		content: '';
		position: absolute;
		top: 0;
		right: 0;
		bottom: 0;
		width: 5px;
		background: transparent;
		transform: scaleX(1);
		transform-origin: 100% 50%;
		transition: var(--sidebar-transition);
	}

	.dash_main_links a:hover {
		color: var(--sidebar-text-active);
	}

	/* WordPress EXACT: Active state with cyan right border indicator */
	.dash_main_links li.is-active a {
		color: var(--sidebar-text-active);
	}

	.dash_main_links li.is-active a::after {
		background-color: var(--sidebar-accent);
	}

	.dash_main_links a:focus-visible {
		outline: 2px solid var(--sidebar-accent);
		outline-offset: -2px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CATEGORY HEADERS (WordPress EXACT: .dashboard__nav-category)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-category {
		font-weight: 600;
		padding: 30px 30px 0;
		color: hsla(0, 0%, 100%, 0.7);
		text-transform: uppercase;
		font-size: 10px;
		letter-spacing: 0.5px;
		margin: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   NAV ITEM ICONS (WordPress EXACT: .dashboard__nav-item-icon)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-item-icon {
		position: absolute;
		top: 50%;
		left: 30px;
		margin-top: -12px;
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--sidebar-icon-color);
		transform: scale(1);
		transition: var(--sidebar-transition);
	}

	/* SVG icons inside the container */
	.dashboard__nav-item-icon :global(svg) {
		width: 24px;
		height: 24px;
		stroke: currentColor;
		stroke-width: 1.5;
	}

	/* WordPress EXACT: Icon hover - scale and color change */
	.dash_main_links a:hover .dashboard__nav-item-icon {
		color: var(--sidebar-text-active);
		transform: scale(0.95);
	}

	.dash_main_links li.is-active .dashboard__nav-item-icon {
		color: var(--sidebar-text-active);
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
	   SERVICE ICON IMAGES
	   ═══════════════════════════════════════════════════════════════════════════ */

	.service-icon-img {
		width: 24px;
		height: 24px;
		border-radius: 50%;
		object-fit: cover;
	}

	/* Font Awesome icons fallback */
	.dashboard__nav-item-icon i {
		font-size: 16px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media screen and (max-width: 1279px) {
		.dashboard__nav-primary {
			padding-bottom: 100px; /* Space for mobile toggle */
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   REDUCED MOTION
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		.dashboard__profile-nav-item,
		.dash_main_links a,
		.dashboard__nav-item-icon {
			transition: none;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   HIGH CONTRAST MODE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-contrast: high) {
		.dash_main_links a {
			border: 1px solid transparent;
		}

		.dash_main_links a:focus-visible {
			border-color: white;
		}
	}
</style>
