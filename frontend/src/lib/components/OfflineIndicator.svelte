<script lang="ts">
	/**
	 * Offline Indicator - Apple ICT9+ Design
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * Shows offline status and pending sync actions.
	 *
	 * @version 1.0.0
	 */

	import { fade, fly, slide } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';
	import {
		offlineStore,
		isOnline,
		hasPendingActions,
		pendingActionsCount,
		isSyncing
	} from '$lib/stores/offline';
	import IconWifi from '@tabler/icons-svelte/icons/wifi';
	import IconWifiOff from '@tabler/icons-svelte/icons/wifi-off';
	import IconCloudUpload from '@tabler/icons-svelte/icons/cloud-upload';
	import IconLoader2 from '@tabler/icons-svelte/icons/loader-2';
	import IconCheck from '@tabler/icons-svelte/icons/check';
	import IconX from '@tabler/icons-svelte/icons/x';
	import IconRefresh from '@tabler/icons-svelte/icons/refresh';

	let showDetails = $state(false);
	let justCameOnline = $state(false);

	// Show "back online" notification briefly
	$effect(() => {
		if ($isOnline && $hasPendingActions) {
			justCameOnline = true;
			setTimeout(() => {
				justCameOnline = false;
			}, 3000);
		}
	});
</script>

{#if !$isOnline}
	<div
		class="offline-banner"
		in:fly={{ y: -50, duration: 300, easing: quintOut }}
		out:fade={{ duration: 150 }}
	>
		<div class="banner-content">
			<div class="banner-icon">
				<IconWifiOff size={18} />
			</div>
			<div class="banner-text">
				<span class="banner-title">You're offline</span>
				<span class="banner-subtitle">Changes will sync when you're back online</span>
			</div>
			{#if $hasPendingActions}
				<div class="pending-badge">
					<IconCloudUpload size={14} />
					{$pendingActionsCount} pending
				</div>
			{/if}
		</div>
	</div>
{:else if justCameOnline}
	<div
		class="online-banner"
		in:fly={{ y: -50, duration: 300, easing: quintOut }}
		out:fade={{ duration: 150 }}
	>
		<div class="banner-content">
			<div class="banner-icon online">
				<IconWifi size={18} />
			</div>
			<span class="banner-title">Back online</span>
			{#if $isSyncing}
				<div class="syncing-indicator">
					<IconLoader2 size={14} class="spinning" />
					Syncing...
				</div>
			{:else if $hasPendingActions}
				<button class="sync-btn" onclick={() => offlineStore.sync()}>
					<IconRefresh size={14} />
					Sync now
				</button>
			{:else}
				<div class="synced-indicator">
					<IconCheck size={14} />
					All synced
				</div>
			{/if}
		</div>
	</div>
{/if}

<!-- Floating sync indicator when online with pending actions -->
{#if $isOnline && $hasPendingActions && !justCameOnline}
	<button
		class="sync-float-btn"
		onclick={() => showDetails = !showDetails}
		in:fly={{ y: 20, duration: 200 }}
	>
		{#if $isSyncing}
			<IconLoader2 size={18} class="spinning" />
		{:else}
			<IconCloudUpload size={18} />
		{/if}
		<span>{$pendingActionsCount}</span>
	</button>

	{#if showDetails}
		<div
			class="sync-details"
			transition:fly={{ y: 10, duration: 200, easing: quintOut }}
		>
			<div class="details-header">
				<span>Pending Changes</span>
				<button class="close-details" onclick={() => showDetails = false}>
					<IconX size={14} />
				</button>
			</div>
			<div class="details-content">
				<p>{$pendingActionsCount} changes waiting to sync</p>
				<button
					class="sync-all-btn"
					onclick={() => offlineStore.sync()}
					disabled={$isSyncing}
				>
					{#if $isSyncing}
						<IconLoader2 size={14} class="spinning" />
						Syncing...
					{:else}
						<IconRefresh size={14} />
						Sync Now
					{/if}
				</button>
			</div>
		</div>
	{/if}
{/if}

<style>
	.offline-banner,
	.online-banner {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		z-index: 10000;
		padding: 0.75rem 1rem;
	}

	.offline-banner {
		background: linear-gradient(135deg, rgba(239, 68, 68, 0.95), rgba(220, 38, 38, 0.95));
	}

	.online-banner {
		background: linear-gradient(135deg, rgba(16, 185, 129, 0.95), rgba(5, 150, 105, 0.95));
	}

	.banner-content {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
		max-width: 600px;
		margin: 0 auto;
	}

	.banner-icon {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(255, 255, 255, 0.2);
		border-radius: 8px;
		color: white;
	}

	.banner-text {
		display: flex;
		flex-direction: column;
	}

	.banner-title {
		font-size: 0.875rem;
		font-weight: 600;
		color: white;
	}

	.banner-subtitle {
		font-size: 0.75rem;
		color: rgba(255, 255, 255, 0.8);
	}

	.pending-badge {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.625rem;
		background: rgba(255, 255, 255, 0.2);
		border-radius: 6px;
		font-size: 0.75rem;
		font-weight: 500;
		color: white;
	}

	.syncing-indicator,
	.synced-indicator {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.8125rem;
		color: white;
	}

	.sync-btn {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.75rem;
		background: rgba(255, 255, 255, 0.2);
		border: none;
		border-radius: 6px;
		color: white;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.sync-btn:hover {
		background: rgba(255, 255, 255, 0.3);
	}

	.sync-float-btn {
		position: fixed;
		bottom: 6rem;
		right: 1.5rem;
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		border: none;
		border-radius: 12px;
		color: white;
		font-size: 0.875rem;
		font-weight: 600;
		cursor: pointer;
		box-shadow: 0 8px 25px rgba(99, 102, 241, 0.4);
		transition: all 0.2s;
		z-index: 1000;
	}

	.sync-float-btn:hover {
		transform: translateY(-2px);
		box-shadow: 0 12px 30px rgba(99, 102, 241, 0.5);
	}

	.sync-details {
		position: fixed;
		bottom: 10rem;
		right: 1.5rem;
		width: 280px;
		background: linear-gradient(135deg, rgba(30, 41, 59, 0.98), rgba(15, 23, 42, 0.98));
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 14px;
		box-shadow: 0 20px 50px rgba(0, 0, 0, 0.4);
		z-index: 1000;
		overflow: hidden;
	}

	.details-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.875rem 1rem;
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
		color: #f1f5f9;
		font-weight: 600;
		font-size: 0.875rem;
	}

	.close-details {
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(99, 102, 241, 0.1);
		border: none;
		border-radius: 6px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.close-details:hover {
		background: rgba(99, 102, 241, 0.2);
		color: #f1f5f9;
	}

	.details-content {
		padding: 1rem;
	}

	.details-content p {
		font-size: 0.8125rem;
		color: #94a3b8;
		margin: 0 0 1rem;
	}

	.sync-all-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		width: 100%;
		padding: 0.75rem;
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		border: none;
		border-radius: 10px;
		color: white;
		font-size: 0.875rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.sync-all-btn:hover:not(:disabled) {
		transform: translateY(-1px);
		box-shadow: 0 8px 20px rgba(99, 102, 241, 0.3);
	}

	.sync-all-btn:disabled {
		opacity: 0.7;
		cursor: not-allowed;
	}

	:global(.spinning) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}

	@media (max-width: 640px) {
		.banner-text {
			display: none;
		}

		.sync-float-btn {
			right: 1rem;
			bottom: 5rem;
		}

		.sync-details {
			right: 1rem;
			left: 1rem;
			width: auto;
		}
	}
</style>
