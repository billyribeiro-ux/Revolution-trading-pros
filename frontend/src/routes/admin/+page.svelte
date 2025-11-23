<script lang="ts">
	import { onMount } from 'svelte';
	import { gsap } from 'gsap';
	import {
		IconReceipt,
		IconTicket,
		IconUsers,
		IconTrendingUp,
		IconCurrencyDollar
	} from '@tabler/icons-svelte';

	let statsRef: HTMLDivElement;

	// Mock data - replace with real API calls
	let stats = {
		activeSubscriptions: 247,
		monthlyRevenue: 48930,
		activeCoupons: 12,
		totalUsers: 532
	};

	onMount(() => {
		const tl = gsap.timeline();
		tl.from('.stat-card', {
			opacity: 0,
			y: 30,
			duration: 0.6,
			stagger: 0.1,
			ease: 'power3.out'
		});
	});

	function formatCurrency(amount: number): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD',
			minimumFractionDigits: 0
		}).format(amount);
	}
</script>

<div class="dashboard-container">
	<!-- Welcome Section -->
	<div class="welcome-section">
		<h2 class="welcome-title">Welcome back, Admin</h2>
		<p class="welcome-subtitle">Here's what's happening with your business today.</p>
	</div>

	<!-- Stats Grid -->
	<div class="stats-grid" bind:this={statsRef}>
		<!-- Active Subscriptions -->
		<div class="stat-card">
			<div class="stat-icon subscription">
				<IconReceipt size={28} />
			</div>
			<div class="stat-content">
				<div class="stat-label">Active Subscriptions</div>
				<div class="stat-value">{stats.activeSubscriptions}</div>
				<div class="stat-change positive">+12.5% from last month</div>
			</div>
		</div>

		<!-- Monthly Revenue -->
		<div class="stat-card">
			<div class="stat-icon revenue">
				<IconCurrencyDollar size={28} />
			</div>
			<div class="stat-content">
				<div class="stat-label">Monthly Revenue</div>
				<div class="stat-value">{formatCurrency(stats.monthlyRevenue)}</div>
				<div class="stat-change positive">+8.2% from last month</div>
			</div>
		</div>

		<!-- Active Coupons -->
		<div class="stat-card">
			<div class="stat-icon coupons">
				<IconTicket size={28} />
			</div>
			<div class="stat-content">
				<div class="stat-label">Active Coupons</div>
				<div class="stat-value">{stats.activeCoupons}</div>
				<div class="stat-change neutral">Available for use</div>
			</div>
		</div>

		<!-- Total Users -->
		<div class="stat-card">
			<div class="stat-icon users">
				<IconUsers size={28} />
			</div>
			<div class="stat-content">
				<div class="stat-label">Total Users</div>
				<div class="stat-value">{stats.totalUsers}</div>
				<div class="stat-change positive">+24 this week</div>
			</div>
		</div>
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

	/* Stats Grid */
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
		gap: 1.5rem;
		margin-bottom: 3rem;
	}

	.stat-card {
		background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 16px;
		padding: 1.5rem;
		display: flex;
		gap: 1.25rem;
		transition: all 0.3s ease;
	}

	.stat-card:hover {
		border-color: rgba(99, 102, 241, 0.3);
		transform: translateY(-2px);
		box-shadow: 0 10px 30px rgba(0, 0, 0, 0.3);
	}

	.stat-icon {
		width: 60px;
		height: 60px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.stat-icon.subscription {
		background: linear-gradient(135deg, rgba(59, 130, 246, 0.2), rgba(37, 99, 235, 0.1));
		color: #60a5fa;
	}

	.stat-icon.revenue {
		background: linear-gradient(135deg, rgba(34, 197, 94, 0.2), rgba(21, 128, 61, 0.1));
		color: #4ade80;
	}

	.stat-icon.coupons {
		background: linear-gradient(135deg, rgba(168, 85, 247, 0.2), rgba(126, 34, 206, 0.1));
		color: #a78bfa;
	}

	.stat-icon.users {
		background: linear-gradient(135deg, rgba(249, 115, 22, 0.2), rgba(234, 88, 12, 0.1));
		color: #fb923c;
	}

	.stat-content {
		flex: 1;
	}

	.stat-label {
		font-size: 0.875rem;
		color: #94a3b8;
		margin-bottom: 0.5rem;
		font-weight: 500;
	}

	.stat-value {
		font-size: 2rem;
		font-weight: 700;
		color: #f1f5f9;
		margin-bottom: 0.5rem;
	}

	.stat-change {
		font-size: 0.8125rem;
		font-weight: 500;
	}

	.stat-change.positive {
		color: #4ade80;
	}

	.stat-change.neutral {
		color: #94a3b8;
	}

	/* Quick Actions */
	.quick-actions {
		margin-top: 3rem;
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
	}

	.action-card:hover {
		background: linear-gradient(135deg, rgba(99, 102, 241, 0.2), rgba(139, 92, 246, 0.1));
		border-color: rgba(99, 102, 241, 0.4);
		transform: translateY(-2px);
	}

	/* Responsive */
	@media (max-width: 640px) {
		.welcome-title {
			font-size: 1.5rem;
		}

		.stats-grid {
			grid-template-columns: 1fr;
		}

		.stat-value {
			font-size: 1.75rem;
		}
	}
</style>
