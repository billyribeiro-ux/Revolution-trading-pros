<script lang="ts">
	import { IconX, IconCheck, IconPlus } from '#lib/icons/index.js';

	interface Props {
		open: boolean;
		tags: string[];
		availableTags: string[];
		newTag: string;
		onNewTagChange: (value: string) => void;
		onToggleTag: (tag: string) => void;
		onAddCustomTag: () => void;
		onClose: () => void;
	}

	let {
		open,
		tags,
		availableTags,
		newTag,
		onNewTagChange,
		onToggleTag,
		onAddCustomTag,
		onClose
	}: Props = $props();
</script>

{#if open}
	<div
		class="modal-overlay"
		onclick={(e: MouseEvent) => {
			if (e.target === e.currentTarget) onClose();
		}}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && onClose()}
		role="dialog"
		tabindex="-1"
		aria-modal="true"
	>
		<div class="modal-content" role="document">
			<div class="modal-header">
				<h2>Manage Tags</h2>
				<button class="close-btn" onclick={onClose}>
					<IconX size={20} />
				</button>
			</div>
			<div class="modal-body">
				<div class="available-tags">
					<span class="tags-label">Available Tags</span>
					<div class="tags-grid">
						{#each availableTags as tag (tag)}
							<button
								class={['tag-option', { selected: tags.includes(tag) }]}
								onclick={() => onToggleTag(tag)}
							>
								{#if tags.includes(tag)}
									<IconCheck size={14} />
								{:else}
									<IconPlus size={14} />
								{/if}
								{tag}
							</button>
						{/each}
					</div>
				</div>
				<div class="custom-tag">
					<span class="tags-label">Custom Tag</span>
					<div class="custom-tag-input">
						<input autocomplete="off"
							id="page-newtag"
							name="page-newtag"
							type="text"
							value={newTag}
							oninput={(e) => onNewTagChange((e.target as HTMLInputElement).value)}
							placeholder="Enter custom tag..."
							onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && onAddCustomTag()}
						/>
						<button class="btn-primary small" onclick={onAddCustomTag} disabled={!newTag.trim()}>
							Add
						</button>
					</div>
				</div>
			</div>
			<div class="modal-footer">
				<button class="btn-primary" onclick={onClose}>Done</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.8);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 50;
		padding: 2rem;
	}

	.modal-content {
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 20px;
		width: 100%;
		max-width: 500px;
		max-height: 90vh;
		overflow-y: auto;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem;
		border-bottom: 1px solid var(--border-muted, #21262d);
	}

	.modal-header h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
		margin: 0;
	}

	.close-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--bg-surface, #1c2128);
		border: none;
		border-radius: 8px;
		color: var(--text-secondary, #8b949e);
		cursor: pointer;
	}

	.modal-body {
		padding: 1.5rem;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1.5rem;
		border-top: 1px solid var(--border-muted, #21262d);
	}

	.available-tags {
		margin-bottom: 1.5rem;
	}

	.tags-label {
		display: block;
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--text-secondary, #8b949e);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-bottom: 0.75rem;
	}

	.tags-grid {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.tag-option {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 0.875rem;
		background: var(--bg-surface, #1c2128);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
		font-size: 0.8125rem;
		color: var(--text-secondary, #8b949e);
		cursor: pointer;
		transition: all 0.2s;
	}

	.tag-option:hover {
		background: rgba(230, 184, 0, 0.1);
		border-color: rgba(230, 184, 0, 0.3);
		color: var(--primary-400);
	}

	.tag-option.selected {
		background: rgba(230, 184, 0, 0.2);
		border-color: rgba(230, 184, 0, 0.4);
		color: var(--primary-400);
	}

	.custom-tag-input {
		display: flex;
		gap: 0.5rem;
	}

	.custom-tag-input input {
		flex: 1;
		padding: 0.5rem 0.75rem;
		background: var(--bg-base, #0d1117);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
		color: var(--text-primary, #f0f6fc);
		font-size: 0.875rem;
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
		background: var(--primary-500);
		color: var(--bg-base);
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-primary.small {
		padding: 0.5rem 0.875rem;
		font-size: 0.8125rem;
	}
</style>
