<script lang="ts">
	import { onMount } from 'svelte';
	import { gsap } from 'gsap';
	import {
		IconReceipt,
		IconTicket,
		IconUsers,
		IconTrendingUp,
		IconCurrencyDollar,
		IconChartBar,
		IconArrowRight,
		IconRefresh
	} from '@tabler/icons-svelte';
	import { EnterpriseStatsGrid, SkeletonLoader } from '$lib/components/ui';
	import { browser } from '$app/environment';

	let statsRef: HTMLDivElement;
	let isLoading = true;
	let lastUpdated: Date | null = null;

	// Real stats data - fetched from API
	let stats = {
		activeSubscriptions: 0,
		monthlyRevenue: 0,
		activeCoupons: 0,
		totalUsers: 0,
		newUsersThisWeek: 0,
		conversionRate: 0
	};

	// Trend data (percentage change vs last period)
	let trends = {
		subscriptions: 12.5,
		revenue: 8.2,
		coupons: 0,
		users: 15.3
	};

	// Sparkline data for mini charts
	let sparklines = {
		subscriptions: [180, 195, 210, 225, 232, 240, 247],
		revenue: [35000, 38000, 42000, 45000, 47000, 48000, 48930],
		users: [420, 450, 470, 490, 510, 520, 532]
	};

	// Enterprise stats configuration
	$: enterpriseStats = [
		{
			id: 'subscriptions',
			title: 'Active Subscriptions',
			value: stats.activeSubscriptions,
			format: 'number' as const,
			trend: trends.subscriptions,
			trendLabel: 'vs last month',
			icon: IconReceipt,
			color: 'blue' as const,
			sparklineData: sparklines.subscriptions,
			target: 300,
			targetLabel: 'Q4 Goal'
		},
		{
			id: 'revenue',
			title: 'Monthly Revenue',
			value: stats.monthlyRevenue,
			format: 'currency' as const,
			decimals: 0,
			trend: trends.revenue,
			trendLabel: 'vs last month',
			icon: IconCurrencyDollar,
			color: 'green' as const,
			sparklineData: sparklines.revenue,
			target: 60000,
			targetLabel: 'Monthly Goal'
		},
		{
			id: 'coupons',
			title: 'Active Coupons',
			value: stats.activeCoupons,
			format: 'number' as const,
			trend: null,
			icon: IconTicket,
			color: 'purple' as const
		},
		{
			id: 'users',
			title: 'Total Users',
			value: stats.totalUsers,
			format: 'number' as const,
			trend: trends.users,
			trendLabel: 'vs last month',
			icon: IconUsers,
			color: 'orange' as const,
			sparklineData: sparklines.users,
			target: 1000,
			targetLabel: 'Year Goal'
		}
	];

	async function fetchDashboardStats() {
		isLoading = true;
		try {
			// Simulate API call - replace with real endpoint
			await new Promise((resolve) => setTimeout(resolve, 800));

			// In production, fetch from API:
			// const response = await fetch('/api/admin/stats');
			// const data = await response.json();

			stats = {
				activeSubscriptions: 247,
				monthlyRevenue: 48930,
				activeCoupons: 12,
				totalUsers: 532,
				newUsersThisWeek: 24,
				conversionRate: 3.8
			};

			lastUpdated = new Date();
		} catch (error) {
			console.error('Failed to fetch dashboard stats:', error);
		} finally {
			isLoading = false;
		}
	}

	function handleStatClick(stat: any) {
		// Navigate to detailed view
		const routes: Record<string, string> = {
			subscriptions: '/admin/subscriptions',
			revenue: '/admin/analytics',
			coupons: '/admin/coupons',
			users: '/admin/users'
		};
		if (browser && routes[stat.id]) {
			window.location.href = routes[stat.id];
		}
	}

	onMount(() => {
		fetchDashboardStats();

		// Animate welcome section
		gsap.from('.welcome-section', {
			opacity: 0,
			y: -30,
			duration: 0.8,
			ease: 'power3.out'
		});

		// Animate quick actions with stagger
		gsap.from('.action-card', {
			opacity: 0,
			y: 40,
			scale: 0.95,
			duration: 0.6,
			stagger: 0.1,
			delay: 0.5,
			ease: 'power3.out'
		});
	});
</script>

<div class="dashboard-container">
	<!-- Welcome Section -->
	<div class="welcome-section">
		<div class="flex items-center justify-between flex-wrap gap-4">
			<div>
				<h2 class="welcome-title">Welcome back, Admin</h2>
				<p class="welcome-subtitle">Here's what's happening with your business today.</p>
			</div>
			<div class="flex items-center gap-4">
				{#if lastUpdated}
					<span class="text-sm text-slate-500">
						Last updated: {lastUpdated.toLocaleTimeString()}
					</span>
				{/if}
				<button
					on:click={fetchDashboardStats}
					class="flex items-center gap-2 px-4 py-2 bg-indigo-600/20 hover:bg-indigo-600/30 border border-indigo-500/30 rounded-lg text-indigo-400 transition-all duration-200"
					class:animate-spin={isLoading}
					disabled={isLoading}
				>
					<IconRefresh size={18} class={isLoading ? 'animate-spin' : ''} />
					<span class="hidden sm:inline">Refresh</span>
				</button>
			</div>
		</div>
	</div>

	<!-- Enterprise Stats Grid with Animated KPIs -->
	<div class="stats-section mb-8" bind:this={statsRef}>
		<EnterpriseStatsGrid
			stats={enterpriseStats}
			loading={isLoading}
			columns={4}
			staggerDelay={0.15}
			onStatClick={handleStatClick}
		/>
	</div>

	<!-- Quick Actions -->
	<div class="quick-actions">
		<h3 class="section-title">Quick Actions</h3>
		<div class="actions-grid">
			<a href="/admin/coupons" class="action-card">
				<IconTicket size={24} />
				<span>Create Coupon</span>
			</a>
			<a href="/admin/subscriptions" class="action-card">
				<IconReceipt size={24} />
				<span>View Subscriptions</span>
			</a>
			<a href="/admin/users" class="action-card">
				<IconUsers size={24} />
				<span>Manage Users</span>
			</a>
		</div>
	</div>
</div>

<style>
	.dashboard-container {
		max-width: 1400px;
	}

	.welcome-section {
		margin-bottom: 2rem;
	}

	.welcome-title {
		font-size: 2rem;
		font-weight: 700;
		color: #f1f5f9;
		margin-bottom: 0.5rem;
	}

	.welcome-subtitle {
		font-size: 1.125rem;
		color: #64748b;
	}

	/* Stats Section */
	.stats-section {
		margin-bottom: 2rem;
	}

	/* Quick Actions */
	.quick-actions {
		margin-top: 2rem;
	}

	.section-title {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
		margin-bottom: 1.5rem;
	}

	.actions-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 1.5rem;
	}

	.action-card {
		background: linear-gradient(135deg, rgba(99, 102, 241, 0.1), rgba(139, 92, 246, 0.05));
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 12px;
		padding: 1.5rem;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.75rem;
		text-decoration: none;
		color: #a5b4fc;
		font-weight: 600;
		transition: all 0.3s ease;
		opacity: 0; /* For GSAP animation */
	}

	.action-card:hover {
		background: linear-gradient(135deg, rgba(99, 102, 241, 0.2), rgba(139, 92, 246, 0.1));
		border-color: rgba(99, 102, 241, 0.4);
		transform: translateY(-2px);
	}

	/* Refresh button animation */
	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	.animate-spin {
		animation: spin 1s linear infinite;
	}

	/* Responsive */
	@media (max-width: 640px) {
		.welcome-title {
			font-size: 1.5rem;
		}
	}
</style>
