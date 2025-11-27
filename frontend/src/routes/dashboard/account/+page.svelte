<script lang="ts">
	/**
	 * Dashboard - My Account Page
	 * User account settings and profile management
	 */
	import SEOHead from '$lib/components/SEOHead.svelte';
	import { authStore, isAuthenticated, user } from '$lib/stores/auth';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { fade, fly } from 'svelte/transition';
	import {
		IconUser,
		IconMail,
		IconLock,
		IconCreditCard,
		IconBell,
		IconShield,
		IconLogout,
		IconEdit,
		IconCheck
	} from '@tabler/icons-svelte';

	// Redirect if not authenticated
	onMount(() => {
		if (!$isAuthenticated) {
			goto('/login?redirect=/dashboard/account');
		}
	});

	let activeTab = 'profile';

	const tabs = [
		{ id: 'profile', label: 'Profile', icon: IconUser },
		{ id: 'security', label: 'Security', icon: IconShield },
		{ id: 'billing', label: 'Billing', icon: IconCreditCard },
		{ id: 'notifications', label: 'Notifications', icon: IconBell }
	];

	// Form states
	let profileForm = {
		name: $user?.name || '',
		email: $user?.email || '',
		phone: ''
	};

	let isSaving = false;
	let saveSuccess = false;

	async function handleSaveProfile() {
		isSaving = true;
		// Simulate API call
		await new Promise(resolve => setTimeout(resolve, 1000));
		isSaving = false;
		saveSuccess = true;
		setTimeout(() => saveSuccess = false, 3000);
	}

	function handleLogout() {
		authStore.logout();
		goto('/');
	}
</script>

<SEOHead
	title="My Account"
	description="Manage your account settings and preferences."
	noindex
/>

{#if $isAuthenticated}
	<main class="dashboard-page">
		<div class="container">
			<!-- Header -->
			<header class="page-header" in:fly={{ y: -20, duration: 400 }}>
				<div class="page-header__content">
					<h1 class="page-header__title">My Account</h1>
					<p class="page-header__subtitle">Manage your profile and account settings</p>
				</div>
			</header>

			<div class="account-layout">
				<!-- Sidebar -->
				<aside class="account-sidebar" in:fly={{ x: -20, duration: 400 }}>
					<nav class="sidebar-nav">
						{#each tabs as tab}
							{@const Icon = tab.icon}
							<button
								class="sidebar-nav__item"
								class:active={activeTab === tab.id}
								onclick={() => activeTab = tab.id}
							>
								<Icon size={20} />
								<span>{tab.label}</span>
							</button>
						{/each}
					</nav>

					<button class="logout-btn" onclick={handleLogout}>
						<IconLogout size={20} />
						<span>Logout</span>
					</button>
				</aside>

				<!-- Content -->
				<div class="account-content" in:fly={{ y: 20, duration: 400, delay: 100 }}>
					{#if activeTab === 'profile'}
						<div class="content-section">
							<h2 class="section-title">Profile Information</h2>
							<p class="section-description">Update your personal information and contact details.</p>

							<form class="profile-form" onsubmit={(e) => { e.preventDefault(); handleSaveProfile(); }}>
								<div class="form-group">
									<label for="name">Full Name</label>
									<input
										type="text"
										id="name"
										bind:value={profileForm.name}
										placeholder="Enter your full name"
									/>
								</div>

								<div class="form-group">
									<label for="email">Email Address</label>
									<input
										type="email"
										id="email"
										bind:value={profileForm.email}
										placeholder="Enter your email"
									/>
								</div>

								<div class="form-group">
									<label for="phone">Phone Number</label>
									<input
										type="tel"
										id="phone"
										bind:value={profileForm.phone}
										placeholder="Enter your phone number"
									/>
								</div>

								<div class="form-actions">
									<button type="submit" class="save-btn" disabled={isSaving}>
										{#if isSaving}
											Saving...
										{:else if saveSuccess}
											<IconCheck size={18} />
											Saved!
										{:else}
											Save Changes
										{/if}
									</button>
								</div>
							</form>
						</div>
					{:else if activeTab === 'security'}
						<div class="content-section">
							<h2 class="section-title">Security Settings</h2>
							<p class="section-description">Manage your password and security preferences.</p>

							<div class="security-options">
								<div class="security-item">
									<div class="security-item__info">
										<IconLock size={24} />
										<div>
											<h3>Password</h3>
											<p>Last changed 30 days ago</p>
										</div>
									</div>
									<button class="security-item__btn">Change Password</button>
								</div>

								<div class="security-item">
									<div class="security-item__info">
										<IconShield size={24} />
										<div>
											<h3>Two-Factor Authentication</h3>
											<p>Add an extra layer of security</p>
										</div>
									</div>
									<button class="security-item__btn">Enable 2FA</button>
								</div>
							</div>
						</div>
					{:else if activeTab === 'billing'}
						<div class="content-section">
							<h2 class="section-title">Billing & Subscriptions</h2>
							<p class="section-description">Manage your payment methods and subscriptions.</p>

							<div class="billing-card">
								<div class="billing-card__header">
									<h3>Current Plan</h3>
									<span class="plan-badge">Pro</span>
								</div>
								<p class="billing-card__text">Your subscription renews on January 1, 2026</p>
								<div class="billing-card__actions">
									<button class="billing-btn">Manage Subscription</button>
									<button class="billing-btn billing-btn--secondary">View Invoices</button>
								</div>
							</div>
						</div>
					{:else if activeTab === 'notifications'}
						<div class="content-section">
							<h2 class="section-title">Notification Preferences</h2>
							<p class="section-description">Choose how you want to receive updates.</p>

							<div class="notification-options">
								<label class="notification-item">
									<div class="notification-item__info">
										<h3>Email Notifications</h3>
										<p>Receive updates via email</p>
									</div>
									<input type="checkbox" checked />
								</label>

								<label class="notification-item">
									<div class="notification-item__info">
										<h3>Trading Alerts</h3>
										<p>Get notified about trading opportunities</p>
									</div>
									<input type="checkbox" checked />
								</label>

								<label class="notification-item">
									<div class="notification-item__info">
										<h3>Marketing Emails</h3>
										<p>Receive promotional content</p>
									</div>
									<input type="checkbox" />
								</label>
							</div>
						</div>
					{/if}
				</div>
			</div>
		</div>
	</main>
{:else}
	<div class="loading-state">
		<p>Redirecting to login...</p>
	</div>
{/if}

<style>
	.dashboard-page {
		min-height: 100vh;
		background: var(--rtp-bg, #0a0f1a);
		color: var(--rtp-text, #e5e7eb);
		padding: 8rem 1.5rem 4rem;
	}

	.container {
		max-width: 1200px;
		margin: 0 auto;
	}

	.page-header {
		margin-bottom: 2rem;
	}

	.page-header__title {
		font-size: 2.5rem;
		font-weight: 700;
		color: #fff;
		margin-bottom: 0.5rem;
	}

	.page-header__subtitle {
		color: #94a3b8;
		font-size: 1.1rem;
	}

	.account-layout {
		display: grid;
		grid-template-columns: 280px 1fr;
		gap: 2rem;
	}

	@media (max-width: 900px) {
		.account-layout {
			grid-template-columns: 1fr;
		}
	}

	.account-sidebar {
		background: rgba(255, 255, 255, 0.02);
		border: 1px solid rgba(255, 255, 255, 0.08);
		border-radius: 1rem;
		padding: 1rem;
		height: fit-content;
		position: sticky;
		top: 140px;
	}

	.sidebar-nav {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		margin-bottom: 1rem;
		padding-bottom: 1rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.08);
	}

	.sidebar-nav__item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.875rem 1rem;
		background: transparent;
		border: none;
		border-radius: 8px;
		color: #94a3b8;
		font-size: 0.9rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
		text-align: left;
	}

	.sidebar-nav__item:hover {
		background: rgba(255, 255, 255, 0.05);
		color: #e5e7eb;
	}

	.sidebar-nav__item.active {
		background: rgba(250, 204, 21, 0.1);
		color: #facc15;
	}

	.logout-btn {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		width: 100%;
		padding: 0.875rem 1rem;
		background: transparent;
		border: none;
		border-radius: 8px;
		color: #ef4444;
		font-size: 0.9rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.logout-btn:hover {
		background: rgba(239, 68, 68, 0.1);
	}

	.account-content {
		background: rgba(255, 255, 255, 0.02);
		border: 1px solid rgba(255, 255, 255, 0.08);
		border-radius: 1rem;
		padding: 2rem;
	}

	.content-section {
		max-width: 600px;
	}

	.section-title {
		font-size: 1.5rem;
		font-weight: 600;
		color: #fff;
		margin-bottom: 0.5rem;
	}

	.section-description {
		color: #94a3b8;
		margin-bottom: 2rem;
	}

	.profile-form {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.form-group label {
		font-size: 0.875rem;
		font-weight: 500;
		color: #e5e7eb;
	}

	.form-group input {
		padding: 0.875rem 1rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 8px;
		color: #e5e7eb;
		font-size: 0.9rem;
		transition: all 0.2s ease;
	}

	.form-group input:focus {
		outline: none;
		border-color: #facc15;
		background: rgba(255, 255, 255, 0.08);
	}

	.form-group input::placeholder {
		color: #64748b;
	}

	.form-actions {
		padding-top: 1rem;
	}

	.save-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.875rem 2rem;
		background: linear-gradient(135deg, #facc15, #f97316);
		border: none;
		border-radius: 8px;
		color: #0a0f1a;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.save-btn:hover:not(:disabled) {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(250, 204, 21, 0.3);
	}

	.save-btn:disabled {
		opacity: 0.7;
		cursor: not-allowed;
	}

	.security-options {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.security-item {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.25rem;
		background: rgba(255, 255, 255, 0.02);
		border: 1px solid rgba(255, 255, 255, 0.08);
		border-radius: 12px;
		gap: 1rem;
		flex-wrap: wrap;
	}

	.security-item__info {
		display: flex;
		align-items: center;
		gap: 1rem;
		color: #64748b;
	}

	.security-item__info h3 {
		font-size: 1rem;
		font-weight: 600;
		color: #fff;
		margin-bottom: 0.25rem;
	}

	.security-item__info p {
		font-size: 0.8rem;
		color: #64748b;
	}

	.security-item__btn {
		padding: 0.625rem 1.25rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 8px;
		color: #e5e7eb;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.security-item__btn:hover {
		background: rgba(255, 255, 255, 0.1);
	}

	.billing-card {
		padding: 1.5rem;
		background: rgba(250, 204, 21, 0.05);
		border: 1px solid rgba(250, 204, 21, 0.2);
		border-radius: 12px;
	}

	.billing-card__header {
		display: flex;
		align-items: center;
		gap: 1rem;
		margin-bottom: 0.75rem;
	}

	.billing-card__header h3 {
		font-size: 1.125rem;
		font-weight: 600;
		color: #fff;
	}

	.plan-badge {
		padding: 0.25rem 0.75rem;
		background: linear-gradient(135deg, #facc15, #f97316);
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 600;
		color: #0a0f1a;
	}

	.billing-card__text {
		font-size: 0.9rem;
		color: #94a3b8;
		margin-bottom: 1.25rem;
	}

	.billing-card__actions {
		display: flex;
		gap: 0.75rem;
		flex-wrap: wrap;
	}

	.billing-btn {
		padding: 0.625rem 1.25rem;
		background: linear-gradient(135deg, #facc15, #f97316);
		border: none;
		border-radius: 8px;
		color: #0a0f1a;
		font-size: 0.875rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.billing-btn--secondary {
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		color: #e5e7eb;
	}

	.notification-options {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.notification-item {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.25rem;
		background: rgba(255, 255, 255, 0.02);
		border: 1px solid rgba(255, 255, 255, 0.08);
		border-radius: 12px;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.notification-item:hover {
		border-color: rgba(255, 255, 255, 0.15);
	}

	.notification-item__info h3 {
		font-size: 1rem;
		font-weight: 600;
		color: #fff;
		margin-bottom: 0.25rem;
	}

	.notification-item__info p {
		font-size: 0.8rem;
		color: #64748b;
	}

	.notification-item input[type="checkbox"] {
		width: 20px;
		height: 20px;
		accent-color: #facc15;
		cursor: pointer;
	}

	.loading-state {
		min-height: 100vh;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--rtp-bg, #0a0f1a);
		color: #94a3b8;
	}

	@media (max-width: 768px) {
		.dashboard-page {
			padding: 7rem 1rem 3rem;
		}

		.page-header__title {
			font-size: 2rem;
		}

		.account-sidebar {
			position: static;
		}

		.sidebar-nav {
			flex-direction: row;
			overflow-x: auto;
			padding-bottom: 0.5rem;
		}

		.sidebar-nav__item {
			white-space: nowrap;
		}

		.account-content {
			padding: 1.5rem;
		}
	}
</style>
