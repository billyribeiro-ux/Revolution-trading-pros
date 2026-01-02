<script lang="ts">
	import { onMount } from 'svelte';
	import { apiFetch } from '$lib/api/config';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { sanitizeHtml } from '$lib/utils/sanitize';

	let loading = true;
	let error = '';
	let preview: { subject?: string; body_html?: string } | null = null;
	const id = page.params['id']!;

	onMount(async () => {
		try {
			// POST request (no sample data needed)
			preview = await apiFetch(`/admin/email/templates/${id}/preview`, {
				method: 'POST',
				body: JSON.stringify({})
			});
		} catch (e) {
			error = (e as Error).message;
		} finally {
			loading = false;
		}
	});
</script>

<svelte:head>
	<title>Template Preview | Admin | Revolution Trading Pros</title>
</svelte:head>

<div class="preview-page">
	{#if loading}
		<p class="text-muted">Loading preview...</p>
	{:else if error}
		<p class="alert alert-error">{error}</p>
	{:else}
		<h2 class="subject">{preview?.subject ?? ''}</h2>
		<div class="body">{@html sanitizeHtml(preview?.body_html ?? '', 'rich')}</div>
		<button class="btn-secondary" onclick={() => goto('/admin/email/templates')}
			>Back to List</button
		>
	{/if}
</div>

<style>
	.preview-page {
		max-width: 900px;
		margin: 0 auto;
		padding: 2rem;
	}
	.subject {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
		margin-bottom: 1rem;
	}
	.body {
		background: rgba(30, 41, 59, 0.4);
		padding: 1.5rem;
		border-radius: 8px;
		border: 1px solid rgba(99, 102, 241, 0.1);
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
	.btn-secondary {
		background: rgba(100, 116, 139, 0.2);
		color: #cbd5e1;
		border: 1px solid rgba(100, 116, 139, 0.3);
		padding: 0.5rem 1rem;
		border-radius: 6px;
		cursor: pointer;
		margin-top: 1rem;
	}
	.btn-secondary:hover {
		background: rgba(100, 116, 139, 0.3);
	}
</style>
