<script lang="ts">
	/**
	 * R23-C extraction (2026-05-20): read-only metadata grid — created /
	 * updated dates, total uses, internal ID. Renders only when the
	 * coupon has loaded.
	 */
	import type { Coupon } from '#lib/api/admin.js';

	interface Props {
		coupon: Coupon;
	}

	let { coupon }: Props = $props();
</script>

<div class="form-section metadata-section">
	<h2>Metadata</h2>
	<div class="metadata-grid">
		<div class="metadata-item">
			<span class="metadata-label">Created</span>
			<span class="metadata-value">
				{new Date(coupon.created_at).toLocaleDateString('en-US', {
					dateStyle: 'medium'
				})}
			</span>
		</div>
		<div class="metadata-item">
			<span class="metadata-label">Last Updated</span>
			<span class="metadata-value">
				{new Date(coupon.updated_at).toLocaleDateString('en-US', {
					dateStyle: 'medium'
				})}
			</span>
		</div>
		<div class="metadata-item">
			<span class="metadata-label">Total Uses</span>
			<span class="metadata-value">{coupon.usage_count}</span>
		</div>
		<div class="metadata-item">
			<span class="metadata-label">ID</span>
			<span class="metadata-value">#{coupon.id}</span>
		</div>
	</div>
</div>

<style>
	.form-section {
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-muted, #21262d);
		border-radius: 12px;
		padding: 2rem;
	}

	.form-section h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
		margin: 0 0 1.5rem 0;
	}

	.metadata-section {
		background: var(--bg-base, #0d1117);
	}

	.metadata-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
		gap: 1.5rem;
	}

	.metadata-item {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.metadata-label {
		font-size: 0.8rem;
		color: var(--text-tertiary, #6e7681);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.metadata-value {
		font-size: 1rem;
		color: var(--text-secondary, #8b949e);
		font-weight: 500;
	}

	@media (max-width: 767.98px) {
		.metadata-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}
</style>
