<script lang="ts">
	import { page } from '$app/state';
</script>

<svelte:head>
	<title>{page.status} — Dashboard: Revolution Trading Pros</title>
</svelte:head>

<div class="error-page">
	<div class="error-card">
		<h1 class="error-status">{page.status}</h1>
		<p class="error-message">{page.error?.message ?? "Couldn't load your dashboard."}</p>
		<div class="error-actions">
			<a href="/dashboard" class="error-btn-primary">Retry</a>
			<a href="/" class="error-btn-secondary">Go home</a>
		</div>
	</div>
</div>

<style>
	.error-page {
		min-height: 60vh;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 2rem;
	}
	.error-card {
		max-width: 32rem;
		text-align: center;
		padding: 2.5rem;
		border-radius: 1rem;
		background: rgba(20, 20, 28, 0.7);
		backdrop-filter: blur(12px);
		border: 1px solid rgba(255, 255, 255, 0.06);
	}
	.error-status {
		font-size: clamp(2.5rem, 6vw, 4rem);
		font-weight: 700;
		margin: 0 0 0.5rem;
		color: #f5f5f5;
	}
	.error-message {
		color: rgba(245, 245, 245, 0.75);
		margin: 0 0 1.5rem;
	}
	.error-actions {
		display: flex;
		gap: 0.75rem;
		justify-content: center;
		flex-wrap: wrap;
	}
	.error-btn-primary,
	.error-btn-secondary {
		display: inline-block;
		padding: 0.75rem 1.5rem;
		border-radius: 0.5rem;
		text-decoration: none;
		font-weight: 600;
		min-height: 44px;
	}
	.error-btn-primary {
		background: #2563eb;
		color: #fff;
	}
	.error-btn-secondary {
		border: 1px solid rgba(255, 255, 255, 0.12);
		color: #f5f5f5;
	}
</style>
