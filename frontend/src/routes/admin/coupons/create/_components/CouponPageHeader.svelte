<script lang="ts">
	/**
	 * R25-C extraction (2026-05-20): page header for the coupon create page.
	 * Back arrow + title + subtitle. Cancel/back action is dispatched via the
	 * single `onCancel` callback prop so the parent owns the `goto()` call.
	 */
	import { IconX } from '#lib/icons/index.js';

	interface Props {
		onCancel: () => void;
	}

	let { onCancel }: Props = $props();
</script>

<header class="page-header">
	<div class="header-content">
		<button class="btn-back" type="button" onclick={onCancel}>
			<IconX size={20} />
		</button>
		<div>
			<h1>Create New Coupon</h1>
			<p class="subtitle">Set up a discount code for your products and memberships</p>
		</div>
	</div>
</header>

<style>
	.page-header {
		margin-bottom: 2rem;
	}

	.header-content {
		display: flex;
		align-items: flex-start;
		gap: 1rem;
	}

	.btn-back {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		background: var(--bg-hover, #252b33);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
		color: var(--text-secondary, #8b949e);
		cursor: pointer;
		transition: all 0.2s;
		flex-shrink: 0;
	}

	.btn-back:hover {
		background: var(--bg-hover, #252b33);
		color: var(--text-secondary, #8b949e);
	}

	.page-header h1 {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--text-primary, #f0f6fc);
		margin: 0 0 0.25rem 0;
	}

	.subtitle {
		color: var(--text-tertiary, #6e7681);
		font-size: 0.875rem;
		margin: 0;
	}
</style>
