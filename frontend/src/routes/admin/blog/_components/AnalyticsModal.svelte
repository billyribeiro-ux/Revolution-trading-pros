<script lang="ts">
	import { IconX } from '#lib/icons/index.js';

	type Props = {
		open: boolean;
		post: {
			title: string;
			analytics?: {
				views: number;
				unique_visitors: number;
				avg_time: number;
				bounce_rate: number;
				comments: number;
				shares: number;
				likes: number;
				ctr: number;
			};
		} | null;
		formatNumber: (n: number) => string;
		onClose: () => void;
	};

	const { open, post, formatNumber, onClose }: Props = $props();
</script>

{#if open && post}
	<div
		class="modal-overlay"
		role="button"
		tabindex="0"
		onclick={onClose}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && onClose()}
	>
		<div
			class="modal analytics-modal"
			role="dialog"
			aria-modal="true"
			tabindex="-1"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
		>
			<div class="modal-header">
				<h2>Analytics: {post.title}</h2>
				<button class="btn-icon" onclick={onClose}>
					<IconX size={20} />
				</button>
			</div>
			<div class="modal-content">
				{#if post.analytics}
					<div class="analytics-grid">
						<div class="analytics-card">
							<h3>Performance</h3>
							<div class="metric-row">
								<span>Total Views</span>
								<strong>{formatNumber(post.analytics.views)}</strong>
							</div>
							<div class="metric-row">
								<span>Unique Visitors</span>
								<strong>{formatNumber(post.analytics.unique_visitors)}</strong>
							</div>
							<div class="metric-row">
								<span>Avg. Time on Page</span>
								<strong>{post.analytics.avg_time}s</strong>
							</div>
							<div class="metric-row">
								<span>Bounce Rate</span>
								<strong>{post.analytics.bounce_rate}%</strong>
							</div>
						</div>

						<div class="analytics-card">
							<h3>Engagement</h3>
							<div class="metric-row">
								<span>Comments</span>
								<strong>{post.analytics.comments}</strong>
							</div>
							<div class="metric-row">
								<span>Shares</span>
								<strong>{post.analytics.shares}</strong>
							</div>
							<div class="metric-row">
								<span>Likes</span>
								<strong>{post.analytics.likes}</strong>
							</div>
							<div class="metric-row">
								<span>CTR</span>
								<strong>{post.analytics.ctr}%</strong>
							</div>
						</div>
					</div>
				{:else}
					<div class="loading">Loading analytics...</div>
				{/if}
			</div>
		</div>
	</div>
{/if}

<style>
	.modal-overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: rgba(0, 0, 0, 0.8);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
	}

	.modal {
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 12px;
		width: 90%;
		max-width: 600px;
		max-height: 90vh;
		overflow: hidden;
		display: flex;
		flex-direction: column;
	}

	.modal.analytics-modal {
		max-width: 800px;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem;
		border-bottom: 1px solid var(--border-default, #30363d);
	}

	.modal-header h2 {
		font-size: 1.5rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
		margin: 0;
	}

	.modal-content {
		flex: 1;
		padding: 1.5rem;
		overflow-y: auto;
	}

	.analytics-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
		gap: 1.5rem;
	}

	.analytics-card {
		background: var(--bg-base, #0d1117);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
		padding: 1.5rem;
	}

	.analytics-card h3 {
		font-size: 1.1rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
		margin: 0 0 1rem 0;
	}

	.metric-row {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.75rem 0;
		border-bottom: 1px solid var(--border-muted, #21262d);
	}

	.metric-row:last-child {
		border-bottom: none;
	}

	.metric-row span {
		color: var(--text-secondary, #8b949e);
	}

	.metric-row strong {
		color: var(--text-primary, #f0f6fc);
		font-weight: 600;
	}

	.loading {
		color: var(--text-secondary, #8b949e);
		text-align: center;
		padding: 2rem;
	}

	.btn-icon {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem;
		border-radius: 8px;
		font-weight: 500;
		border: none;
		cursor: pointer;
		transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
		background: var(--admin-btn-bg);
		color: var(--admin-text-secondary);
	}

	.btn-icon:hover {
		background: var(--admin-btn-bg-hover);
		color: var(--admin-accent-primary);
	}

	@media (max-width: 380px) {
		.modal-content {
			padding: 1rem;
			margin: 0.5rem;
			max-width: calc(100vw - 1rem);
		}
	}

	@media (hover: none) and (pointer: coarse) {
		.btn-icon {
			min-height: 44px;
			min-width: 44px;
		}
	}

	@media (prefers-contrast: high) {
		.modal-content {
			border-width: 2px;
		}
	}
</style>
