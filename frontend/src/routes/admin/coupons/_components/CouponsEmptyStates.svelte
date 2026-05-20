<script lang="ts">
	/**
	 * R27-C extraction (2026-05-20): the four "non-grid" states for the
	 * coupons list — loading, error, empty (no coupons at all), and
	 * filtered-empty (coupons exist but the search/filter hides them).
	 *
	 * Modeled as a discriminated `kind` prop so the parent renders ONE
	 * component instead of an if/else chain in markup. The shared chrome
	 * (centered column, padding, typography) lives here once.
	 */
	import { IconFilter, IconPlus, IconRefresh } from '$lib/icons';

	type StateProps =
		| { kind: 'loading' }
		| { kind: 'error'; message: string; onRetry: () => void }
		| { kind: 'empty'; onCreate: () => void }
		| { kind: 'filtered-empty'; onClearFilters: () => void };

	let props: StateProps = $props();
</script>

{#if props.kind === 'loading'}
	<div class="loading-state">
		<IconRefresh size={32} class="spinning" />
		<p>Loading coupons...</p>
	</div>
{:else if props.kind === 'error'}
	<div class="error-state">
		<p>{props.message}</p>
		<button class="btn-secondary" onclick={props.onRetry}>Try Again</button>
	</div>
{:else if props.kind === 'empty'}
	<div class="empty-state">
		<div class="empty-icon">
			<IconFilter size={48} />
		</div>
		<h2>No coupons yet</h2>
		<p>Create your first coupon to start offering discounts to your customers.</p>
		<button class="btn-primary" onclick={props.onCreate}>
			<IconPlus size={18} />
			Create First Coupon
		</button>
	</div>
{:else}
	<div class="empty-state">
		<p>No coupons match your search criteria.</p>
		<button class="btn-secondary" onclick={props.onClearFilters}>Clear Filters</button>
	</div>
{/if}

<style>
	.loading-state,
	.error-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		text-align: center;
		color: var(--text-secondary);
	}

	.loading-state p,
	.error-state p,
	.empty-state p {
		margin: 0.75rem 0;
		font-size: 0.9375rem;
		max-width: 400px;
	}

	.empty-state h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--text-primary);
		margin: 0.75rem 0 0.25rem;
	}

	.empty-icon {
		color: var(--text-muted);
		margin-bottom: 0.5rem;
	}

	.error-state {
		color: var(--error-emphasis);
	}

	.error-state p {
		margin-bottom: 1.25rem;
	}

	.btn-primary,
	.btn-secondary {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		border-radius: 8px;
		font-size: 0.875rem;
		font-weight: 600;
		border: none;
		cursor: pointer;
		transition: all 0.2s ease;
		white-space: nowrap;
	}

	.btn-primary {
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		color: var(--bg-base);
	}

	.btn-primary:hover {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(230, 184, 0, 0.3);
	}

	.btn-secondary {
		background: var(--bg-surface);
		color: var(--text-primary);
		border: 1px solid var(--border-default);
	}

	.btn-secondary:hover {
		background: var(--bg-hover);
		border-color: var(--border-emphasis);
	}
</style>
