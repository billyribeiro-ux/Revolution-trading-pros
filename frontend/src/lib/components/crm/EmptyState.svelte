<script lang="ts">
	import { Icon, IconInbox, IconPlus } from '$lib/icons';

	interface Props {
		title: string;
		description: string;
		actionLabel?: string;
		onaction?: () => void;
	}

	let { title, description, actionLabel, onaction }: Props = $props();
</script>

<div class="es-wrap">
	<div class="es-icon-wrap">
		<Icon icon={IconInbox} size={32} class="es-icon" />
	</div>

	<h3 class="es-title">{title}</h3>
	<p class="es-desc">{description}</p>

	{#if actionLabel && onaction}
		<button type="button" class="es-btn" onclick={onaction}>
			<Icon icon={IconPlus} size={18} />
			{actionLabel}
		</button>
	{/if}
</div>

<style>
	.es-wrap {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding-block: 4rem;
		padding-inline: 1.5rem;
		text-align: center;
	}

	.es-icon-wrap {
		margin-block-end: 1rem;
		display: flex;
		block-size: 4rem;
		inline-size: 4rem;
		align-items: center;
		justify-content: center;
		border-radius: 50%;
		background-color: oklch(0.25 0.02 265 / 0.5);
	}

	:global(.es-icon) {
		color: oklch(0.55 0.02 265);
	}

	.es-title {
		margin-block-end: 0.5rem;
		font-size: var(--text-lg);
		font-weight: var(--weight-semibold);
		color: oklch(0.75 0.005 265);
	}

	.es-desc {
		margin-block-end: 1.5rem;
		max-inline-size: 28rem;
		font-size: var(--text-sm);
		color: oklch(0.6 0.005 265);
	}

	.es-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		border-radius: var(--radius-xl);
		background-color: oklch(0.5 0.18 260);
		padding-inline: 1rem;
		padding-block: 0.5rem;
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
		color: oklch(1 0 0);
		box-shadow: var(--shadow-sm);
		transition:
			background-color 200ms,
			box-shadow 200ms;

		&:hover {
			background-color: oklch(0.55 0.18 260);
		}

		&:focus {
			outline: none;
			box-shadow: 0 0 0 2px oklch(0.5 0.18 260 / 0.6);
		}
	}
</style>
