<script lang="ts">
	import { onMount } from 'svelte';
	import {
		IconReceipt,
		IconTicket,
		IconUsers,
		IconChartBar,
		IconRefresh,
		IconNews,
		IconForms,
		IconMail,
		IconBellRinging,
		IconPhoto,
		IconSettings,
		IconSeo,
		IconUserCircle,
		IconFilter,
		IconActivity,
		IconSend,
		IconAlertCircle,
		IconShoppingCart,
		IconVideo,
		IconTag
	} from '@tabler/icons-svelte';
	import { browser } from '$app/environment';
	import { api } from '$lib/api/config';

	let statsRef: HTMLDivElement;
	let isLoading = true;
	let lastUpdated: Date | null = null;
	let error: string | null = null;

	// Real stats data - fetched from API
	let stats = {
		activeSubscriptions: 0,
		monthlyRevenue: 0,
		activeCoupons: 0,
		totalMembers: 0,
		totalPosts: 0,
		totalProducts: 0
	};

	async function fetchDashboardStats() {
		isLoading = true;
		error = null;
		
		try {
			// Fetch real stats from multiple endpoints in parallel
			const [membersRes, subscriptionsRes, couponsRes, postsRes, productsRes] = await Promise.allSettled([
				api.get('/api/admin/members/stats'),
				api.get('/api/admin/subscriptions/plans/stats'),
				api.get('/api/admin/coupons'),
				api.get('/api/admin/posts/stats'),
				api.get('/api/admin/products/stats')
			]);

			// Extract data from responses - handle the member stats structure
			if (membersRes.status === 'fulfilled') {
				const memberData = membersRes.value;
				stats.totalMembers = memberData?.overview?.total_members || memberData?.total || 0;
				stats.activeSubscriptions = memberData?.subscriptions?.active || 0;
			}
			
			if (subscriptionsRes.status === 'fulfilled') {
				const subData = subscriptionsRes.value?.data || subscriptionsRes.value;
				if (!stats.activeSubscriptions) {
					stats.activeSubscriptions = subData?.active_subscriptions || subData?.total_active || 0;
				}
				stats.monthlyRevenue = subData?.monthly_revenue || subData?.mrr || 0;
			}
			
			if (couponsRes.status === 'fulfilled') {
				const couponsData = couponsRes.value;
				const coupons = couponsData?.coupons || couponsData?.data || couponsData || [];
				stats.activeCoupons = Array.isArray(coupons) 
					? coupons.filter((c: any) => c.is_active).length 
					: couponsData?.total || 0;
			}
			
			if (postsRes.status === 'fulfilled') {
				const postsData = postsRes.value;
				stats.totalPosts = postsData?.total || postsData?.data?.total || postsData?.posts?.length || 0;
			}
			
			if (productsRes.status === 'fulfilled') {
				const productsData = productsRes.value;
				stats.totalProducts = productsData?.total || productsData?.data?.total || 0;
			}

			lastUpdated = new Date();
		} catch (err) {
			console.error('Failed to fetch dashboard stats:', err);
			error = 'Failed to load some statistics. Please try again.';
		} finally {
			isLoading = false;
		}
	}

	onMount(() => {
		fetchDashboardStats();
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

	<!-- Real Stats Grid -->
	{#if error}
		<div class="error-banner">
			<IconAlertCircle size={20} />
			<span>{error}</span>
		</div>
	{/if}

	<div class="stats-grid" bind:this={statsRef}>
		<a href="/admin/members" class="stat-card">
			<div class="stat-icon members"><IconUserCircle size={24} /></div>
			<div class="stat-content">
				<span class="stat-value">{isLoading ? '...' : stats.totalMembers.toLocaleString()}</span>
				<span class="stat-label">Total Members</span>
			</div>
		</a>
		<a href="/admin/subscriptions" class="stat-card">
			<div class="stat-icon subscriptions"><IconReceipt size={24} /></div>
			<div class="stat-content">
				<span class="stat-value">{isLoading ? '...' : stats.activeSubscriptions.toLocaleString()}</span>
				<span class="stat-label">Active Subscriptions</span>
			</div>
		</a>
		<a href="/admin/coupons" class="stat-card">
			<div class="stat-icon coupons"><IconTicket size={24} /></div>
			<div class="stat-content">
				<span class="stat-value">{isLoading ? '...' : stats.activeCoupons.toLocaleString()}</span>
				<span class="stat-label">Active Coupons</span>
			</div>
		</a>
		<a href="/admin/blog" class="stat-card">
			<div class="stat-icon posts"><IconNews size={24} /></div>
			<div class="stat-content">
				<span class="stat-value">{isLoading ? '...' : stats.totalPosts.toLocaleString()}</span>
				<span class="stat-label">Blog Posts</span>
			</div>
		</a>
	</div>

	<!-- CMS Functions Grid -->
	<div class="cms-section">
		<h3 class="section-title">Content Management</h3>
		<div class="cms-grid">
			<a href="/admin/blog" class="cms-card">
				<div class="cms-icon blog"><IconNews size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">Blog Posts</span>
					<span class="cms-desc">Create & manage articles</span>
				</div>
			</a>
			<a href="/admin/blog/categories" class="cms-card">
				<div class="cms-icon categories"><IconTag size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">Categories</span>
					<span class="cms-desc">Organize content</span>
				</div>
			</a>
			<a href="/admin/media" class="cms-card">
				<div class="cms-icon media"><IconPhoto size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">Media Library</span>
					<span class="cms-desc">Upload & organize files</span>
				</div>
			</a>
			<a href="/admin/videos" class="cms-card">
				<div class="cms-icon videos"><IconVideo size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">Videos</span>
					<span class="cms-desc">Manage video content</span>
				</div>
			</a>
			<a href="/admin/forms" class="cms-card">
				<div class="cms-icon forms"><IconForms size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">Forms</span>
					<span class="cms-desc">Build & manage forms</span>
				</div>
			</a>
			<a href="/admin/popups" class="cms-card">
				<div class="cms-icon popups"><IconBellRinging size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">Popups</span>
					<span class="cms-desc">Create popup campaigns</span>
				</div>
			</a>
		</div>
	</div>

	<!-- Members & Commerce -->
	<div class="cms-section">
		<h3 class="section-title">Members & Commerce</h3>
		<div class="cms-grid">
			<a href="/admin/members" class="cms-card">
				<div class="cms-icon members"><IconUserCircle size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">Members</span>
					<span class="cms-desc">View all members</span>
				</div>
			</a>
			<a href="/admin/members/segments" class="cms-card">
				<div class="cms-icon segments"><IconFilter size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">Segments</span>
					<span class="cms-desc">Create member segments</span>
				</div>
			</a>
			<a href="/admin/products" class="cms-card">
				<div class="cms-icon products"><IconShoppingCart size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">Products</span>
					<span class="cms-desc">Courses & indicators</span>
				</div>
			</a>
			<a href="/admin/subscriptions" class="cms-card">
				<div class="cms-icon subscriptions"><IconReceipt size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">Subscriptions</span>
					<span class="cms-desc">Manage subscriptions</span>
				</div>
			</a>
			<a href="/admin/coupons" class="cms-card">
				<div class="cms-icon coupons"><IconTicket size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">Coupons</span>
					<span class="cms-desc">Create discount codes</span>
				</div>
			</a>
			<a href="/admin/crm" class="cms-card">
				<div class="cms-icon crm"><IconUsers size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">CRM</span>
					<span class="cms-desc">Contacts & deals</span>
				</div>
			</a>
		</div>
	</div>

	<!-- Email & Marketing -->
	<div class="cms-section">
		<h3 class="section-title">Email & Marketing</h3>
		<div class="cms-grid">
			<a href="/admin/email/campaigns" class="cms-card">
				<div class="cms-icon campaigns"><IconSend size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">Campaigns</span>
					<span class="cms-desc">Email campaigns</span>
				</div>
			</a>
			<a href="/admin/email/templates" class="cms-card">
				<div class="cms-icon templates"><IconMail size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">Email Templates</span>
					<span class="cms-desc">Design email templates</span>
				</div>
			</a>
			<a href="/admin/email/smtp" class="cms-card">
				<div class="cms-icon smtp"><IconSettings size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">Email Settings</span>
					<span class="cms-desc">SMTP configuration</span>
				</div>
			</a>
			<a href="/admin/seo" class="cms-card">
				<div class="cms-icon seo"><IconSeo size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">SEO</span>
					<span class="cms-desc">Search optimization</span>
				</div>
			</a>
		</div>
	</div>

	<!-- Analytics & Reports -->
	<div class="cms-section">
		<h3 class="section-title">Analytics & Reports</h3>
		<div class="cms-grid">
			<a href="/admin/analytics" class="cms-card">
				<div class="cms-icon analytics"><IconChartBar size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">Analytics</span>
					<span class="cms-desc">View detailed analytics</span>
				</div>
			</a>
			<a href="/admin/behavior" class="cms-card">
				<div class="cms-icon behavior"><IconActivity size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">Behavior Tracking</span>
					<span class="cms-desc">User behavior insights</span>
				</div>
			</a>
			<a href="/admin/users" class="cms-card">
				<div class="cms-icon users"><IconUsers size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">Admin Users</span>
					<span class="cms-desc">Manage admin access</span>
				</div>
			</a>
			<a href="/admin/settings" class="cms-card">
				<div class="cms-icon settings"><IconSettings size={28} /></div>
				<div class="cms-info">
					<span class="cms-label">Settings</span>
					<span class="cms-desc">System configuration</span>
				</div>
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
		grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.stat-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem;
		background: linear-gradient(135deg, rgba(30, 41, 59, 0.8), rgba(15, 23, 42, 0.9));
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 12px;
		text-decoration: none;
		transition: all 0.2s ease;
	}

	.stat-card:hover {
		border-color: rgba(99, 102, 241, 0.4);
		transform: translateY(-2px);
		box-shadow: 0 8px 24px rgba(0, 0, 0, 0.3);
	}

	.stat-icon {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.stat-icon.members { background: rgba(99, 102, 241, 0.15); color: #818cf8; }
	.stat-icon.subscriptions { background: rgba(20, 184, 166, 0.15); color: #2dd4bf; }
	.stat-icon.coupons { background: rgba(250, 204, 21, 0.15); color: #facc15; }
	.stat-icon.posts { background: rgba(59, 130, 246, 0.15); color: #60a5fa; }

	.stat-content {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.stat-value {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		line-height: 1;
	}

	.stat-label {
		font-size: 0.85rem;
		color: #94a3b8;
		font-weight: 500;
	}

	.error-banner {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 8px;
		color: #f87171;
		margin-bottom: 1.5rem;
	}

	/* CMS Sections */
	.cms-section {
		margin-bottom: 2.5rem;
	}

	.section-title {
		font-size: 1.25rem;
		font-weight: 700;
		color: #f1f5f9;
		margin-bottom: 1rem;
		padding-bottom: 0.5rem;
		border-bottom: 1px solid rgba(99, 102, 241, 0.2);
	}

	.cms-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 1rem;
	}

	.cms-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem 1.25rem;
		background: rgba(30, 41, 59, 0.5);
		border: 1px solid rgba(99, 102, 241, 0.15);
		border-radius: 12px;
		text-decoration: none;
		transition: all 0.2s ease;
	}

	.cms-card:hover {
		background: rgba(99, 102, 241, 0.1);
		border-color: rgba(99, 102, 241, 0.3);
		transform: translateX(4px);
	}

	.cms-icon {
		width: 48px;
		height: 48px;
		border-radius: 10px;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.cms-icon.blog { background: rgba(59, 130, 246, 0.15); color: #60a5fa; }
	.cms-icon.categories { background: rgba(139, 92, 246, 0.15); color: #a78bfa; }
	.cms-icon.media { background: rgba(168, 85, 247, 0.15); color: #c084fc; }
	.cms-icon.videos { background: rgba(236, 72, 153, 0.15); color: #f472b6; }
	.cms-icon.forms { background: rgba(34, 197, 94, 0.15); color: #4ade80; }
	.cms-icon.popups { background: rgba(251, 146, 60, 0.15); color: #fb923c; }
	.cms-icon.members { background: rgba(99, 102, 241, 0.15); color: #818cf8; }
	.cms-icon.segments { background: rgba(236, 72, 153, 0.15); color: #f472b6; }
	.cms-icon.products { background: rgba(16, 185, 129, 0.15); color: #34d399; }
	.cms-icon.subscriptions { background: rgba(20, 184, 166, 0.15); color: #2dd4bf; }
	.cms-icon.coupons { background: rgba(250, 204, 21, 0.15); color: #facc15; }
	.cms-icon.crm { background: rgba(59, 130, 246, 0.15); color: #60a5fa; }
	.cms-icon.campaigns { background: rgba(239, 68, 68, 0.15); color: #f87171; }
	.cms-icon.templates { background: rgba(59, 130, 246, 0.15); color: #60a5fa; }
	.cms-icon.smtp { background: rgba(148, 163, 184, 0.15); color: #94a3b8; }
	.cms-icon.seo { background: rgba(34, 197, 94, 0.15); color: #4ade80; }
	.cms-icon.analytics { background: rgba(99, 102, 241, 0.15); color: #818cf8; }
	.cms-icon.behavior { background: rgba(251, 146, 60, 0.15); color: #fb923c; }
	.cms-icon.users { background: rgba(168, 85, 247, 0.15); color: #c084fc; }
	.cms-icon.settings { background: rgba(148, 163, 184, 0.15); color: #cbd5e1; }

	.cms-info {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.cms-label {
		font-weight: 600;
		font-size: 0.95rem;
		color: #f1f5f9;
	}

	.cms-desc {
		font-size: 0.8rem;
		color: #64748b;
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
