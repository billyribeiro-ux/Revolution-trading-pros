<script lang="ts">
	import { IconUsers } from '$lib/icons';
</script>

<header class="page-header">
	<h1>
		<IconUsers size={28} />
		Lead Management
	</h1>
	<p class="subtitle">Track, qualify, and convert leads through your sales pipeline</p>
</header>

<style>
	.page-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 12px;
		margin: 0 0 0.5rem;
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.page-header h1 :global(svg) {
		color: var(--primary-500);
	}

	.subtitle {
		margin: 0;
		color: #64748b;
		font-size: 0.875rem;
	}

	@media (max-width: 767.98px) {
		.page-header h1 {
			font-size: 1.5rem;
		}
	}
</style>
