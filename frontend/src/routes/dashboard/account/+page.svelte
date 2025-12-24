<script lang="ts">
	/**
	 * Account Settings Page - Member Dashboard
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * User account management, profile settings, and subscription overview
	 *
	 * @version 1.0.0
	 */
	import { onMount } from 'svelte';
	import { user } from '$lib/stores/auth';
	import { getUserMemberships, type UserMembershipsResponse } from '$lib/api/user-memberships';
	import IconUser from '@tabler/icons-svelte/icons/user';
	import IconMail from '@tabler/icons-svelte/icons/mail';
	import IconCreditCard from '@tabler/icons-svelte/icons/credit-card';
	import IconShield from '@tabler/icons-svelte/icons/shield';
	import IconBell from '@tabler/icons-svelte/icons/bell';

	let loading = $state(true);
	let membershipsData = $state<UserMembershipsResponse | null>(null);

	onMount(async () => {
		try {
			membershipsData = await getUserMemberships();
		} catch (err) {
			console.error('Failed to load memberships:', err);
		} finally {
			loading = false;
		}
	});
</script>

<svelte:head>
	<title>My Account | Revolution Trading Pros</title>
</svelte:head>

<div class="account-page">
	<!-- Page Header -->
	<header class="page-header">
		<h1 class="page-title">My Account</h1>
		<p class="page-subtitle">Manage your profile, subscriptions, and preferences</p>
	</header>

	<div class="account-grid">
		<!-- Profile Section -->
		<section class="account-section">
			<div class="section-header">
				<IconUser size={24} />
				<h2>Profile Information</h2>
			</div>
			<div class="section-content">
				<div class="profile-info">
					<div class="profile-avatar">
						<span class="avatar-placeholder">{$user?.name?.charAt(0) || 'M'}</span>
					</div>
					<div class="profile-details">
						<div class="detail-row">
							<span class="label">Name</span>
							<span class="value">{$user?.name || 'Member'}</span>
						</div>
						<div class="detail-row">
							<span class="label">Email</span>
							<span class="value">{$user?.email || 'Not available'}</span>
						</div>
						<div class="detail-row">
							<span class="label">Member Since</span>
							<span class="value">{new Date().toLocaleDateString('en-US', { year: 'numeric', month: 'long' })}</span>
						</div>
					</div>
				</div>
				<button class="btn btn-primary">Edit Profile</button>
			</div>
		</section>

		<!-- Subscriptions Section -->
		<section class="account-section">
			<div class="section-header">
				<IconCreditCard size={24} />
				<h2>Active Subscriptions</h2>
			</div>
			<div class="section-content">
				{#if loading}
					<p class="loading-text">Loading subscriptions...</p>
				{:else if membershipsData?.stats?.totalActive}
					<div class="subscription-summary">
						<div class="stat-item">
							<span class="stat-value">{membershipsData.stats.totalActive}</span>
							<span class="stat-label">Active Memberships</span>
						</div>
						<div class="stat-item">
							<span class="stat-value">${membershipsData.stats.totalValue.toFixed(2)}</span>
							<span class="stat-label">Monthly Value</span>
						</div>
					</div>
					<a href="/dashboard" class="btn btn-secondary">View All Memberships</a>
				{:else}
					<p class="empty-text">No active subscriptions</p>
					<a href="/pricing" class="btn btn-primary">Browse Plans</a>
				{/if}
			</div>
		</section>

		<!-- Security Section -->
		<section class="account-section">
			<div class="section-header">
				<IconShield size={24} />
				<h2>Security</h2>
			</div>
			<div class="section-content">
				<div class="security-options">
					<button class="option-button">
						<span class="option-label">Change Password</span>
						<span class="option-arrow">→</span>
					</button>
					<button class="option-button">
						<span class="option-label">Two-Factor Authentication</span>
						<span class="option-arrow">→</span>
					</button>
				</div>
			</div>
		</section>

		<!-- Notifications Section -->
		<section class="account-section">
			<div class="section-header">
				<IconBell size={24} />
				<h2>Notifications</h2>
			</div>
			<div class="section-content">
				<div class="notification-options">
					<label class="checkbox-option">
						<input type="checkbox" checked />
						<span>Trade Alerts</span>
					</label>
					<label class="checkbox-option">
						<input type="checkbox" checked />
						<span>Trading Room Sessions</span>
					</label>
					<label class="checkbox-option">
						<input type="checkbox" checked />
						<span>New Course Content</span>
					</label>
					<label class="checkbox-option">
						<input type="checkbox" />
						<span>Marketing Emails</span>
					</label>
				</div>
			</div>
		</section>
	</div>
</div>

<style>
	.account-page {
		padding: 30px;
		max-width: 1200px;
		margin: 0 auto;
	}

	.page-header {
		margin-bottom: 32px;
	}

	.page-title {
		font-size: 28px;
		font-weight: 600;
		color: #333;
		margin: 0 0 8px;
	}

	.page-subtitle {
		font-size: 14px;
		color: #666;
		margin: 0;
	}

	.account-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
		gap: 24px;
	}

	.account-section {
		background: #fff;
		border-radius: 8px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
		overflow: hidden;
	}

	.section-header {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 20px;
		border-bottom: 1px solid #e5e7eb;
		background: #f9fafb;
	}

	.section-header h2 {
		font-size: 18px;
		font-weight: 600;
		color: #333;
		margin: 0;
	}

	.section-content {
		padding: 20px;
	}

	/* Profile */
	.profile-info {
		display: flex;
		gap: 20px;
		margin-bottom: 20px;
	}

	.profile-avatar {
		flex-shrink: 0;
	}

	.avatar-placeholder {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 80px;
		height: 80px;
		border-radius: 50%;
		background: linear-gradient(135deg, #0984ae 0%, #076787 100%);
		color: #fff;
		font-size: 32px;
		font-weight: 600;
	}

	.profile-details {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.detail-row {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 8px 0;
		border-bottom: 1px solid #f3f4f6;
	}

	.detail-row:last-child {
		border-bottom: none;
	}

	.label {
		font-size: 14px;
		color: #6b7280;
		font-weight: 500;
	}

	.value {
		font-size: 14px;
		color: #333;
		font-weight: 600;
	}

	/* Subscriptions */
	.subscription-summary {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 16px;
		margin-bottom: 20px;
	}

	.stat-item {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 16px;
		background: #f9fafb;
		border-radius: 8px;
	}

	.stat-value {
		font-size: 24px;
		font-weight: 700;
		color: #0984ae;
		margin-bottom: 4px;
	}

	.stat-label {
		font-size: 12px;
		color: #6b7280;
		text-align: center;
	}

	/* Security */
	.security-options {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.option-button {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 12px 16px;
		background: #f9fafb;
		border: 1px solid #e5e7eb;
		border-radius: 6px;
		cursor: pointer;
		transition: all 0.15s;
		font-size: 14px;
		color: #333;
	}

	.option-button:hover {
		background: #f3f4f6;
		border-color: #0984ae;
	}

	.option-arrow {
		color: #0984ae;
		font-weight: 600;
	}

	/* Notifications */
	.notification-options {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.checkbox-option {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 8px 0;
		cursor: pointer;
		font-size: 14px;
		color: #333;
	}

	.checkbox-option input[type='checkbox'] {
		width: 18px;
		height: 18px;
		cursor: pointer;
	}

	/* Buttons */
	.btn {
		display: inline-block;
		padding: 10px 20px;
		border-radius: 6px;
		font-size: 14px;
		font-weight: 600;
		text-decoration: none;
		text-align: center;
		cursor: pointer;
		transition: all 0.15s;
		border: none;
	}

	.btn-primary {
		background: #0984ae;
		color: #fff;
	}

	.btn-primary:hover {
		background: #076787;
	}

	.btn-secondary {
		background: #e5e7eb;
		color: #374151;
	}

	.btn-secondary:hover {
		background: #d1d5db;
	}

	.loading-text,
	.empty-text {
		text-align: center;
		color: #6b7280;
		padding: 20px;
		font-size: 14px;
	}

	@media (max-width: 768px) {
		.account-page {
			padding: 20px;
		}

		.account-grid {
			grid-template-columns: 1fr;
		}

		.profile-info {
			flex-direction: column;
			align-items: center;
			text-align: center;
		}

		.detail-row {
			flex-direction: column;
			gap: 4px;
		}

		.subscription-summary {
			grid-template-columns: 1fr;
		}
	}
</style>
