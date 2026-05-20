<script lang="ts">
	/**
	 * R24-C extraction (2026-05-20): page header — back-to-products link,
	 * title, subtitle. Discrete `onBack` callback (R8-C style — names the
	 * action, not a generic event).
	 */
	import { IconArrowLeft } from '$lib/icons';

	interface Props {
		onBack: () => void;
	}

	let { onBack }: Props = $props();
</script>

<div class="page-header">
	<button class="back-btn" onclick={onBack}>
		<IconArrowLeft size={20} />
		Back to Products
	</button>
	<div>
		<h1>Create New Product</h1>
		<p>Add a new course, indicator, membership, or bundle</p>
	</div>
</div>

<style>
	.page-header {
		margin-bottom: 2rem;
	}

	.back-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		background: transparent;
		border: none;
		color: #94a3b8;
		font-size: 0.875rem;
		cursor: pointer;
		margin-bottom: 1rem;
		padding: 0;
		transition: color 0.2s;
	}

	.back-btn:hover {
		color: #f1f5f9;
	}

	.page-header h1 {
		font-size: 2rem;
		font-weight: 700;
		color: #f1f5f9;
		margin-bottom: 0.5rem;
	}

	.page-header p {
		color: #94a3b8;
		font-size: 0.95rem;
	}
</style>
