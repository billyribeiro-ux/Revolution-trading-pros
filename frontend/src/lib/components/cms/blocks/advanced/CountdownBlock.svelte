<!--
/**
 * Countdown Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Countdown timer to target date with celebration state
 * Features purple gradient background and responsive design
 *
 * @version 2.0.0
 * @author Revolution Trading Pros
 */
-->

<script lang="ts">
	import { IconSparkles, IconTrophy } from '$lib/icons';
	import { getBlockStateManager, type BlockId, type CountdownState } from '$lib/stores/blockState.svelte';
	import { onMount, onDestroy } from 'svelte';
	import type { Block, BlockContent } from '../types';

	// =========================================================================
	// Types
	// =========================================================================

	interface Props {
		block: Block;
		blockId: BlockId;
		isSelected: boolean;
		isEditing: boolean;
		onUpdate: (updates: Partial<Block>) => void;
		onError?: (error: Error) => void;
	}

	// =========================================================================
	// Props & State
	// =========================================================================

	let props: Props = $props();
	const stateManager = getBlockStateManager();

	// Local state for countdown values
	let days = $state(0);
	let hours = $state(0);
	let minutes = $state(0);
	let seconds = $state(0);
	let isExpired = $state(false);
	let intervalId: ReturnType<typeof setInterval> | null = null;

	// =========================================================================
	// Derived Values
	// =========================================================================

	let countdownState = $derived<CountdownState>(stateManager.getCountdownState(props.blockId));
	let targetDate = $derived(props.block.content.countdownTarget || '');
	let title = $derived(props.block.content.countdownTitle || 'Offer Ends In');
	let expiredMessage = $derived(
		props.block.content.countdownExpiredMessage || 'Time is up! Celebration time!'
	);

	// =========================================================================
	// Handlers
	// =========================================================================

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	function calculateTimeLeft(): void {
		if (!targetDate) {
			isExpired = true;
			return;
		}

		try {
			const target = new Date(targetDate).getTime();
			const now = Date.now();
			const diff = target - now;

			if (diff <= 0) {
				isExpired = true;
				days = hours = minutes = seconds = 0;
				// Update state manager
				stateManager.setCountdownState(props.blockId, {
					days: 0,
					hours: 0,
					minutes: 0,
					seconds: 0
				});
				return;
			}

			isExpired = false;
			days = Math.floor(diff / (1000 * 60 * 60 * 24));
			hours = Math.floor((diff % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));
			minutes = Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60));
			seconds = Math.floor((diff % (1000 * 60)) / 1000);

			// Update state manager
			stateManager.setCountdownState(props.blockId, {
				days,
				hours,
				minutes,
				seconds
			});
		} catch (error) {
			if (props.onError && error instanceof Error) {
				props.onError(error);
			}
			isExpired = true;
		}
	}

	function handlePaste(e: ClipboardEvent): void {
		e.preventDefault();
		const text = e.clipboardData?.getData('text/plain') || '';
		document.execCommand('insertText', false, text);
	}

	function pad(n: number): string {
		return n.toString().padStart(2, '0');
	}

	// =========================================================================
	// Lifecycle
	// =========================================================================

	onMount(() => {
		// Initial calculation
		calculateTimeLeft();

		// Start the countdown interval
		intervalId = setInterval(calculateTimeLeft, 1000);

		// Register with state manager
		stateManager.startCountdown(props.blockId, targetDate);
	});

	onDestroy(() => {
		// Clear local interval
		if (intervalId) {
			clearInterval(intervalId);
			intervalId = null;
		}

		// Cleanup state manager resources
		stateManager.cleanup(props.blockId);
	});

	// Watch for target date changes
	$effect(() => {
		if (targetDate) {
			calculateTimeLeft();
			// Restart the countdown in state manager with new target
			stateManager.startCountdown(props.blockId, targetDate);
		}
	});
</script>

<div
	class="countdown-block"
	role="timer"
	aria-label="Countdown timer"
	aria-live="polite"
>
	<!-- Title -->
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

	<!-- Expired State with Celebration -->
	{#if isExpired && !props.isEditing}
		<div class="countdown-expired" role="status" aria-label="Countdown complete">
			<div class="celebration-icons">
				<IconSparkles size={32} aria-hidden="true" />
				<IconTrophy size={36} aria-hidden="true" />
				<IconSparkles size={32} aria-hidden="true" />
			</div>
			<p class="expired-message">{expiredMessage}</p>
		</div>
	{:else}
		<!-- Countdown Display -->
		<div class="countdown-display">
			<div class="countdown-unit">
				<span class="countdown-value" aria-label="{days} days">{pad(days)}</span>
				<span class="countdown-label">Days</span>
			</div>
			<span class="countdown-separator" aria-hidden="true">:</span>
			<div class="countdown-unit">
				<span class="countdown-value" aria-label="{hours} hours">{pad(hours)}</span>
				<span class="countdown-label">Hours</span>
			</div>
			<span class="countdown-separator" aria-hidden="true">:</span>
			<div class="countdown-unit">
				<span class="countdown-value" aria-label="{minutes} minutes">{pad(minutes)}</span>
				<span class="countdown-label">Minutes</span>
			</div>
			<span class="countdown-separator" aria-hidden="true">:</span>
			<div class="countdown-unit">
				<span class="countdown-value" aria-label="{seconds} seconds">{pad(seconds)}</span>
				<span class="countdown-label">Seconds</span>
			</div>
		</div>
	{/if}

	<!-- Settings Panel (Edit Mode) -->
	{#if props.isEditing && props.isSelected}
		<div class="countdown-settings">
			<div class="settings-section">
				<h4 class="settings-title">Countdown Settings</h4>
				<div class="settings-row">
					<label class="setting-field">
						<span>Target Date & Time:</span>
						<input
							type="datetime-local"
							value={targetDate}
							oninput={(e) => updateContent({ countdownTarget: (e.target as HTMLInputElement).value })}
						/>
					</label>
				</div>
				<div class="settings-row">
					<label class="setting-field">
						<span>Expired Message:</span>
						<input
							type="text"
							value={expiredMessage}
							placeholder="Message when countdown ends"
							oninput={(e) => updateContent({ countdownExpiredMessage: (e.target as HTMLInputElement).value })}
						/>
					</label>
				</div>
			</div>
		</div>
	{/if}
</div>

<style>
	/* =========================================================================
	   Base Styles - Purple Gradient Background
	   ========================================================================= */

	.countdown-block {
		padding: 2.5rem 2rem;
		background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
		border-radius: 16px;
		text-align: center;
		color: white;
		position: relative;
		overflow: hidden;
	}

	/* Subtle gradient overlay for depth */
	.countdown-block::before {
		content: '';
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: linear-gradient(
			180deg,
			rgba(255, 255, 255, 0.1) 0%,
			transparent 50%,
			rgba(0, 0, 0, 0.1) 100%
		);
		pointer-events: none;
	}

	/* =========================================================================
	   Title
	   ========================================================================= */

	.countdown-title {
		position: relative;
		margin: 0 0 1.75rem;
		font-size: 1.25rem;
		font-weight: 600;
		color: rgba(255, 255, 255, 0.9);
		text-transform: uppercase;
		letter-spacing: 0.15em;
		outline: none;
	}

	.countdown-title:focus {
		background: rgba(255, 255, 255, 0.1);
		border-radius: 4px;
	}

	/* =========================================================================
	   Countdown Display
	   ========================================================================= */

	.countdown-display {
		position: relative;
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 0.75rem;
	}

	.countdown-unit {
		display: flex;
		flex-direction: column;
		align-items: center;
		min-width: 85px;
		padding: 1rem 0.5rem;
		background: rgba(255, 255, 255, 0.15);
		backdrop-filter: blur(10px);
		-webkit-backdrop-filter: blur(10px);
		border-radius: 12px;
		border: 1px solid rgba(255, 255, 255, 0.2);
	}

	.countdown-value {
		font-size: 3rem;
		font-weight: 800;
		line-height: 1;
		font-variant-numeric: tabular-nums;
		color: white;
		text-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
	}

	.countdown-label {
		font-size: 0.75rem;
		font-weight: 600;
		color: rgba(255, 255, 255, 0.8);
		text-transform: uppercase;
		letter-spacing: 0.08em;
		margin-top: 0.5rem;
	}

	.countdown-separator {
		font-size: 2.5rem;
		font-weight: 300;
		color: rgba(255, 255, 255, 0.6);
		margin-bottom: 1.5rem;
	}

	/* =========================================================================
	   Expired / Celebration State
	   ========================================================================= */

	.countdown-expired {
		position: relative;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1.25rem;
		padding: 2rem;
		background: rgba(255, 255, 255, 0.15);
		backdrop-filter: blur(10px);
		-webkit-backdrop-filter: blur(10px);
		border-radius: 16px;
		border: 1px solid rgba(255, 255, 255, 0.25);
		animation: celebrationPulse 2s ease-in-out infinite;
	}

	.celebration-icons {
		display: flex;
		align-items: center;
		gap: 1rem;
		color: #fcd34d;
		animation: bounce 1s ease-in-out infinite;
	}

	.celebration-icons :global(svg:nth-child(1)) {
		transform: rotate(-15deg);
	}

	.celebration-icons :global(svg:nth-child(3)) {
		transform: rotate(15deg);
	}

	.expired-message {
		margin: 0;
		font-size: 1.25rem;
		font-weight: 700;
		color: white;
		text-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
	}

	@keyframes celebrationPulse {
		0%, 100% {
			box-shadow: 0 0 20px rgba(252, 211, 77, 0.3);
		}
		50% {
			box-shadow: 0 0 40px rgba(252, 211, 77, 0.5);
		}
	}

	@keyframes bounce {
		0%, 100% {
			transform: translateY(0);
		}
		50% {
			transform: translateY(-5px);
		}
	}

	/* =========================================================================
	   Settings Panel
	   ========================================================================= */

	.countdown-settings {
		position: relative;
		margin-top: 2rem;
		padding: 1.5rem;
		background: rgba(255, 255, 255, 0.1);
		backdrop-filter: blur(12px);
		-webkit-backdrop-filter: blur(12px);
		border-radius: 12px;
		border: 1px solid rgba(255, 255, 255, 0.2);
	}

	.settings-section {
		margin-bottom: 1rem;
	}

	.settings-section:last-child {
		margin-bottom: 0;
	}

	.settings-title {
		margin: 0 0 1rem;
		font-size: 0.8125rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: rgba(255, 255, 255, 0.9);
	}

	.settings-row {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.setting-field {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		text-align: left;
	}

	.setting-field span {
		font-size: 0.75rem;
		font-weight: 600;
		color: rgba(255, 255, 255, 0.85);
		text-transform: uppercase;
		letter-spacing: 0.03em;
	}

	.setting-field input {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(255, 255, 255, 0.15);
		border: 1px solid rgba(255, 255, 255, 0.3);
		border-radius: 8px;
		color: white;
		font-size: 0.9375rem;
		transition: all 0.2s ease;
	}

	.setting-field input::placeholder {
		color: rgba(255, 255, 255, 0.5);
	}

	.setting-field input:focus {
		outline: none;
		border-color: rgba(255, 255, 255, 0.6);
		background: rgba(255, 255, 255, 0.2);
		box-shadow: 0 0 0 3px rgba(255, 255, 255, 0.1);
	}

	/* Datetime-local input color scheme */
	.setting-field input[type="datetime-local"]::-webkit-calendar-picker-indicator {
		filter: invert(1);
		cursor: pointer;
	}

	/* =========================================================================
	   Responsive Design
	   ========================================================================= */

	@media (max-width: 640px) {
		.countdown-block {
			padding: 2rem 1.25rem;
		}

		.countdown-title {
			font-size: 1rem;
			margin-bottom: 1.25rem;
		}

		.countdown-display {
			gap: 0.25rem;
		}

		.countdown-unit {
			min-width: 60px;
			padding: 0.75rem 0.25rem;
		}

		.countdown-value {
			font-size: 1.75rem;
		}

		.countdown-label {
			font-size: 0.625rem;
			letter-spacing: 0.05em;
		}

		/* Hide separators on mobile */
		.countdown-separator {
			display: none;
		}

		.countdown-expired {
			padding: 1.5rem;
		}

		.expired-message {
			font-size: 1rem;
		}

		.celebration-icons :global(svg) {
			width: 24px;
			height: 24px;
		}
	}

	@media (max-width: 400px) {
		.countdown-unit {
			min-width: 50px;
		}

		.countdown-value {
			font-size: 1.5rem;
		}

		.countdown-label {
			font-size: 0.5625rem;
		}
	}

	/* =========================================================================
	   Dark Mode Adjustments
	   ========================================================================= */

	:global(.dark) .countdown-block {
		background: linear-gradient(135deg, #5a67d8 0%, #6b46c1 100%);
	}

	:global(.dark) .countdown-unit {
		background: rgba(0, 0, 0, 0.2);
		border-color: rgba(255, 255, 255, 0.15);
	}

	:global(.dark) .countdown-settings {
		background: rgba(0, 0, 0, 0.3);
		border-color: rgba(255, 255, 255, 0.15);
	}

	:global(.dark) .setting-field input {
		background: rgba(0, 0, 0, 0.3);
		border-color: rgba(255, 255, 255, 0.2);
	}

	:global(.dark) .setting-field input:focus {
		background: rgba(0, 0, 0, 0.4);
		border-color: rgba(255, 255, 255, 0.4);
	}
</style>
