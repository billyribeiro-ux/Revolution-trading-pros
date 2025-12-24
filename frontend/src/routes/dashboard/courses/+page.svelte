<script lang="ts">
	/**
	 * My Courses Page - Member Dashboard
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Course library and learning center access
	 *
	 * @version 1.0.0
	 */
	import { onMount } from 'svelte';
	import { getUserMemberships, type UserMembershipsResponse } from '$lib/api/user-memberships';
	import DynamicIcon from '$lib/components/DynamicIcon.svelte';
	import IconPlayerPlayFilled from '@tabler/icons-svelte/icons/player-play-filled';
	import IconClock from '@tabler/icons-svelte/icons/clock';
	import IconCheck from '@tabler/icons-svelte/icons/check';

	let loading = $state(true);
	let membershipsData = $state<UserMembershipsResponse | null>(null);

	onMount(async () => {
		try {
			membershipsData = await getUserMemberships();
		} catch (err) {
			console.error('Failed to load courses:', err);
		} finally {
			loading = false;
		}
	});
</script>

<svelte:head>
	<title>My Courses | Revolution Trading Pros</title>
</svelte:head>

<div class="courses-page">
	<!-- Page Header -->
	<header class="page-header">
		<div class="header-left">
			<h1 class="page-title">My Courses</h1>
			<p class="page-subtitle">Access your educational content and learning materials</p>
		</div>
	</header>

	<!-- Courses Grid -->
	<div class="courses-content">
		{#if loading}
			<div class="loading-state">
				<div class="spinner"></div>
				<p>Loading your courses...</p>
			</div>
		{:else if membershipsData?.courses && membershipsData.courses.length > 0}
			<div class="courses-grid">
				{#each membershipsData.courses as course (course.id)}
					<article class="course-card">
						<a href="/dashboard/{course.slug}" class="course-link">
							<div class="course-icon">
								<DynamicIcon name={course.icon || 'book'} size={32} />
							</div>
							<div class="course-info">
								<h3 class="course-title">{course.name}</h3>
								<div class="course-meta">
									<span class="meta-item">
										<IconClock size={14} />
										Active
									</span>
									<span class="meta-item status-active">
										<IconCheck size={14} />
										Enrolled
									</span>
								</div>
							</div>
						</a>
						<div class="course-actions">
							<a href="/dashboard/{course.slug}" class="btn btn-primary">
								<IconPlayerPlayFilled size={16} />
								Continue Learning
							</a>
						</div>
					</article>
				{/each}
			</div>
		{:else}
			<div class="empty-state">
				<div class="empty-icon">
					<IconPlayerPlayFilled size={48} />
				</div>
				<h3>No Courses Yet</h3>
				<p>You don't have any active courses. Browse our course catalog to get started.</p>
				<a href="/pricing" class="btn btn-primary">Browse Courses</a>
			</div>
		{/if}
	</div>
</div>

<style>
	.courses-page {
		padding: 30px;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
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

	/* Loading State */
	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 400px;
		gap: 16px;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid #e5e7eb;
		border-top-color: #0984ae;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.loading-state p {
		color: #666;
		font-size: 14px;
	}

	/* Courses Grid */
	.courses-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
		gap: 24px;
	}

	.course-card {
		background: #fff;
		border-radius: 8px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
		overflow: hidden;
		transition: all 0.15s;
	}

	.course-card:hover {
		box-shadow: 0 4px 16px rgba(0, 0, 0, 0.12);
		transform: translateY(-2px);
	}

	.course-link {
		display: flex;
		align-items: center;
		gap: 16px;
		padding: 20px;
		text-decoration: none;
		color: inherit;
	}

	.course-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 64px;
		height: 64px;
		background: linear-gradient(135deg, #0984ae 0%, #076787 100%);
		border-radius: 12px;
		color: #fff;
		flex-shrink: 0;
	}

	.course-info {
		flex: 1;
		min-width: 0;
	}

	.course-title {
		font-size: 18px;
		font-weight: 600;
		color: #333;
		margin: 0 0 8px;
	}

	.course-meta {
		display: flex;
		gap: 12px;
		flex-wrap: wrap;
	}

	.meta-item {
		display: flex;
		align-items: center;
		gap: 4px;
		font-size: 12px;
		color: #6b7280;
	}

	.meta-item.status-active {
		color: #10b981;
	}

	.course-actions {
		padding: 0 20px 20px;
	}

	/* Empty State */
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 400px;
		text-align: center;
		padding: 40px;
	}

	.empty-icon {
		width: 80px;
		height: 80px;
		background: linear-gradient(135deg, #0984ae 0%, #076787 100%);
		border-radius: 20px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #fff;
		margin-bottom: 24px;
	}

	.empty-state h3 {
		font-size: 20px;
		font-weight: 600;
		color: #333;
		margin: 0 0 8px;
	}

	.empty-state p {
		font-size: 14px;
		color: #666;
		margin: 0 0 24px;
		max-width: 400px;
	}

	/* Buttons */
	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 10px 20px;
		border-radius: 6px;
		font-size: 14px;
		font-weight: 600;
		text-decoration: none;
		cursor: pointer;
		transition: all 0.15s;
		border: none;
		width: 100%;
	}

	.btn-primary {
		background: #0984ae;
		color: #fff;
	}

	.btn-primary:hover {
		background: #076787;
	}

	@media (max-width: 768px) {
		.courses-page {
			padding: 20px;
		}

		.courses-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
