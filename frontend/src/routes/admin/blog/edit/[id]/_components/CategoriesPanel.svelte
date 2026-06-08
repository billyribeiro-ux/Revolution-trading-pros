<script lang="ts">
	import IconCheck from '@tabler/icons-svelte-runes/icons/check';
	import IconTags from '@tabler/icons-svelte-runes/icons/tags';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import { predefinedCategories, getPredefinedCategoryById } from '$lib/data/predefined-categories';
	import type { PostState } from './types';

	type Props = {
		post: PostState;
	};

	let { post = $bindable() }: Props = $props();

	function toggle(categoryId: string) {
		const index = post.categories.indexOf(categoryId);
		if (index === -1) {
			post.categories = [...post.categories, categoryId];
		} else {
			post.categories = post.categories.filter((c) => c !== categoryId);
		}
	}

	function isSelected(categoryId: string): boolean {
		return post.categories.includes(categoryId);
	}
</script>

<div class="sidebar-panel categories-panel">
	<h3>
		<IconTags size={16} />
		Categories
	</h3>
	<div class="categories-grid">
		{#each predefinedCategories as category (category.id)}
			<button
				type="button"
				class={{ 'category-btn': true, selected: isSelected(category.id) }}
				style:--tag-color={category.color}
				onclick={() => toggle(category.id)}
			>
				{#if isSelected(category.id)}
					<IconCheck size={14} />
				{/if}
				{category.name}
			</button>
		{/each}
	</div>

	{#if post.categories.length > 0}
		<div class="selected-categories">
			<span class="selected-count">{post.categories.length} selected:</span>
			{#each post.categories as categoryId (categoryId)}
				{const category = getPredefinedCategoryById(categoryId)}
				{#if category}
					<span class="selected-tag" style:--tag-color={category.color}>
						{category.name}
						<button
							type="button"
							onclick={() => toggle(categoryId)}
							aria-label="Remove {category.name}"
						>
							<IconX size={12} />
						</button>
					</span>
				{/if}
			{/each}
		</div>
	{/if}
</div>

<style>
	.sidebar-panel {
		background: white;
		padding: 1.5rem;
		border-radius: 8px;
		border: 1px solid #e5e5e5;
	}

	.sidebar-panel h3 {
		font-size: 1rem;
		font-weight: 600;
		color: #1a1a1a;
		margin: 0 0 1rem;
		padding-bottom: 0.75rem;
		border-bottom: 1px solid #f0f0f0;
	}

	.categories-panel h3 {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.categories-grid {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		padding: 0.75rem;
		background: #f8f9fa;
		border: 1px solid #e5e5e5;
		border-radius: 8px;
		max-height: 280px;
		overflow-y: auto;
	}

	.category-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 0.75rem;
		background: rgba(100, 116, 139, 0.1);
		border: 1px solid rgba(100, 116, 139, 0.2);
		border-radius: 6px;
		font-size: 0.8rem;
		font-weight: 500;
		color: #64748b;
		cursor: pointer;
		transition: all 0.15s;
	}

	.category-btn:hover {
		background: color-mix(in srgb, var(--tag-color, #6366f1) 15%, transparent);
		border-color: color-mix(in srgb, var(--tag-color, #6366f1) 30%, transparent);
		color: var(--tag-color, #6366f1);
	}

	.category-btn.selected {
		background: color-mix(in srgb, var(--tag-color, #6366f1) 20%, transparent);
		border-color: var(--tag-color, #6366f1);
		color: var(--tag-color, #6366f1);
	}

	.selected-categories {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		margin-top: 0.75rem;
		padding: 0.75rem;
		background: rgba(59, 130, 246, 0.05);
		border: 1px solid rgba(59, 130, 246, 0.1);
		border-radius: 8px;
	}

	.selected-count {
		font-size: 0.75rem;
		color: #64748b;
		margin-right: 0.5rem;
		width: 100%;
	}

	.selected-tag {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.625rem;
		background: color-mix(in srgb, var(--tag-color, #6366f1) 15%, transparent);
		border: 1px solid color-mix(in srgb, var(--tag-color, #6366f1) 30%, transparent);
		border-radius: 4px;
		font-size: 0.75rem;
		font-weight: 500;
		color: var(--tag-color, #6366f1);
	}

	.selected-tag button {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0;
		background: transparent;
		border: none;
		color: inherit;
		cursor: pointer;
		opacity: 0.7;
		transition: opacity 0.15s;
	}

	.selected-tag button:hover {
		opacity: 1;
	}
</style>
