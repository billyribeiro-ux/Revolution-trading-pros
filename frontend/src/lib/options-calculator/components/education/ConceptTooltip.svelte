<script lang="ts">
	import IconBulb from '@tabler/icons-svelte-runes/icons/bulb';
	import IconTrendingUp from '@tabler/icons-svelte-runes/icons/trending-up';
	import IconTrendingDown from '@tabler/icons-svelte-runes/icons/trending-down';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import { backOut, cubicIn } from 'svelte/easing';
	import { prefersReducedMotion } from 'svelte/motion';
	import type { EasingFunction, TransitionConfig } from 'svelte/transition';
	import type { EducationEntry } from '../../engine/types.js';

	interface Props {
		entry: EducationEntry;
		visible: boolean;
		onClose: () => void;
	}

	let { entry, visible, onClose }: Props = $props();

	/** Explanations longer than this are clipped behind "Read more". */
	const PREVIEW_LENGTH = 200;

	const isLong = $derived(entry.fullExplanation.length > PREVIEW_LENGTH);

	interface PopParams {
		duration?: number;
		easing?: EasingFunction;
	}

	/**
	 * Pops the card in from 85% scale, 8px low and transparent, and back out the same way.
	 * Instant under reduced motion.
	 */
	function pop(
		_node: Element,
		{ duration = 300, easing = backOut }: PopParams = {}
	): TransitionConfig {
		return {
			duration: prefersReducedMotion.current ? 0 : duration,
			easing,
			css: (t, u) => `transform: translateY(${u * 8}px) scale(${0.85 + t * 0.15}); opacity: ${t};`
		};
	}
</script>

{#if visible}
	<!-- Scoped to this block, so every showing starts collapsed. -->
	{let isExpanded = $state(false)}
	{const explanation = $derived(
		isLong && !isExpanded
			? `${entry.fullExplanation.slice(0, PREVIEW_LENGTH)}...`
			: entry.fullExplanation
	)}

	<!-- Global: the card usually enters and leaves with its parent's block (EducationOverlay). -->
	<div
		in:pop|global
		out:pop|global={{ duration: 150, easing: cubicIn }}
		class="w-[360px] max-w-[90vw] rounded-2xl p-4 flex flex-col gap-3"
		style="
			background: var(--calc-surface);
			border: 1px solid var(--calc-border);
			box-shadow: 0 16px 48px rgba(0,0,0,0.4);
			backdrop-filter: blur(20px);
		"
		role="tooltip"
	>
		<!-- Header -->
		<div class="flex items-center justify-between">
			<div class="flex items-center gap-2">
				{#if entry.symbol}
					<span
						class="text-xs font-bold px-2 py-0.5 rounded-md"
						style="
							background: var(--calc-accent-glow);
							color: var(--calc-accent);
							font-family: var(--calc-font-mono);
						">{entry.symbol}</span
					>
				{/if}
				<h4
					class="text-sm font-semibold"
					style="color: var(--calc-text); font-family: var(--calc-font-display);"
				>
					{entry.term}
				</h4>
			</div>
			<button
				type="button"
				onclick={onClose}
				class="cursor-pointer rounded-md p-0.5"
				style="color: var(--calc-text-muted);"
				aria-label="Close tooltip"
			>
				<IconX size={14} aria-hidden="true" />
			</button>
		</div>

		<!-- Short description -->
		<p class="text-[11px] font-medium leading-relaxed" style="color: var(--calc-text-secondary);">
			{entry.shortDescription}
		</p>

		<!-- Full explanation (expandable) -->
		<div class="text-[10px] leading-relaxed" style="color: var(--calc-text-muted);">
			<p>{explanation}</p>
			{#if isLong && !isExpanded}
				<button
					type="button"
					onclick={() => (isExpanded = true)}
					class="text-[10px] font-medium cursor-pointer mt-1"
					style="color: var(--calc-accent);"
				>
					Read more
				</button>
			{/if}
		</div>

		<!-- When increases / decreases -->
		{#if entry.whenIncreases || entry.whenDecreases}
			<div class="flex flex-col gap-1.5">
				{#if entry.whenIncreases}
					<div class="flex items-start gap-1.5">
						<IconTrendingUp
							size={11}
							style="color: var(--calc-call); flex-shrink: 0; margin-top: 1px;"
							aria-hidden="true"
						/>
						<span class="text-[10px] leading-relaxed" style="color: var(--calc-text-muted);">
							{entry.whenIncreases}
						</span>
					</div>
				{/if}
				{#if entry.whenDecreases}
					<div class="flex items-start gap-1.5">
						<IconTrendingDown
							size={11}
							style="color: var(--calc-put); flex-shrink: 0; margin-top: 1px;"
							aria-hidden="true"
						/>
						<span class="text-[10px] leading-relaxed" style="color: var(--calc-text-muted);">
							{entry.whenDecreases}
						</span>
					</div>
				{/if}
			</div>
		{/if}

		<!-- Pro Tip -->
		<div
			class="flex items-start gap-2 rounded-lg px-3 py-2"
			style="background: rgba(245,158,11,0.06); border: 1px solid rgba(245,158,11,0.12);"
		>
			<IconBulb
				size={12}
				style="color: #f59e0b; flex-shrink: 0; margin-top: 1px;"
				aria-hidden="true"
			/>
			<p class="text-[10px] leading-relaxed" style="color: var(--calc-text-secondary);">
				<span class="font-semibold" style="color: #f59e0b;">Pro Tip:</span>
				{entry.proTip}
			</p>
		</div>

		<!-- Related terms -->
		{#if entry.relatedTerms && entry.relatedTerms.length > 0}
			<div class="flex items-center gap-1.5 flex-wrap">
				<span
					class="text-[9px] uppercase tracking-wider font-medium"
					style="color: var(--calc-text-muted);"
				>
					Related:
				</span>
				{#each entry.relatedTerms as termId (termId)}
					<span
						class="text-[9px] px-1.5 py-0.5 rounded"
						style="background: var(--calc-surface-hover); color: var(--calc-text-muted); border: 1px solid var(--calc-border);"
					>
						{termId}
					</span>
				{/each}
			</div>
		{/if}
	</div>
{/if}
