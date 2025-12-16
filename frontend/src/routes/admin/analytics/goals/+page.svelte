<script lang="ts">
	import { IconTarget, IconPlus, IconEdit, IconTrash, IconTrendingUp, IconCheck } from '$lib/icons';

	let goals: any[] = [];
</script>

<svelte:head>
	<title>Goals - Analytics</title>
</svelte:head>

<div class="goals-page">
	<div class="page-header">
		<div>
			<h1>Conversion Goals</h1>
			<p class="page-description">Track and measure your conversion objectives</p>
		</div>
		<button class="btn-primary">
			<IconPlus size={18} />
			Create Goal
		</button>
	</div>

	<div class="empty-state">
		<IconTarget size={64} />
		<h3>No goals configured</h3>
		<p>Set up conversion goals to track important user actions</p>
		<button class="btn-primary">
			<IconPlus size={18} />
			Create Your First Goal
		</button>
	</div>
</div>

<style>
	.goals-page { max-width: 1400px; }

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 2rem;
		flex-wrap: wrap;
		gap: 1rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.25rem 0;
	}

	.page-description { color: #64748b; margin: 0; }

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		color: white;
		border: none;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(99, 102, 241, 0.4);
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		text-align: center;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 16px;
	}

	.empty-state :global(svg) { color: #475569; margin-bottom: 1rem; }
	.empty-state h3 { color: #e2e8f0; margin: 0 0 0.5rem 0; }
	.empty-state p { color: #64748b; margin: 0 0 1.5rem 0; }
</style>
