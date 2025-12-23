<script lang="ts">
	/**
	 * Learning Center - Public Landing Page
	 * Displays available courses and lessons across all trading rooms
	 */
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import IconSearch from '@tabler/icons-svelte/icons/search';
	import IconFilter from '@tabler/icons-svelte/icons/filter';

	let searchQuery = $state('');
	let selectedTrainer = $state('');
	let isLoading = $state(true);

	onMount(() => {
		const params = $page.url.searchParams;
		const trainer = params.get('trainer');
		if (trainer) {
			selectedTrainer = trainer;
		}
		isLoading = false;
	});
</script>

<svelte:head>
	<title>Learning Center | Revolution Trading Pros</title>
	<meta name="description" content="Access our comprehensive library of trading education content, courses, and lessons from expert traders." />
</svelte:head>

<div class="learning-center-page">
	<div class="container">
		<header class="page-header">
			<h1>Learning Center</h1>
			<p class="subtitle">Master the markets with expert-led courses and lessons</p>
		</header>

		<div class="filters">
			<div class="search-box">
				<IconSearch size={20} />
				<input
					type="text"
					placeholder="Search courses and lessons..."
					bind:value={searchQuery}
				/>
			</div>
			<button class="filter-btn">
				<IconFilter size={18} />
				Filters
			</button>
		</div>

		{#if isLoading}
			<div class="loading">Loading courses...</div>
		{:else}
			<div class="courses-grid">
				<div class="course-card">
					<h3>Coming Soon</h3>
					<p>Our learning center is being built. Check back soon for comprehensive trading education.</p>
				</div>
			</div>
		{/if}
	</div>
</div>

<style>
	.learning-center-page {
		min-height: 100vh;
		padding: 4rem 0;
		background: #f8f9fa;
	}

	.container {
		max-width: 1200px;
		margin: 0 auto;
		padding: 0 2rem;
	}

	.page-header {
		text-align: center;
		margin-bottom: 3rem;
	}

	.page-header h1 {
		font-size: 2.5rem;
		font-weight: 700;
		color: #1a1a1a;
		margin-bottom: 0.5rem;
	}

	.subtitle {
		font-size: 1.125rem;
		color: #6b7280;
	}

	.filters {
		display: flex;
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.search-box {
		flex: 1;
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		background: white;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
	}

	.search-box input {
		flex: 1;
		border: none;
		outline: none;
		font-size: 0.875rem;
	}

	.filter-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: white;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		cursor: pointer;
		font-size: 0.875rem;
		font-weight: 500;
		transition: all 0.2s;
	}

	.filter-btn:hover {
		background: #f9fafb;
		border-color: #d1d5db;
	}

	.loading {
		text-align: center;
		padding: 4rem;
		color: #6b7280;
	}

	.courses-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		gap: 1.5rem;
	}

	.course-card {
		background: white;
		border-radius: 12px;
		padding: 2rem;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.course-card h3 {
		font-size: 1.25rem;
		font-weight: 600;
		margin-bottom: 0.5rem;
		color: #1a1a1a;
	}

	.course-card p {
		color: #6b7280;
		line-height: 1.6;
	}
</style>
