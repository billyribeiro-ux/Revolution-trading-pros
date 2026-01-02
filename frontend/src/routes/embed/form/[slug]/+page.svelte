<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import FormRenderer from '$lib/components/forms/FormRenderer.svelte';
	import { previewForm } from '$lib/api/forms';
	import type { Form } from '$lib/api/forms';

	let form = $state<Form | null>(null);
	let loading = $state(true);
	let error = $state('');
	let submitted = $state(false);

	let formSlug = $derived(page.params.slug!);

	onMount(async () => {
		try {
			form = await previewForm(formSlug);
		} catch (err) {
			error = err instanceof Error ? err.message : 'Form not found';
		} finally {
			loading = false;
		}
	});

	function handleSuccess(submissionId: string) {
		submitted = true;

		// Send message to parent window if embedded in iframe
		if (window.parent !== window) {
			window.parent.postMessage(
				{
					type: 'form-submitted',
					submissionId,
					formSlug
				},
				'*'
			);
		}
	}

	function handleError(errorMessage: string) {
		// Send error to parent window if embedded
		if (window.parent !== window) {
			window.parent.postMessage(
				{
					type: 'form-error',
					error: errorMessage,
					formSlug
				},
				'*'
			);
		}
	}
</script>

<svelte:head>
	<title>{form?.title || 'Form'}</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<div class="embed-container">
	{#if loading}
		<div class="loading-state">
			<div class="spinner"></div>
			<p>Loading form...</p>
		</div>
	{:else if error}
		<div class="error-state">
			<p>{error}</p>
		</div>
	{:else if form}
		<FormRenderer {form} onSuccess={handleSuccess} onError={handleError} />
	{/if}
</div>

<style>
	.embed-container {
		min-height: 100vh;
		padding: 1rem;
		background: transparent;
	}

	.loading-state,
	.error-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 200px;
		padding: 2rem;
	}

	.loading-state {
		color: #6b7280;
	}

	.error-state {
		color: #dc2626;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid #e5e7eb;
		border-top-color: #2563eb;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 1rem;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Make form renderer styles work in embed context */
	:global(.revolution-form) {
		background: white;
		padding: 2rem;
		border-radius: 0.5rem;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}
</style>
