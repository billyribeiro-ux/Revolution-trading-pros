<!--
/**
 * LoadingState - Spinner + "Loading assets..." block
 * Extracted from AssetManager.svelte (R6-C). Pure leaf — no props.
 */
-->
<div class="loading-state">
	<div class="spinner"></div>
	<span>Loading assets...</span>
</div>

<style>
	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 300px;
		color: #64748b;
		gap: 1rem;
	}

	.spinner {
		width: 32px;
		height: 32px;
		border: 3px solid rgba(59, 130, 246, 0.2);
		border-top-color: #3b82f6;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>
