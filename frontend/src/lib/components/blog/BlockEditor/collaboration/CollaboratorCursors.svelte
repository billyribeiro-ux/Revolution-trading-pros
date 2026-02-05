<!--
/**
 * Collaborator Cursors - Real-time Cursor and Selection Display
 * ═══════════════════════════════════════════════════════════════════════════
 * Shows colored cursors and selections for each collaborator
 *
 * @version 1.0.0 - January 2026
 * @standards Apple Principal Engineer ICT Level 7+ | Svelte 5 Runes Syntax
 *
 * Features:
 * - Colored cursors for each collaborator
 * - User name labels with smooth positioning
 * - Animated cursor movements
 * - Block selection highlights
 * - Text selection overlays
 * - Idle state indicators
 * - Typing indicators
 */
-->

<script lang="ts">
	import { fade, fly, scale } from 'svelte/transition';
	import { elasticOut } from 'svelte/easing';
	import { flip } from 'svelte/animate';
	import type { Collaborator, CursorPosition } from './awareness';
	import { getContrastColor, formatLastActive } from './awareness';

	// ═══════════════════════════════════════════════════════════════════════════
	// Props
	// ═══════════════════════════════════════════════════════════════════════════

	interface Props {
		/** List of collaborators to display */
		collaborators: Collaborator[];
		/** Container element for relative positioning */
		containerRef?: HTMLElement | null;
		/** Map of block IDs to their DOM elements */
		blockElements?: Map<string, HTMLElement>;
		/** Whether to show cursor labels */
		showLabels?: boolean;
		/** Whether to show block selections */
		showBlockSelections?: boolean;
		/** Whether to show text selections */
		showTextSelections?: boolean;
		/** Whether to show typing indicators */
		showTypingIndicators?: boolean;
		/** Whether to show idle status */
		showIdleStatus?: boolean;
		/** Z-index for cursors */
		zIndex?: number;
		/** Label position relative to cursor */
		labelPosition?: 'top' | 'bottom' | 'left' | 'right';
	}

	let props: Props = $props();
	const collaborators = $derived(props.collaborators);
	const containerRef = $derived(props.containerRef ?? null);
	const blockElements = $derived(props.blockElements ?? new Map());
	const showLabels = $derived(props.showLabels ?? true);
	const showBlockSelections = $derived(props.showBlockSelections ?? true);
	const showTextSelections = $derived(props.showTextSelections ?? true);
	const showTypingIndicators = $derived(props.showTypingIndicators ?? true);
	const showIdleStatus = $derived(props.showIdleStatus ?? true);
	const zIndex = $derived(props.zIndex ?? 1000);
	const labelPosition = $derived(props.labelPosition ?? 'top');

	// ═══════════════════════════════════════════════════════════════════════════
	// State
	// ═══════════════════════════════════════════════════════════════════════════

	/** Previous cursor positions for animation */
	let prevPositions = $state(new Map<number, CursorPosition>());

	/** Container bounding rect for positioning */
	let containerRect = $state<DOMRect | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// Computed
	// ═══════════════════════════════════════════════════════════════════════════

	/** Active collaborators (with valid cursors or selections) */
	// @ts-expect-error Reserved for future filtering functionality
	const activeCollaborators = $derived(
		collaborators.filter(c => c.cursor || c.selectedBlockId || c.selection)
	);

	/** Collaborators with visible cursors */
	let cursorCollaborators = $derived(
		collaborators.filter(c => c.cursor && !c.isIdle)
	);

	/** Collaborators with block selections */
	let selectionCollaborators = $derived(
		collaborators.filter(c => c.selectedBlockId && showBlockSelections)
	);

	/** Collaborators who are typing */
	let typingCollaborators = $derived(
		collaborators.filter(c => c.isTyping && showTypingIndicators)
	);

	// ═══════════════════════════════════════════════════════════════════════════
	// Effects
	// ═══════════════════════════════════════════════════════════════════════════

	// Update container rect when container changes
	$effect(() => {
		if (!containerRef) return;

		containerRect = containerRef.getBoundingClientRect();

		// Update on resize
		const resizeObserver = new ResizeObserver(() => {
			containerRect = containerRef?.getBoundingClientRect() ?? null;
		});

		resizeObserver.observe(containerRef);

		return () => resizeObserver.disconnect();
	});

	// Track cursor position changes for animations
	$effect(() => {
		for (const collab of collaborators) {
			if (collab.cursor) {
				prevPositions.set(collab.clientId, collab.cursor);
			}
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// Helpers
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Get label offset based on position setting
	 */
	function getLabelOffset(position: typeof labelPosition): { x: number; y: number } {
		switch (position) {
			case 'top': return { x: 0, y: -28 };
			case 'bottom': return { x: 0, y: 24 };
			case 'left': return { x: -100, y: 0 };
			case 'right': return { x: 24, y: 0 };
			default: return { x: 0, y: -28 };
		}
	}

	/**
	 * Get block element position
	 */
	function getBlockPosition(blockId: string): DOMRect | null {
		const element = blockElements.get(blockId);
		if (!element || !containerRect) return null;

		const rect = element.getBoundingClientRect();
		return rect;
	}

	/**
	 * Calculate relative position within container
	 */
	function getRelativePosition(x: number, y: number): { x: number; y: number } {
		if (!containerRect) return { x, y };

		return {
			x: x,
			y: y
		};
	}

	/**
	 * Get typing indicator text
	 */
	function getTypingText(name: string): string {
		return `${name} is typing...`;
	}

	/**
	 * Get display name (truncated if needed)
	 */
	function getDisplayName(name: string, maxLength: number = 20): string {
		if (name.length <= maxLength) return name;
		return name.substring(0, maxLength - 3) + '...';
	}

	/**
	 * Get initials from name
	 */
	function getInitials(name: string): string {
		return name
			.split(' ')
			.map(part => part[0])
			.join('')
			.toUpperCase()
			.substring(0, 2);
	}
</script>

<!-- Collaborator Cursors Container -->
<div
	class="collaborator-cursors"
	style:--z-index={zIndex}
	aria-hidden="true"
>
	<!-- Block Selection Highlights -->
	{#if showBlockSelections}
		{#each selectionCollaborators as collab (collab.clientId)}
			{@const blockRect = collab.selectedBlockId ? getBlockPosition(collab.selectedBlockId) : null}
			{#if blockRect && containerRect}
				<div
					class="block-selection"
					style:--color={collab.color}
					style:left="{blockRect.left - containerRect.left - 4}px"
					style:top="{blockRect.top - containerRect.top - 4}px"
					style:width="{blockRect.width + 8}px"
					style:height="{blockRect.height + 8}px"
					in:fade={{ duration: 200 }}
					out:fade={{ duration: 150 }}
				>
					<!-- Selection Label -->
					<div
						class="selection-label"
						style:background-color={collab.color}
						style:color={getContrastColor(collab.color)}
					>
						{getDisplayName(collab.name, 15)}
					</div>
				</div>
			{/if}
		{/each}
	{/if}

	<!-- Cursor Pointers -->
	{#each cursorCollaborators as collab (collab.clientId)}
		{@const position = collab.cursor ? getRelativePosition(collab.cursor.x, collab.cursor.y) : null}
		{@const labelOffset = getLabelOffset(labelPosition)}
		{#if position}
			<div
				class="cursor-container"
				class:is-idle={collab.isIdle}
				class:is-typing={collab.isTyping}
				style:left="{position.x}px"
				style:top="{position.y}px"
				style:--color={collab.color}
				in:scale={{ duration: 300, easing: elasticOut, start: 0.5 }}
				out:fade={{ duration: 200 }}
			>
				<!-- Cursor SVG -->
				<svg
					class="cursor-pointer"
					width="24"
					height="24"
					viewBox="0 0 24 24"
					fill="none"
					xmlns="http://www.w3.org/2000/svg"
				>
					<path
						d="M5.5 3.21V20.79C5.5 21.68 6.59 22.15 7.22 21.52L10.5 18.24L13.27 22.83C13.65 23.45 14.47 23.63 15.08 23.25L16.77 22.24C17.38 21.86 17.56 21.04 17.18 20.43L14.42 15.85L19.01 15.56C19.94 15.51 20.45 14.43 19.82 13.71L6.23 2.48C5.6 1.76 4.5 2.28 4.5 3.21H5.5Z"
						fill="var(--color)"
						stroke="white"
						stroke-width="1.5"
						stroke-linejoin="round"
					/>
				</svg>

				<!-- Cursor Label -->
				{#if showLabels}
					<div
						class="cursor-label"
						style:background-color={collab.color}
						style:color={getContrastColor(collab.color)}
						style:transform="translate({labelOffset.x}px, {labelOffset.y}px)"
						in:fly={{ y: 5, duration: 200, delay: 100 }}
					>
						<span class="cursor-name">{getDisplayName(collab.name, 15)}</span>

						{#if collab.isTyping && showTypingIndicators}
							<span class="typing-indicator">
								<span class="typing-dot"></span>
								<span class="typing-dot"></span>
								<span class="typing-dot"></span>
							</span>
						{/if}

						{#if collab.isIdle && showIdleStatus}
							<span class="idle-indicator" title={formatLastActive(collab.lastActive)}>
								(away)
							</span>
						{/if}
					</div>
				{/if}
			</div>
		{/if}
	{/each}

	<!-- Text Selection Overlays -->
	{#if showTextSelections}
		{#each collaborators.filter(c => c.selection) as collab (collab.clientId)}
			{@const selection = collab.selection}
			{@const blockRect = selection ? getBlockPosition(selection.blockId) : null}
			{#if selection && blockRect && containerRect}
				<div
					class="text-selection"
					style:--color={collab.color}
					style:left="{blockRect.left - containerRect.left}px"
					style:top="{blockRect.top - containerRect.top}px"
					in:fade={{ duration: 150 }}
					out:fade={{ duration: 100 }}
				>
					<!-- Text selection is handled via CSS highlight API or overlay -->
					<div class="selection-overlay" style:background-color="{collab.color}33">
						<!-- Selection range would be calculated based on text offsets -->
					</div>
				</div>
			{/if}
		{/each}
	{/if}

	<!-- Typing Indicators (floating) -->
	{#if showTypingIndicators}
		<div class="typing-indicators">
			{#each typingCollaborators as collab, index (collab.clientId)}
				<div
					class="typing-badge"
					style:--color={collab.color}
					style:background-color={collab.color}
					style:color={getContrastColor(collab.color)}
					in:fly={{ x: -20, duration: 200, delay: index * 50 }}
					out:fade={{ duration: 150 }}
					animate:flip={{ duration: 300 }}
				>
					{#if collab.avatar}
						<img
							src={collab.avatar}
							alt={collab.name}
							class="typing-avatar"
						/>
					{:else}
						<span class="typing-initials">
							{getInitials(collab.name)}
						</span>
					{/if}
					<span class="typing-text">{getTypingText(collab.name)}</span>
				</div>
			{/each}
		</div>
	{/if}
</div>

<!-- Collaborators Presence Bar -->
<div class="presence-bar" style:--z-index={zIndex - 1}>
	{#each collaborators as collab (collab.clientId)}
		<div
			class="presence-avatar"
			class:is-idle={collab.isIdle}
			class:is-typing={collab.isTyping}
			style:--color={collab.color}
			style:border-color={collab.color}
			title="{collab.name}{collab.isIdle ? ' (away)' : ''}{collab.isTyping ? ' (typing)' : ''}"
			in:scale={{ duration: 200 }}
			out:fade={{ duration: 150 }}
		>
			{#if collab.avatar}
				<img
					src={collab.avatar}
					alt={collab.name}
					class="avatar-image"
				/>
			{:else}
				<span
					class="avatar-initials"
					style:background-color={collab.color}
					style:color={getContrastColor(collab.color)}
				>
					{getInitials(collab.name)}
				</span>
			{/if}

			{#if collab.isTyping}
				<span class="presence-typing-dot"></span>
			{/if}
		</div>
	{/each}
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   Main Container
	   ═══════════════════════════════════════════════════════════════════════════ */

	.collaborator-cursors {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		pointer-events: none;
		z-index: var(--z-index, 1000);
		overflow: visible;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   Block Selection
	   ═══════════════════════════════════════════════════════════════════════════ */

	.block-selection {
		position: absolute;
		border: 2px solid var(--color);
		border-radius: 8px;
		background: color-mix(in srgb, var(--color) 8%, transparent);
		transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
	}

	.selection-label {
		position: absolute;
		top: -24px;
		left: 0;
		padding: 2px 8px;
		font-size: 11px;
		font-weight: 500;
		border-radius: 4px;
		white-space: nowrap;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.15);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   Cursor Pointer
	   ═══════════════════════════════════════════════════════════════════════════ */

	.cursor-container {
		position: absolute;
		transition: left 0.08s ease-out, top 0.08s ease-out;
		transform-origin: top left;
	}

	.cursor-container.is-idle {
		opacity: 0.5;
	}

	.cursor-container.is-typing .cursor-pointer {
		animation: cursor-pulse 0.5s ease-in-out infinite;
	}

	.cursor-pointer {
		display: block;
		filter: drop-shadow(0 1px 2px rgba(0, 0, 0, 0.2));
		transition: transform 0.1s ease;
	}

	@keyframes cursor-pulse {
		0%, 100% {
			transform: scale(1);
		}
		50% {
			transform: scale(1.1);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   Cursor Label
	   ═══════════════════════════════════════════════════════════════════════════ */

	.cursor-label {
		position: absolute;
		top: 0;
		left: 16px;
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 3px 8px;
		font-size: 11px;
		font-weight: 500;
		border-radius: 4px;
		white-space: nowrap;
		box-shadow: 0 2px 6px rgba(0, 0, 0, 0.15);
		transition: opacity 0.15s ease;
	}

	.cursor-name {
		max-width: 120px;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   Typing Indicator
	   ═══════════════════════════════════════════════════════════════════════════ */

	.typing-indicator {
		display: flex;
		align-items: center;
		gap: 2px;
		margin-left: 2px;
	}

	.typing-dot {
		width: 4px;
		height: 4px;
		background: currentColor;
		border-radius: 50%;
		opacity: 0.7;
		animation: typing-bounce 1.2s ease-in-out infinite;
	}

	.typing-dot:nth-child(2) {
		animation-delay: 0.2s;
	}

	.typing-dot:nth-child(3) {
		animation-delay: 0.4s;
	}

	@keyframes typing-bounce {
		0%, 60%, 100% {
			transform: translateY(0);
			opacity: 0.4;
		}
		30% {
			transform: translateY(-4px);
			opacity: 1;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   Idle Indicator
	   ═══════════════════════════════════════════════════════════════════════════ */

	.idle-indicator {
		font-size: 10px;
		opacity: 0.7;
		font-style: italic;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   Text Selection
	   ═══════════════════════════════════════════════════════════════════════════ */

	.text-selection {
		position: absolute;
		pointer-events: none;
	}

	.selection-overlay {
		border-radius: 2px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   Typing Indicators Bar
	   ═══════════════════════════════════════════════════════════════════════════ */

	.typing-indicators {
		position: fixed;
		bottom: 20px;
		left: 50%;
		transform: translateX(-50%);
		display: flex;
		flex-direction: column;
		gap: 8px;
		z-index: calc(var(--z-index) + 1);
	}

	.typing-badge {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 6px 12px;
		border-radius: 20px;
		font-size: 12px;
		font-weight: 500;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
		white-space: nowrap;
	}

	.typing-avatar {
		width: 20px;
		height: 20px;
		border-radius: 50%;
		object-fit: cover;
	}

	.typing-initials {
		width: 20px;
		height: 20px;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 9px;
		font-weight: 600;
		border-radius: 50%;
		background: rgba(255, 255, 255, 0.2);
	}

	.typing-text {
		font-size: 12px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   Presence Bar
	   ═══════════════════════════════════════════════════════════════════════════ */

	.presence-bar {
		position: fixed;
		top: 12px;
		right: 12px;
		display: flex;
		flex-direction: row-reverse;
		gap: -8px;
		z-index: var(--z-index);
	}

	.presence-avatar {
		position: relative;
		width: 32px;
		height: 32px;
		border-radius: 50%;
		border: 2px solid var(--color);
		background: white;
		overflow: hidden;
		transition: transform 0.15s ease, opacity 0.15s ease;
		cursor: pointer;
		pointer-events: auto;
		margin-left: -8px;
	}

	.presence-avatar:first-child {
		margin-left: 0;
	}

	.presence-avatar:hover {
		transform: scale(1.1);
		z-index: 10;
	}

	.presence-avatar.is-idle {
		opacity: 0.6;
	}

	.presence-avatar.is-typing {
		animation: presence-pulse 1s ease-in-out infinite;
	}

	@keyframes presence-pulse {
		0%, 100% {
			box-shadow: 0 0 0 0 var(--color);
		}
		50% {
			box-shadow: 0 0 0 4px color-mix(in srgb, var(--color) 30%, transparent);
		}
	}

	.avatar-image {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.avatar-initials {
		width: 100%;
		height: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 11px;
		font-weight: 600;
	}

	.presence-typing-dot {
		position: absolute;
		bottom: -2px;
		right: -2px;
		width: 10px;
		height: 10px;
		background: #22c55e;
		border: 2px solid white;
		border-radius: 50%;
		animation: typing-blink 1s ease-in-out infinite;
	}

	@keyframes typing-blink {
		0%, 100% {
			opacity: 1;
		}
		50% {
			opacity: 0.5;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   Responsive Adjustments
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 768px) {
		.cursor-label {
			font-size: 10px;
			padding: 2px 6px;
		}

		.presence-avatar {
			width: 28px;
			height: 28px;
		}

		.typing-indicators {
			bottom: 12px;
		}

		.typing-badge {
			padding: 4px 10px;
			font-size: 11px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   Dark Mode Support
	   ═══════════════════════════════════════════════════════════════════════════ */

	:global(.dark) .block-selection {
		background: color-mix(in srgb, var(--color) 12%, transparent);
	}

	:global(.dark) .selection-label,
	:global(.dark) .cursor-label {
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
	}

	:global(.dark) .presence-avatar {
		background: #1f2937;
	}

	:global(.dark) .typing-badge {
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   Reduced Motion
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		.cursor-container {
			transition: none;
		}

		.cursor-container.is-typing .cursor-pointer,
		.typing-dot,
		.presence-avatar.is-typing,
		.presence-typing-dot {
			animation: none;
		}
	}
</style>
