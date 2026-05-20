<script lang="ts" generics="T extends { id: number; name: string; slug: string; color: string; is_visible: boolean; post_count: number; description?: string }">
	/**
	 * R27-E extraction (2026-05-20): generic categories-OR-tags list
	 * panel. Eliminates the 95% duplication between the
	 * `<!-- Categories Section -->` and `<!-- Tags Section -->`
	 * blocks of the original page.
	 *
	 * Parameterized by:
	 *   - `kind`        — 'category' | 'tag' (drives label + icon +
	 *                      empty-state CTA copy; the parent owns the
	 *                      icon choice via a `headerIcon` snippet)
	 *   - `headerIcon`  — snippet rendering the section icon
	 *   - `emptyIcon`   — snippet rendering the empty-state icon
	 *   - `items`       — filtered list, already filtered by the
	 *                      parent's $effect
	 *   - `selectedIds` — `$bindable` Set<number> — the parent reads
	 *                      `.size` for the bulk-delete button
	 *   - callbacks    — onEdit / onDelete / onBulkDelete /
	 *                      onCreate
	 *
	 * Incidental fix preserved from original: each row gets a unique
	 * `{kind}-checkbox-{id}` id (the pre-extraction code had already
	 * fixed this for P3-2; the comment is forwarded).
	 */
	import { fade } from 'svelte/transition';
	import {
		IconChartBar,
		IconCopy,
		IconEdit,
		IconEyeOff,
		IconPlus,
		IconTrash
	} from '$lib/icons';
	import type { Snippet } from 'svelte';

	interface Props {
		kind: 'category' | 'tag';
		label: string;
		items: T[];
		loading: boolean;
		selectedIds: Set<number>;
		headerIcon: Snippet;
		emptyIcon: Snippet;
		onCreate: () => void;
		onEdit: (item: T) => void;
		onDelete: (id: number) => void;
		onBulkDelete: () => void;
	}

	let {
		kind,
		label,
		items,
		loading,
		selectedIds = $bindable(),
		headerIcon,
		emptyIcon,
		onCreate,
		onEdit,
		onDelete,
		onBulkDelete
	}: Props = $props();

	function toggleSelection(id: number, checked: boolean) {
		// FIX-2026-04-26 (P2-2): Set mutations are reactive in Svelte 5,
		// but reassigning with a fresh Set is the documented idiom and
		// keeps the diff explicit at the call site.
		if (checked) {
			selectedIds.add(id);
		} else {
			selectedIds.delete(id);
		}
		selectedIds = new Set(selectedIds);
	}

	function copySlug(slug: string) {
		navigator.clipboard.writeText(slug);
	}
</script>

<div class="section">
	<div class="section-header">
		<div class="section-title">
			{@render headerIcon()}
			<h2>{label}</h2>
			<span class="count-badge">{items.length}</span>
		</div>
		<div class="section-actions">
			{#if selectedIds.size > 0}
				<button class="btn-danger-ghost" onclick={onBulkDelete}>
					<IconTrash size={16} />
					Delete ({selectedIds.size})
				</button>
			{/if}
			<button class="btn-primary" onclick={onCreate}>
				<IconPlus size={18} />
				Add {label.replace(/s$/, '')}
			</button>
		</div>
	</div>

	<div class="items-list">
		{#if loading}
			{#each Array(3) as _, i (i)}
				<div class="skeleton-card"></div>
			{/each}
		{:else if items.length === 0}
			<div class="empty-state">
				{@render emptyIcon()}
				<p>No {label.toLowerCase()} found</p>
				<button class="btn-primary" onclick={onCreate}>
					Create your first {label.toLowerCase().replace(/s$/, '')}
				</button>
			</div>
		{:else}
			{#each items as item (item.id)}
				<div class="item-card" transition:fade>
					<label class="checkbox-wrapper">
						<input
							id="{kind}-checkbox-{item.id}"
							name="{kind}-checkbox-{item.id}"
							type="checkbox"
							checked={selectedIds.has(item.id)}
							onchange={(e: Event) =>
								toggleSelection(item.id, (e.currentTarget as HTMLInputElement).checked)}
						/>
					</label>
					<div class="item-color" style="background: {item.color}"></div>
					<div class="item-info">
						<div class="item-header">
							<h3>{item.name}</h3>
							{#if !item.is_visible}
								<span class="badge badge-gray">
									<IconEyeOff size={12} />
									Hidden
								</span>
							{/if}
						</div>
						{#if 'description' in item && item.description}
							<p class="item-description">{item.description}</p>
						{/if}
						<div class="item-meta">
							<span class="item-count">
								<IconChartBar size={14} />
								{item.post_count} posts
							</span>
							<span class="item-slug">/{item.slug}</span>
						</div>
					</div>
					<div class="item-actions">
						<button class="action-btn" onclick={() => onEdit(item)} title="Edit">
							<IconEdit size={18} />
						</button>
						<button class="action-btn" onclick={() => copySlug(item.slug)} title="Copy slug">
							<IconCopy size={18} />
						</button>
						<button class="action-btn danger" onclick={() => onDelete(item.id)} title="Delete">
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
		background: rgba(30, 41, 59, 0.4);
		border-radius: 8px;
		border: 1px solid rgba(148, 163, 184, 0.2);
		padding: 1.5rem;
	}

	.section-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.5rem;
		padding-bottom: 1rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.2);
	}

	.section-title {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: #94a3b8;
	}

	.section-title h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
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
		background: rgba(100, 116, 139, 0.3);
		border-radius: 12px;
		font-size: 0.75rem;
		font-weight: 600;
		color: #94a3b8;
	}

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		color: white;
		border: none;
		border-radius: 6px;
		font-weight: 500;
		font-size: 0.9rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 8px 20px rgba(99, 102, 241, 0.4);
	}

	.btn-danger-ghost {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 6px;
		font-weight: 500;
		font-size: 0.9rem;
		cursor: pointer;
	}

	.btn-danger-ghost:hover {
		background: rgba(239, 68, 68, 0.2);
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
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		transition: all 0.2s;
		background: rgba(15, 23, 42, 0.4);
	}

	.item-card:hover {
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
		border-color: rgba(148, 163, 184, 0.3);
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
		color: #f1f5f9;
		margin: 0 0 0.25rem;
	}

	.item-description {
		font-size: 0.85rem;
		color: #94a3b8;
		margin: 0.25rem 0;
	}

	.item-meta {
		display: flex;
		align-items: center;
		gap: 1rem;
		font-size: 0.8rem;
		color: #64748b;
	}

	.item-count {
		display: flex;
		align-items: center;
		gap: 0.25rem;
	}

	.item-slug {
		font-family: monospace;
		background: rgba(100, 116, 139, 0.2);
		padding: 0.125rem 0.5rem;
		border-radius: 4px;
		color: #94a3b8;
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
		background: rgba(100, 116, 139, 0.2);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 6px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.action-btn:hover {
		background: rgba(100, 116, 139, 0.3);
		color: #f1f5f9;
	}

	.action-btn.danger {
		color: #f87171;
		border-color: rgba(248, 113, 113, 0.3);
	}

	.action-btn.danger:hover {
		background: rgba(239, 68, 68, 0.2);
	}

	.empty-state {
		text-align: center;
		padding: 3rem 1rem;
		color: #64748b;
	}

	.empty-state p {
		color: #64748b;
		margin: 1rem 0;
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
		background: rgba(100, 116, 139, 0.3);
		color: #94a3b8;
	}

	.skeleton-card {
		height: 80px;
		background: linear-gradient(
			90deg,
			rgba(148, 163, 184, 0.1) 25%,
			rgba(148, 163, 184, 0.2) 50%,
			rgba(148, 163, 184, 0.1) 75%
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
