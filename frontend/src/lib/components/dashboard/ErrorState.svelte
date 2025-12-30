<script lang="ts">
	/**
	 * ErrorState - Shared error state component
	 * Svelte 5 with $props()
	 */
	interface Props {
		message: string;
		onRetry?: () => void;
	}

	let { message, onRetry }: Props = $props();
</script>

<div class="error-state">
	<p>{message}</p>
	{#if onRetry}
		<button class="error-state__button" onclick={onRetry}>
			Try Again
		</button>
	{/if}
</div>

<style>
	.error-state {
		text-align: center;
		padding: 40px 20px;
		color: #d32f2f;
		background: #ffebee;
		border-radius: 4px;
	}

	.error-state p {
		margin: 0 0 16px;
		font-size: 14px;
		font-family: 'Open Sans', sans-serif;
	}

	.error-state__button {
		display: inline-block;
		padding: 10px 20px;
		font-size: 14px;
		font-weight: 600;
		font-family: 'Open Sans', sans-serif;
		background: #0984ae;
		color: #fff;
		border: none;
		border-radius: 4px;
		cursor: pointer;
		transition: background-color 0.2s ease-in-out;
	}

	.error-state__button:hover {
		background: #077a9e;
	}
</style>
