<!--
	AlertBanner — error / success notification with dismiss button.
	Extracted from /admin/schedules in R21-C (was inline in +page.svelte).
-->
<script lang="ts">
	import IconAlertCircle from '@tabler/icons-svelte-runes/icons/alert-circle';
	import IconCheck from '@tabler/icons-svelte-runes/icons/check';
	import IconX from '@tabler/icons-svelte-runes/icons/x';

	interface Props {
		variant: 'error' | 'success';
		message: string;
		ondismiss: () => void;
	}

	let { variant, message, ondismiss }: Props = $props();
</script>

{#if variant === 'error'}
	<div class="alert alert-error" role="alert">
		<IconAlertCircle size={18} />
		<span>{message}</span>
		<button onclick={ondismiss} aria-label="Dismiss">
			<IconX size={18} />
		</button>
	</div>
{:else}
	<div class="alert alert-success" role="status">
		<IconCheck size={18} />
		<span>{message}</span>
		<button onclick={ondismiss} aria-label="Dismiss">
			<IconX size={18} />
		</button>
	</div>
{/if}

<style>
	.alert {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 12px 16px;
		border-radius: 8px;
		margin-bottom: 16px;
		font-size: 14px;
	}

	.alert-error {
		background: #fef2f2;
		color: #dc2626;
		border: 1px solid #fecaca;
	}

	.alert-success {
		background: #f0fdf4;
		color: #16a34a;
		border: 1px solid #bbf7d0;
	}

	.alert button {
		margin-left: auto;
		background: none;
		border: none;
		cursor: pointer;
		opacity: 0.7;
		padding: 4px;
		display: flex;
	}

	.alert button:hover {
		opacity: 1;
	}
</style>
