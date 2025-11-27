<script lang="ts">
	/**
	 * MarketingNav - Navigation for marketing/public pages
	 * Extracted from root layout for better separation of concerns
	 * 
	 * @version 2.0.0
	 * @author Revolution Trading Pros
	 */
	import { page } from '$app/stores';
	import { isAuthenticated, user } from '$lib/stores/auth';
	import {
		IconMenu2,
		IconX,
		IconChevronDown,
		IconUser,
		IconLogout,
		IconDashboard
	} from '@tabler/icons-svelte';

	let mobileMenuOpen = false;
	let userMenuOpen = false;

	const navLinks = [
		{ href: '/live-trading-rooms/day-trading', label: 'Live Trading Rooms' },
		{ href: '/alerts', label: 'Alerts' },
		{ href: '/courses', label: 'Courses' },
		{ href: '/indicators', label: 'Indicators' },
		{ href: '/mentorship', label: 'Mentorship' },
		{ href: '/blog', label: 'Blog' }
	];

	function toggleMobileMenu() {
		mobileMenuOpen = !mobileMenuOpen;
	}

	function toggleUserMenu() {
		userMenuOpen = !userMenuOpen;
	}

	function closeMobileMenu() {
		mobileMenuOpen = false;
	}

	$: currentPath = $page.url.pathname;
</script>

<nav class="marketing-nav">
	<div class="nav-container">
		<!-- Logo -->
		<a href="/" class="nav-logo">
			<img src="/revolution-trading-pros.png" alt="Revolution Trading Pros" />
		</a>

		<!-- Desktop Navigation -->
		<div class="nav-links desktop-only">
			{#each navLinks as link}
				<a 
					href={link.href} 
					class="nav-link"
					class:active={currentPath.startsWith(link.href)}
				>
					{link.label}
				</a>
			{/each}
		</div>

		<!-- Auth Section -->
		<div class="nav-auth">
			{#if $isAuthenticated}
				<div class="user-menu-container">
					<button class="user-menu-trigger" on:click={toggleUserMenu}>
						<span class="user-avatar">
							{$user?.name?.charAt(0).toUpperCase() || 'U'}
						</span>
						<span class="user-name desktop-only">{$user?.name || 'User'}</span>
						<IconChevronDown size={16} />
					</button>
					
					{#if userMenuOpen}
						<div class="user-menu-dropdown">
							<a href="/dashboard" class="dropdown-item">
								<IconDashboard size={18} />
								Dashboard
							</a>
							<a href="/account" class="dropdown-item">
								<IconUser size={18} />
								Account
							</a>
							<hr class="dropdown-divider" />
							<button class="dropdown-item danger">
								<IconLogout size={18} />
								Sign Out
							</button>
						</div>
					{/if}
				</div>
			{:else}
				<a href="/login" class="btn-ghost desktop-only">Sign In</a>
				<a href="/signup" class="btn-primary">Get Started</a>
			{/if}

			<!-- Mobile Menu Toggle -->
			<button class="mobile-menu-toggle mobile-only" on:click={toggleMobileMenu}>
				{#if mobileMenuOpen}
					<IconX size={24} />
				{:else}
					<IconMenu2 size={24} />
				{/if}
			</button>
		</div>
	</div>

	<!-- Mobile Menu -->
	{#if mobileMenuOpen}
		<div class="mobile-menu">
			{#each navLinks as link}
				<a 
					href={link.href} 
					class="mobile-nav-link"
					class:active={currentPath.startsWith(link.href)}
					on:click={closeMobileMenu}
				>
					{link.label}
				</a>
			{/each}
			
			{#if !$isAuthenticated}
				<hr class="mobile-divider" />
				<a href="/login" class="mobile-nav-link" on:click={closeMobileMenu}>Sign In</a>
			{/if}
		</div>
	{/if}
</nav>

<!-- Backdrop for mobile menu -->
{#if mobileMenuOpen}
	<button 
		class="mobile-backdrop" 
		on:click={closeMobileMenu}
		aria-label="Close menu"
	></button>
{/if}

<!-- Backdrop for user menu -->
{#if userMenuOpen}
	<button 
		class="user-menu-backdrop" 
		on:click={() => userMenuOpen = false}
		aria-label="Close user menu"
	></button>
{/if}

<style>
	.marketing-nav {
		position: sticky;
		top: 0;
		z-index: var(--z-sticky, 200);
		background: rgba(15, 23, 42, 0.95);
		backdrop-filter: blur(12px);
		-webkit-backdrop-filter: blur(12px);
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
	}

	.nav-container {
		max-width: 1400px;
		margin: 0 auto;
		padding: 0 1rem;
		height: 72px;
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 2rem;
	}

	.nav-logo img {
		height: 40px;
		width: auto;
	}

	.nav-links {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.nav-link {
		padding: 0.5rem 1rem;
		color: var(--color-rtp-muted, #94a3b8);
		text-decoration: none;
		font-weight: 500;
		font-size: 0.9375rem;
		border-radius: var(--radius-md, 0.5rem);
		transition: all 0.2s;
	}

	.nav-link:hover {
		color: var(--color-rtp-text, #f1f5f9);
		background: rgba(99, 102, 241, 0.1);
	}

	.nav-link.active {
		color: var(--color-rtp-primary, #818cf8);
		background: rgba(99, 102, 241, 0.15);
	}

	.nav-auth {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.btn-ghost {
		padding: 0.5rem 1rem;
		color: var(--color-rtp-muted, #94a3b8);
		text-decoration: none;
		font-weight: 500;
		border-radius: var(--radius-md, 0.5rem);
		transition: all 0.2s;
	}

	.btn-ghost:hover {
		color: var(--color-rtp-text, #f1f5f9);
	}

	.btn-primary {
		padding: 0.625rem 1.25rem;
		background: linear-gradient(135deg, var(--color-rtp-primary, #6366f1), var(--color-rtp-indigo, #8b5cf6));
		color: white;
		text-decoration: none;
		font-weight: 600;
		border-radius: var(--radius-md, 0.5rem);
		transition: all 0.2s;
	}

	.btn-primary:hover {
		transform: translateY(-1px);
		box-shadow: 0 4px 15px rgba(99, 102, 241, 0.4);
	}

	/* User Menu */
	.user-menu-container {
		position: relative;
	}

	.user-menu-trigger {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.375rem 0.75rem;
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: var(--radius-lg, 0.75rem);
		color: var(--color-rtp-text, #e2e8f0);
		cursor: pointer;
		transition: all 0.2s;
	}

	.user-menu-trigger:hover {
		background: rgba(99, 102, 241, 0.2);
	}

	.user-avatar {
		width: 32px;
		height: 32px;
		border-radius: var(--radius-md, 0.5rem);
		background: linear-gradient(135deg, var(--color-rtp-primary, #6366f1), var(--color-rtp-indigo, #8b5cf6));
		display: flex;
		align-items: center;
		justify-content: center;
		font-weight: 600;
		font-size: 0.875rem;
	}

	.user-name {
		font-weight: 500;
		font-size: 0.875rem;
	}

	.user-menu-dropdown {
		position: absolute;
		top: calc(100% + 0.5rem);
		right: 0;
		min-width: 200px;
		background: var(--color-rtp-surface, #1e293b);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: var(--radius-lg, 0.75rem);
		padding: 0.5rem;
		box-shadow: 0 10px 40px rgba(0, 0, 0, 0.3);
		z-index: var(--z-dropdown, 100);
	}

	.dropdown-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		color: var(--color-rtp-muted, #94a3b8);
		text-decoration: none;
		border-radius: var(--radius-md, 0.5rem);
		font-size: 0.9375rem;
		width: 100%;
		background: none;
		border: none;
		cursor: pointer;
		transition: all 0.2s;
	}

	.dropdown-item:hover {
		background: rgba(99, 102, 241, 0.1);
		color: var(--color-rtp-text, #f1f5f9);
	}

	.dropdown-item.danger:hover {
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
	}

	.dropdown-divider {
		margin: 0.5rem 0;
		border: none;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
	}

	.user-menu-backdrop {
		position: fixed;
		inset: 0;
		background: transparent;
		z-index: calc(var(--z-dropdown, 100) - 1);
		border: none;
		cursor: default;
	}

	/* Mobile */
	.mobile-menu-toggle {
		display: none;
		padding: 0.5rem;
		background: none;
		border: none;
		color: var(--color-rtp-muted, #94a3b8);
		cursor: pointer;
	}

	.mobile-menu {
		display: none;
		padding: 1rem;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
	}

	.mobile-nav-link {
		display: block;
		padding: 0.875rem 1rem;
		color: var(--color-rtp-muted, #94a3b8);
		text-decoration: none;
		font-weight: 500;
		border-radius: var(--radius-md, 0.5rem);
		transition: all 0.2s;
	}

	.mobile-nav-link:hover,
	.mobile-nav-link.active {
		color: var(--color-rtp-text, #f1f5f9);
		background: rgba(99, 102, 241, 0.1);
	}

	.mobile-divider {
		margin: 0.75rem 0;
		border: none;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
	}

	.mobile-backdrop {
		display: none;
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.5);
		z-index: calc(var(--z-sticky, 200) - 1);
		border: none;
	}

	/* Responsive */
	.desktop-only {
		display: flex;
	}

	.mobile-only {
		display: none;
	}

	@media (max-width: 1024px) {
		.desktop-only {
			display: none;
		}

		.mobile-only {
			display: flex;
		}

		.mobile-menu-toggle {
			display: flex;
		}

		.mobile-menu {
			display: block;
		}

		.mobile-backdrop {
			display: block;
		}

		.user-name {
			display: none;
		}
	}
</style>
