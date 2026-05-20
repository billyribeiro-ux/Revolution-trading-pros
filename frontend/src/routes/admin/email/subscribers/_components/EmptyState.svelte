<script lang="ts">
	import { IconUsers, IconUserPlus } from '$lib/icons';

	interface Props {
		onadd: () => void;
	}

	let { onadd }: Props = $props();
</script>

<div class="empty-state">
	<IconUsers size={48} stroke={1} />
	<h3>No subscribers found</h3>
	<p>Add your first subscriber or import from a CSV file</p>
	<button class="btn-primary" onclick={onadd}>
		<IconUserPlus size={18} />
		Add Subscriber
	</button>
</div>

<style>
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		color: #64748b;
		text-align: center;
	}

	.empty-state h3 {
		color: #f1f5f9;
		margin: 1rem 0 0.5rem;
	}

	.empty-state p {
		margin-bottom: 1.5rem;
	}

	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
		font-size: 0.875rem;
		background: linear-gradient(135deg, var(--primary-500) 0%, var(--primary-600) 100%);
		color: var(--bg-base);
	}
</style>
