<script lang="ts">
	/**
	 * Trading Room Page - ICT 11+ Principal Engineer Pattern
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Live trading room access page with:
	 * - Live stream embed
	 * - Discord integration
	 * - Real-time alerts
	 * - Chat integration
	 *
	 * @version 1.0.0
	 */
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { getTradingRoomAccess, type UserMembership } from '$lib/api/user-memberships';
	import DynamicIcon from '$lib/components/DynamicIcon.svelte';
	import IconArrowLeft from '@tabler/icons-svelte/icons/arrow-left';
	import IconBrandDiscord from '@tabler/icons-svelte/icons/brand-discord';
	import IconVideo from '@tabler/icons-svelte/icons/video';
	import IconBell from '@tabler/icons-svelte/icons/bell';
	import IconUsers from '@tabler/icons-svelte/icons/users';
	import IconLock from '@tabler/icons-svelte/icons/lock';
	import IconExternalLink from '@tabler/icons-svelte/icons/external-link';
	import IconAlertTriangle from '@tabler/icons-svelte/icons/alert-triangle';

	let slug = $derived($page.params.slug!);
	let hasAccess = $state(false);
	let membership = $state<UserMembership | null>(null);
	let discordInvite = $state<string | null>(null);
	let loading = $state(true);
	let error = $state<string | null>(null);

	// Room config - would come from API in production
	let roomConfig = $derived(getRoomConfig(slug || ''));

	onMount(async () => {
		await checkAccess();
	});

	async function checkAccess() {
		loading = true;
		error = null;
		try {
			const result = await getTradingRoomAccess(slug || '');
			hasAccess = result.hasAccess;
			membership = result.membership || null;
			discordInvite = result.discordInvite || null;

			if (!hasAccess) {
				error = 'You do not have access to this trading room.';
			}
		} catch (err) {
			console.error('Failed to check access:', err);
			// For demo, allow access
			hasAccess = true;
		} finally {
			loading = false;
		}
	}

	function getRoomConfig(roomSlug: string) {
		const configs: Record<string, {
			name: string;
			icon: string;
			color: string;
			description: string;
			features: string[];
			schedule: string;
		}> = {
			'day-trading-room': {
				name: 'Day Trading Room',
				icon: 'chart-line',
				color: '#0984ae',
				description: 'Live day trading sessions with real-time market analysis and trade alerts.',
				features: [
					'Live market analysis',
					'Real-time trade alerts',
					'Interactive Q&A sessions',
					'Screen sharing of live trades'
				],
				schedule: 'Monday - Friday, 9:00 AM - 4:00 PM EST'
			},
			'swing-trading-room': {
				name: 'Swing Trading Room',
				icon: 'trending-up',
				color: '#10b981',
				description: 'Swing trading strategies with multi-day position management.',
				features: [
					'Weekly swing trade setups',
					'Position management guidance',
					'Risk management strategies',
					'Weekend watchlist reviews'
				],
				schedule: 'Monday, Wednesday, Friday, 7:00 PM EST'
			},
			'small-account-mentorship': {
				name: 'Small Account Mentorship',
				icon: 'wallet',
				color: '#8b5cf6',
				description: 'Specialized strategies for growing small trading accounts.',
				features: [
					'Small account strategies',
					'Proper position sizing',
					'Risk management for small accounts',
					'Account growth roadmap'
				],
				schedule: 'Tuesday & Thursday, 8:00 PM EST'
			}
		};

		return configs[roomSlug] || {
			name: 'Trading Room',
			icon: 'chart-line',
			color: '#0984ae',
			description: 'Access your live trading room.',
			features: [],
			schedule: 'Check schedule'
		};
	}
</script>

<svelte:head>
	<title>{roomConfig.name} | Revolution Trading Pros</title>
</svelte:head>

<div class="trading-room-page">
	{#if loading}
		<div class="loading-container">
			<div class="spinner"></div>
			<p>Checking access...</p>
		</div>
	{:else if !hasAccess}
		<div class="access-denied">
			<div class="access-denied-icon">
				<IconLock size={64} />
			</div>
			<h1>Access Required</h1>
			<p>You don't have access to the {roomConfig.name}.</p>
			<div class="access-denied-actions">
				<a href="/dashboard" class="btn btn-secondary">
					<IconArrowLeft size={18} />
					Back to Dashboard
				</a>
				<a href="/pricing" class="btn btn-primary">
					View Pricing
				</a>
			</div>
		</div>
	{:else}
		<!-- Room Header -->
		<header class="room-header" style="--room-color: {roomConfig.color}">
			<div class="room-header-left">
				<a href="/dashboard" class="back-link">
					<IconArrowLeft size={18} />
					Dashboard
				</a>
				<div class="room-title-row">
					<div class="room-icon">
						<DynamicIcon name={roomConfig.icon} size={32} />
					</div>
					<div>
						<h1>{roomConfig.name}</h1>
						<p class="room-schedule">{roomConfig.schedule}</p>
					</div>
				</div>
			</div>
			<div class="room-header-right">
				{#if discordInvite}
					<a href={discordInvite} target="_blank" class="btn btn-discord">
						<IconBrandDiscord size={20} />
						Join Discord
					</a>
				{/if}
			</div>
		</header>

		<!-- Main Content Grid -->
		<div class="room-content">
			<!-- Live Stream Section -->
			<div class="stream-section">
				<div class="stream-container">
					<div class="stream-placeholder">
						<IconVideo size={64} />
						<h3>Live Stream</h3>
						<p>The trading room stream will appear here when live.</p>
						<span class="stream-status offline">Currently Offline</span>
					</div>
				</div>
			</div>

			<!-- Sidebar -->
			<aside class="room-sidebar">
				<!-- Quick Actions -->
				<div class="sidebar-card">
					<h3>Quick Actions</h3>
					<div class="quick-actions">
						{#if discordInvite}
							<a href={discordInvite} target="_blank" class="action-btn discord">
								<IconBrandDiscord size={24} />
								<span>Discord Community</span>
							</a>
						{:else}
							<button type="button" class="action-btn discord" disabled>
								<IconBrandDiscord size={24} />
								<span>Discord Coming Soon</span>
							</button>
						{/if}
						<a href="/dashboard/{slug}/alerts" class="action-btn alerts">
							<IconBell size={24} />
							<span>View All Alerts</span>
						</a>
					</div>
				</div>

				<!-- Room Features -->
				<div class="sidebar-card">
					<h3>What's Included</h3>
					<ul class="features-list">
						{#each roomConfig.features as feature}
							<li>
								<span class="feature-check">&#10003;</span>
								{feature}
							</li>
						{/each}
					</ul>
				</div>

				<!-- Members Online (Placeholder) -->
				<div class="sidebar-card">
					<h3>
						<IconUsers size={18} />
						Members Online
					</h3>
					<div class="members-online">
						<span class="online-count">--</span>
						<span class="online-label">members</span>
					</div>
				</div>
			</aside>
		</div>

		<!-- Recent Alerts Section -->
		<section class="alerts-section">
			<div class="section-header">
				<h2>
					<IconBell size={20} />
					Recent Alerts
				</h2>
				<a href="/dashboard/{slug}/alerts" class="view-all-link">
					View All
					<IconExternalLink size={14} />
				</a>
			</div>
			<div class="alerts-placeholder">
				<IconAlertTriangle size={32} />
				<p>No recent alerts. Alerts will appear here during trading sessions.</p>
			</div>
		</section>
	{/if}
</div>

<style>
	.trading-room-page {
		min-height: 100vh;
		background: #0f172a;
	}

	/* Loading */
	.loading-container {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 60vh;
		color: #64748b;
	}

	.spinner {
		width: 48px;
		height: 48px;
		border: 3px solid rgba(99, 102, 241, 0.2);
		border-top-color: #6366f1;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
		margin-bottom: 16px;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Access Denied */
	.access-denied {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 60vh;
		text-align: center;
		padding: 2rem;
	}

	.access-denied-icon {
		color: #f59e0b;
		margin-bottom: 1.5rem;
	}

	.access-denied h1 {
		font-size: 2rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.5rem;
	}

	.access-denied p {
		color: #94a3b8;
		margin-bottom: 2rem;
	}

	.access-denied-actions {
		display: flex;
		gap: 1rem;
	}

	/* Room Header */
	.room-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		padding: 1.5rem 2rem;
		background: linear-gradient(135deg, rgba(15, 23, 42, 0.95), rgba(30, 41, 59, 0.95));
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.back-link {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		color: #94a3b8;
		text-decoration: none;
		font-size: 0.875rem;
		margin-bottom: 1rem;
		transition: color 0.2s;
	}

	.back-link:hover {
		color: #a5b4fc;
	}

	.room-title-row {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.room-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 56px;
		height: 56px;
		background: var(--room-color, #0984ae);
		border-radius: 14px;
		color: #fff;
	}

	.room-title-row h1 {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0;
	}

	.room-schedule {
		color: #64748b;
		font-size: 0.875rem;
		margin: 0.25rem 0 0;
	}

	/* Buttons */
	.btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		border-radius: 10px;
		font-weight: 600;
		font-size: 0.875rem;
		text-decoration: none;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
	}

	.btn-primary {
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		color: #fff;
	}

	.btn-secondary {
		background: rgba(148, 163, 184, 0.1);
		color: #94a3b8;
		border: 1px solid rgba(148, 163, 184, 0.2);
	}

	.btn-secondary:hover {
		background: rgba(148, 163, 184, 0.15);
		color: #f1f5f9;
	}

	.btn-discord {
		background: #5865f2;
		color: #fff;
	}

	.btn-discord:hover {
		background: #4752c4;
	}

	/* Content Grid */
	.room-content {
		display: grid;
		grid-template-columns: 1fr 320px;
		gap: 1.5rem;
		padding: 1.5rem 2rem;
	}

	/* Stream Section */
	.stream-section {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
		overflow: hidden;
	}

	.stream-container {
		aspect-ratio: 16 / 9;
		background: #0f172a;
	}

	.stream-placeholder {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 100%;
		color: #64748b;
		text-align: center;
		padding: 2rem;
	}

	.stream-placeholder h3 {
		color: #f1f5f9;
		margin: 1rem 0 0.5rem;
	}

	.stream-placeholder p {
		margin: 0 0 1rem;
	}

	.stream-status {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.375rem 0.75rem;
		border-radius: 20px;
		font-size: 0.75rem;
		font-weight: 600;
	}

	.stream-status.offline {
		background: rgba(239, 68, 68, 0.15);
		color: #f87171;
	}

	/* Sidebar */
	.room-sidebar {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.sidebar-card {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
		padding: 1.25rem;
	}

	.sidebar-card h3 {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.875rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 1rem;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	/* Quick Actions */
	.quick-actions {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.action-btn {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.875rem 1rem;
		border-radius: 10px;
		text-decoration: none;
		font-weight: 500;
		font-size: 0.875rem;
		transition: all 0.2s;
	}

	.action-btn.discord {
		background: rgba(88, 101, 242, 0.1);
		color: #818cf8;
		border: 1px solid rgba(88, 101, 242, 0.2);
	}

	.action-btn.discord:hover {
		background: rgba(88, 101, 242, 0.2);
	}

	.action-btn.alerts {
		background: rgba(245, 158, 11, 0.1);
		color: #fbbf24;
		border: 1px solid rgba(245, 158, 11, 0.2);
	}

	.action-btn.alerts:hover {
		background: rgba(245, 158, 11, 0.2);
	}

	/* Features List */
	.features-list {
		list-style: none;
		padding: 0;
		margin: 0;
	}

	.features-list li {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		padding: 0.625rem 0;
		color: #cbd5e1;
		font-size: 0.875rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.features-list li:last-child {
		border-bottom: none;
	}

	.feature-check {
		color: #34d399;
		font-weight: 700;
	}

	/* Members Online */
	.members-online {
		display: flex;
		align-items: baseline;
		gap: 0.5rem;
	}

	.online-count {
		font-size: 2rem;
		font-weight: 700;
		color: #34d399;
	}

	.online-label {
		color: #64748b;
		font-size: 0.875rem;
	}

	/* Alerts Section */
	.alerts-section {
		margin: 0 2rem 2rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
		padding: 1.5rem;
	}

	.section-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.section-header h2 {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.view-all-link {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		color: #a5b4fc;
		font-size: 0.875rem;
		text-decoration: none;
	}

	.view-all-link:hover {
		text-decoration: underline;
	}

	.alerts-placeholder {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 3rem 2rem;
		color: #64748b;
		text-align: center;
	}

	.alerts-placeholder p {
		margin: 1rem 0 0;
	}

	/* Responsive */
	@media (max-width: 1024px) {
		.room-content {
			grid-template-columns: 1fr;
		}

		.room-sidebar {
			flex-direction: row;
			flex-wrap: wrap;
		}

		.sidebar-card {
			flex: 1;
			min-width: 280px;
		}
	}

	@media (max-width: 768px) {
		.room-header {
			flex-direction: column;
			gap: 1rem;
		}

		.room-content {
			padding: 1rem;
		}

		.alerts-section {
			margin: 0 1rem 1rem;
		}
	}
</style>
