<script lang="ts">
	/**
	 * EmptyState - Consistent empty state display
	 * 
	 * @version 2.0.0
	 * @author Revolution Trading Pros
	 */
	import { IconInbox } from '@tabler/icons-svelte';
	import type { ComponentType } from 'svelte';

	export let title = 'No data found';
	export let description = '';
	export let icon: ComponentType = IconInbox;
	export let actionLabel = '';
	export let size: 'sm' | 'md' | 'lg' = 'md';

	import { createEventDispatcher } from 'svelte';
	const dispatch = createEventDispatcher();

	function handleAction() {
		dispatch('action');
	}

	const sizes = {
		sm: { icon: 32, padding: '2rem 1rem' },
		md: { icon: 48, padding: '4rem 2rem' },
		lg: { icon: 64, padding: '6rem 2rem' }
	};
</script>

<div class="empty-state size-{size}" style="padding: {sizes[size].padding}">
	<div class="empty-icon">
		<svelte:component this={icon} size={sizes[size].icon} />
	</div>
	
	<h3 class="empty-title">{title}</h3>
	
	{#if description}
		<p class="empty-description">{description}</p>
	{/if}
	
	<slot />
	
	{#if actionLabel}
		<button class="empty-action" on:click={handleAction}>
			{actionLabel}
		</button>
	{/if}
</div>

<style>
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		text-align: center;
	}

	.empty-icon {
		color: var(--color-rtp-muted, #475569);
		margin-bottom: 1rem;
		opacity: 0.6;
	}

	.empty-title {
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--color-rtp-text, #e2e8f0);
		margin: 0 0 0.5rem 0;
	}

	.empty-description {
		font-size: 0.9375rem;
		color: var(--color-rtp-muted, #64748b);
		margin: 0 0 1.5rem 0;
		max-width: 400px;
	}

	.empty-action {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: linear-gradient(135deg, var(--color-rtp-primary, #6366f1), var(--color-rtp-indigo, #8b5cf6));
		color: white;
		border: none;
		border-radius: var(--radius-md, 0.5rem);
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.empty-action:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(99, 102, 241, 0.4);
	}

	/* Size variants */
	.size-sm .empty-title {
		font-size: 1rem;
	}

	.size-sm .empty-description {
		font-size: 0.875rem;
	}

	.size-lg .empty-title {
		font-size: 1.5rem;
	}

	.size-lg .empty-description {
		font-size: 1rem;
	}
</style>
