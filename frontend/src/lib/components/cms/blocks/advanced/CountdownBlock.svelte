<!--
/**
 * Countdown Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Countdown timer to target date
 */
-->

<script lang="ts">
	import { IconClock } from '$lib/icons';
	import { getBlockStateManager, type BlockId } from '$lib/stores/blockState.svelte';
	import { onMount } from 'svelte';
	import type { Block, BlockContent } from '../types';

	interface Props {
		block: Block;
		blockId: BlockId;
		isSelected: boolean;
		isEditing: boolean;
		onUpdate: (updates: Partial<Block>) => void;
		onError?: (error: Error) => void;
	}

	const props: Props = $props();
	const stateManager = getBlockStateManager();

	let countdownState = $derived(stateManager.getCountdownState(props.blockId));

	let targetDate = $derived(props.block.content.countdownTarget || '');
	let title = $derived(props.block.content.countdownTitle || 'Offer Ends In');
	let expiredMessage = $derived(props.block.content.countdownExpiredMessage || 'This offer has expired');

	let days = $state(0);
	let hours = $state(0);
	let minutes = $state(0);
	let seconds = $state(0);
	let isExpired = $state(false);

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	function calculateTimeLeft(): void {
		if (!targetDate) {
			isExpired = true;
			return;
		}

		const target = new Date(targetDate).getTime();
		const now = Date.now();
		const diff = target - now;

		if (diff <= 0) {
			isExpired = true;
			days = hours = minutes = seconds = 0;
			return;
		}

		isExpired = false;
		days = Math.floor(diff / (1000 * 60 * 60 * 24));
		hours = Math.floor((diff % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));
		minutes = Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60));
		seconds = Math.floor((diff % (1000 * 60)) / 1000);
	}

	onMount(() => {
		calculateTimeLeft();
		const interval = setInterval(calculateTimeLeft, 1000);
		stateManager.startCountdown(props.blockId, targetDate);
		return () => clearInterval(interval);
	});

	function handlePaste(e: ClipboardEvent): void {
		e.preventDefault();
		document.execCommand('insertText', false, e.clipboardData?.getData('text/plain') || '');
	}

	function pad(n: number): string {
		return n.toString().padStart(2, '0');
	}
</script>

<div class="countdown-block" role="timer" aria-label="Countdown timer">
	{#if props.isEditing}
		<h3
			contenteditable="true"
			class="countdown-title"
			oninput={(e) => updateContent({ countdownTitle: (e.target as HTMLElement).textContent || '' })}
			onpaste={handlePaste}
		>{title}</h3>
	{:else}
		<h3 class="countdown-title">{title}</h3>
	{/if}

	{#if isExpired && !props.isEditing}
		<div class="countdown-expired">
			<IconClock size={32} aria-hidden="true" />
			<p>{expiredMessage}</p>
		</div>
	{:else}
		<div class="countdown-display">
			<div class="countdown-unit">
				<span class="countdown-value">{pad(days)}</span>
				<span class="countdown-label">Days</span>
			</div>
			<span class="countdown-separator">:</span>
			<div class="countdown-unit">
				<span class="countdown-value">{pad(hours)}</span>
				<span class="countdown-label">Hours</span>
			</div>
			<span class="countdown-separator">:</span>
			<div class="countdown-unit">
				<span class="countdown-value">{pad(minutes)}</span>
				<span class="countdown-label">Min</span>
			</div>
			<span class="countdown-separator">:</span>
			<div class="countdown-unit">
				<span class="countdown-value">{pad(seconds)}</span>
				<span class="countdown-label">Sec</span>
			</div>
		</div>
	{/if}

	{#if props.isEditing && props.isSelected}
		<div class="countdown-settings">
			<label class="setting-field">
				<span>Target Date & Time:</span>
				<input
					type="datetime-local"
					value={targetDate}
					oninput={(e) => updateContent({ countdownTarget: (e.target as HTMLInputElement).value })}
				/>
			</label>
			<label class="setting-field">
				<span>Expired Message:</span>
				<input
					type="text"
					value={expiredMessage}
					oninput={(e) => updateContent({ countdownExpiredMessage: (e.target as HTMLInputElement).value })}
				/>
			</label>
		</div>
	{/if}
</div>

<style>
	.countdown-block {
		padding: 2rem;
		background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%);
		border-radius: 16px;
		text-align: center;
		color: white;
	}

	.countdown-title {
		margin: 0 0 1.5rem;
		font-size: 1.25rem;
		font-weight: 600;
		color: #94a3b8;
		text-transform: uppercase;
		letter-spacing: 0.1em;
		outline: none;
	}

	.countdown-display {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 0.5rem;
	}

	.countdown-unit {
		display: flex;
		flex-direction: column;
		align-items: center;
		min-width: 80px;
	}

	.countdown-value {
		font-size: 3rem;
		font-weight: 800;
		line-height: 1;
		font-variant-numeric: tabular-nums;
		background: linear-gradient(180deg, #f8fafc 0%, #94a3b8 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.countdown-label {
		font-size: 0.75rem;
		font-weight: 500;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-top: 0.5rem;
	}

	.countdown-separator {
		font-size: 2.5rem;
		font-weight: 300;
		color: #475569;
		margin-bottom: 1.5rem;
	}

	.countdown-expired {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1rem;
		padding: 2rem;
		color: #f97316;
	}

	.countdown-expired p {
		margin: 0;
		font-size: 1.125rem;
		font-weight: 500;
	}

	.countdown-settings {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		margin-top: 1.5rem;
		padding-top: 1.5rem;
		border-top: 1px solid #334155;
	}

	.setting-field {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
		text-align: left;
	}

	.setting-field span {
		font-size: 0.75rem;
		font-weight: 600;
		color: #94a3b8;
		text-transform: uppercase;
	}

	.setting-field input {
		padding: 0.625rem 0.75rem;
		background: #1e293b;
		border: 1px solid #475569;
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 0.9375rem;
	}

	.setting-field input:focus {
		outline: none;
		border-color: #3b82f6;
	}

	@media (max-width: 640px) {
		.countdown-unit { min-width: 60px; }
		.countdown-value { font-size: 2rem; }
		.countdown-separator { font-size: 1.5rem; margin-bottom: 1rem; }
	}

	:global(.dark) .countdown-block {
		background: linear-gradient(135deg, #0f172a 0%, #020617 100%);
	}
</style>
