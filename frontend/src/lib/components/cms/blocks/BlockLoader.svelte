<!--
/**
 * Block Loader - Lazy Loading Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Code splitting per block type for optimal bundle size
 */
-->

<script lang="ts">
	import type { Component as SvelteComponent } from 'svelte';
	import { IconLoader2 } from '$lib/icons';
	import type { Block, BlockType } from './types';
	import type { BlockId } from '$lib/stores/blockState.svelte';

	// Prop shape every block component receives from the loader — and exactly
	// what BlockLoader itself accepts (it's a pass-through wrapper). Individual
	// block components are free to declare a stricter `Block` type (e.g.
	// narrowed to their own content shape); at the dynamic-import boundary we
	// only know it matches this superset.
	interface Props {
		block: Block;
		blockId: BlockId;
		isEditing: boolean;
		isSelected: boolean;
		onUpdate: (updates: Partial<Block>) => void;
		onError?: (error: Error) => void;
	}

	type BlockComponent = SvelteComponent<Props>;
	type BlockModule = { default: BlockComponent };

	let props: Props = $props();

	let isLoading = $state(true);
	let loadError = $state<string | null>(null);
	let Component = $state<BlockComponent | null>(null);

	// Lookup intentionally keyed on a subset of `BlockType` — a few block-type
	// union members (e.g. `preformatted`, `row`, `separator`, `shortcode`,
	// `reusable`) currently have no dedicated loader and surface as
	// `Unknown block type: …` via the catch below. See LB-R28-1 in
	// R28-A-lib-cleanup.md.
	const blockImports: Partial<Record<BlockType, () => Promise<BlockModule>>> = {
		// Content blocks
		paragraph: () => import('./content/ParagraphBlock.svelte'),
		heading: () => import('./content/HeadingBlock.svelte'),
		code: () => import('./content/CodeBlock.svelte'),
		list: () => import('./content/ListBlock.svelte'),
		quote: () => import('./content/QuoteBlock.svelte'),
		pullquote: () => import('./content/PullQuoteBlock.svelte'),
		checklist: () => import('./content/ChecklistBlock.svelte'),
		// Media blocks
		image: () => import('./media/ImageBlock.svelte'),
		video: () => import('./media/VideoBlock.svelte'),
		gallery: () => import('./media/GalleryBlock.svelte'),
		audio: () => import('./media/AudioBlock.svelte'),
		file: () => import('./media/FileBlock.svelte'),
		embed: () => import('./media/EmbedBlock.svelte'),
		gif: () => import('./media/GifBlock.svelte'),
		// Layout blocks
		columns: () => import('./layout/ColumnsBlock.svelte'),
		group: () => import('./layout/GroupBlock.svelte'),
		divider: () => import('./layout/DividerBlock.svelte'),
		spacer: () => import('./layout/SpacerBlock.svelte'),
		// Interactive blocks
		accordion: () => import('./interactive/AccordionBlock.svelte'),
		tabs: () => import('./interactive/TabsBlock.svelte'),
		toggle: () => import('./interactive/ToggleBlock.svelte'),
		toc: () => import('./interactive/TocBlock.svelte'),
		buttons: () => import('./interactive/ButtonsBlock.svelte'),
		// Trading blocks
		ticker: () => import('./trading/TickerBlock.svelte'),
		priceAlert: () => import('./trading/PriceAlertBlock.svelte'),
		tradingIdea: () => import('./trading/TradingIdeaBlock.svelte'),
		chart: () => import('./trading/ChartBlock.svelte'),
		riskDisclaimer: () => import('./trading/RiskDisclaimerBlock.svelte'),
		// AI blocks
		aiGenerated: () => import('./ai/AIGeneratedBlock.svelte'),
		aiSummary: () => import('./ai/AISummaryBlock.svelte'),
		aiTranslation: () => import('./ai/AITranslationBlock.svelte'),
		// Advanced blocks
		card: () => import('./advanced/CardBlock.svelte'),
		testimonial: () => import('./advanced/TestimonialBlock.svelte'),
		cta: () => import('./advanced/CtaBlock.svelte'),
		countdown: () => import('./advanced/CountdownBlock.svelte'),
		socialShare: () => import('./advanced/SocialShareBlock.svelte'),
		author: () => import('./advanced/AuthorBlock.svelte'),
		newsletter: () => import('./advanced/NewsletterBlock.svelte'),
		callout: () => import('./advanced/CalloutBlock.svelte'),
		button: () => import('./advanced/ButtonBlock.svelte'),
		relatedPosts: () => import('./advanced/RelatedPostsBlock.svelte'),
		html: () => import('./advanced/HtmlBlock.svelte')
	};

	$effect(() => {
		loadBlock(props.block.type);
	});

	async function loadBlock(type: BlockType): Promise<void> {
		isLoading = true;
		loadError = null;

		try {
			const importer = blockImports[type];
			if (!importer) {
				throw new Error(`Unknown block type: ${type}`);
			}

			const module = await importer();
			Component = module.default;
		} catch (error) {
			loadError = error instanceof Error ? error.message : 'Failed to load block';
			props.onError?.(error instanceof Error ? error : new Error('Failed to load block'));
		} finally {
			isLoading = false;
		}
	}
</script>

{#if isLoading}
	<div class="block-loader" aria-busy="true" aria-label="Loading block">
		<IconLoader2 size={24} class="spinner" />
		<span>Loading {props.block.type}...</span>
	</div>
{:else if loadError}
	<div class="block-error" role="alert">
		<span class="error-icon">⚠️</span>
		<span class="error-text">{loadError}</span>
		<button type="button" onclick={() => loadBlock(props.block.type)}>Retry</button>
	</div>
{:else if Component}
	<Component
		block={props.block}
		blockId={props.blockId}
		isEditing={props.isEditing}
		isSelected={props.isSelected}
		onUpdate={props.onUpdate}
		onError={props.onError}
	/>
{/if}

<style>
	.block-loader {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
		padding: 2rem;
		background: #f8fafc;
		border: 1px dashed #e5e7eb;
		border-radius: 8px;
		color: #64748b;
		font-size: 0.875rem;
	}

	.block-loader :global(.spinner) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.block-error {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		background: #fef2f2;
		border: 1px solid #fecaca;
		border-radius: 8px;
	}

	.error-icon {
		font-size: 1.25rem;
	}

	.error-text {
		flex: 1;
		color: #dc2626;
		font-size: 0.875rem;
	}

	.block-error button {
		padding: 0.375rem 0.75rem;
		background: #dc2626;
		border: none;
		border-radius: 6px;
		color: white;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
	}

	.block-error button:hover {
		background: #b91c1c;
	}

	:global(.dark) .block-loader {
		background: #1e293b;
		border-color: #334155;
		color: #94a3b8;
	}

	:global(.dark) .block-error {
		background: #450a0a;
		border-color: #991b1b;
	}

	:global(.dark) .error-text {
		color: #fca5a5;
	}
</style>
