<script lang="ts">
	/**
	 * R28-C extraction (2026-05-20): tags list panel — mirrors
	 * `CategorySection.svelte` but for the simpler `Tag` shape (no
	 * description). Kept as a sibling component rather than parameterizing
	 * the category one to keep the icons / labels / strings explicit at the
	 * call-site (Svelte 5's compiler benefits when slot content is static).
	 */
	import { fade } from 'svelte/transition';
	import {
		IconPlus,
		IconEdit,
		IconTrash,
		IconTag,
		IconChartBar,
		IconEyeOff,
		IconCopy
	} from '#lib/icons/index.js';
	import type { Tag } from '#lib/api/admin.js';

	interface Props {
		tags: Tag[];
		loading: boolean;
		selected: Set<number>;
		onAdd: () => void;
		onEdit: (tag: Tag) => void;
		onDelete: (id: number) => void;
		onCopySlug: (slug: string) => void;
		onBulkDelete: () => void;
		onToggleSelect: (id: number, checked: boolean) => void;
	}

	let {
		tags,
		loading,
		selected,
		onAdd,
		onEdit,
		onDelete,
		onCopySlug,
		onBulkDelete,
		onToggleSelect
	}: Props = $props();
</script>

<div class="section">
	<div class="section-header">
		<div class="section-title">
			<IconTag size={24} />
			<h2>Tags</h2>
			<span class="count-badge">{tags.length}</span>
		</div>
		<div class="section-actions">
			{#if selected.size > 0}
				<button class="btn-danger-ghost" onclick={onBulkDelete}>
					<IconTrash size={16} />
					Delete ({selected.size})
				</button>
			{/if}
			<button class="btn-primary" onclick={onAdd}>
				<IconPlus size={18} />
				Add Tag
			</button>
		</div>
	</div>

	<div class="items-list">
		{#if loading}
			{#each Array(3) as _, i (i)}
				<div class="skeleton-card"></div>
			{/each}
		{:else if tags.length === 0}
			<div class="empty-state">
				<IconTag size={48} />
				<p>No tags found</p>
				<button class="btn-primary" onclick={onAdd}> Create your first tag </button>
			</div>
		{:else}
			{#each tags as tag (tag.id)}
				<div class="item-card" transition:fade>
					<label class="checkbox-wrapper">
						<!-- FIX-2026-04-26 (P3-2): unique id per row. -->
						<!-- FIX-2026-04-26 (P2-2): parent reassigns with `new Set(...)`. -->
						<input
							id="tag-checkbox-{tag.id}"
							name="tag-checkbox-{tag.id}"
							type="checkbox"
							checked={selected.has(tag.id)}
							onchange={(e: Event) =>
								onToggleSelect(tag.id, (e.currentTarget as HTMLInputElement).checked)}
						/>
					</label>
					<div class="item-color" style:background={tag.color}></div>
					<div class="item-info">
						<div class="item-header">
							<h3>{tag.name}</h3>
							{#if !tag.is_visible}
								<span class="badge badge-gray">
									<IconEyeOff size={12} />
									Hidden
								</span>
							{/if}
						</div>
						<div class="item-meta">
							<span class="item-count">
								<IconChartBar size={14} />
								{tag.post_count} posts
							</span>
							<span class="item-slug">/{tag.slug}</span>
						</div>
					</div>
					<div class="item-actions">
						<button class="action-btn" onclick={() => onEdit(tag)} title="Edit">
							<IconEdit size={18} />
						</button>
						<button class="action-btn" onclick={() => onCopySlug(tag.slug)} title="Copy slug">
							<IconCopy size={18} />
						</button>
						<button class="action-btn danger" onclick={() => onDelete(tag.id)} title="Delete">
							<IconTrash size={18} />
						</button>
					</div>
				</div>
			{/each}
		{/if}
	</div>
</div>

<style>
	.section {
		background: var(--bg-elevated, #161b22);
		border-radius: 8px;
		border: 1px solid var(--border-default, #30363d);
		padding: 1.5rem;
	}

	.section-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.5rem;
		padding-bottom: 1rem;
		border-bottom: 1px solid var(--border-default, #30363d);
	}

	.section-title {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: var(--text-secondary, #8b949e);
	}

	.section-title h2 {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
		margin: 0;
	}

	.section-actions {
		display: flex;
		gap: 0.5rem;
	}

	.count-badge {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-width: 24px;
		height: 24px;
		padding: 0 0.5rem;
		background: var(--bg-hover, #252b33);
		border-radius: 12px;
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--text-secondary, #8b949e);
	}

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background: var(--primary-500);
		color: var(--bg-base, #0d1117);
		border: none;
		border-radius: 6px;
		font-weight: 500;
		font-size: 0.9rem;
		cursor: pointer;
		transition: all 0.2s;
	}


	.btn-danger-ghost {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background: rgba(248, 81, 73, 0.1);
		color: var(--error-emphasis, #f85149);
		border: 1px solid rgba(248, 81, 73, 0.3);
		border-radius: 6px;
		font-weight: 500;
		font-size: 0.9rem;
		cursor: pointer;
	}

	.btn-danger-ghost:hover {
		background: rgba(248, 81, 73, 0.2);
	}

	.items-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.item-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem;
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
		transition: all 0.2s;
		background: var(--bg-base, #0d1117);
	}

	.item-card:hover {
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
		border-color: var(--border-default, #30363d);
	}

	.checkbox-wrapper {
		display: flex;
		align-items: center;
		cursor: pointer;
	}

	.checkbox-wrapper input[type='checkbox'] {
		width: 18px;
		height: 18px;
		cursor: pointer;
	}

	.item-color {
		width: 40px;
		height: 40px;
		border-radius: 8px;
		flex-shrink: 0;
	}

	.item-info {
		flex: 1;
	}

	.item-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 0.25rem;
	}

	.item-info h3 {
		font-size: 1rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
		margin: 0 0 0.25rem;
	}

	.item-meta {
		display: flex;
		align-items: center;
		gap: 1rem;
		font-size: 0.8rem;
		color: var(--text-tertiary, #6e7681);
	}

	.item-count {
		display: flex;
		align-items: center;
		gap: 0.25rem;
	}

	.item-slug {
		font-family: monospace;
		background: var(--bg-hover, #252b33);
		padding: 0.125rem 0.5rem;
		border-radius: 4px;
		color: var(--text-secondary, #8b949e);
	}

	.item-actions {
		display: flex;
		gap: 0.5rem;
	}

	.action-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		background: var(--bg-hover, #252b33);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 6px;
		color: var(--text-secondary, #8b949e);
		cursor: pointer;
		transition: all 0.2s;
	}

	.action-btn:hover {
		background: var(--bg-hover, #252b33);
		color: var(--text-primary, #f0f6fc);
	}

	.action-btn.danger {
		color: var(--error-emphasis, #f85149);
		border-color: rgba(248, 113, 113, 0.3);
	}

	.action-btn.danger:hover {
		background: rgba(248, 81, 73, 0.2);
	}

	.badge {
		display: inline-flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.25rem 0.5rem;
		border-radius: 4px;
		font-size: 0.75rem;
		font-weight: 500;
	}

	.badge-gray {
		background: var(--bg-hover, #252b33);
		color: var(--text-secondary, #8b949e);
	}

	.empty-state {
		text-align: center;
		padding: 3rem 1rem;
		color: var(--text-tertiary, #6e7681);
	}

	.empty-state p {
		color: var(--text-tertiary, #6e7681);
		margin: 1rem 0;
	}

	.skeleton-card {
		height: 80px;
		background: linear-gradient(
			90deg,
			var(--bg-surface, #1c2128) 25%,
			var(--bg-hover, #252b33) 50%,
			var(--bg-surface, #1c2128) 75%
		);
		background-size: 200% 100%;
		animation: loading 1.5s infinite;
		border-radius: 8px;
	}

	@keyframes loading {
		0% {
			background-position: 200% 0;
		}
		100% {
			background-position: -200% 0;
		}
	}
</style>
