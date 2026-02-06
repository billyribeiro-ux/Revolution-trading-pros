<script lang="ts">
	import type { Block } from './types';
	import type { BlockId } from '$lib/stores/blockState.svelte';

	// Content blocks
	import ParagraphBlock from './content/ParagraphBlock.svelte';
	import HeadingBlock from './content/HeadingBlock.svelte';
	import QuoteBlock from './content/QuoteBlock.svelte';
	import PullQuoteBlock from './content/PullQuoteBlock.svelte';
	import CodeBlock from './content/CodeBlock.svelte';
	import ListBlock from './content/ListBlock.svelte';
	import ChecklistBlock from './content/ChecklistBlock.svelte';

	// Media blocks
	import ImageBlock from './media/ImageBlock.svelte';
	import VideoBlock from './media/VideoBlock.svelte';
	import AudioBlock from './media/AudioBlock.svelte';
	import GalleryBlock from './media/GalleryBlock.svelte';
	import FileBlock from './media/FileBlock.svelte';
	import EmbedBlock from './media/EmbedBlock.svelte';
	import GifBlock from './media/GifBlock.svelte';

	// Interactive blocks
	import AccordionBlock from './interactive/AccordionBlock.svelte';
	import TabsBlock from './interactive/TabsBlock.svelte';
	import ToggleBlock from './interactive/ToggleBlock.svelte';
	import TocBlock from './interactive/TocBlock.svelte';
	import ButtonsBlock from './interactive/ButtonsBlock.svelte';

	// Layout blocks
	import ColumnsBlock from './layout/ColumnsBlock.svelte';
	import GroupBlock from './layout/GroupBlock.svelte';
	import DividerBlock from './layout/DividerBlock.svelte';
	import SpacerBlock from './layout/SpacerBlock.svelte';

	// Trading blocks
	import TickerBlock from './trading/TickerBlock.svelte';
	import PriceAlertBlock from './trading/PriceAlertBlock.svelte';
	import TradingIdeaBlock from './trading/TradingIdeaBlock.svelte';
	import ChartBlock from './trading/ChartBlock.svelte';
	import RiskDisclaimerBlock from './trading/RiskDisclaimerBlock.svelte';

	// AI blocks
	import AIGeneratedBlock from './ai/AIGeneratedBlock.svelte';
	import AISummaryBlock from './ai/AISummaryBlock.svelte';
	import AITranslationBlock from './ai/AITranslationBlock.svelte';

	// Advanced blocks
	import CardBlock from './advanced/CardBlock.svelte';
	import TestimonialBlock from './advanced/TestimonialBlock.svelte';
	import CtaBlock from './advanced/CtaBlock.svelte';
	import CountdownBlock from './advanced/CountdownBlock.svelte';
	import SocialShareBlock from './advanced/SocialShareBlock.svelte';
	import AuthorBlock from './advanced/AuthorBlock.svelte';
	import RelatedPostsBlock from './advanced/RelatedPostsBlock.svelte';
	import NewsletterBlock from './advanced/NewsletterBlock.svelte';
	import HtmlBlock from './advanced/HtmlBlock.svelte';
	import ButtonBlock from './advanced/ButtonBlock.svelte';
	import CalloutBlock from './advanced/CalloutBlock.svelte';

	interface Props {
		block: Block;
		blockId: BlockId;
		isSelected: boolean;
		isEditing: boolean;
		onUpdate: (updates: Partial<Block>) => void;
		onError?: (error: Error) => void;
	}

	let props: Props = $props();

	// Map block types to components
	const componentMap: Record<string, any> = {
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
		audio: AudioBlock,
		gallery: GalleryBlock,
		file: FileBlock,
		embed: EmbedBlock,
		gif: GifBlock,
		// Interactive
		accordion: AccordionBlock,
		tabs: TabsBlock,
		toggle: ToggleBlock,
		toc: TocBlock,
		buttons: ButtonsBlock,
		// Layout
		columns: ColumnsBlock,
		group: GroupBlock,
		divider: DividerBlock,
		spacer: SpacerBlock,
		separator: DividerBlock, // Alias for divider
		// Trading
		ticker: TickerBlock,
		priceAlert: PriceAlertBlock,
		tradingIdea: TradingIdeaBlock,
		chart: ChartBlock,
		riskDisclaimer: RiskDisclaimerBlock,
		// AI
		aiGenerated: AIGeneratedBlock,
		aiSummary: AISummaryBlock,
		aiTranslation: AITranslationBlock,
		// Advanced
		card: CardBlock,
		testimonial: TestimonialBlock,
		cta: CtaBlock,
		countdown: CountdownBlock,
		socialShare: SocialShareBlock,
		author: AuthorBlock,
		relatedPosts: RelatedPostsBlock,
		newsletter: NewsletterBlock,
		html: HtmlBlock,
		button: ButtonBlock,
		callout: CalloutBlock
	};

	const BlockComponent = $derived(componentMap[props.block.type]);
</script>

{#if BlockComponent}
	<BlockComponent
		block={props.block}
		blockId={props.blockId}
		isSelected={props.isSelected}
		isEditing={props.isEditing}
		onUpdate={props.onUpdate}
		onError={props.onError}
	/>
{:else}
	<div class="block-renderer__unknown">
		<span class="block-renderer__unknown-icon">?</span>
		<span>Unknown block type: <code>{props.block.type}</code></span>
	</div>
{/if}

<style>
	.block-renderer__unknown {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		background: #fef2f2;
		border: 1px solid #fecaca;
		border-radius: 0.5rem;
		color: #dc2626;
		font-size: 0.875rem;
	}

	.block-renderer__unknown-icon {
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

	.block-renderer__unknown code {
		font-family: ui-monospace, SFMono-Regular, 'SF Mono', Menlo, Consolas, monospace;
		background: #fee2e2;
		padding: 0.125rem 0.375rem;
		border-radius: 0.25rem;
		font-size: 0.8125rem;
	}

	:global(.dark) .block-renderer__unknown {
		background: #450a0a;
		border-color: #7f1d1d;
		color: #fca5a5;
	}

	:global(.dark) .block-renderer__unknown-icon {
		background: #7f1d1d;
	}

	:global(.dark) .block-renderer__unknown code {
		background: #7f1d1d;
	}
</style>
