<script lang="ts">
	import { page } from '$app/state';
	import { browser } from '$app/environment';

	let status = $derived(page.status);
	let message = $derived(page.error?.message || 'An unexpected error occurred');

	function handleRetry() {
		if (browser) window.location.reload();
	}
</script>

<div class="dashboard-error">
	<div class="dashboard-error__inner">
		<div class="dashboard-error__code">{status}</div>
		<h1 class="dashboard-error__title">
			{#if status === 401}
				Session Expired
			{:else if status === 403}
				Access Denied
			{:else if status === 404}
				Page Not Found
			{:else}
				Something Went Wrong
			{/if}
		</h1>
		<p class="dashboard-error__message">
			{#if status === 401}
				Your session has expired. Please sign in again to continue.
			{:else if status === 403}
				You don't have permission to access this dashboard area.
			{:else if status === 404}
				This dashboard page doesn't exist or has been moved.
			{:else}
				{message}
			{/if}
		</p>
		<div class="dashboard-error__actions">
			{#if status === 401}
				<a href="/login?redirect={encodeURIComponent(page.url.pathname)}" class="btn-primary">
					Sign In
				</a>
			{:else if status === 403}
				<a href="/dashboard" class="btn-primary">Back to Dashboard</a>
			{:else if status === 404}
				<a href="/dashboard" class="btn-primary">Back to Dashboard</a>
			{:else}
				<button type="button" class="btn-primary" onclick={handleRetry}>Try Again</button>
				<a href="/dashboard" class="btn-secondary">Back to Dashboard</a>
			{/if}
		</div>
	</div>
</div>

<style>
	.dashboard-error {
		display: flex;
		align-items: center;
		justify-content: center;
		min-height: 60vh;
		padding: 2rem;
	}

	.dashboard-error__inner {
		max-width: 420px;
		text-align: center;
	}

	.dashboard-error__code {
		font-size: 5rem;
		font-weight: 800;
		color: var(--color-profit, #f97316);
		line-height: 1;
		margin-bottom: 1rem;
	}

	.dashboard-error__title {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--color-text-primary, #1f2937);
		margin-bottom: 0.75rem;
	}

	.dashboard-error__message {
		font-size: 1rem;
		color: var(--color-text-secondary, #6b7280);
		margin-bottom: 2rem;
		line-height: 1.6;
	}

	.dashboard-error__actions {
		display: flex;
		gap: 1rem;
		justify-content: center;
		flex-wrap: wrap;
	}

	.btn-primary,
	.btn-secondary {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: 0.75rem 1.5rem;
		font-size: 0.9375rem;
		font-weight: 600;
		border-radius: var(--radius-md, 0.5rem);
		text-decoration: none;
		cursor: pointer;
		border: none;
		transition: all 0.2s ease;
	}

	.btn-primary {
		background: var(--color-profit, #f97316);
		color: #fff;
	}

	.btn-primary:hover {
		opacity: 0.9;
		transform: translateY(-1px);
	}

	.btn-secondary {
		background: var(--color-bg-card, #f3f4f6);
		color: var(--color-text-primary, #1f2937);
	}

	.btn-secondary:hover {
		background: var(--color-border-default, #e5e7eb);
	}
</style>
