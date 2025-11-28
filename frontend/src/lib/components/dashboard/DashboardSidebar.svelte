<script lang="ts">
	/**
	 * Dashboard Sidebar Component
	 * Matches WordPress WooCommerce account navigation structure
	 * Two-part sidebar: collapsed icons + expanded menu
	 */
	import { page } from '$app/stores';
	import {
		IconHome,
		IconUser,
		IconCreditCard,
		IconMapPin,
		IconReceipt,
		IconLogout
	} from '@tabler/icons-svelte';
	import type { UserMembership } from '$lib/api/user-memberships';

	interface Props {
		memberships?: UserMembership[];
		isCollapsed?: boolean;
	}

	let { memberships = [], isCollapsed = false }: Props = $props();

	// Account menu items matching WordPress structure
	const accountMenuItems = [
		{ href: '/dashboard', label: 'Dashboard', icon: IconHome, id: 'dashboard' },
		{ href: '/dashboard/account', label: 'Account Details', icon: IconUser, id: 'account' },
		{ href: '/dashboard/orders', label: 'Orders', icon: IconReceipt, id: 'orders' },
		{ href: '/dashboard/subscriptions', label: 'Subscriptions', icon: IconCreditCard, id: 'subscriptions' },
		{ href: '/dashboard/addresses', label: 'Addresses', icon: IconMapPin, id: 'addresses' },
		{ href: '/dashboard/payment-methods', label: 'Payment Methods', icon: IconCreditCard, id: 'payment-methods' }
	];

	function isActive(href: string): boolean {
		if (href === '/dashboard') {
			return $page.url.pathname === '/dashboard';
		}
		return $page.url.pathname === href || $page.url.pathname.startsWith(href + '/');
	}

	// Group memberships by type
	const tradingRooms = $derived(memberships.filter(m => m.type === 'trading-room'));
	const alertServices = $derived(memberships.filter(m => m.type === 'alert-service'));
</script>

<nav class="dashboard-sidebar" class:is-collapsed={isCollapsed}>
	<!-- Primary Menu (Icons) -->
	<ul class="account-primary-menu" class:is-collapsed={isCollapsed}>
		<!-- Profile -->
		<li>
			<a href="/dashboard" class="dashboard-profile-nav-item">
				<span class="dashboard-profile-photo"></span>
				{#if !isCollapsed}
					<span class="dashboard-profile-name">My Account</span>
				{/if}
			</a>
		</li>

		<!-- Trading Rooms Section -->
		{#if tradingRooms.length > 0}
			<li>
				<p class="dashboard-menu-category">Trading Rooms</p>
			</li>
			{#each tradingRooms as room (room.id)}
				<li class="{room.slug}-mp cstooltip">
					{#if isCollapsed}
						<span class="cstooltiptext">{room.name}</span>
					{/if}
					<a href="/live-trading-rooms/{room.slug}">
						<span class="dashboard-menu-item-icon st-icon-{room.slug}">
							<i class="fa fa-graduation-cap"></i>
						</span>
						{#if !isCollapsed}
							<span class="menu-label">{room.name}</span>
						{/if}
					</a>
				</li>
			{/each}
		{/if}

		<!-- Alert Services Section -->
		{#if alertServices.length > 0}
			<li>
				<p class="dashboard-menu-category">Alert Services</p>
			</li>
			{#each alertServices as alert (alert.id)}
				<li class="{alert.slug}-mp cstooltip">
					{#if isCollapsed}
						<span class="cstooltiptext">{alert.name}</span>
					{/if}
					<a href="/alerts/{alert.slug}">
						<span class="dashboard-menu-item-icon st-icon-{alert.slug}">
							<i class="fa fa-graduation-cap"></i>
						</span>
						{#if !isCollapsed}
							<span class="menu-label">{alert.name}</span>
						{/if}
					</a>
				</li>
			{/each}
		{/if}
	</ul>

	<!-- Secondary Menu (Expanded with labels) -->
	{#if !isCollapsed}
		<ul class="account-secondry-menu">
			<li class="woocommerce-MyAccount-navigation-link">
				<p class="dashboard-menu-category">Account</p>
			</li>
			{#each accountMenuItems as item (item.id)}
				{@const Icon = item.icon}
				<li
					class="woocommerce-MyAccount-navigation-link woocommerce-MyAccount-navigation-link--{item.id}"
					class:is-active={isActive(item.href)}
				>
					<a href={item.href}>
						<span class="dashboard-menu-item-icon">
							<Icon size={18} />
						</span>
						<span>{item.label}</span>
					</a>
				</li>
			{/each}
			<li class="woocommerce-MyAccount-navigation-link woocommerce-MyAccount-navigation-link--logout">
				<a href="/logout">
					<span class="dashboard-menu-item-icon">
						<IconLogout size={18} />
					</span>
					<span>Logout</span>
				</a>
			</li>
		</ul>
	{/if}
</nav>

<style>
	/* Dashboard Sidebar - Matches WordPress structure */
	.dashboard-sidebar {
		display: flex;
		flex: 0 0 auto;
		flex-flow: row;
		background-color: #0f2d41;
		min-height: calc(100vh - 80px);
	}

	.dashboard-sidebar.is-collapsed {
		width: 80px;
	}

	/* Primary Menu (Icon sidebar) */
	.account-primary-menu {
		display: block;
		padding: 30px 0 30px 30px;
		font-size: 16px;
		width: 100%;
		list-style: none;
		margin: 0;
	}

	.account-primary-menu.is-collapsed {
		width: 80px;
		padding: 30px 0 30px 10px;
	}

	.account-primary-menu li {
		position: relative;
		list-style: none;
		margin-bottom: 10px;
	}

	.account-primary-menu a {
		color: hsla(0, 0%, 100%, 0.5);
		min-height: 40px;
		display: flex;
		align-items: center;
		gap: 10px;
		font-weight: 300;
		text-decoration: none;
		transition: all 0.15s ease-in-out;
	}

	.account-primary-menu a:hover {
		color: #fff;
	}

	.account-primary-menu.is-collapsed a {
		padding: 5px 0 0;
		justify-content: center;
	}

	/* Profile */
	.dashboard-profile-nav-item {
		height: 50px;
		line-height: 50px;
		margin-top: 20px;
		margin-bottom: 20px;
	}

	.dashboard-profile-photo {
		display: block;
		width: 34px;
		height: 34px;
		border: 2px solid #fff;
		border-radius: 50%;
		background: url('https://secure.gravatar.com/avatar/?s=32&d=mm&r=g') no-repeat center;
		background-size: cover;
		transition: all 0.15s ease-in-out;
	}

	.dashboard-profile-name {
		display: block;
		color: #fff;
		font-weight: 600;
	}

	/* Menu Category Headers */
	.dashboard-menu-category {
		font-weight: 700;
		margin-bottom: 5px;
		margin-top: 20px;
		color: #fff;
		text-transform: uppercase;
		font-size: 12px;
		letter-spacing: 0.5px;
	}

	/* Menu Item Icons */
	.dashboard-menu-item-icon {
		line-height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #8796A0;
		transition: all 0.15s ease-in-out;
	}

	.account-primary-menu a:hover .dashboard-menu-item-icon {
		color: #fff;
	}

	/* Tooltip for collapsed state */
	.cstooltip {
		position: relative;
	}

	.cstooltiptext {
		background-color: #fff;
		color: #0984ae;
		transform: translate(5px);
		transition: all 0.15s ease-in-out;
		border-radius: 6px;
		z-index: 2;
		box-shadow: 0 10px 30px rgba(0, 0, 0, 0.15);
		left: 50px;
		top: 15px;
		height: 30px;
		visibility: hidden;
		line-height: 30px;
		padding: 0 10px;
		position: absolute;
		white-space: nowrap;
	}

	.cstooltip:hover .cstooltiptext {
		visibility: visible;
	}

	/* Secondary Menu */
	.account-secondry-menu {
		width: calc(100% - 20px);
		font-size: 14px;
		font-weight: 600;
		background-color: #153e59;
		padding: 20px 15px;
		list-style: none;
		margin: 0;
		transition: all 0.3s ease-in-out;
	}

	.account-secondry-menu li {
		display: block;
		list-style: none;
	}

	.account-secondry-menu li > a {
		cursor: pointer;
		display: inline-flex;
		align-items: center;
		padding: 15px;
		color: hsla(0, 0%, 100%, 0.75);
		border-radius: 5px;
		background-color: transparent;
		gap: 10px;
		text-decoration: none;
		width: 100%;
		transition: all 0s;
	}

	.account-secondry-menu li a:hover {
		background: #12354c;
	}

	.account-secondry-menu .is-active a {
		background: #0984ae;
		color: white;
		opacity: 1;
	}

	.woocommerce-MyAccount-navigation-link {
		padding: 0;
	}

	/* Responsive */
	@media screen and (max-width: 980px) {
		.dashboard-sidebar {
			width: 70%;
			bottom: 50px;
			left: 0;
			opacity: 0;
			overflow-x: hidden;
			overflow-y: auto;
			position: fixed;
			top: 0;
			transition: all 0.3s ease-in-out;
			visibility: hidden;
			z-index: 1000011;
		}

		.dashboard-sidebar.is-open {
			opacity: 1;
			visibility: visible;
			width: 280px;
		}

		.account-secondry-menu {
			visibility: hidden;
			bottom: 50px;
			left: 80px;
			opacity: 0;
			overflow-x: hidden;
			overflow-y: auto;
			padding-top: 15px;
			position: fixed;
			top: 0;
			width: 240px;
		}
	}
</style>
