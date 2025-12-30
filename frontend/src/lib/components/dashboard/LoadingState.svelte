<script lang="ts">
	/**
	 * LoadingState - Shared loading spinner component
	 * Svelte 5 with $props()
	 */
	interface Props {
		message?: string;
	}

	let { message = 'Loading...' }: Props = $props();
</script>

<div class="loading-state">
	<div class="spinner"></div>
	<p>{message}</p>
</div>

<style>
	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 200px;
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
		font-family: 'Open Sans', sans-serif;
		margin: 0;
	}
</style>
