<script lang="ts">
	import { page } from '$app/state';
	import { browser } from '$app/environment';

	let status = $derived(page.status);
	let message = $derived(page.error?.message || 'An unexpected error occurred');

	function handleRetry() {
		if (browser) window.location.reload();
	}
</script>

<div class="es-error">
	<div class="es-error__inner">
		<div class="es-error__icon">📊</div>
		<div class="es-error__code">{status}</div>
		<h1 class="es-error__title">
			{#if status === 401}
				Session Expired
			{:else if status === 403}
				Membership Required
			{:else if status === 404}
				Content Not Found
			{:else}
				Dashboard Error
			{/if}
		</h1>
		<p class="es-error__message">
			{#if status === 401}
				Please sign in to access the Explosive Swings dashboard.
			{:else if status === 403}
				An active Explosive Swings membership is required to view this content.
			{:else if status === 404}
				This content doesn't exist or may have been removed.
			{:else}
				{message}
			{/if}
		</p>
		<div class="es-error__actions">
			{#if status === 401}
				<a href="/login?redirect={encodeURIComponent(page.url.pathname)}" class="btn-primary">
					Sign In
				</a>
			{:else if status === 403}
				<a href="/alerts/explosive-swings" class="btn-primary">View Plans</a>
				<a href="/dashboard" class="btn-secondary">Back to Dashboard</a>
			{:else}
				<button type="button" class="btn-primary" onclick={handleRetry}>Try Again</button>
				<a href="/dashboard/explosive-swings" class="btn-secondary">Back to Explosive Swings</a>
			{/if}
		</div>
	</div>
</div>

<style>
	.es-error {
		display: flex;
		align-items: center;
		justify-content: center;
		min-height: 60vh;
		padding: 2rem;
	}

	.es-error__inner {
		max-width: 420px;
		text-align: center;
	}

	.es-error__icon {
		font-size: 3rem;
		margin-bottom: 0.5rem;
	}

	.es-error__code {
		font-size: 4rem;
		font-weight: 800;
		color: var(--color-profit, #f97316);
		line-height: 1;
		margin-bottom: 1rem;
	}

	.es-error__title {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--color-text-primary, #1f2937);
		margin-bottom: 0.75rem;
	}

	.es-error__message {
		font-size: 1rem;
		color: var(--color-text-secondary, #6b7280);
		margin-bottom: 2rem;
		line-height: 1.6;
	}

	.es-error__actions {
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
