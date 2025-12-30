<script lang="ts">
	/**
	 * Dashboard Home Page - Member Dashboard Landing
	 *
	 * Svelte 5 Best Practices:
	 * - SSR-safe rendering
	 * - $props() for page data
	 * - Component composition
	 *
	 * Matches Simpler Trading core file:
	 * - Header with "Member Dashboard" title
	 * - Trading room dropdown
	 * - Memberships section with cards
	 * - Tools section
	 */

	import { DashboardHeader, MembershipCard } from '$lib/components/dashboard';

	// Props from layout
	interface Props {
		data: {
			user: {
				id: number;
				name: string;
				email: string;
			};
		};
	}

	let { data }: Props = $props();

	// Mock memberships - in production, fetch from API
	const memberships = [
		{
			name: 'Mastering the Trade',
			slug: 'mastering-the-trade',
			tradingRoomUrl: '#' // Would be JWT-signed URL in production
		},
		{
			name: 'Simpler Showcase',
			slug: 'simpler-showcase',
			tradingRoomUrl: '#'
		}
	];

	// Trading rooms for dropdown
	const tradingRooms = [
		{ name: 'Mastering The Trade Room', href: '#' },
		{ name: 'Simpler Showcase Room', href: '#' }
	];
</script>

<svelte:head>
	<title>Member Dashboard | Revolution Trading Pros</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<!-- Dashboard Header -->
<DashboardHeader
	title="Member Dashboard"
	{tradingRooms}
/>

<!-- Dashboard Content -->
<div class="dashboard__content">
	<div class="dashboard__content-main">
		<!-- Memberships Section -->
		<section class="dashboard__content-section">
			<h2 class="section-title">Memberships</h2>
			<div class="membership-cards">
				{#each memberships as membership}
					<div class="membership-card-wrapper">
						<MembershipCard
							name={membership.name}
							slug={membership.slug}
							tradingRoomUrl={membership.tradingRoomUrl}
						/>
					</div>
				{/each}
			</div>
		</section>

		<!-- Tools Section -->
		<section class="dashboard__content-section">
			<h2 class="section-title">Tools</h2>
			<div class="tools-grid">
				<a href="/dashboard/ww" class="tool-card">
					<div class="tool-card__icon">
						<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<path d="M3 3v18h18" />
							<path d="M18 17V9" />
							<path d="M13 17V5" />
							<path d="M8 17v-3" />
						</svg>
					</div>
					<div class="tool-card__content">
						<h3>Weekly Watchlist</h3>
						<p>View this week's top trading opportunities</p>
					</div>
				</a>

				<a href="/dashboard/latest-updates" class="tool-card">
					<div class="tool-card__icon">
						<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<path d="M12 8v4l3 3" />
							<circle cx="12" cy="12" r="9" />
						</svg>
					</div>
					<div class="tool-card__content">
						<h3>Latest Updates</h3>
						<p>Stay current with the newest content</p>
					</div>
				</a>

				<a href="/dashboard/platform-tutorials" class="tool-card">
					<div class="tool-card__icon">
						<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<rect x="2" y="3" width="20" height="14" rx="2" />
							<path d="M8 21h8" />
							<path d="M12 17v4" />
						</svg>
					</div>
					<div class="tool-card__content">
						<h3>Platform Tutorials</h3>
						<p>Learn how to use your trading platforms</p>
					</div>
				</a>
			</div>
		</section>
	</div>
</div>

<style>
	.membership-card-wrapper {
		/* Grid handled by parent .membership-cards in dashboard.css */
	}

	/* Tools Grid */
	.tools-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 20px;
	}

	.tool-card {
		display: flex;
		align-items: flex-start;
		gap: 16px;
		padding: 20px;
		background: linear-gradient(135deg, rgba(255, 255, 255, 0.05) 0%, rgba(255, 255, 255, 0.02) 100%);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: var(--dashboard-border-radius);
		text-decoration: none;
		color: var(--dashboard-text-primary);
		transition: transform var(--dashboard-transition), box-shadow var(--dashboard-transition), border-color var(--dashboard-transition);
	}

	.tool-card:hover {
		transform: translateY(-2px);
		box-shadow: var(--dashboard-shadow);
		border-color: rgba(255, 255, 255, 0.2);
	}

	.tool-card__icon {
		width: 48px;
		height: 48px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--dashboard-accent);
		border-radius: 8px;
		flex-shrink: 0;
	}

	.tool-card__icon svg {
		color: white;
	}

	.tool-card__content h3 {
		margin: 0 0 4px;
		font-size: 16px;
		font-weight: 600;
	}

	.tool-card__content p {
		margin: 0;
		font-size: 13px;
		color: var(--dashboard-text-secondary);
	}

	@media (max-width: 576px) {
		.tools-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
