<script lang="ts">
	/**
	 * EmptyState - Consistent empty state display
	 *
	 * @version 2.0.0
	 * @author Revolution Trading Pros
	 */
	import { IconInbox } from '$lib/icons';
	import type { ComponentType, Snippet } from 'svelte';

	interface Props {
		title?: string;
		description?: string;
		icon?: ComponentType;
		actionLabel?: string;
		size?: 'sm' | 'md' | 'lg';
		onaction?: () => void;
		children?: Snippet;
	}

	let props: Props = $props();

	function handleAction() {
		props.onaction?.();
	}

	const sizes = {
		sm: { icon: 32, padding: '2rem 1rem' },
		md: { icon: 48, padding: '4rem 2rem' },
		lg: { icon: 64, padding: '6rem 2rem' }
	};

	let currentSize = $derived(props.size ?? 'md');
	let Icon = $derived(props.icon ?? IconInbox);
</script>

<div class="empty-state size-{currentSize}" style="padding: {sizes[currentSize].padding}">
	<div class="empty-icon">
		<Icon size={sizes[currentSize].icon} />
	</div>

	<h3 class="empty-title">{props.title ?? 'No data found'}</h3>

	{#if props.description}
		<p class="empty-description">{props.description}</p>
	{/if}

	{@render props.children?.()}

	{#if props.actionLabel}
		<button class="empty-action" onclick={handleAction}>
			{props.actionLabel}
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
		background: linear-gradient(
			135deg,
			var(--color-rtp-primary, #6366f1),
			var(--color-rtp-indigo, #8b5cf6)
		);
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
