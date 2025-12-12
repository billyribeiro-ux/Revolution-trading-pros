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
	 * @version 4.0.0 (WordPress-exact / December 2025)
	 */

	import { page } from '$app/stores';
	import { browser } from '$app/environment';
	import type { UserMembership } from '$lib/api/user-memberships';
	import '$lib/styles/st-icons.css';

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
		userAvatar?: string;
		userName?: string;
		subscriptionInfo?: SubscriptionInfo;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// PROPS & STATE (Svelte 5 Runes)
	// ═══════════════════════════════════════════════════════════════════════════

	let {
		memberships = [],
		isMobileOpen = $bindable(false),
		onCloseMobile,
		userAvatar = '',
		userName = 'My Account',
		subscriptionInfo
	}: Props = $props();

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

	function getServiceIconClass(slug: string): string {
		const iconMap: Record<string, string> = {
			'day-trading': 'st-icon-day-trading',
			'small-accounts': 'st-icon-small-accounts',
			'spx-profit-pulse': 'st-icon-spx-profit-pulse',
			'explosive-swings': 'st-icon-explosive-swings',
			'swing-trading': 'st-icon-swing-trading'
		};
		return iconMap[slug] || 'st-icon-chart';
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

<!-- WordPress: .dashboard__nav-primary -->
<nav class="dashboard__nav-primary" aria-label="Dashboard navigation">

	<!-- ═══════════════════════════════════════════════════════════════════════
	     PROFILE SECTION (WordPress: .dashboard__profile-nav-item)
	     ═══════════════════════════════════════════════════════════════════════ -->
	<a
		href="/dashboard/account"
		class="dashboard__profile-nav-item"
		class:is-account-active={isAccountSection}
		aria-current={isAccountSection ? 'page' : undefined}
	>
		<span
			class="dashboard__profile-photo"
			style:background-image={userAvatar ? `url(${userAvatar})` : undefined}
			aria-hidden="true"
		></span>
		<span class="dashboard__profile-name">{userName}</span>
	</a>

	<!-- ═══════════════════════════════════════════════════════════════════════
	     MEMBER SUBSCRIPTION INFO (Simpler Trading style)
	     ═══════════════════════════════════════════════════════════════════════ -->
	{#if hasSubscriptionInfo}
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
				<span class="dashboard__nav-item-icon st-icon-home"></span>
				<span class="dashboard__nav-item-text">Member Dashboard</span>
			</a>
		</li>
		<li class:is-active={isActive('/dashboard/courses')}>
			<a href="/dashboard/courses">
				<span class="dashboard__nav-item-icon st-icon-learning-center"></span>
				<span class="dashboard__nav-item-text">My Classes</span>
			</a>
		</li>
		<li class:is-active={isActive('/dashboard/indicators')}>
			<a href="/dashboard/indicators">
				<span class="dashboard__nav-item-icon st-icon-handle-stick"></span>
				<span class="dashboard__nav-item-text">My Indicators</span>
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
					class:is-active={isActive(`/live-trading-rooms/${room.slug}`)}
				>
					<a href="/live-trading-rooms/{room.slug}">
						<span class="dashboard__nav-item-icon {getServiceIconClass(room.slug)}">
							{#if room.icon}
								<img src={room.icon} alt="" class="service-icon-img" loading="lazy" />
							{:else}
								<i class="fa fa-graduation-cap" aria-hidden="true"></i>
							{/if}
						</span>
						<span class="dashboard__nav-item-text">{room.name}</span>
					</a>
				</li>
			{/each}
			{#each alertServices as alert (alert.id)}
				<li
					class="{alert.slug}-mp"
					class:is-active={isActive(`/alerts/${alert.slug}`)}
				>
					<a href="/alerts/{alert.slug}">
						<span class="dashboard__nav-item-icon {getServiceIconClass(alert.slug)}">
							{#if alert.icon}
								<img src={alert.icon} alt="" class="service-icon-img" loading="lazy" />
							{:else}
								<i class="fa fa-bell" aria-hidden="true"></i>
							{/if}
						</span>
						<span class="dashboard__nav-item-text">{alert.name}</span>
					</a>
				</li>
			{/each}
		</ul>
	{/if}

	<!-- ═══════════════════════════════════════════════════════════════════════
	     MASTERY SECTION
	     ═══════════════════════════════════════════════════════════════════════ -->
	{#if hasCourses}
		<ul>
			<li><p class="dashboard__nav-category">mastery</p></li>
		</ul>
		<ul class="dash_main_links">
			{#each courses as course (course.id)}
				<li
					class="{course.slug}-mp"
					class:is-active={isActive(`/dashboard/courses/${course.slug}`)}
				>
					<a href="/dashboard/courses/{course.slug}">
						<span class="dashboard__nav-item-icon st-icon-learning-center">
							{#if course.icon}
								<img src={course.icon} alt="" class="service-icon-img" loading="lazy" />
							{:else}
								<i class="fa fa-book" aria-hidden="true"></i>
							{/if}
						</span>
						<span class="dashboard__nav-item-text">{course.name}</span>
					</a>
				</li>
			{/each}
		</ul>
	{/if}

	<!-- ═══════════════════════════════════════════════════════════════════════
	     TOOLS SECTION (Simpler Trading EXACT - Always shown)
	     ═══════════════════════════════════════════════════════════════════════ -->
	<ul>
		<li><p class="dashboard__nav-category">tools</p></li>
	</ul>
	<ul class="dash_main_links">
		<li class:is-active={isActive('/dashboard/watchlist')}>
			<a href="/dashboard/watchlist">
				<span class="dashboard__nav-item-icon st-icon-trade-of-the-week"></span>
				<span class="dashboard__nav-item-text">Weekly Watchlist</span>
			</a>
		</li>
		<li class:is-active={isActive('/dashboard/support')}>
			<a href="/dashboard/support">
				<span class="dashboard__nav-item-icon st-icon-support"></span>
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
				<span class="dashboard__nav-item-icon st-icon-settings"></span>
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
	   CSS CUSTOM PROPERTIES (WordPress Reference)
	   ═══════════════════════════════════════════════════════════════════════════ */

	:root {
		--sidebar-bg-primary: #0f2d41;
		--sidebar-bg-secondary: #153e59;
		--sidebar-bg-hover: rgba(255, 255, 255, 0.05);
		--sidebar-text: hsla(0, 0%, 100%, 0.5);
		--sidebar-text-active: #fff;
		--sidebar-accent: #0984ae;
		--sidebar-accent-hover: #076787;
		--sidebar-icon-color: #8796A0;
		--sidebar-transition: all 0.15s ease-in-out;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   PRIMARY NAVIGATION (WordPress: .dashboard__nav-primary)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-primary {
		display: block;
		padding: 30px;
		font-size: 16px;
		width: 100%;
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
	   PROFILE SECTION (WordPress: .dashboard__profile-nav-item)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__profile-nav-item {
		display: flex;
		align-items: center;
		gap: 12px;
		height: 50px;
		margin-top: 20px;
		margin-bottom: 30px;
		text-decoration: none;
		transition: var(--sidebar-transition);
	}

	.dashboard__profile-nav-item:hover .dashboard__profile-name {
		color: var(--sidebar-text-active);
	}

	/* Lighter blue highlight when on account section */
	.dashboard__profile-nav-item.is-account-active {
		background: rgba(9, 132, 174, 0.15);
		border-radius: 8px;
		padding: 10px 12px;
		margin: 20px -12px 30px;
	}

	.dashboard__profile-nav-item.is-account-active .dashboard__profile-name {
		color: #5bc0de;
	}

	.dashboard__profile-nav-item.is-account-active .dashboard__profile-photo {
		border-color: #5bc0de;
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

	.dashboard__profile-photo {
		display: block;
		width: 34px;
		height: 34px;
		border: 2px solid #fff;
		border-radius: 50%;
		background: url('https://secure.gravatar.com/avatar/?s=32&d=mm&r=g') no-repeat center;
		background-size: cover;
		flex-shrink: 0;
		transition: var(--sidebar-transition);
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
	   MAIN LINKS (WordPress: .dash_main_links)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dash_main_links {
		margin-bottom: 10px;
	}

	.dash_main_links li {
		margin-bottom: 2px;
	}

	.dash_main_links a {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 10px 12px;
		color: var(--sidebar-text);
		text-decoration: none;
		border-radius: 8px;
		font-weight: 300;
		font-size: 15px;
		transition: var(--sidebar-transition);
	}

	.dash_main_links a:hover {
		color: var(--sidebar-text-active);
		background: var(--sidebar-bg-hover);
	}

	.dash_main_links li.is-active a {
		color: var(--sidebar-text-active);
		background: var(--sidebar-accent);
	}

	.dash_main_links a:focus-visible {
		outline: 2px solid var(--sidebar-accent);
		outline-offset: 2px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CATEGORY HEADERS (WordPress: .dashboard__nav-category)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-category {
		color: var(--sidebar-text-active);
		font-weight: 700;
		font-size: 11px;
		letter-spacing: 0.5px;
		text-transform: uppercase;
		margin: 25px 0 10px;
		padding: 0 12px;
		opacity: 0.7;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   NAV ITEM ICONS (WordPress: .dashboard__nav-item-icon)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-item-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		color: var(--sidebar-icon-color);
		font-size: 18px;
		flex-shrink: 0;
		transition: var(--sidebar-transition);
	}

	.dash_main_links a:hover .dashboard__nav-item-icon,
	.dash_main_links li.is-active .dashboard__nav-item-icon {
		color: var(--sidebar-text-active);
	}

	/* StIcon specific styles */
	.dashboard__nav-item-icon.st-icon-home::before,
	.dashboard__nav-item-icon.st-icon-learning-center::before,
	.dashboard__nav-item-icon.st-icon-handle-stick::before,
	.dashboard__nav-item-icon.st-icon-settings::before,
	.dashboard__nav-item-icon.st-icon-logout::before,
	.dashboard__nav-item-icon.st-icon-chart::before {
		font-size: 20px;
		line-height: 24px;
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

	@media screen and (max-width: 980px) {
		.dashboard__nav-primary {
			padding: 20px;
			padding-bottom: 100px; /* Space for mobile toggle */
		}
	}

	@media screen and (max-width: 480px) {
		.dashboard__nav-primary {
			padding: 16px;
		}

		.dash_main_links a {
			padding: 12px 10px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   REDUCED MOTION
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		.dashboard__profile-nav-item,
		.dash_main_links a,
		.dashboard__nav-item-icon,
		.dashboard__profile-photo {
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
