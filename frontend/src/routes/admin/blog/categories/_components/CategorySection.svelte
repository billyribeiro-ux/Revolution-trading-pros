<script lang="ts">
	/**
	 * R28-C extraction (2026-05-20): categories list panel — section header,
	 * bulk-delete button, empty-state, skeleton-loaders, and the row list.
	 * Row markup is kept inline (per R8-C: avoid further splitting once a
	 * component would exceed >5 props + >5 callbacks; the row would).
	 *
	 * Callback props use the discriminated-callback style (one verb per
	 * callback) so the parent's intent is explicit at the call-site.
	 */
	import { fade } from 'svelte/transition';
	import {
		IconPlus,
		IconEdit,
		IconTrash,
		IconFolder,
		IconChartBar,
		IconEyeOff,
		IconCopy
	} from '$lib/icons';
	import type { Category } from '$lib/api/admin';

	interface Props {
		categories: Category[];
		loading: boolean;
		selected: Set<number>;
		onAdd: () => void;
		onEdit: (category: Category) => void;
		onDelete: (id: number) => void;
		onCopySlug: (slug: string) => void;
		onBulkDelete: () => void;
		onToggleSelect: (id: number, checked: boolean) => void;
	}

	let {
		categories,
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
			<IconFolder size={24} />
			<h2>Categories</h2>
			<span class="count-badge">{categories.length}</span>
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
				Add Category
			</button>
		</div>
	</div>

	<div class="items-list">
		{#if loading}
			{#each Array(3) as _, i (i)}
				<div class="skeleton-card"></div>
			{/each}
		{:else if categories.length === 0}
			<div class="empty-state">
				<IconFolder size={48} />
				<p>No categories found</p>
				<button class="btn-primary" onclick={onAdd}> Create your first category </button>
			</div>
		{:else}
			{#each categories as category (category.id)}
				<div class="item-card" transition:fade>
					<label class="checkbox-wrapper">
						<!-- FIX-2026-04-26 (P3-2): unique id per row; previously every row
						     shared id="page-checkbox" — invalid HTML, breaks <label for=…>. -->
						<!-- FIX-2026-04-26 (P2-2): drop self-assignment; Set mutations are
						     reactive in Svelte 5. Parent reassigns with `new Set(...)`. -->
						<input
							id="cat-checkbox-{category.id}"
							name="cat-checkbox-{category.id}"
							type="checkbox"
							checked={selected.has(category.id)}
							onchange={(e: Event) =>
								onToggleSelect(category.id, (e.currentTarget as HTMLInputElement).checked)}
						/>
					</label>
					<div class="item-color" style:background={category.color}></div>
					<div class="item-info">
						<div class="item-header">
							<h3>{category.name}</h3>
							{#if !category.is_visible}
								<span class="badge badge-gray">
									<IconEyeOff size={12} />
									Hidden
								</span>
							{/if}
						</div>
						{#if category.description}
							<p class="item-description">{category.description}</p>
						{/if}
						<div class="item-meta">
							<span class="item-count">
								<IconChartBar size={14} />
								{category.post_count} posts
							</span>
							<span class="item-slug">/{category.slug}</span>
						</div>
					</div>
					<div class="item-actions">
						<button class="action-btn" onclick={() => onEdit(category)} title="Edit">
							<IconEdit size={18} />
						</button>
						<button class="action-btn" onclick={() => onCopySlug(category.slug)} title="Copy slug">
							<IconCopy size={18} />
						</button>
						<button class="action-btn danger" onclick={() => onDelete(category.id)} title="Delete">
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
		display: flex;
		align-items: center;
		gap: 0.5rem;
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

	.empty-state {
		text-align: center;
		padding: 3rem 1rem;
		color: #64748b;
	}

	.empty-state p {
		color: #64748b;
		margin: 1rem 0;
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
