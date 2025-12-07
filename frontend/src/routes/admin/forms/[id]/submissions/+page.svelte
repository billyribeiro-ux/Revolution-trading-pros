<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import SubmissionsList from '$lib/components/forms/SubmissionsList.svelte';
	import { getForm } from '$lib/api/forms';
	import type { Form } from '$lib/api/forms';

	let form: Form | null = null;
	let loading = true;
	let error = '';

	let formId = $derived(parseInt($page.params.id));

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
	<title>Form Submissions - Admin Dashboard</title>
</svelte:head>

<div class="submissions-page">
	<div class="page-header">
		<div>
			<button class="btn-back" onclick={() => goto('/admin/forms')}> ← Back to Forms </button>
			<h1>Form Submissions</h1>
			{#if form}
				<p class="page-description">{form.title}</p>
			{/if}
		</div>
		{#if form}
			<button class="btn-edit" onclick={() => goto(`/admin/forms/${formId}/edit`)}>
				Edit Form
			</button>
		{/if}
	</div>

	{#if loading}
		<div class="loading-state">
			<p>Loading...</p>
		</div>
	{:else if error}
		<div class="error-state">
			<p>{error}</p>
		</div>
	{:else if form}
		<SubmissionsList {formId} />
	{/if}
</div>

<style>
	.submissions-page {
		max-width: 1400px;
		margin: 0 auto;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 2rem;
		padding-bottom: 1.5rem;
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
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
		color: #a5b4fc;
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

	.btn-edit {
		padding: 0.875rem 1.5rem;
		background: rgba(99, 102, 241, 0.1);
		color: #a5b4fc;
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 10px;
		font-weight: 600;
		font-size: 0.9375rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-edit:hover {
		background: rgba(99, 102, 241, 0.2);
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
</style>
