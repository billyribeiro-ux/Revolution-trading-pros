<script lang="ts">
	import { IconCode } from '$lib/icons';

	interface EmbedData {
		video_id: number;
		title: string;
		embed_html: string;
	}

	interface Props {
		open: boolean;
		data: EmbedData | null;
		onClose: () => void;
		onCopy: () => void;
	}

	let { open, data, onClose, onCopy }: Props = $props();
</script>

{#if open && data}
	<div
		class="modal-overlay"
		role="button"
		tabindex="0"
		onclick={onClose}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && onClose()}
	>
		<div
			class="modal"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
			role="dialog"
			aria-modal="true"
			tabindex="-1"
		>
			<div class="modal-header">
				<h2>
					<IconCode size={24} />
					Embed Code
				</h2>
				<button class="modal-close" onclick={onClose} type="button" aria-label="Close">&times;</button>
			</div>
			<div class="modal-body">
				<p class="embed-title"><strong>{data.title}</strong></p>
				<div class="embed-code-box">
					<pre><code>{data.embed_html}</code></pre>
				</div>
				<p class="embed-hint">Copy this code and paste it into your website to embed this video.</p>
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={onClose} type="button">Close</button>
				<button class="btn-primary" onclick={onCopy}>
					<IconCode size={16} />
					Copy to Clipboard
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	/* Modal scaffold (duplicated from parent — still used by other modals there) */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.7);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 100;
		padding: 1rem;
	}

	.modal {
		background: var(--bg-elevated);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 16px;
		width: 100%;
		max-width: 600px;
		max-height: 90vh;
		overflow: auto;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.25rem;
		border-bottom: 1px solid rgba(230, 184, 0, 0.1);
	}

	.modal-header h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--text-primary);
		margin: 0;
	}

	.modal-close {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: none;
		color: var(--text-tertiary);
		font-size: 1.5rem;
		cursor: pointer;
		border-radius: 8px;
		transition: all 0.2s;
	}

	.modal-close:hover {
		background: rgba(230, 184, 0, 0.1);
		color: var(--text-primary);
	}

	.modal-body {
		padding: 1.25rem;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1.25rem;
		border-top: 1px solid rgba(230, 184, 0, 0.1);
	}

	/* Buttons (duplicated from parent) */
	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: linear-gradient(135deg, #E6B800 0%, #B38F00 100%);
		border: none;
		border-radius: 10px;
		color: #0d1117;
		font-size: 0.875rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(230, 184, 0, 0.3);
	}

	.btn-primary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: rgba(100, 116, 139, 0.1);
		border: 1px solid rgba(100, 116, 139, 0.2);
		border-radius: 10px;
		color: var(--text-secondary);
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover {
		background: rgba(100, 116, 139, 0.2);
		color: var(--text-primary);
	}

	/* Embed-specific */
	.embed-title {
		margin-bottom: 1rem;
		color: var(--text-primary);
	}

	.embed-code-box {
		background: rgba(13, 17, 23, 0.8);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 8px;
		padding: 1rem;
		overflow-x: auto;
	}

	.embed-code-box pre {
		margin: 0;
		white-space: pre-wrap;
		word-break: break-all;
	}

	.embed-code-box code {
		font-family: 'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, monospace;
		font-size: 0.8rem;
		color: var(--primary-500);
	}

	.embed-hint {
		margin-top: 1rem;
		font-size: 0.85rem;
		color: var(--text-tertiary);
	}
</style>
