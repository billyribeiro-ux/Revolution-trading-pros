<script lang="ts">
	import { emailTemplatesApi, AdminApiError } from '$lib/api/admin';
	import TemplateForm from '$lib/components/admin/TemplateForm.svelte';
	import { goto } from '$app/navigation';

	let loading = false;
	let error = '';
</script>

<svelte:head>
	<title>New Email Template | Admin | Revolution Trading Pros</title>
</svelte:head>

<div class="new-template-page">
	{#if error}
		<p class="alert alert-error">{error}</p>
	{/if}
	<TemplateForm isEdit={false} />
</div>

<style>
	.new-template-page {
		max-width: 1000px;
		margin: 0 auto;
		padding: 2rem;
	}
	.alert-error {
		background: rgba(239, 68, 68, 0.1);
		color: #ef4444;
		padding: 0.75rem 1rem;
		border-radius: 6px;
		margin-bottom: 1rem;
	}
</style>
