<script lang="ts">
	import { IconX } from '$lib/icons';

	type Props = {
		post: { title: string; slug: string } | null;
		onClose: () => void;
	};

	const { post, onClose }: Props = $props();
</script>

{#if post}
	<div
		class="modal-overlay"
		role="button"
		tabindex="0"
		onclick={onClose}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && onClose()}
	>
		<div
			class="modal preview-modal"
			role="dialog"
			aria-modal="true"
			tabindex="-1"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
		>
			<div class="modal-header">
				<h2>Preview: {post.title}</h2>
				<button class="btn-icon" onclick={onClose}>
					<IconX size={20} />
				</button>
			</div>
			<div class="modal-content">
				<!-- FIX-2026-04-26 (P1-9): URL-encode the slug before interpolating
				     to prevent a stray `?`/`&`/`<` in DB-stored slug from
				     hijacking the iframe URL. -->
				<iframe
					src="/blog/{encodeURIComponent(post.slug)}?preview=true"
					title="Post Preview"
				></iframe>
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
		background: rgba(30, 41, 59, 0.98);
		border: 1px solid rgba(148, 163, 184, 0.3);
		border-radius: 12px;
		width: 90%;
		max-width: 600px;
		max-height: 90vh;
		overflow: hidden;
		display: flex;
		flex-direction: column;
	}

	.modal.preview-modal {
		max-width: 1200px;
		height: 80vh;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.2);
	}

	.modal-header h2 {
		font-size: 1.5rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.modal-content {
		flex: 1;
		padding: 1.5rem;
		overflow-y: auto;
	}

	.modal-content iframe {
		width: 100%;
		height: 100%;
		border: none;
		border-radius: 8px;
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
