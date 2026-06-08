<script lang="ts">
	import { fly } from 'svelte/transition';
	import { IconCheck, IconX, IconAlertCircle } from '$lib/icons';

	type Notification = {
		id: string;
		type: 'success' | 'error' | 'warning' | 'info';
		message: string;
	};

	type Props = {
		notifications: Notification[];
	};

	const { notifications }: Props = $props();
</script>

<div class="notifications">
	{#each notifications as notification (notification.id)}
		<div class={['notification', notification.type]} transition:fly={{ y: -20, duration: 300 }}>
			{#if notification.type === 'success'}
				<IconCheck size={20} />
			{:else if notification.type === 'error'}
				<IconX size={20} />
			{:else}
				<IconAlertCircle size={20} />
			{/if}
			{notification.message}
		</div>
	{/each}
</div>

<style>
	.notifications {
		position: fixed;
		top: 1rem;
		right: 1rem;
		z-index: 1000;
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.notification {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.5rem;
		background: white;
		border-radius: 8px;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
		font-weight: 500;
		min-width: 300px;
	}

	.notification.success {
		background: #dcfce7;
		color: #16a34a;
	}

	.notification.error {
		background: #fee2e2;
		color: #dc2626;
	}

	.notification.warning {
		background: #fef3c7;
		color: #d97706;
	}

	.notification.info {
		background: #dbeafe;
		color: #2563eb;
	}
</style>
