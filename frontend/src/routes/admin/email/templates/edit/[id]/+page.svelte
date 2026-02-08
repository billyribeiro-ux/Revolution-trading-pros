<script lang="ts">
	import { onMount } from 'svelte';
	import { apiFetch } from '$lib/api/config';
	import TemplateForm from '$lib/components/admin/TemplateForm.svelte';
	import { page } from '$app/state';

	let loading = $state(true);
	let error = $state('');
	let template: Record<string, unknown> | null = $state(null);

	const id = page.params['id']!;

	onMount(async () => {
		try {
			template = await apiFetch(`/admin/email/templates/${id}`);
		} catch (e) {
			error = (e as Error).message;
		} finally {
			loading = false;
		}
	});
</script>

<svelte:head>
	<title>Edit Email Template | Admin | Revolution Trading Pros</title>
</svelte:head>

<div class="edit-template-page">
	{#if loading}
		<p class="text-muted">Loading...</p>
	{:else if error}
		<p class="alert alert-error">{error}</p>
	{:else}
		<TemplateForm {template} isEdit={true} />
	{/if}
</div>

<style>
	.edit-template-page {
		max-width: 1000px;
		margin: 0 auto;
		padding: 2rem;
	}
	.text-muted {
		color: #94a3b8;
	}
	.alert-error {
		background: rgba(239, 68, 68, 0.1);
		color: #ef4444;
		padding: 0.75rem 1rem;
		border-radius: 6px;
		margin-bottom: 1rem;
	}
</style>
