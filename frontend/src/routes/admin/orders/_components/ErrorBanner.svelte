<script lang="ts">
	import { IconAlertTriangle } from '$lib/icons';

	interface Props {
		error: string;
		onretry: () => void;
	}

	let { error, onretry }: Props = $props();
</script>

<div class="error-banner">
	<IconAlertTriangle size={20} />
	<span>{error}</span>
	<button onclick={onretry}>Retry</button>
</div>

<style>
	.error-banner {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.5rem;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 12px;
		color: var(--error-emphasis);
		margin-bottom: 1.5rem;
	}

	.error-banner span {
		flex: 1;
		font-weight: 500;
	}

	.error-banner button {
		padding: 0.5rem 1rem;
		background: rgba(239, 68, 68, 0.2);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 6px;
		color: var(--error-emphasis);
		font-weight: 600;
		cursor: pointer;
	}
</style>
