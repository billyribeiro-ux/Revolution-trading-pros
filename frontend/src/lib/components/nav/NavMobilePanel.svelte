<script lang="ts">
	/**
	 * NavMobilePanel - Full-height mobile navigation panel
	 * 
	 * Features:
	 * - Slide-in animation from right
	 * - Focus trap for accessibility
	 * - Scroll lock on body
	 * - Escape key to close
	 * - Nested accordion for submenus
	 * 
	 * @version 1.0.0
	 */
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { 
		IconX, 
		IconChevronDown,
		IconShoppingCart,
		IconUser
	} from '@tabler/icons-svelte';
	import { isAuthenticated, user } from '$lib/stores/auth';
	import { cartItemCount, hasCartItems } from '$lib/stores/cart';
	import { logout as logoutApi } from '$lib/api/auth';
	import { authStore } from '$lib/stores/auth';
	import type { NavItem } from './types';

	interface Props {
		isOpen: boolean;
		items: NavItem[];
		onClose: () => void;
	}

	let { isOpen, items, onClose }: Props = $props();

	let panelRef = $state<HTMLDivElement | null>(null);
	let closeButtonRef = $state<HTMLButtonElement | null>(null);
	let expandedSubmenu = $state<string | null>(null);

	const currentPath = $derived($page.url.pathname);

	// Dashboard items for authenticated users
	const dashboardItems = [
		{ href: '/dashboard', label: 'Dashboard Home' },
		{ href: '/dashboard/courses', label: 'My Courses' },
		{ href: '/dashboard/indicators', label: 'My Indicators' },
		{ href: '/dashboard/account', label: 'My Account' }
	];

	function toggleSubmenu(id: string) {
		expandedSubmenu = expandedSubmenu === id ? null : id;
	}

	function handleNavigation(href: string) {
		onClose();
		goto(href);
	}

	async function handleLogout() {
		onClose();
		try {
			await logoutApi();
		} catch (e) {
			console.error('Logout error:', e);
		}
		authStore.clearAuth();
		await goto('/login');
	}

	function handleKeyDown(event: KeyboardEvent) {
		if (event.key === 'Escape') {
			onClose();
		}
	}

	// Focus trap
	function trapFocus(event: KeyboardEvent) {
		if (event.key !== 'Tab' || !panelRef) return;

		const focusableElements = panelRef.querySelectorAll<HTMLElement>(
			'button, a[href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
		);
		const firstElement = focusableElements[0];
		const lastElement = focusableElements[focusableElements.length - 1];

		if (event.shiftKey && document.activeElement === firstElement) {
			event.preventDefault();
			lastElement?.focus();
		} else if (!event.shiftKey && document.activeElement === lastElement) {
			event.preventDefault();
			firstElement?.focus();
		}
	}

	// Lock body scroll when open
	$effect(() => {
		if (isOpen) {
			document.body.style.overflow = 'hidden';
			// Focus close button when panel opens
			setTimeout(() => closeButtonRef?.focus(), 100);
		} else {
			document.body.style.overflow = '';
		}
		
		return () => {
			document.body.style.overflow = '';
		};
	});

	onMount(() => {
		document.addEventListener('keydown', handleKeyDown);
		return () => document.removeEventListener('keydown', handleKeyDown);
	});
</script>

{#if isOpen}
	<!-- Backdrop -->
	<button 
		class="mobile-backdrop" 
		onclick={onClose}
		aria-label="Close menu"
		tabindex="-1"
	></button>

	<!-- Panel -->
	<!-- svelte-ignore a11y_interactive_supports_focus -->
	<div
		bind:this={panelRef}
		class="mobile-panel"
		role="dialog"
		aria-modal="true"
		aria-label="Navigation menu"
		onkeydown={trapFocus}
	>
		<!-- Header -->
		<div class="mobile-panel-header">
			<span class="mobile-panel-title">Menu</span>
			<button
				bind:this={closeButtonRef}
				class="mobile-close-btn"
				onclick={onClose}
				aria-label="Close menu"
			>
				<IconX size={24} />
			</button>
		</div>

		<!-- Content -->
		<div class="mobile-panel-content">
			<!-- User info if authenticated -->
			{#if $isAuthenticated && $user}
				<div class="mobile-user-info">
					<div class="mobile-user-avatar">
						{$user.name?.charAt(0).toUpperCase() || 'U'}
					</div>
					<div class="mobile-user-details">
						<span class="mobile-user-name">{$user.name || 'User'}</span>
						<span class="mobile-user-email">{$user.email || ''}</span>
					</div>
				</div>
			{/if}

			<!-- Navigation Items -->
			<nav class="mobile-nav">
				{#each items as item (item.id)}
					{#if item.submenu}
						<!-- Accordion item -->
						<div class="mobile-nav-group">
							<button
								class="mobile-nav-item has-submenu"
								class:expanded={expandedSubmenu === item.id}
								onclick={() => toggleSubmenu(item.id)}
								aria-expanded={expandedSubmenu === item.id}
							>
								<span>{item.label}</span>
								<IconChevronDown 
									size={18} 
									class="submenu-chevron {expandedSubmenu === item.id ? 'rotate' : ''}"
								/>
							</button>
							{#if expandedSubmenu === item.id}
								<div class="mobile-submenu">
									{#each item.submenu as sub (sub.href)}
										<a
											href={sub.href}
											class="mobile-submenu-item"
											class:active={currentPath === sub.href}
											onclick={() => handleNavigation(sub.href)}
										>
											{sub.label}
										</a>
									{/each}
								</div>
							{/if}
						</div>
					{:else}
						<!-- Direct link -->
						<a
							href={item.href}
							class="mobile-nav-item"
							class:active={currentPath === item.href}
							onclick={() => item.href && handleNavigation(item.href)}
						>
							{item.label}
						</a>
					{/if}
				{/each}

				<!-- Dashboard section for authenticated users -->
				{#if $isAuthenticated}
					<div class="mobile-nav-divider"></div>
					<div class="mobile-nav-group">
						<button
							class="mobile-nav-item has-submenu"
							class:expanded={expandedSubmenu === 'dashboard'}
							onclick={() => toggleSubmenu('dashboard')}
							aria-expanded={expandedSubmenu === 'dashboard'}
						>
							<span>Dashboard</span>
							<IconChevronDown 
								size={18} 
								class="submenu-chevron {expandedSubmenu === 'dashboard' ? 'rotate' : ''}"
							/>
						</button>
						{#if expandedSubmenu === 'dashboard'}
							<div class="mobile-submenu">
								{#each dashboardItems as item (item.href)}
									<a
										href={item.href}
										class="mobile-submenu-item"
										class:active={currentPath === item.href}
										onclick={() => handleNavigation(item.href)}
									>
										{item.label}
									</a>
								{/each}
							</div>
						{/if}
					</div>
				{/if}
			</nav>
		</div>

		<!-- Footer Actions -->
		<div class="mobile-panel-footer">
			{#if $hasCartItems}
				<a 
					href="/cart" 
					class="mobile-cart-btn"
					onclick={() => handleNavigation('/cart')}
				>
					<IconShoppingCart size={20} />
					<span>Cart</span>
					{#if $cartItemCount > 0}
						<span class="mobile-cart-badge">{$cartItemCount}</span>
					{/if}
				</a>
			{/if}

			{#if $isAuthenticated}
				<button class="mobile-logout-btn" onclick={handleLogout}>
					Logout
				</button>
			{:else}
				<a 
					href="/login" 
					class="mobile-login-btn"
					onclick={() => handleNavigation('/login')}
				>
					Login
				</a>
				<a 
					href="/get-started" 
					class="mobile-cta-btn"
					onclick={() => handleNavigation('/get-started')}
				>
					Get Started
				</a>
			{/if}
		</div>
	</div>
{/if}

<style>
	.mobile-backdrop {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.6);
		backdrop-filter: blur(4px);
		z-index: 9998;
		border: none;
		cursor: pointer;
	}

	.mobile-panel {
		position: fixed;
		top: 0;
		right: 0;
		width: min(85vw, 360px);
		height: 100%;
		display: flex;
		flex-direction: column;
		background: #151f31;
		border-left: 1px solid rgba(255, 255, 255, 0.1);
		z-index: 9999;
		animation: slideIn 0.25s ease-out;
	}

	@keyframes slideIn {
		from {
			transform: translateX(100%);
		}
		to {
			transform: translateX(0);
		}
	}

	.mobile-panel-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		height: 64px;
		padding: 0 20px;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
		flex-shrink: 0;
	}

	.mobile-panel-title {
		font-size: 18px;
		font-weight: 600;
		color: white;
	}

	.mobile-close-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 44px;
		height: 44px;
		color: rgba(255, 255, 255, 0.7);
		background: transparent;
		border: none;
		border-radius: 10px;
		cursor: pointer;
		transition: background-color 0.15s ease, color 0.15s ease;
	}

	.mobile-close-btn:hover {
		background: rgba(255, 255, 255, 0.1);
		color: white;
	}

	.mobile-close-btn:focus-visible {
		outline: none;
		box-shadow: 0 0 0 2px rgba(250, 204, 21, 0.5);
	}

	.mobile-panel-content {
		flex: 1;
		overflow-y: auto;
		padding: 16px 0;
	}

	.mobile-user-info {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 12px 20px;
		margin-bottom: 8px;
		background: rgba(255, 255, 255, 0.03);
	}

	.mobile-user-avatar {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 44px;
		height: 44px;
		background: linear-gradient(135deg, #facc15, #eab308);
		color: #0a101c;
		font-size: 18px;
		font-weight: 700;
		border-radius: 50%;
	}

	.mobile-user-details {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.mobile-user-name {
		font-size: 15px;
		font-weight: 600;
		color: white;
	}

	.mobile-user-email {
		font-size: 13px;
		color: rgba(255, 255, 255, 0.5);
	}

	.mobile-nav {
		display: flex;
		flex-direction: column;
	}

	.mobile-nav-group {
		display: flex;
		flex-direction: column;
	}

	.mobile-nav-item {
		display: flex;
		align-items: center;
		justify-content: space-between;
		width: 100%;
		padding: 14px 20px;
		color: rgba(255, 255, 255, 0.85);
		font-size: 15px;
		font-weight: 500;
		text-decoration: none;
		background: transparent;
		border: none;
		cursor: pointer;
		transition: background-color 0.15s ease, color 0.15s ease;
		text-align: left;
	}

	.mobile-nav-item:hover {
		background: rgba(255, 255, 255, 0.05);
		color: white;
	}

	.mobile-nav-item.active {
		color: #facc15;
		background: rgba(250, 204, 21, 0.08);
	}

	.mobile-nav-item:focus-visible {
		outline: none;
		box-shadow: inset 0 0 0 2px rgba(250, 204, 21, 0.5);
	}

	.mobile-nav-item :global(.submenu-chevron) {
		transition: transform 0.2s ease;
	}

	.mobile-nav-item :global(.submenu-chevron.rotate) {
		transform: rotate(180deg);
	}

	.mobile-submenu {
		display: flex;
		flex-direction: column;
		background: rgba(0, 0, 0, 0.15);
	}

	.mobile-submenu-item {
		padding: 12px 20px 12px 36px;
		color: rgba(255, 255, 255, 0.7);
		font-size: 14px;
		text-decoration: none;
		transition: background-color 0.15s ease, color 0.15s ease;
	}

	.mobile-submenu-item:hover {
		background: rgba(255, 255, 255, 0.05);
		color: white;
	}

	.mobile-submenu-item.active {
		color: #facc15;
	}

	.mobile-nav-divider {
		height: 1px;
		margin: 12px 20px;
		background: rgba(255, 255, 255, 0.1);
	}

	.mobile-panel-footer {
		display: flex;
		flex-direction: column;
		gap: 10px;
		padding: 16px 20px;
		border-top: 1px solid rgba(255, 255, 255, 0.1);
		flex-shrink: 0;
	}

	.mobile-cart-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		height: 48px;
		color: #a78bfa;
		font-size: 15px;
		font-weight: 600;
		text-decoration: none;
		background: rgba(139, 92, 246, 0.1);
		border-radius: 10px;
		transition: background-color 0.15s ease;
	}

	.mobile-cart-btn:hover {
		background: rgba(139, 92, 246, 0.2);
	}

	.mobile-cart-badge {
		display: flex;
		align-items: center;
		justify-content: center;
		min-width: 22px;
		height: 22px;
		padding: 0 6px;
		background: #facc15;
		color: #0a101c;
		font-size: 12px;
		font-weight: 700;
		border-radius: 11px;
	}

	.mobile-login-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 48px;
		color: white;
		font-size: 15px;
		font-weight: 600;
		text-decoration: none;
		background: transparent;
		border: 1px solid rgba(255, 255, 255, 0.25);
		border-radius: 10px;
		transition: background-color 0.15s ease, border-color 0.15s ease;
	}

	.mobile-login-btn:hover {
		background: rgba(255, 255, 255, 0.05);
		border-color: rgba(255, 255, 255, 0.4);
	}

	.mobile-cta-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 48px;
		color: #0a101c;
		font-size: 15px;
		font-weight: 700;
		text-decoration: none;
		background: linear-gradient(135deg, #facc15, #eab308);
		border-radius: 10px;
		box-shadow: 0 4px 12px rgba(250, 204, 21, 0.3);
		transition: transform 0.15s ease, box-shadow 0.15s ease;
	}

	.mobile-cta-btn:hover {
		transform: translateY(-1px);
		box-shadow: 0 6px 16px rgba(250, 204, 21, 0.4);
	}

	.mobile-logout-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 48px;
		color: #f87171;
		font-size: 15px;
		font-weight: 600;
		background: rgba(248, 113, 113, 0.1);
		border: none;
		border-radius: 10px;
		cursor: pointer;
		transition: background-color 0.15s ease;
	}

	.mobile-logout-btn:hover {
		background: rgba(248, 113, 113, 0.2);
	}
</style>
