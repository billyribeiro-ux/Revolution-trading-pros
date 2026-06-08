<script lang="ts">
	import IconPlus from '@tabler/icons-svelte-runes/icons/plus';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import type { PostState, TagRow } from './types';

	type Props = {
		post: PostState;
		availableTags: TagRow[];
		tagsLoading: boolean;
		newTag: string;
		onCreateTag: () => void;
	};

	let {
		post = $bindable(),
		availableTags,
		tagsLoading,
		newTag = $bindable(),
		onCreateTag
	}: Props = $props();
</script>

<div class="sidebar-panel">
	<h3>Tags</h3>

	<div class="tags-selected">
		{#each post.tags as tagId (tagId)}
			{const tag = availableTags.find((t) => t.id === tagId)}
			{#if tag}
				<span class="tag-badge" style:background={`${tag.color}20`} style:color={tag.color}>
					{tag.name}
					<button
						type="button"
						onclick={() => (post.tags = post.tags.filter((id) => id !== tagId))}
					>
						<IconX size={14} />
					</button>
				</span>
			{/if}
		{/each}
	</div>

	<div class="tag-input-group">
		<input
			id="page-newtag"
			name="page-newtag"
			type="text"
			bind:value={newTag}
			placeholder="Add new tag..."
			onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && (e.preventDefault(), onCreateTag())}
		/>
		<button type="button" class="btn-add-tag" onclick={onCreateTag}>
			<IconPlus size={16} />
		</button>
	</div>

	<div class="checkbox-list">
		{#if tagsLoading}
			<!-- FIX-2026-04-26 (P3-4): show explicit loading state for tags
			     so an empty availableTags list mid-load isn't confused with
			     "no tags exist". -->
			<p class="help-text">Loading tags…</p>
		{:else if availableTags.length === 0}
			<p class="help-text">No tags yet — type a name above to create one.</p>
		{:else}
			{#each availableTags as tag (tag.id)}
				{#if !post.tags.includes(tag.id)}
					<label class="checkbox-item">
						<!-- FIX-2026-04-26 (P3-2): unique id per tag row instead of
						     duplicate id="page-checkbox". -->
						<input
							id="tag-checkbox-{tag.id}"
							name="tag-checkbox-{tag.id}"
							type="checkbox"
							value={tag.id}
							onchange={(e: Event) => {
								if ((e.currentTarget as HTMLInputElement).checked) {
									post.tags = [...post.tags, tag.id];
								}
							}}
						/>
						<span style:color={tag.color}>{tag.name}</span>
					</label>
				{/if}
			{/each}
		{/if}
	</div>
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

	.sidebar-panel input[type='text'] {
		width: 100%;
		padding: 0.625rem;
		border: 1px solid #e5e5e5;
		border-radius: 6px;
		font-size: 0.9rem;
		color: #1a1a1a;
		background: white;
	}

	.sidebar-panel input[type='checkbox'] {
		margin-right: 0.5rem;
	}

	.checkbox-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		max-height: 200px;
		overflow-y: auto;
	}

	.checkbox-item {
		display: flex;
		align-items: center;
		padding: 0.5rem;
		border-radius: 4px;
		cursor: pointer;
	}

	.checkbox-item:hover {
		background: #f8f9fa;
	}

	.tags-selected {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		margin-bottom: 1rem;
	}

	.tag-badge {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.375rem 0.75rem;
		border-radius: 4px;
		font-size: 0.85rem;
		font-weight: 500;
	}

	.tag-badge button {
		background: none;
		border: none;
		padding: 0;
		cursor: pointer;
		display: flex;
		align-items: center;
	}

	.tag-input-group {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 1rem;
	}

	.tag-input-group input {
		flex: 1;
	}

	.btn-add-tag {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		background: #3b82f6;
		color: white;
		border: none;
		border-radius: 6px;
		cursor: pointer;
	}

	.btn-add-tag:hover {
		background: #2563eb;
	}
</style>
