<script lang="ts">
	import { IconVideo, IconFileText, IconPhoto, IconCheck, IconStarFilled } from '$lib/icons';

	type Stats = {
		total: number;
		videos: number;
		pdfs: number;
		documents: number;
		images: number;
		published: number;
		featured: number;
	};

	type Props = {
		stats: Stats;
	};

	const { stats }: Props = $props();
</script>

<div class="stats-bar">
	<div class="stat">
		<span class="stat-value">{stats.total}</span>
		<span class="stat-label">Total</span>
	</div>
	<div class="stat">
		<IconVideo size={16} />
		<span class="stat-value">{stats.videos}</span>
		<span class="stat-label">Videos</span>
	</div>
	<div class="stat">
		<IconFileText size={16} />
		<span class="stat-value">{stats.pdfs}</span>
		<span class="stat-label">PDFs</span>
	</div>
	<div class="stat">
		<IconPhoto size={16} />
		<span class="stat-value">{stats.images}</span>
		<span class="stat-label">Images</span>
	</div>
	<div class="stat">
		<IconCheck size={16} />
		<span class="stat-value">{stats.published}</span>
		<span class="stat-label">Published</span>
	</div>
	<div class="stat">
		<IconStarFilled size={16} />
		<span class="stat-value">{stats.featured}</span>
		<span class="stat-label">Featured</span>
	</div>
</div>

<style>
	.stats-bar {
		display: flex;
		gap: 1.5rem;
		padding: 1rem 1.5rem;
		background: rgba(15, 23, 42, 0.6);
		border-radius: 12px;
		margin-bottom: 1.5rem;
		flex-wrap: wrap;
	}

	.stat {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: #94a3b8;
	}

	.stat-value {
		font-size: 1.25rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.stat-label {
		font-size: 0.875rem;
	}

	/* Mobile */
	@media (max-width: 767px) {
		.stats-bar {
			flex-wrap: wrap;
			justify-content: flex-start;
			gap: 0.75rem 1.25rem;
			padding: 0.875rem 1rem;
		}

		.stat {
			gap: 0.375rem;
		}

		.stat-value {
			font-size: 1.125rem;
		}

		.stat-label {
			font-size: 0.75rem;
		}
	}

	@media (min-width: 768px) {
		.stats-bar {
			justify-content: flex-start;
			gap: 1.5rem;
			padding: 1rem 1.5rem;
		}

		.stat-value {
			font-size: 1.25rem;
		}
	}
</style>
