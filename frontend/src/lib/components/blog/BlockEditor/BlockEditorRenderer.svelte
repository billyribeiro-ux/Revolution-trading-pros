<!--
/**
 * Block Renderer - Component-Based Architecture
 * ═══════════════════════════════════════════════════════════════════════════
 * REFACTORED: Jan 2026 - Reduced from 3,991 lines to ~150 lines
 * 
 * All block rendering is now delegated to individual components in cms/blocks/
 * This file is just a router that maps block types to their components.
 */
-->

<script lang="ts">
	import type { Block } from '$lib/components/cms/blocks/types';
	import type { BlockId } from '$lib/stores/blockState.svelte';
	import type { Component } from 'svelte';

	// ============================================
	// CONTENT BLOCKS
	// ============================================
	import ParagraphBlock from '../../cms/blocks/content/ParagraphBlock.svelte';
	import HeadingBlock from '../../cms/blocks/content/HeadingBlock.svelte';
	import QuoteBlock from '../../cms/blocks/content/QuoteBlock.svelte';
	import PullQuoteBlock from '../../cms/blocks/content/PullQuoteBlock.svelte';
	import CodeBlock from '../../cms/blocks/content/CodeBlock.svelte';
	import ListBlock from '../../cms/blocks/content/ListBlock.svelte';
	import ChecklistBlock from '../../cms/blocks/content/ChecklistBlock.svelte';

	// ============================================
	// MEDIA BLOCKS
	// ============================================
	import ImageBlock from '../../cms/blocks/media/ImageBlock.svelte';
	import VideoBlock from '../../cms/blocks/media/VideoBlock.svelte';
	import GalleryBlock from '../../cms/blocks/media/GalleryBlock.svelte';
	import AudioBlock from '../../cms/blocks/media/AudioBlock.svelte';
	import FileBlock from '../../cms/blocks/media/FileBlock.svelte';
	import EmbedBlock from '../../cms/blocks/media/EmbedBlock.svelte';
	import GifBlock from '../../cms/blocks/media/GifBlock.svelte';

	// ============================================
	// LAYOUT BLOCKS
	// ============================================
	import ColumnsBlock from '../../cms/blocks/layout/ColumnsBlock.svelte';
	import GroupBlock from '../../cms/blocks/layout/GroupBlock.svelte';
	import DividerBlock from '../../cms/blocks/layout/DividerBlock.svelte';
	import SpacerBlock from '../../cms/blocks/layout/SpacerBlock.svelte';

	// ============================================
	// INTERACTIVE BLOCKS
	// ============================================
	import AccordionBlock from '../../cms/blocks/interactive/AccordionBlock.svelte';
	import TabsBlock from '../../cms/blocks/interactive/TabsBlock.svelte';
	import ToggleBlock from '../../cms/blocks/interactive/ToggleBlock.svelte';
	import TocBlock from '../../cms/blocks/interactive/TocBlock.svelte';
	import ButtonsBlock from '../../cms/blocks/interactive/ButtonsBlock.svelte';

	// ============================================
	// ADVANCED BLOCKS
	// ============================================
	import CardBlock from '../../cms/blocks/advanced/CardBlock.svelte';
	import TestimonialBlock from '../../cms/blocks/advanced/TestimonialBlock.svelte';
	import CtaBlock from '../../cms/blocks/advanced/CtaBlock.svelte';
	import CountdownBlock from '../../cms/blocks/advanced/CountdownBlock.svelte';
	import SocialShareBlock from '../../cms/blocks/advanced/SocialShareBlock.svelte';
	import AuthorBlock from '../../cms/blocks/advanced/AuthorBlock.svelte';
	import RelatedPostsBlock from '../../cms/blocks/advanced/RelatedPostsBlock.svelte';
	import NewsletterBlock from '../../cms/blocks/advanced/NewsletterBlock.svelte';
	import CalloutBlock from '../../cms/blocks/advanced/CalloutBlock.svelte';
	import ButtonBlock from '../../cms/blocks/advanced/ButtonBlock.svelte';
	import HtmlBlock from '../../cms/blocks/advanced/HtmlBlock.svelte';

	// ============================================
	// TRADING BLOCKS
	// ============================================
	import ChartBlock from '../../cms/blocks/trading/ChartBlock.svelte';
	import TickerBlock from '../../cms/blocks/trading/TickerBlock.svelte';
	import PriceAlertBlock from '../../cms/blocks/trading/PriceAlertBlock.svelte';
	import TradingIdeaBlock from '../../cms/blocks/trading/TradingIdeaBlock.svelte';
	import RiskDisclaimerBlock from '../../cms/blocks/trading/RiskDisclaimerBlock.svelte';

	// ============================================
	// AI BLOCKS
	// ============================================
	import AIGeneratedBlock from '../../cms/blocks/ai/AIGeneratedBlock.svelte';
	import AISummaryBlock from '../../cms/blocks/ai/AISummaryBlock.svelte';
	import AITranslationBlock from '../../cms/blocks/ai/AITranslationBlock.svelte';

	// ============================================
	// COMPONENT MAP - ALL BLOCK TYPES
	// ============================================
	const blockComponentMap: Record<string, Component<any>> = {
		// Content
		paragraph: ParagraphBlock,
		heading: HeadingBlock,
		quote: QuoteBlock,
		pullquote: PullQuoteBlock,
		code: CodeBlock,
		list: ListBlock,
		checklist: ChecklistBlock,

		// Media
		image: ImageBlock,
		video: VideoBlock,
		gallery: GalleryBlock,
		audio: AudioBlock,
		file: FileBlock,
		embed: EmbedBlock,
		gif: GifBlock,

		// Layout
		columns: ColumnsBlock,
		group: GroupBlock,
		divider: DividerBlock,
		separator: DividerBlock, // Alias for divider
		spacer: SpacerBlock,

		// Interactive
		accordion: AccordionBlock,
		tabs: TabsBlock,
		toggle: ToggleBlock,
		toc: TocBlock,
		buttons: ButtonsBlock,

		// Advanced
		card: CardBlock,
		testimonial: TestimonialBlock,
		cta: CtaBlock,
		countdown: CountdownBlock,
		socialShare: SocialShareBlock,
		author: AuthorBlock,
		relatedPosts: RelatedPostsBlock,
		newsletter: NewsletterBlock,
		callout: CalloutBlock,
		button: ButtonBlock,
		html: HtmlBlock,

		// Trading
		chart: ChartBlock,
		ticker: TickerBlock,
		priceAlert: PriceAlertBlock,
		tradingIdea: TradingIdeaBlock,
		riskDisclaimer: RiskDisclaimerBlock,

		// AI
		aiGenerated: AIGeneratedBlock,
		aiSummary: AISummaryBlock,
		aiTranslation: AITranslationBlock
	};

	// ============================================
	// PROPS
	// ============================================
	interface Props {
		block: Block;
		blockId?: BlockId;
		isSelected?: boolean;
		isEditing?: boolean;
		onUpdate: (updates: Partial<Block>) => void;
		onError?: (error: Error) => void;
	}

	let props: Props = $props();

	// Derive blockId from block.id if not provided
	const blockId = $derived(props.blockId ?? props.block.id);
	const isSelected = $derived(props.isSelected ?? false);
	const isEditing = $derived(props.isEditing ?? true);

	// Get the component for this block type
	const BlockComponent = $derived(blockComponentMap[props.block.type]);
</script>

<!-- ============================================ -->
<!-- TEMPLATE - Component Delegation             -->
<!-- ============================================ -->

{#if BlockComponent}
	<BlockComponent
		block={props.block}
		{blockId}
		{isSelected}
		{isEditing}
		onUpdate={props.onUpdate}
		onError={props.onError}
	/>
{:else}
	<div class="unknown-block">
		<span class="unknown-block__icon">?</span>
		<span>Unknown block type: <code>{props.block.type}</code></span>
	</div>
{/if}

<style>
	.unknown-block {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		background: #fef2f2;
		border: 1px dashed #fecaca;
		border-radius: 8px;
		color: #991b1b;
		font-size: 0.875rem;
	}

	.unknown-block__icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 1.5rem;
		height: 1.5rem;
		background: #fee2e2;
		border-radius: 50%;
		font-weight: 600;
		font-size: 0.75rem;
	}

	.unknown-block code {
		font-family: ui-monospace, SFMono-Regular, 'SF Mono', Menlo, Consolas, monospace;
		background: #fee2e2;
		padding: 0.125rem 0.375rem;
		border-radius: 0.25rem;
		font-size: 0.8125rem;
	}

	:global(.dark) .unknown-block {
		background: #450a0a;
		border-color: #7f1d1d;
		color: #fca5a5;
	}

	:global(.dark) .unknown-block__icon {
		background: #7f1d1d;
	}

	:global(.dark) .unknown-block code {
		background: #7f1d1d;
	}
</style>
