<!--
	Callout Block - Alert/info box
	═══════════════════════════════════════════════════════════════════════════════

	@version 1.0.0
	@author Revolution Trading Pros
	@since January 2026
-->

<script lang="ts">
	import type { ContentBlock } from '$lib/stores/editor.svelte';
	import IconInfoCircle from '@tabler/icons-svelte/icons/info-circle';
	import IconAlertTriangle from '@tabler/icons-svelte/icons/alert-triangle';
	import IconCircleCheck from '@tabler/icons-svelte/icons/circle-check';
	import IconAlertCircle from '@tabler/icons-svelte/icons/alert-circle';
	import IconBulb from '@tabler/icons-svelte/icons/bulb';

	interface Props {
		block: ContentBlock;
		readonly?: boolean;
		onUpdate?: (data: Record<string, unknown>) => void;
	}

	let { block, readonly = false, onUpdate }: Props = $props();

	let data = $derived(block.data as { type?: string; title?: string; content?: string });

	const types = [
		{ id: 'info', label: 'Info', icon: IconInfoCircle, color: '#3b82f6' },
		{ id: 'warning', label: 'Warning', icon: IconAlertTriangle, color: '#f59e0b' },
		{ id: 'success', label: 'Success', icon: IconCircleCheck, color: '#10b981' },
		{ id: 'error', label: 'Error', icon: IconAlertCircle, color: '#ef4444' },
		{ id: 'tip', label: 'Tip', icon: IconBulb, color: '#8b5cf6' }
	];

	let currentType = $derived(types.find(t => t.id === (data.type ?? 'info')) ?? types[0]);
	let CalloutIcon = $derived(currentType.icon);

	function handleTypeChange(typeId: string) {
		onUpdate?.({ ...data, type: typeId });
	}

	function handleTitleChange(e: Event) {
		const target = e.target as HTMLInputElement;
		onUpdate?.({ ...data, title: target.value });
	}

	function handleContentChange(e: Event) {
		const target = e.target as HTMLTextAreaElement;
		onUpdate?.({ ...data, content: target.value });
	}
</script>

<div class="callout-block">
	{#if !readonly}
		<div class="type-selector">
			{#each types as type}
				{@const TypeIcon = type.icon}
				<button
					type="button"
					class="type-btn"
					class:active={data.type === type.id || (!data.type && type.id === 'info')}
					style:--type-color={type.color}
					onclick={() => handleTypeChange(type.id)}
				>
					<TypeIcon size={14} />
					{type.label}
				</button>
			{/each}
		</div>
	{/if}

	<div
		class="callout callout-{data.type ?? 'info'}"
		style:--callout-color={currentType.color}
	>
		<div class="callout-icon">
			<CalloutIcon size={20} />
		</div>
		<div class="callout-content">
			{#if readonly}
				{#if data.title}
					<strong class="callout-title">{data.title}</strong>
				{/if}
				<p class="callout-text">{data.content || ''}</p>
			{:else}
				<input
					type="text"
					class="title-input"
					value={data.title ?? ''}
					oninput={handleTitleChange}
					placeholder="Callout title (optional)..."
				/>
				<textarea
					class="content-input"
					value={data.content ?? ''}
					oninput={handleContentChange}
					placeholder="Callout content..."
					rows="2"
				></textarea>
			{/if}
		</div>
	</div>
</div>

<style>
	.callout-block {
		width: 100%;
	}

	.type-selector {
		display: flex;
		flex-wrap: wrap;
		gap: 0.25rem;
		margin-bottom: 0.75rem;
	}

	.type-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.25rem 0.5rem;
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid transparent;
		border-radius: 0.25rem;
		color: #64748b;
		font-size: 0.6875rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.type-btn:hover {
		background: color-mix(in srgb, var(--type-color) 15%, transparent);
		color: var(--type-color);
	}

	.type-btn.active {
		background: color-mix(in srgb, var(--type-color) 20%, transparent);
		border-color: color-mix(in srgb, var(--type-color) 30%, transparent);
		color: var(--type-color);
	}

	.callout {
		display: flex;
		gap: 0.75rem;
		padding: 1rem;
		background: color-mix(in srgb, var(--callout-color) 10%, transparent);
		border-left: 4px solid var(--callout-color);
		border-radius: 0.5rem;
	}

	.callout-icon {
		flex-shrink: 0;
		color: var(--callout-color);
	}

	.callout-content {
		flex: 1;
		min-width: 0;
	}

	.callout-title {
		display: block;
		margin-bottom: 0.25rem;
		color: #f1f5f9;
		font-weight: 600;
	}

	.callout-text {
		margin: 0;
		color: #cbd5e1;
		line-height: 1.5;
	}

	.title-input {
		width: 100%;
		padding: 0.375rem 0.5rem;
		margin-bottom: 0.5rem;
		background: rgba(0, 0, 0, 0.15);
		border: 1px solid rgba(51, 65, 85, 0.3);
		border-radius: 0.25rem;
		color: #f1f5f9;
		font-size: 0.9375rem;
		font-weight: 600;
	}

	.title-input:focus {
		outline: none;
		border-color: var(--callout-color);
	}

	.content-input {
		width: 100%;
		padding: 0.375rem 0.5rem;
		background: rgba(0, 0, 0, 0.15);
		border: 1px solid rgba(51, 65, 85, 0.3);
		border-radius: 0.25rem;
		color: #cbd5e1;
		font-size: 0.875rem;
		line-height: 1.5;
		resize: vertical;
	}

	.content-input:focus {
		outline: none;
		border-color: var(--callout-color);
	}
</style>
