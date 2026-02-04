<!--
	URL: /admin/forms/[id]/analytics
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import FormAnalytics from '$lib/components/forms/FormAnalytics.svelte';
	import { getForm } from '$lib/api/forms';
	import type { Form } from '$lib/api/forms';

	let form = $state<Form | null>(null);
	let loading = $state(true);
	let error = $state('');

	let formId = $derived(parseInt(page.params['id']!));

	onMount(async () => {
		try {
			form = await getForm(formId);
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load form';
		} finally {
			loading = false;
		}
	});
</script>

<svelte:head>
	<title>Form Analytics - Admin Dashboard</title>
</svelte:head>

<div class="analytics-page">
	<div class="page-header">
		<div>
			<button class="btn-back" onclick={() => goto('/admin/forms')}> ← Back to Forms </button>
			<h1>Form Analytics</h1>
			{#if form}
				<p class="page-description">{form.title}</p>
			{/if}
		</div>
		<div class="header-actions">
			{#if form}
				<button class="btn-action" onclick={() => goto(`/admin/forms/${formId}/edit`)}>
					✏️ Edit Form
				</button>
				<button class="btn-action" onclick={() => goto(`/admin/forms/${formId}/submissions`)}>
					📊 View Submissions
				</button>
			{/if}
		</div>
	</div>

	{#if loading}
		<div class="loading-state">
			<p>Loading analytics...</p>
		</div>
	{:else if error}
		<div class="error-state">
			<p>{error}</p>
		</div>
	{:else if form}
		<FormAnalytics {form} />
	{/if}
</div>

<style>
	.analytics-page {
		max-width: 1400px;
		margin: 0 auto;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 2rem;
		padding-bottom: 1.5rem;
		border-bottom: 1px solid rgba(230, 184, 0, 0.1);
	}

	.btn-back {
		background: none;
		border: none;
		color: #94a3b8;
		font-size: 0.875rem;
		cursor: pointer;
		padding: 0.5rem 0;
		margin-bottom: 0.75rem;
		transition: color 0.2s;
	}

	.btn-back:hover {
		color: var(--primary-500);
	}

	h1 {
		font-size: 2rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.5rem 0;
	}

	.page-description {
		color: #94a3b8;
		font-size: 0.9375rem;
		margin: 0;
	}

	.header-actions {
		display: flex;
		gap: 1rem;
	}

	.btn-action {
		padding: 0.875rem 1.5rem;
		background: rgba(230, 184, 0, 0.1);
		color: var(--primary-500);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 10px;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-action:hover {
		background: rgba(230, 184, 0, 0.2);
		transform: translateY(-1px);
	}

	.loading-state,
	.error-state {
		text-align: center;
		padding: 4rem 2rem;
		color: #94a3b8;
	}

	.error-state {
		color: #f87171;
	}

	@media (max-width: 768px) {
		.page-header {
			flex-direction: column;
			gap: 1rem;
		}

		.header-actions {
			width: 100%;
			flex-direction: column;
		}

		.btn-action {
			width: 100%;
		}
	}
</style>
