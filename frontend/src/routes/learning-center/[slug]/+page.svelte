<script lang="ts">
	/**
	 * Learning Center - Individual Lesson Page
	 * Displays a specific lesson by slug
	 */
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import IconArrowLeft from '@tabler/icons-svelte/icons/arrow-left';

	let slug = $derived($page.params.slug);
	let isLoading = $state(true);
	let lesson = $state<any>(null);

	onMount(async () => {
		// TODO: Fetch lesson data from API
		isLoading = false;
	});
</script>

<svelte:head>
	<title>{lesson?.title || 'Lesson'} | Learning Center | Revolution Trading Pros</title>
</svelte:head>

<div class="lesson-page">
	<div class="container">
		<a href="/learning-center" class="back-link">
			<IconArrowLeft size={18} />
			Back to Learning Center
		</a>

		{#if isLoading}
			<div class="loading">Loading lesson...</div>
		{:else if lesson}
			<article class="lesson-content">
				<h1>{lesson.title}</h1>
				<div class="lesson-body">
					{@html lesson.content}
				</div>
			</article>
		{:else}
			<div class="not-found">
				<h2>Lesson Not Found</h2>
				<p>The lesson you're looking for doesn't exist or has been removed.</p>
				<a href="/learning-center" class="btn-primary">Browse All Lessons</a>
			</div>
		{/if}
	</div>
</div>

<style>
	.lesson-page {
		min-height: 100vh;
		padding: 4rem 0;
		background: #f8f9fa;
	}

	.container {
		max-width: 900px;
		margin: 0 auto;
		padding: 0 2rem;
	}

	.back-link {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		color: #6b7280;
		text-decoration: none;
		font-size: 0.875rem;
		margin-bottom: 2rem;
		transition: color 0.2s;
	}

	.back-link:hover {
		color: #1a1a1a;
	}

	.loading {
		text-align: center;
		padding: 4rem;
		color: #6b7280;
	}

	.lesson-content {
		background: white;
		border-radius: 12px;
		padding: 3rem;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.lesson-content h1 {
		font-size: 2rem;
		font-weight: 700;
		color: #1a1a1a;
		margin-bottom: 2rem;
	}

	.lesson-body {
		line-height: 1.8;
		color: #374151;
	}

	.not-found {
		text-align: center;
		padding: 4rem 2rem;
		background: white;
		border-radius: 12px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.not-found h2 {
		font-size: 1.5rem;
		font-weight: 600;
		color: #1a1a1a;
		margin-bottom: 1rem;
	}

	.not-found p {
		color: #6b7280;
		margin-bottom: 2rem;
	}

	.btn-primary {
		display: inline-block;
		padding: 0.75rem 1.5rem;
		background: #0984ae;
		color: white;
		text-decoration: none;
		border-radius: 8px;
		font-weight: 500;
		transition: background 0.2s;
	}

	.btn-primary:hover {
		background: #076d8f;
	}
</style>
