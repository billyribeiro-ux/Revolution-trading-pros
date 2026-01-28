<script lang="ts">
	/**
	 * SearchSkeleton - Loading skeleton for search results
	 * @standards Svelte 5 January 2026 | Apple Principal Engineer ICT 7+
	 */
</script>

<div class="skeleton-container">
	<!-- Meta Skeleton -->
	<div class="skeleton-meta">
		<div class="skeleton-text short"></div>
		<div class="skeleton-text tiny"></div>
	</div>

	<!-- Result Sections -->
	{#each [1, 2, 3] as section}
		<div class="skeleton-section">
			<div class="skeleton-header">
				<div class="skeleton-icon"></div>
				<div class="skeleton-text medium"></div>
				<div class="skeleton-badge"></div>
			</div>

			<div class="skeleton-results">
				{#each [1, 2, 3] as result}
					<div class="skeleton-card">
						<div class="skeleton-card-header">
							<div class="skeleton-badge small"></div>
							<div class="skeleton-badge small"></div>
							<div class="skeleton-text short"></div>
						</div>
						<div class="skeleton-text wide"></div>
						<div class="skeleton-text full"></div>
					</div>
				{/each}
			</div>
		</div>
	{/each}
</div>

<style>
	.skeleton-container {
		display: flex;
		flex-direction: column;
		gap: 24px;
	}

	.skeleton-meta {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.skeleton-section {
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-default);
		border-radius: 12px;
		overflow: hidden;
	}

	.skeleton-header {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 16px 20px;
		background: var(--color-bg-subtle);
		border-bottom: 1px solid var(--color-border-default);
	}

	.skeleton-results {
		display: flex;
		flex-direction: column;
	}

	.skeleton-card {
		display: flex;
		flex-direction: column;
		gap: 10px;
		padding: 16px 20px;
		border-bottom: 1px solid var(--color-border-default);
	}

	.skeleton-card:last-child {
		border-bottom: none;
	}

	.skeleton-card-header {
		display: flex;
		align-items: center;
		gap: 10px;
	}

	/* Skeleton Elements */
	.skeleton-icon {
		width: 18px;
		height: 18px;
		border-radius: 4px;
		background: linear-gradient(
			90deg,
			var(--color-bg-muted) 25%,
			var(--color-bg-subtle) 50%,
			var(--color-bg-muted) 75%
		);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
	}

	.skeleton-badge {
		width: 60px;
		height: 22px;
		border-radius: 4px;
		background: linear-gradient(
			90deg,
			var(--color-bg-muted) 25%,
			var(--color-bg-subtle) 50%,
			var(--color-bg-muted) 75%
		);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
	}

	.skeleton-badge.small {
		width: 48px;
		height: 20px;
	}

	.skeleton-text {
		height: 14px;
		border-radius: 4px;
		background: linear-gradient(
			90deg,
			var(--color-bg-muted) 25%,
			var(--color-bg-subtle) 50%,
			var(--color-bg-muted) 75%
		);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
	}

	.skeleton-text.tiny {
		width: 40px;
	}

	.skeleton-text.short {
		width: 80px;
	}

	.skeleton-text.medium {
		width: 120px;
	}

	.skeleton-text.wide {
		width: 70%;
	}

	.skeleton-text.full {
		width: 100%;
	}

	@keyframes shimmer {
		0% {
			background-position: 200% 0;
		}
		100% {
			background-position: -200% 0;
		}
	}

	/* Mobile */
	@media (max-width: 768px) {
		.skeleton-header,
		.skeleton-card {
			padding: 12px 16px;
		}
	}
</style>
