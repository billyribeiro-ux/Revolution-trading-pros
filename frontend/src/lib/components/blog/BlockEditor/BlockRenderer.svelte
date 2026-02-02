<!--
/**
 * Block Renderer - Renders different block types
 * ═══════════════════════════════════════════════════════════════════════════
 * Dynamic block rendering with inline editing support
 */
-->

<script lang="ts">
	import {
		IconPhoto,
		IconVideo,
		IconCheck,
		IconChartCandle,
		IconChartLine,
		IconAlertTriangle,
		IconPlus,
		IconX,
		IconGripVertical,
		IconChevronDown,
		IconChevronRight,
		IconChevronUp,
		IconMinus,
		IconListTree,
		IconExternalLink,
		IconToggleRight,
		IconBrandX,
		IconBrandFacebook,
		IconBrandLinkedin,
		IconMail,
		IconLink,
		IconQuote,
		IconUser,
		IconArrowRight,
		IconArticle,
		IconLoader2,
		IconAlertCircle,
		IconRobot,
		IconFileDescription,
		IconLanguage,
		IconRefresh,
		IconSparkles,
		IconWand,
		IconCopy,
		// Media block icons
		IconLayoutGrid,
		IconVolume,
		IconVolumeOff,
		IconPlayerPlay,
		IconPlayerPause,
		IconWaveSine,
		IconFile,
		IconDownload,
		IconGif,
		IconMaximize,
		IconSearch,
		IconBrandInstagram,
		IconBrandTiktok,
		IconWorld,
		// Trading block icons
		IconTrendingUp,
		IconTrendingDown,
		IconBellRinging,
		IconTarget,
		IconArrowUp,
		IconArrowDown,
		IconCurrencyDollar,
		IconShieldCheck,
		IconFlame,
		IconClock,
		IconBulb
	} from '$lib/icons';
	import { onMount, onDestroy } from 'svelte';
	import DOMPurify from 'isomorphic-dompurify';

	import { API_BASE_URL } from '$lib/api/config';
	import { getAuthToken } from '$lib/stores/auth.svelte';
	import type { Block, BlockContent } from './types';
	import type { Component } from 'svelte';

	// ==========================================================================
	// Block Component Imports (Phase 3 Advanced Blocks)
	// ==========================================================================

	// Media blocks
	import ImageBlock from '../../cms/blocks/media/ImageBlock.svelte';
	import VideoBlock from '../../cms/blocks/media/VideoBlock.svelte';
	import GalleryBlock from '../../cms/blocks/media/GalleryBlock.svelte';

	// Layout blocks
	import ColumnsBlock from '../../cms/blocks/layout/ColumnsBlock.svelte';
	import GroupBlock from '../../cms/blocks/layout/GroupBlock.svelte';
	import DividerBlock from '../../cms/blocks/layout/DividerBlock.svelte';
	import SpacerBlock from '../../cms/blocks/layout/SpacerBlock.svelte';

	// Content blocks
	import PullQuoteBlock from '../../cms/blocks/content/PullQuoteBlock.svelte';
	import ChecklistBlock from '../../cms/blocks/content/ChecklistBlock.svelte';

	// Trading blocks
	import ChartBlock from '../../cms/blocks/trading/ChartBlock.svelte';
	import RiskDisclaimerBlock from '../../cms/blocks/trading/RiskDisclaimerBlock.svelte';

	// Advanced blocks
	import CalloutBlock from '../../cms/blocks/advanced/CalloutBlock.svelte';
	import ButtonBlock from '../../cms/blocks/advanced/ButtonBlock.svelte';
	import RelatedPostsBlock from '../../cms/blocks/advanced/RelatedPostsBlock.svelte';
	import HtmlBlock from '../../cms/blocks/advanced/HtmlBlock.svelte';

	// ==========================================================================
	// Block Component Map
	// ==========================================================================

	/**
	 * Maps block types to their corresponding Svelte components.
	 * Components in this map will be rendered using svelte:component
	 * instead of inline template rendering.
	 */
	const blockComponentMap: Record<string, Component<any>> = {
		// Media blocks
		image: ImageBlock,
		video: VideoBlock,
		gallery: GalleryBlock,
		// Layout blocks
		columns: ColumnsBlock,
		group: GroupBlock,
		divider: DividerBlock,
		spacer: SpacerBlock,
		// Content blocks
		pullquote: PullQuoteBlock,
		checklist: ChecklistBlock,
		// Trading blocks
		chart: ChartBlock,
		riskDisclaimer: RiskDisclaimerBlock,
		// Advanced blocks
		callout: CalloutBlock,
		button: ButtonBlock,
		relatedPosts: RelatedPostsBlock,
		html: HtmlBlock
	};

	/**
	 * Gets the component for a block type from the component map.
	 * Returns undefined if the block type should use inline rendering.
	 */
	function getBlockComponent(blockType: string): Component<any> | undefined {
		return blockComponentMap[blockType];
	}

	/**
	 * Check if a block type has a dedicated component
	 */
	function hasBlockComponent(blockType: string): boolean {
		return blockType in blockComponentMap;
	}

	// ==========================================================================
	// Props
	// ==========================================================================

	interface Props {
		block: Block;
		isSelected?: boolean;
		isEditing?: boolean;
		onUpdate: (updates: Partial<Block>) => void;
	}

	let { block, isSelected = false, isEditing = true, onUpdate }: Props = $props();

	// ==========================================================================
	// State
	// ==========================================================================

	// Used in bind:this directives for paragraph and heading blocks (lines 133, 165)
	// Prefixed with _ to indicate intentional: used in template, not in script
	let _editableRef = $state<HTMLElement | undefined>(undefined);

	// Countdown timer state
	let countdownInterval: ReturnType<typeof setInterval> | null = null;
	let countdownValues = $state<{ days: number; hours: number; minutes: number; seconds: number }>({
		days: 0, hours: 0, minutes: 0, seconds: 0
	});

	// Newsletter state
	let newsletterEmail = $state<string>('');
	let newsletterSubmitting = $state<boolean>(false);
	let newsletterSuccess = $state<boolean>(false);

	// Social share state
	let linkCopied = $state<boolean>(false);

	// Reusable/Global component state
	let reusableComponentData = $state<Block[] | null>(null);
	let reusableComponentLoading = $state<boolean>(false);
	let reusableComponentError = $state<string | null>(null);
	let reusableComponentVersion = $state<number>(0);
	let reusableIsSynced = $state<boolean>(true);
	let reusableNeedsUpdate = $state<boolean>(false);

	// Interactive block states
	let accordionOpenItems = $state<Set<string>>(new Set());
	let activeTabId = $state<string>('');
	let toggleOpen = $state<boolean>(false);
	let tocOpen = $state<boolean>(true);
	let activeTocHeading = $state<string>('');

	// Media block states
	let audioPlaying = $state<boolean>(false);
	let audioProgress = $state<number>(0);
	let audioVolume = $state<number>(1);
	let audioMuted = $state<boolean>(false);
	let audioRef = $state<HTMLAudioElement | null>(null);
	let gifPlaying = $state<boolean>(true);
	let lightboxOpen = $state<boolean>(false);
	let lightboxIndex = $state<number>(0);

	// Error boundary state
	let renderError = $state<Error | null>(null);

	// P0 Security: Sanitized HTML content
	let sanitizedHTML = $derived(
		block.content.html ? DOMPurify.sanitize(block.content.html, {
			ALLOWED_TAGS: ['p', 'br', 'strong', 'em', 'u', 's', 'a', 'ul', 'ol', 'li', 'h1', 'h2', 'h3', 'h4', 'h5', 'h6', 'blockquote', 'pre', 'code', 'img', 'div', 'span', 'table', 'thead', 'tbody', 'tr', 'th', 'td'],
			ALLOWED_ATTR: ['href', 'src', 'alt', 'title', 'class', 'id', 'target', 'rel', 'style']
		}) : ''
	);

	// ==========================================================================
	// Interactive Block Functions
	// ==========================================================================

	function toggleAccordionItem(itemId: string, allowMultiple: boolean = false) {
		const newSet = new Set(accordionOpenItems);
		if (newSet.has(itemId)) { newSet.delete(itemId); }
		else { if (!allowMultiple) { newSet.clear(); } newSet.add(itemId); }
		accordionOpenItems = newSet;
	}
	function setActiveTab(tabId: string) { activeTabId = tabId; }
	function handleToggle() { toggleOpen = !toggleOpen; }
	function scrollToHeading(headingId: string) {
		const element = document.getElementById(headingId);
		if (element) { element.scrollIntoView({ behavior: 'smooth', block: 'start' }); activeTocHeading = headingId; }
	}
	function toggleTocMobile() { tocOpen = !tocOpen; }

	// ==========================================================================
	// Media Block Functions
	// ==========================================================================

	// Audio player functions
	function toggleAudioPlay() {
		if (!audioRef) return;
		if (audioPlaying) { audioRef.pause(); }
		else { audioRef.play(); }
		audioPlaying = !audioPlaying;
	}

	function handleAudioTimeUpdate() {
		if (!audioRef) return;
		audioProgress = (audioRef.currentTime / audioRef.duration) * 100 || 0;
	}

	function handleAudioSeek(e: MouseEvent) {
		if (!audioRef) return;
		const container = e.currentTarget as HTMLElement;
		const rect = container.getBoundingClientRect();
		const percent = (e.clientX - rect.left) / rect.width;
		audioRef.currentTime = percent * audioRef.duration;
	}

	function toggleAudioMute() {
		if (!audioRef) return;
		audioMuted = !audioMuted;
		audioRef.muted = audioMuted;
	}

	function handleVolumeChange(e: Event) {
		if (!audioRef) return;
		const value = parseFloat((e.target as HTMLInputElement).value);
		audioVolume = value;
		audioRef.volume = value;
		audioMuted = value === 0;
	}

	function formatAudioTime(seconds: number): string {
		if (!seconds || isNaN(seconds)) return '0:00';
		const mins = Math.floor(seconds / 60);
		const secs = Math.floor(seconds % 60);
		return mins + ':' + secs.toString().padStart(2, '0');
	}

	// Gallery lightbox functions
	function openLightbox(index: number) {
		lightboxIndex = index;
		lightboxOpen = true;
		document.body.style.overflow = 'hidden';
	}

	function closeLightbox() {
		lightboxOpen = false;
		document.body.style.overflow = '';
	}

	function nextImage() {
		const images = block.content.galleryImages || [];
		lightboxIndex = (lightboxIndex + 1) % images.length;
	}

	function prevImage() {
		const images = block.content.galleryImages || [];
		lightboxIndex = (lightboxIndex - 1 + images.length) % images.length;
	}

	function addGalleryImage() {
		const images = block.content.galleryImages || [];
		updateContent({ galleryImages: [...images, { id: crypto.randomUUID(), url: '', alt: '' }] });
	}

	function updateGalleryImage(index: number, field: string, value: string) {
		const images = [...(block.content.galleryImages || [])];
		images[index] = { ...images[index], [field]: value };
		updateContent({ galleryImages: images });
	}

	function removeGalleryImage(index: number) {
		const images = [...(block.content.galleryImages || [])];
		images.splice(index, 1);
		updateContent({ galleryImages: images });
	}

	// GIF functions
	function toggleGifPlay() {
		gifPlaying = !gifPlaying;
	}

	// File helper functions
	function formatFileSize(bytes: number): string {
		if (bytes === 0) return '0 Bytes';
		const k = 1024;
		const sizes = ['Bytes', 'KB', 'MB', 'GB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
	}

	function getFileIcon(filename: string): string {
		const ext = filename.split('.').pop()?.toLowerCase() || '';
		if (['pdf'].includes(ext)) return 'pdf';
		if (['doc', 'docx'].includes(ext)) return 'doc';
		if (['xls', 'xlsx', 'csv'].includes(ext)) return 'xls';
		if (['zip', 'rar', '7z', 'tar', 'gz'].includes(ext)) return 'zip';
		if (['jpg', 'jpeg', 'png', 'gif', 'webp', 'svg'].includes(ext)) return 'img';
		return 'other';
	}

	// Embed helper functions
	function getEmbedPlatform(url: string): string {
		if (!url) return 'unknown';
		if (url.includes('twitter.com') || url.includes('x.com')) return 'twitter';
		if (url.includes('instagram.com')) return 'instagram';
		if (url.includes('tiktok.com')) return 'tiktok';
		if (url.includes('spotify.com')) return 'spotify';
		if (url.includes('soundcloud.com')) return 'soundcloud';
		return 'unknown';
	}

	function getEmbedId(url: string, platform: string): string {
		try {
			const urlObj = new URL(url);
			switch (platform) {
				case 'spotify': {
					const parts = urlObj.pathname.split('/');
					return parts[parts.length - 1];
				}
				default:
					return '';
			}
		} catch {
			return '';
		}
	}

	// ==========================================================================
	// Countdown Timer Functions
	// ==========================================================================

	function updateCountdown(targetDate: string) {
		const target = new Date(targetDate).getTime();
		const now = new Date().getTime();
		const diff = target - now;
		if (diff <= 0) {
			countdownValues = { days: 0, hours: 0, minutes: 0, seconds: 0 };
			if (countdownInterval) { clearInterval(countdownInterval); countdownInterval = null; }
			return;
		}
		countdownValues = {
			days: Math.floor(diff / (1000 * 60 * 60 * 24)),
			hours: Math.floor((diff % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60)),
			minutes: Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60)),
			seconds: Math.floor((diff % (1000 * 60)) / 1000)
		};
	}

	function startCountdown(targetDate: string) {
		if (countdownInterval) { clearInterval(countdownInterval); }
		updateCountdown(targetDate);
		countdownInterval = setInterval(() => updateCountdown(targetDate), 1000);
	}

	// ==========================================================================
	// Social Share Functions
	// ==========================================================================

	function shareOnTwitter() {
		const url = encodeURIComponent(window.location.href);
		const text = encodeURIComponent(document.title);
		window.open(`https://twitter.com/intent/tweet?url=${url}&text=${text}`, '_blank', 'width=600,height=400');
	}

	function shareOnFacebook() {
		const url = encodeURIComponent(window.location.href);
		window.open(`https://www.facebook.com/sharer/sharer.php?u=${url}`, '_blank', 'width=600,height=400');
	}

	function shareOnLinkedIn() {
		const url = encodeURIComponent(window.location.href);
		window.open(`https://www.linkedin.com/sharing/share-offsite/?url=${url}`, '_blank', 'width=600,height=400');
	}

	function shareViaEmail() {
		const subject = encodeURIComponent(document.title);
		const body = encodeURIComponent(`Check out this article: ${window.location.href}`);
		window.location.href = `mailto:?subject=${subject}&body=${body}`;
	}

	async function copyShareLink() {
		try {
			await navigator.clipboard.writeText(window.location.href);
			linkCopied = true;
			setTimeout(() => { linkCopied = false; }, 2000);
		} catch {
			const input = document.createElement('input');
			input.value = window.location.href;
			document.body.appendChild(input);
			input.select();
			document.execCommand('copy');
			document.body.removeChild(input);
			linkCopied = true;
			setTimeout(() => { linkCopied = false; }, 2000);
		}
	}

	// ==========================================================================
	// Newsletter Functions
	// ==========================================================================

	async function handleNewsletterSubmit(e: Event) {
		e.preventDefault();
		if (!newsletterEmail || newsletterSubmitting) return;
		newsletterSubmitting = true;
		await new Promise((resolve) => setTimeout(resolve, 1000));
		newsletterSuccess = true;
		newsletterSubmitting = false;
		newsletterEmail = '';
	}

	// ==========================================================================
	// Lifecycle
	// ==========================================================================

	onMount(() => {
		if (block.type === 'countdown' && block.content.text) {
			startCountdown(block.content.text);
		}
	});

	onDestroy(() => {
		// P0: Clear all timers
		if (countdownInterval) {
			clearInterval(countdownInterval);
			countdownInterval = null;
		}

		// P0: Clean up audio resources
		if (audioRef) {
			audioRef.pause();
			audioRef.src = '';
			audioRef.load();
			audioRef = null;
		}

		// P0: Restore body scroll if lightbox was open
		if (lightboxOpen) {
			document.body.style.overflow = '';
			lightboxOpen = false;
		}

		// P0: Reset error state
		renderError = null;
	});

	// ==========================================================================
	// Content Update
	// ==========================================================================

	function updateContent(updates: Partial<BlockContent>) {
		onUpdate({
			content: { ...block.content, ...updates }
		});
	}

	function handleTextInput(e: Event) {
		const target = e.target as HTMLElement;
		updateContent({ text: target.textContent || '' });
	}

	function handlePaste(e: ClipboardEvent) {
		e.preventDefault();
		const text = e.clipboardData?.getData('text/plain') || '';
		document.execCommand('insertText', false, text);
	}

	// List handling
	function addListItem(index: number) {
		const items = [...(block.content.listItems || [])];
		items.splice(index + 1, 0, '');
		updateContent({ listItems: items });
	}

	function updateListItem(index: number, value: string) {
		const items = [...(block.content.listItems || [])];
		items[index] = value;
		updateContent({ listItems: items });
	}

	function removeListItem(index: number) {
		const items = [...(block.content.listItems || [])];
		if (items.length > 1) {
			items.splice(index, 1);
			updateContent({ listItems: items });
		}
	}

	// Checklist handling
	function toggleCheckItem(itemId: string) {
		const items =
			block.content.items?.map((item) =>
				item.id === itemId ? { ...item, checked: !item.checked } : item
			) || [];
		updateContent({ items });
	}

	// ==========================================================================
	// Style Computation
	// ==========================================================================

	function getBlockStyles(): string {
		const s = block.settings;
		const styles: string[] = [];

		if (s.textAlign) styles.push(`text-align: ${s.textAlign}`);
		if (s.fontSize) styles.push(`font-size: ${s.fontSize}`);
		if (s.fontWeight) styles.push(`font-weight: ${s.fontWeight}`);
		if (s.fontFamily) styles.push(`font-family: ${s.fontFamily}`);
		if (s.lineHeight) styles.push(`line-height: ${s.lineHeight}`);
		if (s.textColor) styles.push(`color: ${s.textColor}`);
		if (s.backgroundColor) styles.push(`background-color: ${s.backgroundColor}`);
		if (s.marginTop) styles.push(`margin-top: ${s.marginTop}`);
		if (s.marginBottom) styles.push(`margin-bottom: ${s.marginBottom}`);
		if (s.paddingTop) styles.push(`padding-top: ${s.paddingTop}`);
		if (s.paddingBottom) styles.push(`padding-bottom: ${s.paddingBottom}`);
		if (s.paddingLeft) styles.push(`padding-left: ${s.paddingLeft}`);
		if (s.paddingRight) styles.push(`padding-right: ${s.paddingRight}`);
		if (s.borderWidth) styles.push(`border-width: ${s.borderWidth}`);
		if (s.borderColor) styles.push(`border-color: ${s.borderColor}`);
		if (s.borderStyle) styles.push(`border-style: ${s.borderStyle}`);
		if (s.borderRadius) styles.push(`border-radius: ${s.borderRadius}`);
		if (s.boxShadow) styles.push(`box-shadow: ${s.boxShadow}`);

		return styles.join('; ');
	}

	// ==========================================================================
	// AI Block State
	// ==========================================================================

	let aiLoading = $state<boolean>(false);
	let aiError = $state<string | null>(null);
	let aiSummaryCollapsed = $state<boolean>(false);
	let aiTranslationView = $state<'stacked' | 'side-by-side'>('stacked');
	let copiedText = $state<boolean>(false);

	// ==========================================================================
	// AI Block Functions
	// ==========================================================================

	async function generateAIContent(): Promise<void> {
		if (!block.content.aiPrompt) return;
		aiLoading = true;
		aiError = null;
		try {
			// Simulate AI generation - replace with actual API call
			await new Promise((resolve) => setTimeout(resolve, 2000));
			const mockOutput = `This is AI-generated content based on your prompt: "${block.content.aiPrompt}"\n\nThe content would be generated by the selected model (${block.content.aiModel || 'gpt-4'}) and appear here with proper formatting.`;
			updateContent({ aiOutput: mockOutput });
			onUpdate({ metadata: { ...block.metadata, aiGenerated: true } });
		} catch (err) {
			aiError = err instanceof Error ? err.message : 'Failed to generate content';
		} finally {
			aiLoading = false;
		}
	}

	async function regenerateAIContent(): Promise<void> {
		await generateAIContent();
	}

	async function generateAISummary(): Promise<void> {
		aiLoading = true;
		aiError = null;
		try {
			// Simulate AI summary generation - replace with actual API call
			await new Promise((resolve) => setTimeout(resolve, 1500));
			const lengthMap = { short: 50, medium: 100, long: 200 };
			const length = lengthMap[block.settings.summaryLength || 'medium'];
			const mockSummary = `This is an AI-generated summary of the content. The summary is configured to be ${block.settings.summaryLength || 'medium'} length (approximately ${length} words). It provides a concise overview of the main points discussed in the article.`;
			updateContent({ aiSummary: mockSummary });
		} catch (err) {
			aiError = err instanceof Error ? err.message : 'Failed to generate summary';
		} finally {
			aiLoading = false;
		}
	}

	function toggleSummaryCollapsed(): void {
		aiSummaryCollapsed = !aiSummaryCollapsed;
	}

	async function translateContent(): Promise<void> {
		if (!block.content.sourceText) return;
		aiLoading = true;
		aiError = null;
		try {
			// Simulate AI translation - replace with actual API call
			await new Promise((resolve) => setTimeout(resolve, 1800));
			const mockTranslation = `[Translated to ${block.settings.targetLanguage || 'Spanish'}]\n\n${block.content.sourceText}`;
			updateContent({ translatedText: mockTranslation });
		} catch (err) {
			aiError = err instanceof Error ? err.message : 'Failed to translate content';
		} finally {
			aiLoading = false;
		}
	}

	function toggleTranslationView(): void {
		aiTranslationView = aiTranslationView === 'stacked' ? 'side-by-side' : 'stacked';
	}

	async function copyToClipboard(text: string): Promise<void> {
		try {
			await navigator.clipboard.writeText(text);
			copiedText = true;
			setTimeout(() => { copiedText = false; }, 2000);
		} catch (err) {
			console.error('Failed to copy text:', err);
		}
	}

	function retryAIAction(action: 'generate' | 'summary' | 'translate'): void {
		aiError = null;
		if (action === 'generate') generateAIContent();
		else if (action === 'summary') generateAISummary();
		else if (action === 'translate') translateContent();
	}

	// ==========================================================================
	// Reusable/Global Component Functions
	// ==========================================================================

	function getReusableAuthHeaders(): Record<string, string> {
		const token = getAuthToken();
		return {
			'Content-Type': 'application/json',
			...(token && { Authorization: `Bearer ${token}` })
		};
	}

	async function loadReusableComponent(): Promise<void> {
		const componentId = block.content.reusableComponentId;
		if (!componentId) {
			reusableComponentError = 'No component ID specified';
			return;
		}
		reusableComponentLoading = true;
		reusableComponentError = null;
		try {
			let response = await fetch(
				`${API_BASE_URL}/api/cms/global-components/public/${componentId}`,
				{ credentials: 'include' }
			);
			if (!response.ok) {
				response = await fetch(
					`${API_BASE_URL}/api/cms/global-components/${componentId}`,
					{ headers: getReusableAuthHeaders(), credentials: 'include' }
				);
			}
			if (!response.ok) throw new Error('Failed to load component');
			const data = await response.json();
			reusableComponentData = data.component_data || [];
			reusableComponentVersion = data.version || 1;
			const syncedVersion = block.content.reusableSyncedVersion || 0;
			reusableNeedsUpdate = syncedVersion > 0 && syncedVersion < reusableComponentVersion;
			reusableIsSynced = block.content.reusableIsSynced !== false;
		} catch (err) {
			reusableComponentError = err instanceof Error ? err.message : 'Failed to load component';
		} finally {
			reusableComponentLoading = false;
		}
	}

	function handleDetachComponent(): void {
		if (!reusableComponentData) return;
		onUpdate({
			type: 'group',
			content: {
				...block.content,
				children: reusableComponentData,
				reusableDetachedFrom: block.content.reusableComponentId,
				reusableComponentId: undefined,
				reusableIsSynced: false
			}
		});
	}

	async function handleSyncComponent(): Promise<void> {
		if (!block.content.reusableComponentId) return;
		await loadReusableComponent();
		onUpdate({
			content: {
				...block.content,
				reusableSyncedVersion: reusableComponentVersion
			}
		});
		reusableNeedsUpdate = false;
	}

	// Load reusable component on mount if needed
	$effect(() => {
		if (block.type === 'reusable' && block.content.reusableComponentId) {
			loadReusableComponent();
		}
	});
</script>

<div class="block-renderer block-{block.type}" style={getBlockStyles()}>
	<!-- Component-Based Block Rendering -->
	<!-- These blocks use imported Svelte components instead of inline templates -->
	{#if hasBlockComponent(block.type)}
		{@const BlockComponent = getBlockComponent(block.type)}
		{#if BlockComponent}
			<svelte:component
				this={BlockComponent}
				{block}
				{isSelected}
				{isEditing}
				{onUpdate}
			/>
		{/if}

	<!-- Paragraph Block -->
	{:else if block.type === 'paragraph'}
		<p
			bind:this={_editableRef}
			contenteditable={isEditing}
			class="editable-content"
			class:placeholder={!block.content.text}
			oninput={handleTextInput}
			onpaste={handlePaste}
			data-placeholder="Start writing or type / for commands..."
		>
			{block.content.text || ''}
		</p>

		<!-- Heading Block -->
	{:else if block.type === 'heading'}
		{@const level = block.settings.level || 2}
		<div class="heading-block-wrapper">
			{#if isEditing && isSelected}
				<div class="heading-level-selector">
					{#each [1, 2, 3, 4, 5, 6] as lvl}
						<button
							type="button"
							class="level-btn"
							class:active={level === lvl}
							onclick={() =>
								onUpdate({ settings: { ...block.settings, level: lvl as 1 | 2 | 3 | 4 | 5 | 6 } })}
							title="Heading {lvl}"
						>
							H{lvl}
						</button>
					{/each}
				</div>
			{/if}
			<svelte:element
				this={`h${level}`}
				bind:this={_editableRef}
				contenteditable={isEditing}
				class="editable-content heading-{level}"
				class:placeholder={!block.content.text}
				oninput={handleTextInput}
				onpaste={handlePaste}
				data-placeholder={`Heading ${level}`}
				id={block.settings.anchor || undefined}
			>
				{block.content.text || ''}
			</svelte:element>
		</div>

		<!-- Quote Block -->
	{:else if block.type === 'quote'}
		<blockquote class="quote-block">
			<div
				contenteditable={isEditing}
				class="quote-text editable-content"
				class:placeholder={!block.content.text}
				oninput={handleTextInput}
				onpaste={handlePaste}
				data-placeholder="Write a quote..."
			>
				{block.content.text || ''}
			</div>
			{#if block.content.html}
				<cite class="quote-cite">{block.content.html}</cite>
			{/if}
		</blockquote>

		<!-- Pull Quote Block -->
	{:else if block.type === 'pullquote'}
		<figure class="pullquote-block">
			<blockquote
				contenteditable={isEditing}
				class="pullquote-text editable-content"
				class:placeholder={!block.content.text}
				oninput={handleTextInput}
				onpaste={handlePaste}
				data-placeholder="Add a notable quote..."
			>
				{block.content.text || ''}
			</blockquote>
		</figure>

		<!-- Code Block -->
	{:else if block.type === 'code'}
		<div class="code-block" role="region" aria-label="Code block">
			<div class="code-header">
				<select
					value={block.content.language || 'javascript'}
					onchange={(e: Event) =>
						updateContent({ language: (e.target as HTMLSelectElement).value })}
					disabled={!isEditing}
					aria-label="Select programming language"
				>
					<option value="javascript">JavaScript</option>
					<option value="typescript">TypeScript</option>
					<option value="python">Python</option>
					<option value="html">HTML</option>
					<option value="css">CSS</option>
					<option value="json">JSON</option>
					<option value="bash">Bash</option>
					<option value="sql">SQL</option>
					<option value="php">PHP</option>
					<option value="rust">Rust</option>
					<option value="go">Go</option>
				</select>
			</div>
			<pre class="code-content"><code
					contenteditable={isEditing}
					class="language-{block.content.language || 'javascript'}"
					oninput={(e: Event) =>
						updateContent({ code: (e.target as HTMLElement).textContent || '' })}
					onpaste={handlePaste}>{block.content.code || ''}</code
				></pre>
		</div>

		<!-- List Block -->
	{:else if block.type === 'list'}
		{@const ListTag = block.content.listType === 'number' ? 'ol' : 'ul'}
		<svelte:element this={ListTag} class="list-block">
			{#each block.content.listItems || [''] as item, index}
				<li class="list-item">
					<span
						role="textbox"
						tabindex="0"
						contenteditable={isEditing}
						class="list-text editable-content"
						oninput={(e: Event) =>
							updateListItem(index, (e.target as HTMLElement).textContent || '')}
						onkeydown={(e: KeyboardEvent) => {
							if (e.key === 'Enter') {
								e.preventDefault();
								addListItem(index);
							} else if (
								e.key === 'Backspace' &&
								!item &&
								(block.content.listItems?.length || 0) > 1
							) {
								e.preventDefault();
								removeListItem(index);
							}
						}}
						onpaste={handlePaste}
						data-placeholder="List item"
					>
						{item}
					</span>
					{#if isEditing && (block.content.listItems?.length || 0) > 1}
						<button
							type="button"
							class="remove-item"
							onclick={() => removeListItem(index)}
							aria-label="Remove list item"
						>
							<IconX size={14} aria-hidden="true" />
						</button>
					{/if}
				</li>
			{/each}
		</svelte:element>
		{#if isEditing}
			<button
				type="button"
				class="add-item-btn"
				onclick={() => addListItem((block.content.listItems?.length || 1) - 1)}
				aria-label="Add new list item"
			>
				<IconPlus size={14} aria-hidden="true" />
				Add item
			</button>
		{/if}

		<!-- Checklist Block -->
	{:else if block.type === 'checklist'}
		<div class="checklist-block" role="group" aria-label="Checklist">
			{#each block.content.items || [] as item}
				<label class="check-item" class:checked={item.checked}>
					<input
						type="checkbox"
						checked={item.checked}
						onchange={() => toggleCheckItem(item.id)}
						disabled={!isEditing}
						aria-label={item.content || 'Checklist item'}
					/>
					<span class="check-icon">
						{#if item.checked}
							<IconCheck size={14} />
						{/if}
					</span>
					<span
						class="check-text editable-content"
						oninput={(e: Event) => {
							const items = block.content.items?.map((i) =>
								i.id === item.id
									? { ...i, content: (e.target as HTMLElement).textContent || '' }
									: i
							);
							if (items) updateContent({ items });
						}}
						onpaste={handlePaste}
					>
						{item.content}
					</span>
				</label>
			{/each}
			{#if isEditing}
				<button
					type="button"
					class="add-item-btn"
					onclick={() => {
						const items = [
							...(block.content.items || []),
							{
								id: `item_${Date.now()}`,
								content: '',
								checked: false
							}
						];
						updateContent({ items });
					}}
					aria-label="Add new checklist item"
				>
					<IconPlus size={14} aria-hidden="true" />
					Add item
				</button>
			{/if}
		</div>

		<!-- Image Block -->
	{:else if block.type === 'image'}
		<figure class="image-block" aria-label={block.content.mediaCaption || 'Image'}>
			{#if block.content.mediaUrl}
				<img
					src={block.content.mediaUrl}
					alt={block.content.mediaAlt || 'Blog image'}
					style:object-fit={block.settings.objectFit || 'cover'}
				/>
				{#if block.content.mediaCaption || isEditing}
					<figcaption
						contenteditable={isEditing}
						class="image-caption editable-content"
						class:placeholder={!block.content.mediaCaption}
						oninput={(e: Event) =>
							updateContent({ mediaCaption: (e.target as HTMLElement).textContent || '' })}
						data-placeholder="Add a caption..."
					>
						{block.content.mediaCaption || ''}
					</figcaption>
				{/if}
			{:else if isEditing}
				<div
					class="image-placeholder"
					role="button"
					tabindex="0"
					aria-label="Click to upload an image"
				>
					<IconPhoto size={48} aria-hidden="true" />
					<span>Click to add an image</span>
					<input
						type="file"
						accept="image/*"
						aria-label="Upload image file"
						onchange={(e: Event) => {
							const file = (e.target as HTMLInputElement).files?.[0];
							if (file) {
								const url = URL.createObjectURL(file);
								updateContent({ mediaUrl: url });
							}
						}}
					/>
				</div>
			{/if}
		</figure>

		<!-- Video Block -->
	{:else if block.type === 'video'}
		<div class="video-block" role="region" aria-label="Video content">
			{#if block.content.embedUrl}
				{@const isYouTube =
					block.content.embedUrl.includes('youtube') || block.content.embedUrl.includes('youtu.be')}
				{@const isVimeo = block.content.embedUrl.includes('vimeo')}
				{#if isYouTube}
					{@const videoId = block.content.embedUrl.match(
						/(?:youtube\.com\/watch\?v=|youtu\.be\/)([^&]+)/
					)?.[1]}
					<div class="video-embed">
						<iframe
							src="https://www.youtube.com/embed/{videoId}"
							title="Embedded YouTube video player"
							frameborder="0"
							allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
							allowfullscreen
						></iframe>
					</div>
				{:else if isVimeo}
					{@const videoId = block.content.embedUrl.match(/vimeo\.com\/(\d+)/)?.[1]}
					<div class="video-embed">
						<iframe
							src="https://player.vimeo.com/video/{videoId}"
							title="Embedded Vimeo video player"
							frameborder="0"
							allow="autoplay; fullscreen; picture-in-picture"
							allowfullscreen
						></iframe>
					</div>
				{:else}
					<video src={block.content.embedUrl} controls aria-label="Video player">
						<track kind="captions" src="" label="Captions" default />
						Your browser does not support the video tag.
					</video>
				{/if}
			{:else if isEditing}
				<div class="video-placeholder" role="group" aria-label="Add video">
					<IconVideo size={48} aria-hidden="true" />
					<span>Paste a YouTube or Vimeo URL</span>
					<input
						type="url"
						placeholder="https://youtube.com/watch?v=..."
						aria-label="Video URL"
						onchange={(e: Event) =>
							updateContent({ embedUrl: (e.target as HTMLInputElement).value })}
					/>
				</div>
			{/if}
		</div>

		<!-- Separator Block -->
	{:else if block.type === 'separator'}
		<hr class="separator-block" />

		<!-- Spacer Block -->
	{:else if block.type === 'spacer'}
		<div class="spacer-block" style:height={block.settings.height || '40px'}>
			{#if isEditing}
				<div class="spacer-handle">
					<IconGripVertical size={14} />
					<span>{block.settings.height || '40px'}</span>
				</div>
			{/if}
		</div>

		<!-- Button Block -->
	{:else if block.type === 'button'}
		<div class="button-block-wrapper">
			<a
				href={block.settings.linkUrl || '#'}
				class="button-block button-{block.settings.buttonStyle || 'primary'} button-size-{block
					.settings.buttonSize || 'medium'}"
				target={block.settings.linkTarget || '_self'}
				onclick={(e: MouseEvent) => isEditing && e.preventDefault()}
			>
				<span
					contenteditable={isEditing}
					class="button-text"
					oninput={handleTextInput}
					onpaste={handlePaste}
				>
					{block.content.text || 'Click Here'}
				</span>
			</a>
		</div>

		<!-- Callout Block -->
	{:else if block.type === 'callout'}
		<div class="callout-block" role="note" aria-label="Important note">
			<div class="callout-icon" aria-hidden="true">
				<IconAlertTriangle size={24} />
			</div>
			<div
				contenteditable={isEditing}
				class="callout-content editable-content"
				class:placeholder={!block.content.text}
				oninput={handleTextInput}
				onpaste={handlePaste}
				data-placeholder="Add a callout message..."
			>
				{block.content.text || ''}
			</div>
		</div>

		<!-- Trading Chart Block -->
	{:else if block.type === 'chart'}
		<div
			class="chart-block"
			role="figure"
			aria-label="Trading chart for {block.content.ticker || 'SPY'}"
		>
			<div class="chart-header">
				<IconChartCandle size={20} aria-hidden="true" />
				<input
					type="text"
					value={block.content.ticker || 'SPY'}
					placeholder="Ticker symbol"
					onchange={(e: Event) => updateContent({ ticker: (e.target as HTMLInputElement).value })}
					disabled={!isEditing}
					class="ticker-input"
					aria-label="Stock ticker symbol"
				/>
			</div>
			<div class="chart-placeholder" style:height={block.settings.height || '400px'}>
				<span>Trading chart for {block.content.ticker || 'SPY'}</span>
				<small>TradingView widget will render here</small>
			</div>
		</div>

		<!-- Reusable/Global Component Block -->
	{:else if block.type === 'reusable'}
		<div class="reusable-block" role="region" aria-label="Reusable component">
			{#if isEditing}
				<div class="reusable-header">
					<div class="reusable-info">
						<IconLink size={16} />
						<span class="reusable-name">{block.content.reusableComponentName || 'Global Component'}</span>
						{#if reusableComponentVersion > 0}<span class="reusable-version">v{reusableComponentVersion}</span>{/if}
					</div>
					<div class="reusable-status">
						{#if reusableIsSynced}
							{#if reusableNeedsUpdate}
								<span class="status-badge outdated"><IconAlertCircle size={12} /> Update Available</span>
							{:else}
								<span class="status-badge synced"><IconCheck size={12} /> Synced</span>
							{/if}
						{:else}
							<span class="status-badge detached">Detached</span>
						{/if}
					</div>
					<div class="reusable-actions">
						{#if reusableNeedsUpdate && reusableIsSynced}
							<button type="button" class="reusable-action-btn sync" onclick={handleSyncComponent} disabled={reusableComponentLoading}><IconRefresh size={14} /> Sync</button>
						{/if}
						{#if reusableIsSynced}
							<button type="button" class="reusable-action-btn detach" onclick={handleDetachComponent} disabled={reusableComponentLoading || !reusableComponentData}>Detach</button>
						{/if}
						<button type="button" class="reusable-action-btn reload" onclick={loadReusableComponent} disabled={reusableComponentLoading}><IconRefresh size={14} /></button>
					</div>
				</div>
			{/if}
			<div class="reusable-content">
				{#if reusableComponentLoading}
					<div class="reusable-loading"><IconLoader2 size={24} class="spin" /> <span>Loading...</span></div>
				{:else if reusableComponentError}
					<div class="reusable-error"><IconAlertCircle size={24} /> <span>{reusableComponentError}</span></div>
				{:else if reusableComponentData && reusableComponentData.length > 0}
					<div class="reusable-blocks">
						{#each reusableComponentData as childBlock (childBlock.id)}
							<svelte:self block={childBlock} isSelected={false} isEditing={false} onUpdate={() => {}} />
						{/each}
					</div>
				{:else}
					<div class="reusable-empty"><IconLink size={32} /> <span>Empty component</span></div>
				{/if}
			</div>
		</div>

		<!-- Risk Disclaimer Block -->
	{:else if block.type === 'riskDisclaimer'}
		<div class="disclaimer-block" role="alert" aria-label="Risk disclaimer">
			<IconAlertTriangle size={24} aria-hidden="true" />
			<p>
				{block.content.text ||
					'Trading involves substantial risk of loss and is not suitable for all investors.'}
			</p>
		</div>


	<!-- TRADING-SPECIFIC BLOCKS -->
	{:else if block.type === 'ticker'}
		{@const defaultTicker = { symbol: block.content.ticker || 'SPY', price: 0, change: 0, changePercent: 0 }}
		{@const tickerData = block.content.tickerData || { tickers: [defaultTicker], multiRow: false }}
		{@const tickers = tickerData.tickers}
		{@const isMultiRow = tickerData.multiRow || false}
		<div class="ticker-block" class:ticker-multi-row={isMultiRow} role="region" aria-label="Stock ticker display">
			<div class="ticker-header-bar">
				<IconChartLine size={18} aria-hidden="true" />
				<span class="ticker-title">Market Prices</span>
				{#if isEditing}
					<button type="button" class="ticker-add-btn" onclick={() => { const currentTickers = tickerData.tickers || [{ symbol: 'SPY', price: 0, change: 0, changePercent: 0 }]; updateContent({ tickerData: { ...tickerData, tickers: [...currentTickers, { symbol: 'AAPL', price: 0, change: 0, changePercent: 0 }] } }); }} aria-label="Add ticker"><IconPlus size={14} /> Add</button>
				{/if}
			</div>
			<div class="ticker-list" class:multi-row={isMultiRow}>
				{#each tickers as ticker, index}
					{@const isPositive = (ticker.changePercent || 0) >= 0}
					<div class="ticker-item" class:positive={isPositive} class:negative={!isPositive}>
						<div class="ticker-symbol-row">
							{#if isEditing}
								<input type="text" class="ticker-symbol-input" value={ticker.symbol || 'SPY'} placeholder="SYMBOL" onchange={(e) => { const newTickers = [...tickers]; newTickers[index] = { ...ticker, symbol: (e.target as HTMLInputElement).value.toUpperCase() }; updateContent({ tickerData: { ...tickerData, tickers: newTickers } }); }} aria-label="Ticker symbol" />
								{#if tickers.length > 1}<button type="button" class="ticker-remove-btn" onclick={() => { const newTickers = tickers.filter((_t: typeof ticker, i: number) => i !== index); updateContent({ tickerData: { ...tickerData, tickers: newTickers } }); }} aria-label="Remove ticker"><IconX size={12} /></button>{/if}
							{:else}<span class="ticker-symbol">{ticker.symbol || 'SPY'}</span>{/if}
							<span class="ticker-direction-icon">{#if isPositive}<IconTrendingUp size={16} />{:else}<IconTrendingDown size={16} />{/if}</span>
						</div>
						<div class="ticker-price-row">
							{#if isEditing}<input type="number" class="ticker-price-input" value={ticker.price || 0} step="0.01" placeholder="Price" onchange={(e) => { const newTickers = [...tickers]; newTickers[index] = { ...ticker, price: parseFloat((e.target as HTMLInputElement).value) || 0 }; updateContent({ tickerData: { ...tickerData, tickers: newTickers } }); }} aria-label="Price" />
							{:else}<span class="ticker-price">${(ticker.price || 0).toFixed(2)}</span>{/if}
						</div>
						<div class="ticker-change-row">
							{#if isEditing}<input type="number" class="ticker-change-input" value={ticker.changePercent || 0} step="0.01" placeholder="%" onchange={(e) => { const newTickers = [...tickers]; const pct = parseFloat((e.target as HTMLInputElement).value) || 0; newTickers[index] = { ...ticker, changePercent: pct, change: (ticker.price || 0) * pct / 100 }; updateContent({ tickerData: { ...tickerData, tickers: newTickers } }); }} aria-label="Change percent" />
							{:else}<span class="ticker-change">{isPositive ? '+' : ''}{(ticker.changePercent || 0).toFixed(2)}%</span>{/if}
						</div>
						<div class="ticker-sparkline" aria-hidden="true"><svg viewBox="0 0 60 24" preserveAspectRatio="none">{#if isPositive}<polyline points="0,20 10,18 20,15 30,12 40,8 50,10 60,4" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />{:else}<polyline points="0,4 10,6 20,10 30,8 40,14 50,18 60,20" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />{/if}</svg></div>
					</div>
				{/each}
			</div>
			{#if isEditing}<label class="ticker-multi-toggle"><input type="checkbox" checked={isMultiRow} onchange={(e) => updateContent({ tickerData: { ...tickerData, multiRow: (e.target as HTMLInputElement).checked } })} /><span>Multi-row layout</span></label>{/if}
		</div>

	{:else if block.type === 'priceAlert'}
		{@const alertData = block.content.alertData || {}}
		{@const direction = alertData.direction || 'above'}
		{@const isAbove = direction === 'above'}
		<div class="price-alert-block" class:alert-bullish={isAbove} class:alert-bearish={!isAbove} role="alert" aria-label="Price alert notification">
			<div class="alert-header-section">
				<div class="alert-icon-wrapper"><IconBellRinging size={24} /></div>
				<div class="alert-title-row"><span class="alert-badge">PRICE ALERT</span><span class="alert-direction-badge" class:bullish={isAbove} class:bearish={!isAbove}>{#if isAbove}<IconArrowUp size={14} /> ABOVE{:else}<IconArrowDown size={14} /> BELOW{/if}</span></div>
			</div>
			<div class="alert-body">
				<div class="alert-symbol-section">{#if isEditing}<input type="text" class="alert-symbol-input" value={alertData.symbol || 'SPY'} placeholder="SYMBOL" onchange={(e) => updateContent({ alertData: { ...alertData, symbol: (e.target as HTMLInputElement).value.toUpperCase() } })} aria-label="Symbol" />{:else}<span class="alert-symbol">{alertData.symbol || 'SPY'}</span>{/if}</div>
				<div class="alert-target-section"><IconTarget size={18} aria-hidden="true" /><span class="alert-label">Target Price:</span>{#if isEditing}<input type="number" class="alert-price-input" value={alertData.targetPrice || 0} step="0.01" placeholder="0.00" onchange={(e) => updateContent({ alertData: { ...alertData, targetPrice: parseFloat((e.target as HTMLInputElement).value) || 0 } })} aria-label="Target price" />{:else}<span class="alert-target-price">${(alertData.targetPrice || 0).toFixed(2)}</span>{/if}</div>
				{#if isEditing}<div class="alert-direction-toggle"><label><input type="radio" name="alert-direction-{block.id}" value="above" checked={isAbove} onchange={() => updateContent({ alertData: { ...alertData, direction: 'above' } })} /><span class="direction-option bullish"><IconArrowUp size={14} /> Above</span></label><label><input type="radio" name="alert-direction-{block.id}" value="below" checked={!isAbove} onchange={() => updateContent({ alertData: { ...alertData, direction: 'below' } })} /><span class="direction-option bearish"><IconArrowDown size={14} /> Below</span></label></div>{/if}
			</div>
			<div class="alert-levels">
				<div class="alert-level entry"><span class="alert-level-label">Entry</span>{#if isEditing}<input type="number" class="alert-level-input" value={alertData.entryPrice || 0} step="0.01" onchange={(e) => updateContent({ alertData: { ...alertData, entryPrice: parseFloat((e.target as HTMLInputElement).value) || 0 } })} aria-label="Entry price" />{:else}<span class="alert-level-value">${(alertData.entryPrice || 0).toFixed(2)}</span>{/if}</div>
				<div class="alert-level exit"><span class="alert-level-label">Exit</span>{#if isEditing}<input type="number" class="alert-level-input" value={alertData.exitPrice || 0} step="0.01" onchange={(e) => updateContent({ alertData: { ...alertData, exitPrice: parseFloat((e.target as HTMLInputElement).value) || 0 } })} aria-label="Exit price" />{:else}<span class="alert-level-value">${(alertData.exitPrice || 0).toFixed(2)}</span>{/if}</div>
			</div>
			{#if true}
				{@const entry = alertData.entryPrice || 0}
				{@const exit = alertData.exitPrice || 0}
				{@const target = alertData.targetPrice || 0}
				{@const risk = Math.abs(entry - exit)}
				{@const reward = Math.abs(target - entry)}
				{@const rrRatio = risk > 0 ? (reward / risk).toFixed(2) : '0.00'}
				<div class="alert-rr-display"><div class="alert-rr-item risk"><span class="alert-rr-label">Risk</span><span class="alert-rr-value">${risk.toFixed(2)}</span></div><div class="alert-rr-divider">:</div><div class="alert-rr-item reward"><span class="alert-rr-label">Reward</span><span class="alert-rr-value">${reward.toFixed(2)}</span></div><div class="alert-rr-ratio"><span class="alert-rr-ratio-label">R:R</span><span class="alert-rr-ratio-value">{rrRatio}</span></div></div>
			{/if}
		</div>

	{:else if block.type === 'tradingIdea'}
		{#if true}
			{@const ideaData = block.content.ideaData || {}}
			{@const direction = ideaData.direction || 'long'}
			{@const isLong = direction === 'long'}
			{@const entry = ideaData.entryPrice || 0}
			{@const stopLoss = ideaData.stopLoss || 0}
			{@const takeProfit = ideaData.takeProfit || 0}
			{@const risk = Math.abs(entry - stopLoss)}
			{@const reward = Math.abs(takeProfit - entry)}
			{@const rrRatio = risk > 0 ? (reward / risk).toFixed(2) : '0.00'}
			{@const confidence = ideaData.confidence || 50}
			<div class="trading-idea-block" class:idea-long={isLong} class:idea-short={!isLong} role="article" aria-label="Trading idea">
			<div class="idea-header">
				<div class="idea-direction" class:long={isLong} class:short={!isLong}>{#if isLong}<IconTrendingUp size={20} /><span>LONG</span>{:else}<IconTrendingDown size={20} /><span>SHORT</span>{/if}</div>
				<div class="idea-symbol-container">{#if isEditing}<input type="text" class="idea-symbol-input" value={ideaData.symbol || 'SPY'} placeholder="SYMBOL" onchange={(e) => updateContent({ ideaData: { ...ideaData, symbol: (e.target as HTMLInputElement).value.toUpperCase() } })} aria-label="Symbol" />{:else}<span class="idea-symbol">{ideaData.symbol || 'SPY'}</span>{/if}</div>
				{#if isEditing}<div class="idea-direction-toggle"><button type="button" class="idea-direction-btn long" class:active={isLong} onclick={() => updateContent({ ideaData: { ...ideaData, direction: 'long' } })}><IconTrendingUp size={14} /> Long</button><button type="button" class="idea-direction-btn short" class:active={!isLong} onclick={() => updateContent({ ideaData: { ...ideaData, direction: 'short' } })}><IconTrendingDown size={14} /> Short</button></div>{/if}
			</div>
			<div class="idea-chart-placeholder"><IconChartCandle size={32} aria-hidden="true" /><span>Chart Analysis</span><small>TradingView chart will render here</small></div>
			<div class="idea-levels">
				<div class="idea-level entry-level"><div class="idea-level-icon"><IconCurrencyDollar size={16} /></div><div class="idea-level-info"><span class="idea-level-label">Entry Price</span>{#if isEditing}<input type="number" class="idea-level-input" value={entry} step="0.01" onchange={(e) => updateContent({ ideaData: { ...ideaData, entryPrice: parseFloat((e.target as HTMLInputElement).value) || 0 } })} aria-label="Entry price" />{:else}<span class="idea-level-value">${entry.toFixed(2)}</span>{/if}</div></div>
				<div class="idea-level stop-loss-level"><div class="idea-level-icon"><IconShieldCheck size={16} /></div><div class="idea-level-info"><span class="idea-level-label">Stop Loss</span>{#if isEditing}<input type="number" class="idea-level-input" value={stopLoss} step="0.01" onchange={(e) => updateContent({ ideaData: { ...ideaData, stopLoss: parseFloat((e.target as HTMLInputElement).value) || 0 } })} aria-label="Stop loss" />{:else}<span class="idea-level-value">${stopLoss.toFixed(2)}</span>{/if}</div></div>
				<div class="idea-level take-profit-level"><div class="idea-level-icon"><IconTarget size={16} /></div><div class="idea-level-info"><span class="idea-level-label">Take Profit</span>{#if isEditing}<input type="number" class="idea-level-input" value={takeProfit} step="0.01" onchange={(e) => updateContent({ ideaData: { ...ideaData, takeProfit: parseFloat((e.target as HTMLInputElement).value) || 0 } })} aria-label="Take profit" />{:else}<span class="idea-level-value">${takeProfit.toFixed(2)}</span>{/if}</div></div>
			</div>
			<div class="idea-rr-section">
				<div class="idea-rr-visual"><div class="idea-rr-bar-container"><div class="idea-rr-bar risk-bar" style="width: 50%"></div><div class="idea-rr-bar reward-bar" style="width: {Math.min(parseFloat(rrRatio) * 25, 100)}%"></div></div><div class="idea-rr-labels"><span class="idea-risk-label">Risk: ${risk.toFixed(2)}</span><span class="idea-reward-label">Reward: ${reward.toFixed(2)}</span></div></div>
				<div class="idea-rr-ratio-badge" class:good={parseFloat(rrRatio) >= 2} class:medium={parseFloat(rrRatio) >= 1 && parseFloat(rrRatio) < 2} class:poor={parseFloat(rrRatio) < 1}><span class="idea-ratio-value">{rrRatio}</span><span class="idea-ratio-label">R:R</span></div>
			</div>
			<div class="idea-confidence"><div class="idea-confidence-header"><IconFlame size={16} aria-hidden="true" /><span class="idea-confidence-label">Confidence Level</span><span class="idea-confidence-value">{confidence}%</span></div>{#if isEditing}<input type="range" class="idea-confidence-slider" min="0" max="100" value={confidence} oninput={(e) => updateContent({ ideaData: { ...ideaData, confidence: parseInt((e.target as HTMLInputElement).value) || 50 } })} aria-label="Confidence level" />{:else}<div class="idea-confidence-bar-container"><div class="idea-confidence-bar" style="width: {confidence}%" class:low={confidence < 40} class:medium={confidence >= 40 && confidence < 70} class:high={confidence >= 70}></div></div>{/if}</div>
			<div class="idea-footer"><div class="idea-timestamp"><IconClock size={14} aria-hidden="true" /><span>{ideaData.timestamp || new Date().toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric', hour: '2-digit', minute: '2-digit' })}</span></div><div class="idea-actions"><IconBulb size={16} aria-hidden="true" /><span>Trading Idea</span></div></div>
		</div>
		{/if}
		<!-- Custom HTML Block -->
	{:else if block.type === 'html'}
		{#if isEditing}
			<div class="html-block-editor">
				<div class="html-header">Custom HTML</div>
				<textarea
					value={block.content.html || ''}
					oninput={(e: Event) => updateContent({ html: (e.target as HTMLTextAreaElement).value })}
					placeholder="<div>Your HTML here...</div>"
					rows="8"
				></textarea>
			</div>
		{:else}
			<div class="html-block-preview">
				{@html sanitizedHTML}
			</div>
		{/if}

		
	<!-- ================================================================== -->
	<!-- ADVANCED BLOCKS                                                     -->
	<!-- ================================================================== -->

	<!-- Card Block -->
	{:else if block.type === 'card'}
		<article class="card-block" role="article">
			{#if isEditing && isSelected}
				<div class="card-edit-fields">
					<label class="edit-field"><span>Image URL</span><input type="url" value={block.content.mediaUrl || ''} placeholder="https://example.com/image.jpg" oninput={(e: Event) => updateContent({ mediaUrl: (e.target as HTMLInputElement).value })} /></label>
					<label class="edit-field"><span>CTA URL</span><input type="url" value={block.settings.linkUrl || ''} placeholder="https://example.com" oninput={(e: Event) => onUpdate({ settings: { ...block.settings, linkUrl: (e.target as HTMLInputElement).value } })} /></label>
				</div>
			{/if}
			{#if block.content.mediaUrl}<div class="card-image"><img src={block.content.mediaUrl} alt={block.content.mediaAlt || 'Card image'} /></div>{:else if isEditing}<div class="card-image-placeholder"><IconPhoto size={32} /><span>Add image URL above</span></div>{/if}
			<div class="card-body">
				<h3 contenteditable={isEditing} class="card-title editable-content" class:placeholder={!block.content.text} oninput={handleTextInput} onpaste={handlePaste} data-placeholder="Card Title">{block.content.text || ''}</h3>
				<p contenteditable={isEditing} class="card-description editable-content" class:placeholder={!block.content.html} oninput={(e: Event) => updateContent({ html: (e.target as HTMLElement).textContent || '' })} onpaste={handlePaste} data-placeholder="Card description...">{block.content.html || ''}</p>
				{#if block.settings.linkUrl || isEditing}<a href={block.settings.linkUrl || '#'} class="card-cta" onclick={(e: MouseEvent) => isEditing && e.preventDefault()}><span contenteditable={isEditing} oninput={(e: Event) => updateContent({ code: (e.target as HTMLElement).textContent || '' })}>{block.content.code || 'Learn More'}</span><IconArrowRight size={16} /></a>{/if}
			</div>
		</article>

	<!-- Testimonial Block -->
	{:else if block.type === 'testimonial'}
		<figure class="testimonial-block" role="figure">
			{#if isEditing && isSelected}<div class="testimonial-edit-fields"><label class="edit-field"><span>Author Photo URL</span><input type="url" value={block.content.mediaUrl || ''} placeholder="https://example.com/photo.jpg" oninput={(e: Event) => updateContent({ mediaUrl: (e.target as HTMLInputElement).value })} /></label></div>{/if}
			<div class="testimonial-quote-icon"><IconQuote size={32} /></div>
			<blockquote contenteditable={isEditing} class="testimonial-quote editable-content" class:placeholder={!block.content.text} oninput={handleTextInput} onpaste={handlePaste} data-placeholder="Write the testimonial quote...">{block.content.text || ''}</blockquote>
			<figcaption class="testimonial-author">
				<div class="testimonial-avatar">{#if block.content.mediaUrl}<img src={block.content.mediaUrl} alt="Author photo" />{:else}<IconUser size={24} />{/if}</div>
				<div class="testimonial-info">
					<span contenteditable={isEditing} class="testimonial-name editable-content" class:placeholder={!block.content.html} oninput={(e: Event) => updateContent({ html: (e.target as HTMLElement).textContent || '' })} data-placeholder="Author Name">{block.content.html || ''}</span>
					<span contenteditable={isEditing} class="testimonial-title editable-content" class:placeholder={!block.content.code} oninput={(e: Event) => updateContent({ code: (e.target as HTMLElement).textContent || '' })} data-placeholder="Title, Company">{block.content.code || ''}</span>
				</div>
			</figcaption>
		</figure>

	<!-- CTA Block -->
	{:else if block.type === 'cta'}
		<section class="cta-block" class:cta-dark={block.settings.backgroundColor === 'dark'} class:cta-gradient={block.settings.backgroundColor === 'gradient'} role="region" aria-label="Call to action">
			{#if isEditing && isSelected}<div class="cta-edit-fields"><label class="edit-field"><span>Background Style</span><select value={block.settings.backgroundColor || 'light'} onchange={(e: Event) => onUpdate({ settings: { ...block.settings, backgroundColor: (e.target as HTMLSelectElement).value } })}><option value="light">Light</option><option value="dark">Dark</option><option value="gradient">Gradient</option></select></label><label class="edit-field"><span>Primary Button URL</span><input type="url" value={block.settings.linkUrl || ''} placeholder="https://example.com" oninput={(e: Event) => onUpdate({ settings: { ...block.settings, linkUrl: (e.target as HTMLInputElement).value } })} /></label><label class="edit-field"><span>Secondary Button URL</span><input type="url" value={block.settings.secondaryLinkUrl || ''} placeholder="https://example.com" oninput={(e: Event) => onUpdate({ settings: { ...block.settings, secondaryLinkUrl: (e.target as HTMLInputElement).value } })} /></label></div>{/if}
			<h2 contenteditable={isEditing} class="cta-heading editable-content" class:placeholder={!block.content.text} oninput={handleTextInput} onpaste={handlePaste} data-placeholder="Compelling Headline">{block.content.text || ''}</h2>
			<p contenteditable={isEditing} class="cta-description editable-content" class:placeholder={!block.content.html} oninput={(e: Event) => updateContent({ html: (e.target as HTMLElement).textContent || '' })} onpaste={handlePaste} data-placeholder="Supporting text that encourages action...">{block.content.html || ''}</p>
			<div class="cta-buttons">
				<a href={block.settings.linkUrl || '#'} class="cta-btn cta-btn-primary" onclick={(e: MouseEvent) => isEditing && e.preventDefault()}><span contenteditable={isEditing} oninput={(e: Event) => updateContent({ code: (e.target as HTMLElement).textContent || '' })}>{block.content.code || 'Get Started'}</span></a>
				<a href={block.settings.secondaryLinkUrl || '#'} class="cta-btn cta-btn-secondary" onclick={(e: MouseEvent) => isEditing && e.preventDefault()}><span contenteditable={isEditing} oninput={(e: Event) => updateContent({ language: (e.target as HTMLElement).textContent || '' })}>{block.content.language || 'Learn More'}</span></a>
			</div>
		</section>

	<!-- Countdown Block -->
	{:else if block.type === 'countdown'}
		<div class="countdown-block" role="timer" aria-label="Countdown timer">
			{#if isEditing && isSelected}<div class="countdown-edit-fields"><label class="edit-field"><span>Target Date</span><input type="datetime-local" value={block.content.text || ''} oninput={(e: Event) => { const value = (e.target as HTMLInputElement).value; updateContent({ text: value }); if (value) startCountdown(value); }} /></label><label class="edit-field"><span>Label</span><input type="text" value={block.content.html || ''} placeholder="Sale ends in..." oninput={(e: Event) => updateContent({ html: (e.target as HTMLInputElement).value })} /></label></div>{/if}
			{#if block.content.html}<h3 class="countdown-label">{block.content.html}</h3>{/if}
			<div class="countdown-timer">
				<div class="countdown-unit"><span class="countdown-value">{String(countdownValues.days).padStart(2, '0')}</span><span class="countdown-label-unit">Days</span></div>
				<span class="countdown-separator">:</span>
				<div class="countdown-unit"><span class="countdown-value">{String(countdownValues.hours).padStart(2, '0')}</span><span class="countdown-label-unit">Hours</span></div>
				<span class="countdown-separator">:</span>
				<div class="countdown-unit"><span class="countdown-value">{String(countdownValues.minutes).padStart(2, '0')}</span><span class="countdown-label-unit">Minutes</span></div>
				<span class="countdown-separator">:</span>
				<div class="countdown-unit"><span class="countdown-value">{String(countdownValues.seconds).padStart(2, '0')}</span><span class="countdown-label-unit">Seconds</span></div>
			</div>
		</div>

	<!-- Social Share Block -->
	{:else if block.type === 'socialShare'}
		<div class="social-share-block" role="group" aria-label="Share this article">
			{#if block.content.text}<span class="social-share-label">{block.content.text}</span>{:else if isEditing}<span contenteditable={isEditing} class="social-share-label editable-content" oninput={handleTextInput} data-placeholder="Share this article">Share this article</span>{/if}
			<div class="social-share-buttons">
				<button type="button" class="social-btn social-btn-twitter" onclick={shareOnTwitter} aria-label="Share on Twitter" title="Share on Twitter"><IconBrandX size={20} /></button>
				<button type="button" class="social-btn social-btn-facebook" onclick={shareOnFacebook} aria-label="Share on Facebook" title="Share on Facebook"><IconBrandFacebook size={20} /></button>
				<button type="button" class="social-btn social-btn-linkedin" onclick={shareOnLinkedIn} aria-label="Share on LinkedIn" title="Share on LinkedIn"><IconBrandLinkedin size={20} /></button>
				<button type="button" class="social-btn social-btn-email" onclick={shareViaEmail} aria-label="Share via Email" title="Share via Email"><IconMail size={20} /></button>
				<button type="button" class="social-btn social-btn-copy" class:copied={linkCopied} onclick={copyShareLink} aria-label="Copy link" title="Copy link">{#if linkCopied}<IconCheck size={20} />{:else}<IconLink size={20} />{/if}</button>
			</div>
		</div>

	<!-- Author Block -->
	{:else if block.type === 'author'}
		<aside class="author-block" role="complementary" aria-label="About the author">
			{#if isEditing && isSelected}<div class="author-edit-fields"><label class="edit-field"><span>Author Photo URL</span><input type="url" value={block.content.mediaUrl || ''} placeholder="https://example.com/photo.jpg" oninput={(e: Event) => updateContent({ mediaUrl: (e.target as HTMLInputElement).value })} /></label><label class="edit-field"><span>Twitter URL</span><input type="url" value={block.settings.linkUrl || ''} placeholder="https://twitter.com/username" oninput={(e: Event) => onUpdate({ settings: { ...block.settings, linkUrl: (e.target as HTMLInputElement).value } })} /></label><label class="edit-field"><span>LinkedIn URL</span><input type="url" value={block.settings.secondaryLinkUrl || ''} placeholder="https://linkedin.com/in/username" oninput={(e: Event) => onUpdate({ settings: { ...block.settings, secondaryLinkUrl: (e.target as HTMLInputElement).value } })} /></label></div>{/if}
			<div class="author-avatar">{#if block.content.mediaUrl}<img src={block.content.mediaUrl} alt="Author photo" />{:else}<IconUser size={48} />{/if}</div>
			<div class="author-info">
				<span class="author-label">Written by</span>
				<h4 contenteditable={isEditing} class="author-name editable-content" class:placeholder={!block.content.text} oninput={handleTextInput} onpaste={handlePaste} data-placeholder="Author Name">{block.content.text || ''}</h4>
				<p contenteditable={isEditing} class="author-bio editable-content" class:placeholder={!block.content.html} oninput={(e: Event) => updateContent({ html: (e.target as HTMLElement).textContent || '' })} onpaste={handlePaste} data-placeholder="Short bio about the author...">{block.content.html || ''}</p>
				<div class="author-social">{#if block.settings.linkUrl}<a href={block.settings.linkUrl} target="_blank" rel="noopener noreferrer" class="author-social-link" aria-label="Twitter"><IconBrandX size={18} /></a>{/if}{#if block.settings.secondaryLinkUrl}<a href={block.settings.secondaryLinkUrl} target="_blank" rel="noopener noreferrer" class="author-social-link" aria-label="LinkedIn"><IconBrandLinkedin size={18} /></a>{/if}</div>
			</div>
		</aside>

	<!-- Related Posts Block -->
	{:else if block.type === 'relatedPosts'}
		<section class="related-posts-block" role="complementary" aria-label="Related posts">
			<h3 contenteditable={isEditing} class="related-posts-heading editable-content" class:placeholder={!block.content.text} oninput={handleTextInput} onpaste={handlePaste} data-placeholder="Related Posts">{block.content.text || 'Related Posts'}</h3>
			<div class="related-posts-grid">
				{#each [1, 2, 3] as index}<article class="related-post-card"><div class="related-post-image"><IconArticle size={32} /></div><div class="related-post-content"><span class="related-post-category">Category</span><h4 class="related-post-title">Related Article Title {index}</h4><p class="related-post-excerpt">Brief excerpt or description of the related post content...</p></div></article>{/each}
			</div>
			{#if isEditing}<p class="related-posts-note"><IconAlertCircle size={16} /><span>Related posts will be populated automatically based on tags and categories.</span></p>{/if}
		</section>

	<!-- Newsletter Block -->
	{:else if block.type === 'newsletter'}
		<section class="newsletter-block" role="form" aria-label="Newsletter signup">
			{#if isEditing && isSelected}<div class="newsletter-edit-fields"><label class="edit-field"><span>Form Action URL</span><input type="url" value={block.settings.linkUrl || ''} placeholder="https://api.example.com/subscribe" oninput={(e: Event) => onUpdate({ settings: { ...block.settings, linkUrl: (e.target as HTMLInputElement).value } })} /></label></div>{/if}
			<div class="newsletter-content">
				<h3 contenteditable={isEditing} class="newsletter-heading editable-content" class:placeholder={!block.content.text} oninput={handleTextInput} onpaste={handlePaste} data-placeholder="Subscribe to our newsletter">{block.content.text || ''}</h3>
				<p contenteditable={isEditing} class="newsletter-description editable-content" class:placeholder={!block.content.html} oninput={(e: Event) => updateContent({ html: (e.target as HTMLElement).textContent || '' })} onpaste={handlePaste} data-placeholder="Get the latest updates delivered to your inbox.">{block.content.html || ''}</p>
			</div>
			{#if newsletterSuccess}<div class="newsletter-success"><IconCheck size={24} /><span>Thank you for subscribing!</span></div>
			{:else}<form class="newsletter-form" onsubmit={handleNewsletterSubmit}><div class="newsletter-input-group"><input type="email" bind:value={newsletterEmail} placeholder="Enter your email" required disabled={newsletterSubmitting} aria-label="Email address" /><button type="submit" disabled={newsletterSubmitting || !newsletterEmail}>{#if newsletterSubmitting}<IconLoader2 size={20} class="spin" />{:else}Subscribe{/if}</button></div><p contenteditable={isEditing} class="newsletter-privacy editable-content" class:placeholder={!block.content.code} oninput={(e: Event) => updateContent({ code: (e.target as HTMLElement).textContent || '' })} data-placeholder="We respect your privacy. Unsubscribe at any time.">{block.content.code || 'We respect your privacy. Unsubscribe at any time.'}</p></form>{/if}
		</section>


	<!-- Gallery Block -->
	{:else if block.type === 'gallery'}
		{@const images = block.content.galleryImages || []}
		{@const layout = block.settings.galleryLayout || 'grid'}
		{@const columns = block.settings.galleryColumns || 3}
		<div class="gallery-block gallery-{layout}" role="group" aria-label="Image gallery" style="--gallery-columns: {columns}">
			{#if images.length > 0}
				<div class="gallery-grid">{#each images as image, index}<div class="gallery-item"><button type="button" class="gallery-image-btn" onclick={() => !isEditing && openLightbox(index)} aria-label="View image {index + 1}"><img src={image.url} alt={image.alt || 'Gallery image'} loading="lazy" />{#if !isEditing}<div class="gallery-overlay"><IconMaximize size={24} /></div>{/if}</button>{#if isEditing}<div class="gallery-item-controls"><input type="text" placeholder="Image URL" value={image.url} onchange={(e: Event) => updateGalleryImage(index, 'url', (e.target as HTMLInputElement).value)} /><input type="text" placeholder="Alt text" value={image.alt || ''} onchange={(e: Event) => updateGalleryImage(index, 'alt', (e.target as HTMLInputElement).value)} /><button type="button" onclick={() => removeGalleryImage(index)} aria-label="Remove"><IconX size={16} /></button></div>{/if}</div>{/each}</div>
			{:else if isEditing}<div class="gallery-placeholder"><IconLayoutGrid size={48} /><span>Add images to gallery</span></div>{/if}
			{#if isEditing}<button type="button" class="gallery-add-btn" onclick={addGalleryImage}><IconPlus size={16} />Add Image</button>{/if}
			{#if lightboxOpen && images.length > 0}<div class="lightbox-overlay" role="dialog" onclick={closeLightbox}><div class="lightbox-content" onclick={(e: MouseEvent) => e.stopPropagation()}><button type="button" class="lightbox-close" onclick={closeLightbox}><IconX size={24} /></button><button type="button" class="lightbox-nav lightbox-prev" onclick={prevImage}><IconChevronDown size={32} style="transform: rotate(90deg)" /></button><img src={images[lightboxIndex]?.url} alt={images[lightboxIndex]?.alt || 'Image'} class="lightbox-image" /><button type="button" class="lightbox-nav lightbox-next" onclick={nextImage}><IconChevronDown size={32} style="transform: rotate(-90deg)" /></button><div class="lightbox-counter">{lightboxIndex + 1} / {images.length}</div></div></div>{/if}
		</div>

	<!-- Audio Block -->
	{:else if block.type === 'audio'}
		<div class="audio-block" role="region" aria-label="Audio player">
			{#if block.content.mediaUrl}
				<audio bind:this={audioRef} src={block.content.mediaUrl} ontimeupdate={handleAudioTimeUpdate} onended={() => audioPlaying = false} preload="metadata"></audio>
				<div class="audio-player"><button type="button" class="audio-play-btn" onclick={toggleAudioPlay}>{#if audioPlaying}<IconPlayerPause size={24} />{:else}<IconPlayerPlay size={24} />{/if}</button><div class="audio-waveform"><IconWaveSine size={20} /></div><div class="audio-progress-container" onclick={handleAudioSeek}><div class="audio-progress-bar" style="width: {audioProgress}%"></div></div><div class="audio-time">{#if audioRef}{formatAudioTime(audioRef.currentTime)} / {formatAudioTime(audioRef.duration)}{:else}0:00{/if}</div><div class="audio-volume-container"><button type="button" class="audio-mute-btn" onclick={toggleAudioMute}>{#if audioMuted}<IconVolumeOff size={20} />{:else}<IconVolume size={20} />{/if}</button><input type="range" min="0" max="1" step="0.1" value={audioVolume} oninput={handleVolumeChange} class="audio-volume-slider" /></div></div>
				{#if block.content.mediaCaption || isEditing}<p contenteditable={isEditing} class="audio-caption editable-content" class:placeholder={!block.content.mediaCaption} oninput={(e: Event) => updateContent({ mediaCaption: (e.target as HTMLElement).textContent || '' })} data-placeholder="Caption...">{block.content.mediaCaption || ''}</p>{/if}
			{:else if isEditing}<div class="audio-placeholder"><IconWaveSine size={48} /><span>Add audio URL</span><input type="url" placeholder="https://example.com/audio.mp3" onchange={(e: Event) => updateContent({ mediaUrl: (e.target as HTMLInputElement).value })} /><span>or</span><input type="file" accept="audio/*" onchange={(e: Event) => { const file = (e.target as HTMLInputElement).files?.[0]; if (file) updateContent({ mediaUrl: URL.createObjectURL(file) }); }} /></div>{/if}
		</div>

	<!-- File Block -->
	{:else if block.type === 'file'}
		<div class="file-block" role="region" aria-label="File">
			{#if block.content.fileUrl}
				{@const fileName = block.content.fileName || 'Download'}
				{@const fileSize = block.content.fileSize || 0}
				<a href={block.content.fileUrl} download={fileName} class="file-download-card" target="_blank" rel="noopener noreferrer"><div class="file-icon"><IconFile size={32} /></div><div class="file-info"><span class="file-name">{fileName}</span>{#if fileSize > 0}<span class="file-size">{formatFileSize(fileSize)}</span>{/if}</div><div class="file-download-icon"><IconDownload size={24} /></div></a>
				{#if isEditing}<div class="file-edit-controls"><input type="text" placeholder="File name" value={block.content.fileName || ''} onchange={(e: Event) => updateContent({ fileName: (e.target as HTMLInputElement).value })} /></div>{/if}
			{:else if isEditing}<div class="file-placeholder"><IconFile size={48} /><span>Add file URL</span><input type="url" placeholder="https://example.com/file.pdf" onchange={(e: Event) => { const url = (e.target as HTMLInputElement).value; updateContent({ fileUrl: url, fileName: url.split('/').pop() || 'file' }); }} /><span>or</span><input type="file" onchange={(e: Event) => { const file = (e.target as HTMLInputElement).files?.[0]; if (file) updateContent({ fileUrl: URL.createObjectURL(file), fileName: file.name, fileSize: file.size }); }} /></div>{/if}
		</div>

	<!-- Embed Block -->
	{:else if block.type === 'embed'}
		{@const embedUrl = block.content.embedUrl || ''}
		{@const platform = getEmbedPlatform(embedUrl)}
		{@const embedId = getEmbedId(embedUrl, platform)}
		<div class="embed-block embed-{platform}" role="region" aria-label="{platform} embed">
			{#if embedUrl && platform !== 'unknown'}
				<div class="embed-wrapper">{#if platform === 'twitter'}<div class="embed-fallback"><IconBrandX size={32} /><a href={embedUrl} target="_blank" rel="noopener">View on X</a></div>{:else if platform === 'instagram'}<div class="embed-fallback"><IconBrandInstagram size={32} /><a href={embedUrl} target="_blank" rel="noopener">View on Instagram</a></div>{:else if platform === 'tiktok'}<div class="embed-fallback"><IconBrandTiktok size={32} /><a href={embedUrl} target="_blank" rel="noopener">View on TikTok</a></div>{:else if platform === 'spotify'}{@const spotifyType = embedUrl.includes('/track/') ? 'track' : 'playlist'}<iframe src="https://open.spotify.com/embed/{spotifyType}/{embedId}" width="100%" height={spotifyType === 'track' ? '152' : '352'} frameborder="0" allow="autoplay; encrypted-media" title="Spotify"></iframe>{:else if platform === 'soundcloud'}<iframe width="100%" height="166" frameborder="no" src="https://w.soundcloud.com/player/?url={encodeURIComponent(embedUrl)}" title="SoundCloud"></iframe>{/if}</div>
			{:else if isEditing}<div class="embed-placeholder"><IconWorld size={48} /><span>Paste social media URL</span><small>Twitter/X, Instagram, TikTok, Spotify, SoundCloud</small><input type="url" placeholder="https://twitter.com/..." onchange={(e: Event) => updateContent({ embedUrl: (e.target as HTMLInputElement).value })} /><div class="embed-platform-icons"><IconBrandX size={24} /><IconBrandInstagram size={24} /><IconBrandTiktok size={24} /></div></div>
			{:else}<div class="embed-error"><IconAlertTriangle size={24} /><span>Unsupported URL</span></div>{/if}
		</div>

	<!-- GIF Block -->
	{:else if block.type === 'gif'}
		<div class="gif-block" role="region" aria-label="GIF">
			{#if block.content.mediaUrl}
				<div class="gif-container"><img src={block.content.mediaUrl} alt={block.content.mediaAlt || 'GIF'} class="gif-image" class:paused={!gifPlaying} /><button type="button" class="gif-play-toggle" onclick={toggleGifPlay}>{#if gifPlaying}<IconPlayerPause size={24} />{:else}<IconPlayerPlay size={24} />{/if}</button><span class="gif-badge">GIF</span></div>
				{#if block.content.mediaCaption || isEditing}<p contenteditable={isEditing} class="gif-caption editable-content" class:placeholder={!block.content.mediaCaption} oninput={(e: Event) => updateContent({ mediaCaption: (e.target as HTMLElement).textContent || '' })} data-placeholder="Caption...">{block.content.mediaCaption || ''}</p>{/if}
			{:else if isEditing}<div class="gif-placeholder"><IconGif size={48} /><span>Add GIF URL or search Giphy</span><div class="gif-search-input"><IconSearch size={20} /><input type="text" placeholder="Search Giphy..." value={block.content.giphySearch || ''} oninput={(e: Event) => updateContent({ giphySearch: (e.target as HTMLInputElement).value })} /></div><span>or</span><input type="url" placeholder="https://media.giphy.com/..." onchange={(e: Event) => updateContent({ mediaUrl: (e.target as HTMLInputElement).value })} /></div>{/if}
		</div>

	<!-- AI Generated Content Block -->
	{:else if block.type === 'aiGenerated'}
		<div class="ai-generated-block" role="region" aria-label="AI Generated Content" aria-busy={aiLoading}>
			<div class="ai-block-header">
				<div class="ai-badge">
					<IconRobot size={16} aria-hidden="true" />
					<span>AI Generated</span>
				</div>
				{#if isEditing}
					<div class="ai-model-selector">
						<label for="ai-model-select-{block.id}" class="visually-hidden">Select AI Model</label>
						<select
							id="ai-model-select-{block.id}"
							value={block.content.aiModel || 'gpt-4'}
							onchange={(e: Event) => updateContent({ aiModel: (e.target as HTMLSelectElement).value })}
							disabled={aiLoading}
						>
							<option value="gpt-4">GPT-4</option>
							<option value="gpt-4o">GPT-4o</option>
							<option value="claude-3">Claude 3</option>
							<option value="claude-3.5">Claude 3.5</option>
							<option value="gemini-pro">Gemini Pro</option>
							<option value="llama-3">Llama 3</option>
						</select>
					</div>
				{/if}
			</div>

			{#if isEditing}
				<div class="ai-prompt-section">
					<label for="ai-prompt-{block.id}" class="ai-prompt-label">
						<IconWand size={16} aria-hidden="true" />
						Prompt
					</label>
					<textarea
						id="ai-prompt-{block.id}"
						class="ai-prompt-input"
						value={block.content.aiPrompt || ''}
						oninput={(e: Event) => updateContent({ aiPrompt: (e.target as HTMLTextAreaElement).value })}
						placeholder="Describe the content you want to generate..."
						rows="3"
						disabled={aiLoading}
					></textarea>
					<button
						type="button"
						class="ai-generate-btn"
						onclick={generateAIContent}
						disabled={aiLoading || !block.content.aiPrompt}
						aria-describedby="ai-generate-hint-{block.id}"
					>
						{#if aiLoading}
							<IconLoader2 size={18} class="spin" aria-hidden="true" />
							<span>Generating...</span>
						{:else}
							<IconSparkles size={18} aria-hidden="true" />
							<span>Generate Content</span>
						{/if}
					</button>
					<span id="ai-generate-hint-{block.id}" class="visually-hidden">
						Click to generate AI content based on your prompt
					</span>
				</div>
			{/if}

			{#if aiError}
				<div class="ai-error" role="alert">
					<IconAlertCircle size={18} aria-hidden="true" />
					<span>{aiError}</span>
					<button
						type="button"
						class="ai-retry-btn"
						onclick={() => retryAIAction('generate')}
						aria-label="Retry generation"
					>
						<IconRefresh size={16} aria-hidden="true" />
						Retry
					</button>
				</div>
			{/if}

			{#if aiLoading && !block.content.aiOutput}
				<div class="ai-loading-skeleton" aria-hidden="true">
					<div class="skeleton-line skeleton-line-full"></div>
					<div class="skeleton-line skeleton-line-full"></div>
					<div class="skeleton-line skeleton-line-medium"></div>
					<div class="skeleton-line skeleton-line-short"></div>
				</div>
			{/if}

			{#if block.content.aiOutput}
				<div class="ai-output-section">
					<div class="ai-output-header">
						<span class="ai-output-label">Generated Content</span>
						{#if isEditing}
							<button
								type="button"
								class="ai-regenerate-btn"
								onclick={regenerateAIContent}
								disabled={aiLoading}
								aria-label="Regenerate content"
							>
								<IconRefresh size={16} aria-hidden="true" />
								Regenerate
							</button>
						{/if}
					</div>
					<div
						class="ai-output-content"
						contenteditable={isEditing}
						oninput={(e: Event) => updateContent({ aiOutput: (e.target as HTMLElement).textContent || '' })}
						onpaste={handlePaste}
					>
						{block.content.aiOutput}
					</div>
				</div>
			{/if}
		</div>

	<!-- AI Summary Block -->
	{:else if block.type === 'aiSummary'}
		<div class="ai-summary-block" role="region" aria-label="AI Summary" aria-busy={aiLoading}>
			<div class="ai-block-header">
				<button
					type="button"
					class="ai-summary-toggle"
					onclick={toggleSummaryCollapsed}
					aria-expanded={!aiSummaryCollapsed}
					aria-controls="ai-summary-content-{block.id}"
				>
					{#if aiSummaryCollapsed}
						<IconChevronRight size={18} aria-hidden="true" />
					{:else}
						<IconChevronDown size={18} aria-hidden="true" />
					{/if}
					<div class="ai-badge">
						<IconFileDescription size={16} aria-hidden="true" />
						<span>TL;DR</span>
					</div>
				</button>
				{#if isEditing}
					<div class="ai-summary-controls">
						<label for="summary-length-{block.id}" class="visually-hidden">Summary Length</label>
						<select
							id="summary-length-{block.id}"
							value={block.settings.summaryLength || 'medium'}
							onchange={(e: Event) => onUpdate({ settings: { ...block.settings, summaryLength: (e.target as HTMLSelectElement).value as 'short' | 'medium' | 'long' } })}
							disabled={aiLoading}
						>
							<option value="short">Short</option>
							<option value="medium">Medium</option>
							<option value="long">Long</option>
						</select>
						<button
							type="button"
							class="ai-refresh-btn"
							onclick={generateAISummary}
							disabled={aiLoading}
							aria-label="Generate or refresh summary"
						>
							{#if aiLoading}
								<IconLoader2 size={16} class="spin" aria-hidden="true" />
							{:else}
								<IconRefresh size={16} aria-hidden="true" />
							{/if}
						</button>
					</div>
				{/if}
			</div>

			{#if aiError}
				<div class="ai-error" role="alert">
					<IconAlertCircle size={18} aria-hidden="true" />
					<span>{aiError}</span>
					<button
						type="button"
						class="ai-retry-btn"
						onclick={() => retryAIAction('summary')}
						aria-label="Retry summary generation"
					>
						<IconRefresh size={16} aria-hidden="true" />
						Retry
					</button>
				</div>
			{/if}

			<div
				id="ai-summary-content-{block.id}"
				class="ai-summary-content"
				class:collapsed={aiSummaryCollapsed}
			>
				{#if aiLoading && !block.content.aiSummary}
					<div class="ai-loading-skeleton" aria-hidden="true">
						<div class="skeleton-line skeleton-line-full"></div>
						<div class="skeleton-line skeleton-line-medium"></div>
					</div>
				{:else if block.content.aiSummary}
					<p
						contenteditable={isEditing}
						class="ai-summary-text editable-content"
						oninput={(e: Event) => updateContent({ aiSummary: (e.target as HTMLElement).textContent || '' })}
						onpaste={handlePaste}
					>
						{block.content.aiSummary}
					</p>
				{:else if isEditing}
					<p class="ai-summary-placeholder">
						Click refresh to generate an AI summary of your content.
					</p>
				{/if}
			</div>
		</div>

	<!-- AI Translation Block -->
	{:else if block.type === 'aiTranslation'}
		<div
			class="ai-translation-block"
			class:side-by-side={aiTranslationView === 'side-by-side'}
			role="region"
			aria-label="AI Translation"
			aria-busy={aiLoading}
		>
			<div class="ai-block-header">
				<div class="ai-badge">
					<IconLanguage size={16} aria-hidden="true" />
					<span>AI Translation</span>
				</div>
				<div class="ai-translation-controls">
					<button
						type="button"
						class="ai-view-toggle"
						onclick={toggleTranslationView}
						aria-label="Toggle view mode"
						title={aiTranslationView === 'stacked' ? 'Switch to side-by-side view' : 'Switch to stacked view'}
					>
						{aiTranslationView === 'stacked' ? 'Side-by-Side' : 'Stacked'}
					</button>
				</div>
			</div>

			<div class="ai-translation-content">
				<div class="ai-translation-source">
					<div class="ai-translation-lang-header">
						<label for="source-lang-{block.id}">Source</label>
						{#if isEditing}
							<select
								id="source-lang-{block.id}"
								value={block.settings.sourceLanguage || 'en'}
								onchange={(e: Event) => onUpdate({ settings: { ...block.settings, sourceLanguage: (e.target as HTMLSelectElement).value } })}
								disabled={aiLoading}
							>
								<option value="en">English</option>
								<option value="es">Spanish</option>
								<option value="fr">French</option>
								<option value="de">German</option>
								<option value="it">Italian</option>
								<option value="pt">Portuguese</option>
								<option value="zh">Chinese</option>
								<option value="ja">Japanese</option>
								<option value="ko">Korean</option>
								<option value="ar">Arabic</option>
							</select>
						{:else}
							<span class="lang-display">{block.settings.sourceLanguage || 'English'}</span>
						{/if}
					</div>
					<textarea
						class="ai-translation-textarea"
						value={block.content.sourceText || ''}
						oninput={(e: Event) => updateContent({ sourceText: (e.target as HTMLTextAreaElement).value })}
						placeholder="Enter text to translate..."
						rows="4"
						disabled={!isEditing || aiLoading}
						aria-label="Source text"
					></textarea>
				</div>

				<div class="ai-translation-arrow" aria-hidden="true">
					<IconArrowRight size={20} />
				</div>

				<div class="ai-translation-target">
					<div class="ai-translation-lang-header">
						<label for="target-lang-{block.id}">Target</label>
						{#if isEditing}
							<select
								id="target-lang-{block.id}"
								value={block.settings.targetLanguage || 'es'}
								onchange={(e: Event) => onUpdate({ settings: { ...block.settings, targetLanguage: (e.target as HTMLSelectElement).value } })}
								disabled={aiLoading}
							>
								<option value="en">English</option>
								<option value="es">Spanish</option>
								<option value="fr">French</option>
								<option value="de">German</option>
								<option value="it">Italian</option>
								<option value="pt">Portuguese</option>
								<option value="zh">Chinese</option>
								<option value="ja">Japanese</option>
								<option value="ko">Korean</option>
								<option value="ar">Arabic</option>
							</select>
						{:else}
							<span class="lang-display">{block.settings.targetLanguage || 'Spanish'}</span>
						{/if}
						{#if block.content.translatedText}
							<button
								type="button"
								class="ai-copy-btn"
								onclick={() => copyToClipboard(block.content.translatedText || '')}
								aria-label="Copy translated text"
							>
								{#if copiedText}
									<IconCheck size={16} aria-hidden="true" />
									<span>Copied!</span>
								{:else}
									<IconCopy size={16} aria-hidden="true" />
									<span>Copy</span>
								{/if}
							</button>
						{/if}
					</div>
					{#if aiLoading}
						<div class="ai-loading-skeleton translation-skeleton" aria-hidden="true">
							<div class="skeleton-line skeleton-line-full"></div>
							<div class="skeleton-line skeleton-line-full"></div>
							<div class="skeleton-line skeleton-line-medium"></div>
						</div>
					{:else if block.content.translatedText}
						<div
							class="ai-translation-output"
							contenteditable={isEditing}
							oninput={(e: Event) => updateContent({ translatedText: (e.target as HTMLElement).textContent || '' })}
							onpaste={handlePaste}
						>
							{block.content.translatedText}
						</div>
					{:else}
						<div class="ai-translation-placeholder">
							Translation will appear here
						</div>
					{/if}
				</div>
			</div>

			{#if aiError}
				<div class="ai-error" role="alert">
					<IconAlertCircle size={18} aria-hidden="true" />
					<span>{aiError}</span>
					<button
						type="button"
						class="ai-retry-btn"
						onclick={() => retryAIAction('translate')}
						aria-label="Retry translation"
					>
						<IconRefresh size={16} aria-hidden="true" />
						Retry
					</button>
				</div>
			{/if}

			{#if isEditing}
				<div class="ai-translation-actions">
					<button
						type="button"
						class="ai-translate-btn"
						onclick={translateContent}
						disabled={aiLoading || !block.content.sourceText}
					>
						{#if aiLoading}
							<IconLoader2 size={18} class="spin" aria-hidden="true" />
							<span>Translating...</span>
						{:else}
							<IconLanguage size={18} aria-hidden="true" />
							<span>Translate</span>
						{/if}
					</button>
				</div>
			{/if}
		</div>

	<!-- ===== INTERACTIVE BLOCKS ===== -->

	<!-- Accordion Block -->
	{:else if block.type === 'accordion'}
		{@const items = block.content.accordionItems || [{ id: 'acc_1', title: 'Section 1', content: 'Content for section 1' }]}
		{@const allowMultiple = block.settings.allowMultiple || false}
		{@const iconStyle = block.settings.iconStyle || 'chevron'}
		<div class="accordion-block" role="region" aria-label="Accordion">
			{#each items as item, index (item.id)}
				{@const isOpen = accordionOpenItems.has(item.id)}
				<div class="accordion-item" class:open={isOpen}>
					<button type="button" class="accordion-header" aria-expanded={isOpen} aria-controls="accordion-panel-{item.id}" onclick={() => toggleAccordionItem(item.id, allowMultiple)} onkeydown={(e: KeyboardEvent) => { if (e.key === 'ArrowDown') { e.preventDefault(); const next = items[index + 1]; if (next) document.getElementById(`accordion-btn-${next.id}`)?.focus(); } if (e.key === 'ArrowUp') { e.preventDefault(); const prev = items[index - 1]; if (prev) document.getElementById(`accordion-btn-${prev.id}`)?.focus(); } if (e.key === 'Home') { e.preventDefault(); document.getElementById(`accordion-btn-${items[0].id}`)?.focus(); } if (e.key === 'End') { e.preventDefault(); document.getElementById(`accordion-btn-${items[items.length - 1].id}`)?.focus(); } }} id="accordion-btn-{item.id}">
						{#if isEditing}<span contenteditable="true" class="accordion-title editable-content" onclick={(e: MouseEvent) => e.stopPropagation()} oninput={(e: Event) => { const newItems = items.map((it, i) => i === index ? { ...it, title: (e.target as HTMLElement).textContent || '' } : it); updateContent({ accordionItems: newItems }); }}>{item.title}</span>{:else}<span class="accordion-title">{item.title}</span>{/if}
						<span class="accordion-icon" aria-hidden="true">{#if iconStyle === 'plusminus'}{#if isOpen}<IconMinus size={18} />{:else}<IconPlus size={18} />{/if}{:else}<IconChevronDown size={18} class={isOpen ? 'rotated' : ''} />{/if}</span>
					</button>
					<div id="accordion-panel-{item.id}" class="accordion-panel" role="region" aria-labelledby="accordion-btn-{item.id}" hidden={!isOpen}>
						{#if isEditing}<div contenteditable="true" class="accordion-content editable-content" oninput={(e: Event) => { const newItems = items.map((it, i) => i === index ? { ...it, content: (e.target as HTMLElement).textContent || '' } : it); updateContent({ accordionItems: newItems }); }}>{item.content}</div>{:else}<div class="accordion-content">{item.content}</div>{/if}
					</div>
				</div>
			{/each}
			{#if isEditing}<button type="button" class="add-item-btn" onclick={() => { const newItems = [...items, { id: `acc_${Date.now()}`, title: `Section ${items.length + 1}`, content: '' }]; updateContent({ accordionItems: newItems }); }}><IconPlus size={14} /> Add Section</button>{/if}
		</div>

	<!-- Tabs Block -->
	{:else if block.type === 'tabs'}
		{@const tabs = block.content.tabItems || [{ id: 'tab_1', label: 'Tab 1', content: 'Content for tab 1' }]}
		{@const layout = block.settings.tabLayout || 'horizontal'}
		{@const currentTab = activeTabId || tabs[0]?.id}
		<div class="tabs-block tabs-{layout}" role="tablist" aria-label="Tabbed content">
			<div class="tabs-list" role="tablist" aria-orientation={layout === 'vertical' ? 'vertical' : 'horizontal'}>
				{#each tabs as tab, index (tab.id)}
					<button type="button" role="tab" id="tab-{tab.id}" class="tab-button" class:active={currentTab === tab.id} aria-selected={currentTab === tab.id} aria-controls="tabpanel-{tab.id}" tabindex={currentTab === tab.id ? 0 : -1} onclick={() => setActiveTab(tab.id)} onkeydown={(e: KeyboardEvent) => { const keys = layout === 'vertical' ? ['ArrowDown', 'ArrowUp'] : ['ArrowRight', 'ArrowLeft']; if (e.key === keys[0]) { e.preventDefault(); const next = tabs[index + 1] || tabs[0]; setActiveTab(next.id); document.getElementById(`tab-${next.id}`)?.focus(); } if (e.key === keys[1]) { e.preventDefault(); const prev = tabs[index - 1] || tabs[tabs.length - 1]; setActiveTab(prev.id); document.getElementById(`tab-${prev.id}`)?.focus(); } if (e.key === 'Home') { e.preventDefault(); setActiveTab(tabs[0].id); document.getElementById(`tab-${tabs[0].id}`)?.focus(); } if (e.key === 'End') { e.preventDefault(); setActiveTab(tabs[tabs.length - 1].id); document.getElementById(`tab-${tabs[tabs.length - 1].id}`)?.focus(); } }}>
						{#if isEditing}<span contenteditable="true" class="tab-label editable-content" onclick={(e: MouseEvent) => e.stopPropagation()} oninput={(e: Event) => { const newTabs = tabs.map((t, i) => i === index ? { ...t, label: (e.target as HTMLElement).textContent || '' } : t); updateContent({ tabItems: newTabs }); }}>{tab.label}</span>{:else}<span class="tab-label">{tab.label}</span>{/if}
					</button>
				{/each}
				{#if isEditing}<button type="button" class="tab-add-btn" onclick={() => { const newTabs = [...tabs, { id: `tab_${Date.now()}`, label: `Tab ${tabs.length + 1}`, content: '' }]; updateContent({ tabItems: newTabs }); }}><IconPlus size={14} /></button>{/if}
			</div>
			<div class="tabs-panels">
				{#each tabs as tab, index (tab.id)}<div id="tabpanel-{tab.id}" class="tab-panel" role="tabpanel" aria-labelledby="tab-{tab.id}" hidden={currentTab !== tab.id} tabindex="0">{#if isEditing}<div contenteditable="true" class="tab-content editable-content" oninput={(e: Event) => { const newTabs = tabs.map((t, i) => i === index ? { ...t, content: (e.target as HTMLElement).textContent || '' } : t); updateContent({ tabItems: newTabs }); }}>{tab.content}</div>{:else}<div class="tab-content">{tab.content}</div>{/if}</div>{/each}
			</div>
		</div>

	<!-- Toggle Block -->
	{:else if block.type === 'toggle'}
		{@const defaultOpen = block.settings.defaultOpen || false}
		{@const isOpen = toggleOpen || (defaultOpen && !toggleOpen)}
		<div class="toggle-block" role="region">
			<button type="button" class="toggle-header" aria-expanded={isOpen} aria-controls="toggle-content-{block.id}" onclick={handleToggle}>
				<span class="toggle-icon" aria-hidden="true">{#if isOpen}<IconChevronDown size={18} />{:else}<IconChevronRight size={18} />{/if}</span>
				{#if isEditing}<span contenteditable="true" class="toggle-label editable-content" onclick={(e: MouseEvent) => e.stopPropagation()} oninput={handleTextInput}>{block.content.text || 'Click to toggle'}</span>{:else}<span class="toggle-label">{block.content.text || 'Click to toggle'}</span>{/if}
			</button>
			<div id="toggle-content-{block.id}" class="toggle-content" class:open={isOpen} hidden={!isOpen}>
				{#if isEditing}<div contenteditable="true" class="toggle-inner editable-content" oninput={(e: Event) => updateContent({ html: (e.target as HTMLElement).textContent || '' })}>{block.content.html || 'Hidden content goes here...'}</div>{:else}<div class="toggle-inner">{block.content.html || ''}</div>{/if}
			</div>
		</div>

	<!-- Table of Contents Block -->
	{:else if block.type === 'toc'}
		{@const headings = block.content.tocHeadings || [{ id: 'h1', text: 'Introduction', level: 2 }, { id: 'h2', text: 'Getting Started', level: 2 }, { id: 'h3', text: 'Advanced Topics', level: 3 }]}
		<nav class="toc-block" aria-label="Table of contents">
			<button type="button" class="toc-header" onclick={toggleTocMobile} aria-expanded={tocOpen}>
				<IconListTree size={18} aria-hidden="true" /><span class="toc-title">{block.content.text || 'Table of Contents'}</span><span class="toc-toggle-icon"><IconChevronDown size={16} class={tocOpen ? '' : 'collapsed'} /></span>
			</button>
			<ul class="toc-list" class:collapsed={!tocOpen} role="list">
				{#each headings as heading (heading.id)}<li class="toc-item toc-level-{heading.level}" class:active={activeTocHeading === heading.id}><a href="#{heading.id}" class="toc-link" onclick={(e: MouseEvent) => { e.preventDefault(); scrollToHeading(heading.id); }} aria-current={activeTocHeading === heading.id ? 'location' : undefined}>{heading.text}</a></li>{/each}
			</ul>
		</nav>

	<!-- Buttons Group Block -->
	{:else if block.type === 'buttons'}
		{@const buttons = block.content.buttonItems || [{ id: 'btn_1', text: 'Button 1', url: '#', style: 'primary' }]}
		{@const layout = block.settings.buttonLayout || 'row'}
		{@const gap = block.settings.buttonGap || '0.75rem'}
		{@const align = block.settings.buttonAlign || 'left'}
		<div class="buttons-block buttons-{layout}" style="gap: {gap}; justify-content: {align === 'center' ? 'center' : align === 'right' ? 'flex-end' : 'flex-start'};">
			{#each buttons as btn, index (btn.id)}
				<a href={btn.url || '#'} class="btn btn-{btn.style || 'primary'}" target={btn.newTab ? '_blank' : '_self'} rel={btn.newTab ? 'noopener noreferrer' : undefined} onclick={(e: MouseEvent) => isEditing && e.preventDefault()}>
					{#if btn.icon}<span class="btn-icon" aria-hidden="true"><IconExternalLink size={16} /></span>{/if}
					{#if isEditing}<span contenteditable="true" class="btn-text editable-content" onclick={(e: MouseEvent) => e.stopPropagation()} oninput={(e: Event) => { const newBtns = buttons.map((b, i) => i === index ? { ...b, text: (e.target as HTMLElement).textContent || '' } : b); updateContent({ buttonItems: newBtns }); }}>{btn.text}</span>{:else}<span class="btn-text">{btn.text}</span>{/if}
				</a>
			{/each}
			{#if isEditing}<button type="button" class="btn-add" onclick={() => { const newBtns = [...buttons, { id: `btn_${Date.now()}`, text: `Button ${buttons.length + 1}`, url: '#', style: 'primary' }]; updateContent({ buttonItems: newBtns }); }}><IconPlus size={14} /> Add Button</button>{/if}
		</div>

	<!-- Default/Unknown Block -->
	{:else}
		<div class="unknown-block">
			<span>Unknown block type: {block.type}</span>
		</div>
	{/if}
</div>

<style>
	.block-renderer {
		min-height: 1em;
		position: relative;
	}

	/* Editable Content */
	.editable-content {
		outline: none;
		min-height: 1em;
	}

	.editable-content.placeholder:empty::before {
		content: attr(data-placeholder);
		color: #999;
		pointer-events: none;
	}

	/* Paragraph */
	.block-paragraph p {
		margin: 0;
		line-height: 1.7;
		font-size: 1rem;
	}

	/* Headings */
	.heading-block-wrapper {
		position: relative;
	}

	.heading-level-selector {
		display: flex;
		gap: 0.25rem;
		margin-bottom: 0.5rem;
		padding: 0.375rem;
		background: #f5f5f5;
		border-radius: 8px;
		width: fit-content;
	}

	.level-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		min-width: 32px;
		height: 28px;
		padding: 0 0.5rem;
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 4px;
		font-size: 0.75rem;
		font-weight: 600;
		color: #666;
		cursor: pointer;
		transition: all 0.15s;
	}

	.level-btn:hover {
		background: #e5e5e5;
		color: #1a1a1a;
	}

	.level-btn.active {
		background: #3b82f6;
		border-color: #3b82f6;
		color: white;
	}

	.heading-1 {
		font-size: 2.25rem;
		font-weight: 700;
		margin: 0;
	}
	.heading-2 {
		font-size: 1.875rem;
		font-weight: 600;
		margin: 0;
	}
	.heading-3 {
		font-size: 1.5rem;
		font-weight: 600;
		margin: 0;
	}
	.heading-4 {
		font-size: 1.25rem;
		font-weight: 600;
		margin: 0;
	}
	.heading-5 {
		font-size: 1.125rem;
		font-weight: 600;
		margin: 0;
	}
	.heading-6 {
		font-size: 1rem;
		font-weight: 600;
		margin: 0;
	}

	/* Quote */
	.quote-block {
		border-left: 4px solid #3b82f6;
		padding-left: 1.5rem;
		margin: 0;
	}

	.quote-text {
		font-size: 1.125rem;
		line-height: 1.7;
		font-style: italic;
		color: #374151;
	}

	.quote-cite {
		display: block;
		margin-top: 0.75rem;
		font-size: 0.875rem;
		color: #6b7280;
		font-style: normal;
	}

	/* Pull Quote */
	.pullquote-block {
		margin: 0;
		padding: 2rem;
		text-align: center;
		border-top: 2px solid #e5e7eb;
		border-bottom: 2px solid #e5e7eb;
	}

	.pullquote-text {
		font-size: 1.5rem;
		font-weight: 500;
		line-height: 1.5;
		color: #1f2937;
		margin: 0;
	}

	/* Code */
	.code-block {
		border-radius: 8px;
		overflow: hidden;
		background: #1e293b;
	}

	.code-header {
		padding: 0.5rem 1rem;
		background: #0f172a;
	}

	.code-header select {
		padding: 0.25rem 0.5rem;
		background: #334155;
		color: #e2e8f0;
		border: none;
		border-radius: 4px;
		font-size: 0.75rem;
	}

	.code-content {
		margin: 0;
		padding: 1rem;
		overflow-x: auto;
	}

	.code-content code {
		font-family: 'Fira Code', 'Consolas', monospace;
		font-size: 0.875rem;
		color: #e2e8f0;
		outline: none;
	}

	/* List */
	.list-block {
		margin: 0;
		padding-left: 1.5rem;
	}

	.list-item {
		display: flex;
		align-items: flex-start;
		gap: 0.5rem;
		margin-bottom: 0.375rem;
	}

	.list-text {
		flex: 1;
		line-height: 1.6;
	}

	.remove-item {
		display: flex;
		padding: 0.125rem;
		background: transparent;
		border: none;
		border-radius: 4px;
		cursor: pointer;
		color: #999;
		opacity: 0;
		transition: opacity 0.15s;
	}

	.list-item:hover .remove-item {
		opacity: 1;
	}

	.remove-item:hover {
		color: #ef4444;
	}

	.add-item-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.75rem;
		margin-top: 0.5rem;
		background: transparent;
		border: 1px dashed #d1d5db;
		border-radius: 4px;
		color: #6b7280;
		font-size: 0.8125rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.add-item-btn:hover {
		border-color: #3b82f6;
		color: #3b82f6;
	}

	/* Checklist */
	.checklist-block {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
	}

	.check-item {
		display: flex;
		align-items: flex-start;
		gap: 0.625rem;
		cursor: pointer;
	}

	.check-item input[type='checkbox'] {
		position: absolute;
		opacity: 0;
	}

	.check-icon {
		width: 20px;
		height: 20px;
		display: flex;
		align-items: center;
		justify-content: center;
		border: 2px solid #d1d5db;
		border-radius: 4px;
		flex-shrink: 0;
		margin-top: 2px;
		transition: all 0.15s;
	}

	.check-item.checked .check-icon {
		background: #3b82f6;
		border-color: #3b82f6;
		color: white;
	}

	.check-text {
		flex: 1;
		line-height: 1.5;
	}

	.check-item.checked .check-text {
		text-decoration: line-through;
		color: #9ca3af;
	}

	/* Image */
	.image-block {
		margin: 0;
	}

	.image-block img {
		display: block;
		width: 100%;
		max-width: 100%;
		height: auto;
		border-radius: 8px;
	}

	.image-caption {
		margin-top: 0.75rem;
		text-align: center;
		font-size: 0.875rem;
		color: #6b7280;
	}

	.image-placeholder {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem;
		background: #f9fafb;
		border: 2px dashed #d1d5db;
		border-radius: 8px;
		color: #9ca3af;
		cursor: pointer;
		position: relative;
	}

	.image-placeholder input {
		position: absolute;
		inset: 0;
		opacity: 0;
		cursor: pointer;
	}

	.image-placeholder span {
		margin-top: 0.75rem;
	}

	/* Video */
	.video-embed {
		position: relative;
		padding-bottom: 56.25%;
		height: 0;
		overflow: hidden;
		border-radius: 8px;
	}

	.video-embed iframe {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
	}

	.video-placeholder {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem;
		background: #f9fafb;
		border: 2px dashed #d1d5db;
		border-radius: 8px;
		color: #9ca3af;
	}

	.video-placeholder input {
		margin-top: 1rem;
		padding: 0.5rem 1rem;
		border: 1px solid #d1d5db;
		border-radius: 6px;
		width: 100%;
		max-width: 400px;
	}

	/* Separator */
	.separator-block {
		border: none;
		border-top: 2px solid #e5e7eb;
		margin: 1rem 0;
	}

	/* Spacer */
	.spacer-block {
		position: relative;
		background: repeating-linear-gradient(
			45deg,
			transparent,
			transparent 10px,
			#f3f4f6 10px,
			#f3f4f6 20px
		);
		border-radius: 4px;
	}

	.spacer-handle {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.25rem 0.5rem;
		background: white;
		border-radius: 4px;
		font-size: 0.75rem;
		color: #6b7280;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	/* Button */
	.button-block-wrapper {
		display: flex;
	}

	.button-block {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		text-decoration: none;
		border-radius: 8px;
		font-weight: 500;
		transition: all 0.15s;
	}

	.button-primary {
		background: #3b82f6;
		color: white;
	}

	.button-primary:hover {
		background: #2563eb;
	}

	.button-secondary {
		background: #f3f4f6;
		color: #1f2937;
	}

	.button-outline {
		background: transparent;
		border: 2px solid #3b82f6;
		color: #3b82f6;
	}

	.button-ghost {
		background: transparent;
		color: #3b82f6;
	}

	.button-size-small {
		padding: 0.5rem 1rem;
		font-size: 0.875rem;
	}

	.button-size-medium {
		padding: 0.75rem 1.5rem;
		font-size: 1rem;
	}

	.button-size-large {
		padding: 1rem 2rem;
		font-size: 1.125rem;
	}

	.button-text {
		outline: none;
	}

	/* Callout */
	.callout-block {
		display: flex;
		gap: 1rem;
		padding: 1.25rem;
		background: #fef3c7;
		border-left: 4px solid #f59e0b;
		border-radius: 0 8px 8px 0;
	}

	.callout-icon {
		color: #d97706;
		flex-shrink: 0;
	}

	.callout-content {
		flex: 1;
		line-height: 1.6;
	}

	/* Chart */
	.chart-block {
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		overflow: hidden;
	}

	.chart-header {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		background: #f9fafb;
		border-bottom: 1px solid #e5e7eb;
	}

	.ticker-input {
		padding: 0.375rem 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 4px;
		font-family: monospace;
		font-weight: 600;
		text-transform: uppercase;
		width: 80px;
	}

	.chart-placeholder {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		background: #f3f4f6;
		color: #6b7280;
	}

	.chart-placeholder small {
		margin-top: 0.5rem;
		font-size: 0.75rem;
		color: #9ca3af;
	}

	/* Disclaimer */
	.disclaimer-block {
		display: flex;
		gap: 1rem;
		padding: 1.25rem;
		background: #fef2f2;
		border: 1px solid #fecaca;
		border-radius: 8px;
		color: #991b1b;
	}

	.disclaimer-block p {
		margin: 0;
		font-size: 0.875rem;
		line-height: 1.5;
	}

	/* HTML Block */
	.html-block-editor {
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		overflow: hidden;
	}

	.html-header {
		padding: 0.5rem 1rem;
		background: #f9fafb;
		font-size: 0.75rem;
		font-weight: 500;
		color: #6b7280;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.html-block-editor textarea {
		width: 100%;
		padding: 1rem;
		border: none;
		font-family: monospace;
		font-size: 0.875rem;
		resize: vertical;
		outline: none;
	}

	/* Unknown */
	.unknown-block {
		padding: 1rem;
		background: #f9fafb;
		border: 1px dashed #d1d5db;
		border-radius: 8px;
		color: #6b7280;
		text-align: center;
	}


	/* TRADING-SPECIFIC BLOCK STYLES */
	.ticker-block { border: 1px solid #e5e7eb; border-radius: 12px; overflow: hidden; background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%); }
	.ticker-header-bar { display: flex; align-items: center; gap: 0.75rem; padding: 0.75rem 1rem; background: rgba(0, 0, 0, 0.2); border-bottom: 1px solid rgba(255, 255, 255, 0.1); color: #94a3b8; }
	.ticker-title { font-size: 0.75rem; font-weight: 600; text-transform: uppercase; letter-spacing: 0.05em; }
	.ticker-add-btn { margin-left: auto; display: inline-flex; align-items: center; gap: 0.25rem; padding: 0.375rem 0.75rem; background: rgba(59, 130, 246, 0.2); border: 1px solid rgba(59, 130, 246, 0.5); border-radius: 6px; color: #60a5fa; font-size: 0.75rem; cursor: pointer; transition: all 0.15s; }
	.ticker-add-btn:hover { background: rgba(59, 130, 246, 0.3); }
	.ticker-list { display: flex; flex-wrap: wrap; gap: 1px; background: rgba(255, 255, 255, 0.05); }
	.ticker-list.multi-row { flex-direction: column; }
	.ticker-item { flex: 1; min-width: 140px; display: flex; flex-direction: column; gap: 0.5rem; padding: 1rem; background: rgba(0, 0, 0, 0.2); transition: all 0.2s; }
	.ticker-item.positive { border-left: 3px solid #10b981; }
	.ticker-item.negative { border-left: 3px solid #ef4444; }
	.ticker-item.positive .ticker-direction-icon, .ticker-item.positive .ticker-change, .ticker-item.positive .ticker-sparkline { color: #10b981; }
	.ticker-item.negative .ticker-direction-icon, .ticker-item.negative .ticker-change, .ticker-item.negative .ticker-sparkline { color: #ef4444; }
	.ticker-symbol-row { display: flex; align-items: center; gap: 0.5rem; }
	.ticker-symbol { font-size: 1rem; font-weight: 700; color: #f1f5f9; font-family: monospace; }
	.ticker-symbol-input { width: 70px; padding: 0.25rem 0.5rem; background: rgba(255, 255, 255, 0.1); border: 1px solid rgba(255, 255, 255, 0.2); border-radius: 4px; color: #f1f5f9; font-family: monospace; font-weight: 600; text-transform: uppercase; }
	.ticker-remove-btn { padding: 0.25rem; background: rgba(239, 68, 68, 0.2); border: none; border-radius: 4px; color: #ef4444; cursor: pointer; }
	.ticker-direction-icon { margin-left: auto; }
	.ticker-price-row { display: flex; align-items: center; }
	.ticker-price { font-size: 1.25rem; font-weight: 600; color: #f1f5f9; }
	.ticker-price-input { width: 100px; padding: 0.375rem 0.5rem; background: rgba(255, 255, 255, 0.1); border: 1px solid rgba(255, 255, 255, 0.2); border-radius: 4px; color: #f1f5f9; font-size: 1rem; }
	.ticker-change-row { display: flex; align-items: center; }
	.ticker-change { font-size: 0.875rem; font-weight: 600; }
	.ticker-change-input { width: 80px; padding: 0.25rem 0.5rem; background: rgba(255, 255, 255, 0.1); border: 1px solid rgba(255, 255, 255, 0.2); border-radius: 4px; color: #f1f5f9; }
	.ticker-sparkline { height: 24px; margin-top: 0.5rem; }
	.ticker-sparkline svg { width: 100%; height: 100%; }
	.ticker-multi-toggle { display: flex; align-items: center; gap: 0.5rem; padding: 0.75rem 1rem; background: rgba(0, 0, 0, 0.2); color: #94a3b8; font-size: 0.8125rem; cursor: pointer; }
	.ticker-multi-toggle input { accent-color: #3b82f6; }

	.price-alert-block { border-radius: 12px; overflow: hidden; box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15); }
	.price-alert-block.alert-bullish { background: linear-gradient(135deg, #064e3b 0%, #065f46 100%); border: 1px solid #10b981; }
	.price-alert-block.alert-bearish { background: linear-gradient(135deg, #7f1d1d 0%, #991b1b 100%); border: 1px solid #ef4444; }
	.alert-header-section { display: flex; align-items: center; gap: 1rem; padding: 1rem 1.25rem; background: rgba(0, 0, 0, 0.2); }
	.alert-icon-wrapper { display: flex; align-items: center; justify-content: center; width: 48px; height: 48px; border-radius: 50%; background: rgba(255, 255, 255, 0.1); color: white; animation: pulse 2s infinite; }
	@keyframes pulse { 0%, 100% { transform: scale(1); opacity: 1; } 50% { transform: scale(1.05); opacity: 0.8; } }
	.alert-title-row { display: flex; flex-direction: column; gap: 0.25rem; }
	.alert-badge { font-size: 0.75rem; font-weight: 700; color: rgba(255, 255, 255, 0.7); letter-spacing: 0.1em; }
	.alert-direction-badge { display: inline-flex; align-items: center; gap: 0.25rem; padding: 0.25rem 0.75rem; border-radius: 20px; font-size: 0.875rem; font-weight: 600; width: fit-content; }
	.alert-direction-badge.bullish { background: rgba(16, 185, 129, 0.3); color: #6ee7b7; }
	.alert-direction-badge.bearish { background: rgba(239, 68, 68, 0.3); color: #fca5a5; }
	.alert-body { padding: 1.25rem; display: flex; flex-direction: column; gap: 1rem; }
	.alert-symbol-section { text-align: center; }
	.alert-symbol { font-size: 2rem; font-weight: 700; color: white; font-family: monospace; }
	.alert-symbol-input { width: 120px; padding: 0.5rem 1rem; background: rgba(255, 255, 255, 0.1); border: 1px solid rgba(255, 255, 255, 0.3); border-radius: 8px; color: white; font-size: 1.5rem; font-weight: 700; font-family: monospace; text-align: center; text-transform: uppercase; }
	.alert-target-section { display: flex; align-items: center; justify-content: center; gap: 0.5rem; color: rgba(255, 255, 255, 0.9); }
	.alert-label { font-size: 0.875rem; color: rgba(255, 255, 255, 0.7); }
	.alert-price-input { width: 100px; padding: 0.375rem 0.75rem; background: rgba(255, 255, 255, 0.1); border: 1px solid rgba(255, 255, 255, 0.3); border-radius: 6px; color: white; font-size: 1rem; font-weight: 600; }
	.alert-target-price { font-size: 1.25rem; font-weight: 700; color: white; }
	.alert-direction-toggle { display: flex; justify-content: center; gap: 1rem; }
	.alert-direction-toggle label { cursor: pointer; }
	.alert-direction-toggle input { display: none; }
	.direction-option { display: inline-flex; align-items: center; gap: 0.375rem; padding: 0.5rem 1rem; border-radius: 8px; font-size: 0.875rem; font-weight: 500; transition: all 0.15s; }
	.direction-option.bullish { background: rgba(16, 185, 129, 0.2); color: #6ee7b7; border: 1px solid transparent; }
	.direction-option.bearish { background: rgba(239, 68, 68, 0.2); color: #fca5a5; border: 1px solid transparent; }
	.alert-direction-toggle input:checked + .direction-option.bullish { background: rgba(16, 185, 129, 0.4); border-color: #10b981; }
	.alert-direction-toggle input:checked + .direction-option.bearish { background: rgba(239, 68, 68, 0.4); border-color: #ef4444; }
	.alert-levels { display: flex; justify-content: center; gap: 2rem; padding: 1rem; background: rgba(0, 0, 0, 0.2); border-radius: 8px; }
	.alert-level { text-align: center; }
	.alert-level-label { display: block; font-size: 0.75rem; color: rgba(255, 255, 255, 0.6); text-transform: uppercase; letter-spacing: 0.05em; margin-bottom: 0.25rem; }
	.alert-level-value { font-size: 1.125rem; font-weight: 600; color: white; }
	.alert-level-input { width: 90px; padding: 0.375rem 0.5rem; background: rgba(255, 255, 255, 0.1); border: 1px solid rgba(255, 255, 255, 0.2); border-radius: 4px; color: white; text-align: center; }
	.alert-rr-display { display: flex; align-items: center; justify-content: center; gap: 1rem; padding: 1rem 1.25rem; background: rgba(0, 0, 0, 0.3); }
	.alert-rr-item { text-align: center; }
	.alert-rr-label { display: block; font-size: 0.625rem; color: rgba(255, 255, 255, 0.5); text-transform: uppercase; margin-bottom: 0.125rem; }
	.alert-rr-value { font-size: 1rem; font-weight: 600; color: white; }
	.alert-rr-item.risk .alert-rr-value { color: #fca5a5; }
	.alert-rr-item.reward .alert-rr-value { color: #6ee7b7; }
	.alert-rr-divider { font-size: 1.25rem; color: rgba(255, 255, 255, 0.5); }
	.alert-rr-ratio { display: flex; flex-direction: column; align-items: center; padding: 0.5rem 1rem; background: rgba(255, 255, 255, 0.1); border-radius: 8px; }
	.alert-rr-ratio-label { font-size: 0.625rem; color: rgba(255, 255, 255, 0.5); text-transform: uppercase; }
	.alert-rr-ratio-value { font-size: 1.25rem; font-weight: 700; color: #fbbf24; }

	.trading-idea-block { border-radius: 16px; overflow: hidden; box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2); }
	.trading-idea-block.idea-long { background: linear-gradient(180deg, #064e3b 0%, #022c22 100%); border: 2px solid #10b981; }
	.trading-idea-block.idea-short { background: linear-gradient(180deg, #7f1d1d 0%, #450a0a 100%); border: 2px solid #ef4444; }
	.idea-header { display: flex; align-items: center; gap: 1rem; padding: 1.25rem; background: rgba(0, 0, 0, 0.2); }
	.idea-direction { display: flex; align-items: center; gap: 0.5rem; padding: 0.5rem 1rem; border-radius: 8px; font-weight: 700; font-size: 0.875rem; }
	.idea-direction.long { background: rgba(16, 185, 129, 0.3); color: #6ee7b7; }
	.idea-direction.short { background: rgba(239, 68, 68, 0.3); color: #fca5a5; }
	.idea-symbol-container { flex: 1; }
	.idea-symbol { font-size: 1.75rem; font-weight: 700; color: white; font-family: monospace; }
	.idea-symbol-input { width: 100px; padding: 0.5rem 0.75rem; background: rgba(255, 255, 255, 0.1); border: 1px solid rgba(255, 255, 255, 0.3); border-radius: 8px; color: white; font-size: 1.25rem; font-weight: 700; font-family: monospace; text-transform: uppercase; }
	.idea-direction-toggle { display: flex; gap: 0.5rem; }
	.idea-direction-btn { display: inline-flex; align-items: center; gap: 0.375rem; padding: 0.5rem 0.75rem; border-radius: 6px; font-size: 0.75rem; font-weight: 600; cursor: pointer; transition: all 0.15s; border: 1px solid transparent; }
	.idea-direction-btn.long { background: rgba(16, 185, 129, 0.2); color: #6ee7b7; }
	.idea-direction-btn.long.active { background: rgba(16, 185, 129, 0.4); border-color: #10b981; }
	.idea-direction-btn.short { background: rgba(239, 68, 68, 0.2); color: #fca5a5; }
	.idea-direction-btn.short.active { background: rgba(239, 68, 68, 0.4); border-color: #ef4444; }
	.idea-chart-placeholder { display: flex; flex-direction: column; align-items: center; justify-content: center; height: 180px; background: rgba(0, 0, 0, 0.3); color: rgba(255, 255, 255, 0.5); gap: 0.5rem; }
	.idea-chart-placeholder small { font-size: 0.75rem; color: rgba(255, 255, 255, 0.3); }
	.idea-levels { display: grid; grid-template-columns: repeat(3, 1fr); gap: 1px; background: rgba(255, 255, 255, 0.1); }
	.idea-level { display: flex; align-items: center; gap: 0.75rem; padding: 1rem; background: rgba(0, 0, 0, 0.2); }
	.idea-level-icon { display: flex; align-items: center; justify-content: center; width: 36px; height: 36px; border-radius: 8px; background: rgba(255, 255, 255, 0.1); }
	.idea-level.entry-level .idea-level-icon { color: #60a5fa; }
	.idea-level.stop-loss-level .idea-level-icon { color: #ef4444; }
	.idea-level.take-profit-level .idea-level-icon { color: #10b981; }
	.idea-level-info { display: flex; flex-direction: column; gap: 0.25rem; }
	.idea-level-label { font-size: 0.75rem; color: rgba(255, 255, 255, 0.6); }
	.idea-level-value { font-size: 1.125rem; font-weight: 600; color: white; }
	.idea-level-input { width: 90px; padding: 0.375rem 0.5rem; background: rgba(255, 255, 255, 0.1); border: 1px solid rgba(255, 255, 255, 0.2); border-radius: 4px; color: white; }
	.idea-rr-section { display: flex; align-items: center; gap: 1.5rem; padding: 1.25rem; background: rgba(0, 0, 0, 0.2); }
	.idea-rr-visual { flex: 1; }
	.idea-rr-bar-container { height: 8px; background: rgba(255, 255, 255, 0.1); border-radius: 4px; display: flex; overflow: hidden; margin-bottom: 0.5rem; }
	.idea-rr-bar { height: 100%; }
	.idea-rr-bar.risk-bar { background: linear-gradient(90deg, #ef4444, #f87171); }
	.idea-rr-bar.reward-bar { background: linear-gradient(90deg, #10b981, #34d399); }
	.idea-rr-labels { display: flex; justify-content: space-between; font-size: 0.75rem; }
	.idea-risk-label { color: #fca5a5; }
	.idea-reward-label { color: #6ee7b7; }
	.idea-rr-ratio-badge { display: flex; flex-direction: column; align-items: center; padding: 0.75rem 1rem; border-radius: 12px; min-width: 70px; }
	.idea-rr-ratio-badge.good { background: linear-gradient(135deg, rgba(16, 185, 129, 0.3), rgba(16, 185, 129, 0.1)); border: 1px solid #10b981; }
	.idea-rr-ratio-badge.medium { background: linear-gradient(135deg, rgba(251, 191, 36, 0.3), rgba(251, 191, 36, 0.1)); border: 1px solid #fbbf24; }
	.idea-rr-ratio-badge.poor { background: linear-gradient(135deg, rgba(239, 68, 68, 0.3), rgba(239, 68, 68, 0.1)); border: 1px solid #ef4444; }
	.idea-ratio-value { font-size: 1.5rem; font-weight: 700; color: white; }
	.idea-ratio-label { font-size: 0.625rem; color: rgba(255, 255, 255, 0.6); text-transform: uppercase; }
	.idea-confidence { padding: 1.25rem; }
	.idea-confidence-header { display: flex; align-items: center; gap: 0.5rem; margin-bottom: 0.75rem; color: rgba(255, 255, 255, 0.8); }
	.idea-confidence-label { flex: 1; font-size: 0.875rem; }
	.idea-confidence-value { font-weight: 600; color: white; }
	.idea-confidence-slider { width: 100%; height: 8px; appearance: none; background: rgba(255, 255, 255, 0.1); border-radius: 4px; cursor: pointer; }
	.idea-confidence-slider::-webkit-slider-thumb { appearance: none; width: 20px; height: 20px; background: #fbbf24; border-radius: 50%; cursor: pointer; }
	.idea-confidence-bar-container { height: 8px; background: rgba(255, 255, 255, 0.1); border-radius: 4px; overflow: hidden; }
	.idea-confidence-bar { height: 100%; border-radius: 4px; transition: width 0.3s ease; }
	.idea-confidence-bar.low { background: linear-gradient(90deg, #ef4444, #f87171); }
	.idea-confidence-bar.medium { background: linear-gradient(90deg, #fbbf24, #fcd34d); }
	.idea-confidence-bar.high { background: linear-gradient(90deg, #10b981, #34d399); }
	.idea-footer { display: flex; justify-content: space-between; align-items: center; padding: 1rem 1.25rem; background: rgba(0, 0, 0, 0.3); border-top: 1px solid rgba(255, 255, 255, 0.1); }
	.idea-timestamp { display: flex; align-items: center; gap: 0.5rem; font-size: 0.8125rem; color: rgba(255, 255, 255, 0.6); }
	.idea-actions { display: flex; align-items: center; gap: 0.375rem; color: rgba(255, 255, 255, 0.5); font-size: 0.8125rem; }
	@media (max-width: 640px) { .ticker-list { flex-direction: column; } .ticker-item { min-width: 100%; } .alert-levels { flex-direction: column; gap: 1rem; } .idea-levels { grid-template-columns: 1fr; } .idea-rr-section { flex-direction: column; } .idea-header { flex-wrap: wrap; } }

	/* ========================================================================== */
	/* ADVANCED BLOCK STYLES                                                       */
	/* ========================================================================== */

	.edit-field { display: flex; flex-direction: column; gap: 0.25rem; }
	.edit-field span { font-size: 0.75rem; font-weight: 500; color: #6b7280; text-transform: uppercase; letter-spacing: 0.025em; }
	.edit-field input, .edit-field select { padding: 0.5rem 0.75rem; border: 1px solid #e5e7eb; border-radius: 6px; font-size: 0.875rem; background: white; transition: border-color 0.15s; }
	.edit-field input:focus, .edit-field select:focus { outline: none; border-color: #3b82f6; }

	/* Card Block */
	.card-block { background: white; border-radius: 16px; overflow: hidden; box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1), 0 1px 2px rgba(0, 0, 0, 0.06); transition: all 0.3s ease; }
	.card-block:hover { box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1), 0 4px 10px rgba(0, 0, 0, 0.05); transform: translateY(-2px); }
	.card-edit-fields { padding: 1rem; background: #f8fafc; border-bottom: 1px solid #e5e7eb; display: flex; flex-direction: column; gap: 0.75rem; }
	.card-image { aspect-ratio: 16/9; overflow: hidden; }
	.card-image img { width: 100%; height: 100%; object-fit: cover; transition: transform 0.3s ease; }
	.card-block:hover .card-image img { transform: scale(1.02); }
	.card-image-placeholder { aspect-ratio: 16/9; display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 0.5rem; background: #f3f4f6; color: #9ca3af; }
	.card-body { padding: 1.5rem; }
	.card-title { margin: 0 0 0.75rem; font-size: 1.25rem; font-weight: 600; color: #1f2937; line-height: 1.3; }
	.card-description { margin: 0 0 1rem; font-size: 0.9375rem; color: #6b7280; line-height: 1.6; }
	.card-cta { display: inline-flex; align-items: center; gap: 0.375rem; color: #3b82f6; font-weight: 500; font-size: 0.9375rem; text-decoration: none; transition: gap 0.2s ease; }
	.card-cta:hover { gap: 0.625rem; }

	/* Testimonial Block */
	.testimonial-block { position: relative; margin: 0; padding: 2.5rem; background: linear-gradient(135deg, #f8fafc 0%, #f1f5f9 100%); border-radius: 16px; text-align: center; }
	.testimonial-edit-fields { position: absolute; top: 0.75rem; right: 0.75rem; background: white; padding: 0.75rem; border-radius: 8px; box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1); z-index: 10; }
	.testimonial-quote-icon { display: flex; justify-content: center; margin-bottom: 1.25rem; color: #3b82f6; opacity: 0.5; }
	.testimonial-quote { margin: 0 0 1.5rem; font-size: 1.25rem; font-style: italic; color: #374151; line-height: 1.7; max-width: 640px; margin-left: auto; margin-right: auto; }
	.testimonial-author { display: flex; align-items: center; justify-content: center; gap: 1rem; }
	.testimonial-avatar { width: 56px; height: 56px; border-radius: 50%; overflow: hidden; background: #e5e7eb; display: flex; align-items: center; justify-content: center; color: #9ca3af; flex-shrink: 0; }
	.testimonial-avatar img { width: 100%; height: 100%; object-fit: cover; }
	.testimonial-info { display: flex; flex-direction: column; text-align: left; }
	.testimonial-name { font-weight: 600; color: #1f2937; font-size: 1rem; }
	.testimonial-title { font-size: 0.875rem; color: #6b7280; }

	/* CTA Block */
	.cta-block { padding: 3rem 2rem; background: #f8fafc; border-radius: 16px; text-align: center; position: relative; }
	.cta-block.cta-dark { background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%); }
	.cta-block.cta-dark .cta-heading, .cta-block.cta-dark .cta-description { color: white; }
	.cta-block.cta-gradient { background: linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%); }
	.cta-block.cta-gradient .cta-heading, .cta-block.cta-gradient .cta-description { color: white; }
	.cta-edit-fields { position: absolute; top: 0.75rem; right: 0.75rem; background: white; padding: 1rem; border-radius: 8px; box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1); z-index: 10; display: flex; flex-direction: column; gap: 0.75rem; min-width: 200px; }
	.cta-heading { margin: 0 0 1rem; font-size: 1.875rem; font-weight: 700; color: #1f2937; line-height: 1.2; }
	.cta-description { margin: 0 0 2rem; font-size: 1.125rem; color: #6b7280; line-height: 1.6; max-width: 540px; margin-left: auto; margin-right: auto; }
	.cta-buttons { display: flex; gap: 1rem; justify-content: center; flex-wrap: wrap; }
	.cta-btn { display: inline-flex; align-items: center; justify-content: center; padding: 0.875rem 2rem; border-radius: 10px; font-weight: 600; font-size: 1rem; text-decoration: none; transition: all 0.2s ease; }
	.cta-btn-primary { background: #3b82f6; color: white; }
	.cta-btn-primary:hover { background: #2563eb; transform: translateY(-1px); }
	.cta-block.cta-dark .cta-btn-primary, .cta-block.cta-gradient .cta-btn-primary { background: white; color: #1f2937; }
	.cta-btn-secondary { background: transparent; color: #374151; border: 2px solid #e5e7eb; }
	.cta-btn-secondary:hover { background: #f3f4f6; }
	.cta-block.cta-dark .cta-btn-secondary, .cta-block.cta-gradient .cta-btn-secondary { border-color: rgba(255, 255, 255, 0.3); color: white; }
	.cta-block.cta-dark .cta-btn-secondary:hover, .cta-block.cta-gradient .cta-btn-secondary:hover { background: rgba(255, 255, 255, 0.1); }

	/* Countdown Block */
	.countdown-block { padding: 2rem; background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%); border-radius: 16px; text-align: center; position: relative; }
	.countdown-edit-fields { position: absolute; top: 0.75rem; right: 0.75rem; background: white; padding: 1rem; border-radius: 8px; box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2); z-index: 10; display: flex; flex-direction: column; gap: 0.75rem; min-width: 200px; }
	.countdown-label { margin: 0 0 1.5rem; font-size: 1.125rem; font-weight: 500; color: #94a3b8; text-transform: uppercase; letter-spacing: 0.1em; }
	.countdown-timer { display: flex; align-items: center; justify-content: center; gap: 0.5rem; flex-wrap: wrap; }
	.countdown-unit { display: flex; flex-direction: column; align-items: center; padding: 1rem 1.5rem; background: rgba(255, 255, 255, 0.05); border-radius: 12px; min-width: 80px; }
	.countdown-value { font-size: 2.5rem; font-weight: 700; color: white; line-height: 1; font-variant-numeric: tabular-nums; }
	.countdown-label-unit { margin-top: 0.5rem; font-size: 0.75rem; font-weight: 500; color: #94a3b8; text-transform: uppercase; letter-spacing: 0.05em; }
	.countdown-separator { font-size: 2rem; font-weight: 700; color: #475569; }

	/* Social Share Block */
	.social-share-block { display: flex; align-items: center; gap: 1rem; padding: 1rem 0; flex-wrap: wrap; }
	.social-share-label { font-size: 0.875rem; font-weight: 500; color: #6b7280; }
	.social-share-buttons { display: flex; gap: 0.5rem; }
	.social-btn { display: flex; align-items: center; justify-content: center; width: 40px; height: 40px; border-radius: 10px; border: 1px solid #e5e7eb; background: white; color: #6b7280; cursor: pointer; transition: all 0.2s ease; }
	.social-btn:hover { transform: translateY(-2px); }
	.social-btn-twitter:hover { background: #1da1f2; border-color: #1da1f2; color: white; }
	.social-btn-facebook:hover { background: #1877f2; border-color: #1877f2; color: white; }
	.social-btn-linkedin:hover { background: #0a66c2; border-color: #0a66c2; color: white; }
	.social-btn-email:hover { background: #ea4335; border-color: #ea4335; color: white; }
	.social-btn-copy:hover { background: #6b7280; border-color: #6b7280; color: white; }
	.social-btn-copy.copied { background: #10b981; border-color: #10b981; color: white; }

	/* Author Block */
	.author-block { display: flex; gap: 1.5rem; padding: 1.5rem; background: #f8fafc; border-radius: 16px; position: relative; }
	.author-edit-fields { position: absolute; top: 0.75rem; right: 0.75rem; background: white; padding: 1rem; border-radius: 8px; box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1); z-index: 10; display: flex; flex-direction: column; gap: 0.75rem; min-width: 200px; }
	.author-avatar { width: 80px; height: 80px; border-radius: 50%; overflow: hidden; background: #e5e7eb; display: flex; align-items: center; justify-content: center; color: #9ca3af; flex-shrink: 0; }
	.author-avatar img { width: 100%; height: 100%; object-fit: cover; }
	.author-info { flex: 1; }
	.author-label { font-size: 0.75rem; font-weight: 500; color: #6b7280; text-transform: uppercase; letter-spacing: 0.05em; }
	.author-name { margin: 0.25rem 0 0.5rem; font-size: 1.25rem; font-weight: 600; color: #1f2937; }
	.author-bio { margin: 0 0 0.75rem; font-size: 0.9375rem; color: #6b7280; line-height: 1.6; }
	.author-social { display: flex; gap: 0.75rem; }
	.author-social-link { display: flex; align-items: center; justify-content: center; width: 36px; height: 36px; border-radius: 8px; background: white; color: #6b7280; transition: all 0.2s ease; }
	.author-social-link:hover { background: #3b82f6; color: white; }

	/* Related Posts Block */
	.related-posts-block { padding: 2rem 0; }
	.related-posts-heading { margin: 0 0 1.5rem; font-size: 1.5rem; font-weight: 600; color: #1f2937; }
	.related-posts-grid { display: grid; grid-template-columns: repeat(3, 1fr); gap: 1.5rem; }
	.related-post-card { background: white; border-radius: 12px; overflow: hidden; box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1), 0 1px 2px rgba(0, 0, 0, 0.06); transition: all 0.2s ease; }
	.related-post-card:hover { box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1), 0 2px 4px rgba(0, 0, 0, 0.05); transform: translateY(-2px); }
	.related-post-image { aspect-ratio: 16/9; background: #f3f4f6; display: flex; align-items: center; justify-content: center; color: #9ca3af; }
	.related-post-content { padding: 1rem; }
	.related-post-category { font-size: 0.75rem; font-weight: 500; color: #3b82f6; text-transform: uppercase; letter-spacing: 0.05em; }
	.related-post-title { margin: 0.375rem 0; font-size: 1rem; font-weight: 600; color: #1f2937; line-height: 1.3; }
	.related-post-excerpt { margin: 0; font-size: 0.875rem; color: #6b7280; line-height: 1.5; display: -webkit-box; -webkit-line-clamp: 2; -webkit-box-orient: vertical; overflow: hidden; }
	.related-posts-note { display: flex; align-items: center; gap: 0.5rem; margin-top: 1rem; padding: 0.75rem 1rem; background: #fef3c7; border-radius: 8px; font-size: 0.875rem; color: #92400e; }

	/* Newsletter Block */
	.newsletter-block { padding: 2.5rem; background: linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%); border-radius: 16px; text-align: center; position: relative; }
	.newsletter-edit-fields { position: absolute; top: 0.75rem; right: 0.75rem; background: white; padding: 1rem; border-radius: 8px; box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2); z-index: 10; min-width: 200px; }
	.newsletter-content { margin-bottom: 1.5rem; }
	.newsletter-heading { margin: 0 0 0.75rem; font-size: 1.5rem; font-weight: 700; color: white; }
	.newsletter-description { margin: 0; font-size: 1rem; color: rgba(255, 255, 255, 0.9); max-width: 400px; margin-left: auto; margin-right: auto; }
	.newsletter-form { max-width: 440px; margin: 0 auto; }
	.newsletter-input-group { display: flex; gap: 0.5rem; }
	.newsletter-input-group input { flex: 1; padding: 0.875rem 1rem; border: 2px solid transparent; border-radius: 10px; font-size: 1rem; background: white; transition: border-color 0.15s; }
	.newsletter-input-group input:focus { outline: none; border-color: #1d4ed8; }
	.newsletter-input-group button { padding: 0.875rem 1.5rem; background: #1e293b; color: white; border: none; border-radius: 10px; font-weight: 600; font-size: 1rem; cursor: pointer; transition: all 0.2s ease; white-space: nowrap; }
	.newsletter-input-group button:hover:not(:disabled) { background: #0f172a; }
	.newsletter-input-group button:disabled { opacity: 0.6; cursor: not-allowed; }
	.newsletter-privacy { margin: 1rem 0 0; font-size: 0.8125rem; color: rgba(255, 255, 255, 0.7); }
	.newsletter-success { display: flex; align-items: center; justify-content: center; gap: 0.5rem; padding: 1rem; background: rgba(255, 255, 255, 0.15); border-radius: 10px; color: white; font-weight: 500; }

	/* Spinner */
	:global(.spin) { animation: spin 1s linear infinite; }
	@keyframes spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }

	/* Responsive for Advanced Blocks */
	@media (max-width: 768px) {
		.card-body { padding: 1.25rem; }
		.testimonial-block { padding: 1.5rem; }
		.testimonial-quote { font-size: 1.125rem; }
		.testimonial-author { flex-direction: column; text-align: center; }
		.testimonial-info { text-align: center; }
		.cta-block { padding: 2rem 1.5rem; }
		.cta-heading { font-size: 1.5rem; }
		.cta-description { font-size: 1rem; }
		.cta-edit-fields, .countdown-edit-fields, .author-edit-fields, .newsletter-edit-fields { position: relative; top: auto; right: auto; margin-bottom: 1rem; }
		.countdown-unit { min-width: 60px; padding: 0.75rem 1rem; }
		.countdown-value { font-size: 1.75rem; }
		.social-share-block { flex-direction: column; align-items: flex-start; }
		.author-block { flex-direction: column; text-align: center; }
		.author-avatar { margin: 0 auto; }
		.author-social { justify-content: center; }
		.related-posts-grid { grid-template-columns: 1fr; }
		.newsletter-block { padding: 1.5rem; }
		.newsletter-input-group { flex-direction: column; }
		.newsletter-input-group button { width: 100%; }
	}

	/* Dark Mode for Advanced Blocks */
	:global(.dark) .card-block { background: #1e293b; }
	:global(.dark) .card-title { color: #f1f5f9; }
	:global(.dark) .card-description { color: #94a3b8; }
	:global(.dark) .card-image-placeholder { background: #334155; }
	:global(.dark) .testimonial-block { background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%); }
	:global(.dark) .testimonial-quote { color: #e2e8f0; }
	:global(.dark) .testimonial-name { color: #f1f5f9; }
	:global(.dark) .testimonial-title { color: #94a3b8; }
	:global(.dark) .testimonial-avatar { background: #334155; }
	:global(.dark) .cta-block { background: #1e293b; }
	:global(.dark) .cta-block .cta-heading { color: #f1f5f9; }
	:global(.dark) .cta-block .cta-description { color: #94a3b8; }
	:global(.dark) .cta-btn-secondary { border-color: #475569; color: #e2e8f0; }
	:global(.dark) .cta-btn-secondary:hover { background: #334155; }
	:global(.dark) .social-btn { background: #1e293b; border-color: #334155; color: #94a3b8; }
	:global(.dark) .author-block { background: #1e293b; }
	:global(.dark) .author-name { color: #f1f5f9; }
	:global(.dark) .author-bio { color: #94a3b8; }
	:global(.dark) .author-avatar { background: #334155; }
	:global(.dark) .author-social-link { background: #334155; color: #94a3b8; }
	:global(.dark) .related-posts-heading { color: #f1f5f9; }
	:global(.dark) .related-post-card { background: #1e293b; }
	:global(.dark) .related-post-image { background: #334155; }
	:global(.dark) .related-post-title { color: #f1f5f9; }
	:global(.dark) .related-post-excerpt { color: #94a3b8; }
	:global(.dark) .related-posts-note { background: #1e293b; color: #fcd34d; }
	:global(.dark) .edit-field input, :global(.dark) .edit-field select { background: #1e293b; border-color: #334155; color: #e2e8f0; }

	/* ===== INTERACTIVE BLOCK STYLES ===== */

	/* Accordion Block */
	.accordion-block { border: 1px solid #e5e7eb; border-radius: 8px; overflow: hidden; }
	.accordion-item { border-bottom: 1px solid #e5e7eb; }
	.accordion-item:last-child { border-bottom: none; }
	.accordion-header { display: flex; align-items: center; justify-content: space-between; width: 100%; padding: 1rem 1.25rem; background: #f9fafb; border: none; cursor: pointer; font-size: 1rem; font-weight: 500; color: #1f2937; text-align: left; transition: background 0.15s; }
	.accordion-header:hover { background: #f3f4f6; }
	.accordion-header:focus-visible { outline: 2px solid #3b82f6; outline-offset: -2px; }
	.accordion-title { flex: 1; }
	.accordion-icon { display: flex; color: #6b7280; transition: transform 0.2s; }
	.accordion-icon :global(.rotated) { transform: rotate(180deg); }
	.accordion-panel { overflow: hidden; animation: accordion-expand 0.2s ease-out; }
	.accordion-panel[hidden] { display: none; }
	.accordion-content { padding: 1rem 1.25rem; background: white; line-height: 1.6; }
	@keyframes accordion-expand { from { opacity: 0; max-height: 0; } to { opacity: 1; max-height: 500px; } }

	/* Tabs Block */
	.tabs-block { border: 1px solid #e5e7eb; border-radius: 8px; overflow: hidden; }
	.tabs-horizontal .tabs-list { display: flex; overflow-x: auto; scrollbar-width: thin; background: #f9fafb; border-bottom: 1px solid #e5e7eb; }
	.tabs-vertical { display: flex; }
	.tabs-vertical .tabs-list { display: flex; flex-direction: column; border-right: 1px solid #e5e7eb; border-bottom: none; min-width: 160px; background: #f9fafb; }
	.tabs-vertical .tabs-panels { flex: 1; }
	.tab-button { display: flex; align-items: center; padding: 0.75rem 1.25rem; background: transparent; border: none; border-bottom: 2px solid transparent; cursor: pointer; font-size: 0.9375rem; font-weight: 500; color: #6b7280; white-space: nowrap; transition: all 0.15s; }
	.tabs-vertical .tab-button { border-bottom: none; border-left: 2px solid transparent; }
	.tab-button:hover { color: #3b82f6; background: rgba(59, 130, 246, 0.05); }
	.tab-button:focus-visible { outline: 2px solid #3b82f6; outline-offset: -2px; }
	.tab-button.active { color: #3b82f6; border-bottom-color: #3b82f6; background: white; }
	.tabs-vertical .tab-button.active { border-left-color: #3b82f6; border-bottom-color: transparent; }
	.tab-add-btn { display: flex; align-items: center; justify-content: center; padding: 0.75rem; background: transparent; border: none; color: #9ca3af; cursor: pointer; transition: color 0.15s; }
	.tab-add-btn:hover { color: #3b82f6; }
	.tab-panel { padding: 1.25rem; background: white; animation: tab-fade 0.2s ease-out; }
	.tab-panel[hidden] { display: none; }
	.tab-content { line-height: 1.6; }
	@keyframes tab-fade { from { opacity: 0; transform: translateY(4px); } to { opacity: 1; transform: translateY(0); } }

	/* Toggle Block */
	.toggle-block { border: 1px solid #e5e7eb; border-radius: 8px; overflow: hidden; }
	.toggle-header { display: flex; align-items: center; gap: 0.75rem; width: 100%; padding: 1rem 1.25rem; background: #f9fafb; border: none; cursor: pointer; font-size: 1rem; font-weight: 500; color: #1f2937; text-align: left; transition: background 0.15s; }
	.toggle-header:hover { background: #f3f4f6; }
	.toggle-header:focus-visible { outline: 2px solid #3b82f6; outline-offset: -2px; }
	.toggle-icon { display: flex; color: #6b7280; transition: transform 0.2s; }
	.toggle-label { flex: 1; }
	.toggle-content { overflow: hidden; animation: toggle-expand 0.2s ease-out; }
	.toggle-content[hidden] { display: none; }
	.toggle-inner { padding: 1rem 1.25rem; background: white; line-height: 1.6; }
	@keyframes toggle-expand { from { opacity: 0; max-height: 0; } to { opacity: 1; max-height: 1000px; } }

	/* Table of Contents Block */
	.toc-block { border: 1px solid #e5e7eb; border-radius: 8px; overflow: hidden; background: #f9fafb; }
	.toc-header { display: flex; align-items: center; gap: 0.75rem; width: 100%; padding: 1rem 1.25rem; background: transparent; border: none; cursor: pointer; font-size: 1rem; font-weight: 600; color: #1f2937; }
	.toc-header:focus-visible { outline: 2px solid #3b82f6; outline-offset: -2px; }
	.toc-title { flex: 1; text-align: left; }
	.toc-toggle-icon { display: none; color: #6b7280; transition: transform 0.2s; }
	.toc-toggle-icon :global(.collapsed) { transform: rotate(-90deg); }
	.toc-list { margin: 0; padding: 0 1.25rem 1rem; list-style: none; }
	.toc-list.collapsed { display: none; }
	.toc-item { padding: 0.375rem 0; }
	.toc-level-3 { padding-left: 1rem; }
	.toc-level-4 { padding-left: 2rem; }
	.toc-level-5 { padding-left: 3rem; }
	.toc-level-6 { padding-left: 4rem; }
	.toc-link { display: block; color: #4b5563; text-decoration: none; font-size: 0.9375rem; transition: color 0.15s; }
	.toc-link:hover { color: #3b82f6; }
	.toc-item.active .toc-link { color: #3b82f6; font-weight: 500; }

	/* Buttons Group Block */
	.buttons-block { display: flex; flex-wrap: wrap; }
	.buttons-column { flex-direction: column; align-items: stretch; }
	.btn { display: inline-flex; align-items: center; justify-content: center; gap: 0.5rem; padding: 0.75rem 1.5rem; border-radius: 8px; font-size: 1rem; font-weight: 500; text-decoration: none; transition: all 0.15s; cursor: pointer; border: 2px solid transparent; }
	.btn:focus-visible { outline: 2px solid #3b82f6; outline-offset: 2px; }
	.btn-primary { background: #3b82f6; color: white; border-color: #3b82f6; }
	.btn-primary:hover { background: #2563eb; border-color: #2563eb; }
	.btn-secondary { background: #f3f4f6; color: #1f2937; border-color: #e5e7eb; }
	.btn-secondary:hover { background: #e5e7eb; }
	.btn-outline { background: transparent; color: #3b82f6; border-color: #3b82f6; }
	.btn-outline:hover { background: rgba(59, 130, 246, 0.1); }
	.btn-ghost { background: transparent; color: #3b82f6; border-color: transparent; }
	.btn-ghost:hover { background: rgba(59, 130, 246, 0.1); }
	.btn-icon { display: flex; }
	.btn-add { display: inline-flex; align-items: center; gap: 0.375rem; padding: 0.5rem 1rem; background: transparent; border: 1px dashed #d1d5db; border-radius: 6px; color: #6b7280; font-size: 0.875rem; cursor: pointer; transition: all 0.15s; }
	.btn-add:hover { border-color: #3b82f6; color: #3b82f6; }

	/* Interactive blocks responsive */
	@media (max-width: 768px) {
		.toc-toggle-icon { display: flex; }
		.toc-header { border-bottom: 1px solid #e5e7eb; }
		.tabs-horizontal .tabs-list { -webkit-overflow-scrolling: touch; }
		.tabs-vertical { flex-direction: column; }
		.tabs-vertical .tabs-list { flex-direction: row; overflow-x: auto; border-right: none; border-bottom: 1px solid #e5e7eb; min-width: auto; }
		.tabs-vertical .tab-button { border-left: none; border-bottom: 2px solid transparent; }
		.tabs-vertical .tab-button.active { border-left-color: transparent; border-bottom-color: #3b82f6; }
		.buttons-block { flex-direction: column; }
		.btn { width: 100%; }
	}

	/* Dark mode for interactive blocks */
	:global(.dark) .accordion-block { border-color: #374151; }
	:global(.dark) .accordion-item { border-color: #374151; }
	:global(.dark) .accordion-header { background: #1f2937; color: #f9fafb; }
	:global(.dark) .accordion-header:hover { background: #374151; }
	:global(.dark) .accordion-content { background: #111827; color: #e5e7eb; }
	:global(.dark) .tabs-block { border-color: #374151; }
	:global(.dark) .tabs-list { background: #1f2937; border-color: #374151; }
	:global(.dark) .tab-button { color: #9ca3af; }
	:global(.dark) .tab-button.active { background: #111827; color: #60a5fa; }
	:global(.dark) .tab-panel { background: #111827; color: #e5e7eb; }
	:global(.dark) .toggle-block { border-color: #374151; }
	:global(.dark) .toggle-header { background: #1f2937; color: #f9fafb; }
	:global(.dark) .toggle-header:hover { background: #374151; }
	:global(.dark) .toggle-inner { background: #111827; color: #e5e7eb; }
	:global(.dark) .toc-block { background: #1f2937; border-color: #374151; }
	:global(.dark) .toc-header { color: #f9fafb; }
	:global(.dark) .toc-link { color: #9ca3af; }
	:global(.dark) .toc-link:hover { color: #60a5fa; }
	:global(.dark) .toc-item.active .toc-link { color: #60a5fa; }

	/* ========================================================================== */
	/* MEDIA BLOCK STYLES                                                          */
	/* ========================================================================== */

	/* Gallery Block */
	.gallery-block {
		width: 100%;
	}

	.gallery-grid {
		display: grid;
		grid-template-columns: repeat(var(--gallery-columns, 3), 1fr);
		gap: 0.75rem;
	}

	.gallery-item {
		position: relative;
		border-radius: 8px;
		overflow: hidden;
	}

	.gallery-image-btn {
		display: block;
		width: 100%;
		padding: 0;
		border: none;
		background: none;
		cursor: pointer;
		position: relative;
	}

	.gallery-image-btn img {
		display: block;
		width: 100%;
		height: auto;
		aspect-ratio: 1;
		object-fit: cover;
		border-radius: 8px;
		transition: transform 0.3s ease;
	}

	.gallery-image-btn:hover img {
		transform: scale(1.02);
	}

	.gallery-overlay {
		position: absolute;
		inset: 0;
		background: rgba(0, 0, 0, 0.4);
		display: flex;
		align-items: center;
		justify-content: center;
		opacity: 0;
		transition: opacity 0.2s ease;
		color: white;
		border-radius: 8px;
	}

	.gallery-image-btn:hover .gallery-overlay,
	.gallery-image-btn:focus .gallery-overlay {
		opacity: 1;
	}

	.gallery-item-controls {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		padding: 0.5rem;
		background: #f8fafc;
		border-radius: 0 0 8px 8px;
	}

	.gallery-url-input,
	.gallery-alt-input {
		width: 100%;
		padding: 0.5rem;
		border: 1px solid #e5e7eb;
		border-radius: 6px;
		font-size: 0.875rem;
	}

	.gallery-remove-btn {
		position: absolute;
		top: 0.5rem;
		right: 0.5rem;
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(239, 68, 68, 0.9);
		border: none;
		border-radius: 50%;
		color: white;
		cursor: pointer;
		z-index: 5;
	}

	.gallery-caption {
		margin: 0.5rem 0 0;
		font-size: 0.875rem;
		color: #6b7280;
		text-align: center;
	}

	.gallery-placeholder {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem;
		background: #f9fafb;
		border: 2px dashed #d1d5db;
		border-radius: 8px;
		color: #9ca3af;
		cursor: pointer;
	}

	.gallery-placeholder span {
		margin-top: 0.75rem;
	}

	.gallery-add-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.75rem 1rem;
		margin-top: 1rem;
		background: #3b82f6;
		border: none;
		border-radius: 8px;
		color: white;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: background 0.15s;
		min-height: 44px;
	}

	.gallery-add-btn:hover {
		background: #2563eb;
	}

	/* Lightbox */
	.lightbox-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.95);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 9999;
		padding: 2rem;
	}

	.lightbox-content {
		position: relative;
		max-width: 90vw;
		max-height: 90vh;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.lightbox-image {
		max-width: 100%;
		max-height: 85vh;
		object-fit: contain;
		border-radius: 4px;
	}

	.lightbox-close {
		position: absolute;
		top: -3rem;
		right: 0;
		width: 44px;
		height: 44px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(255, 255, 255, 0.1);
		border: none;
		border-radius: 50%;
		color: white;
		cursor: pointer;
		transition: background 0.15s;
	}

	.lightbox-close:hover {
		background: rgba(255, 255, 255, 0.2);
	}

	.lightbox-nav {
		position: absolute;
		top: 50%;
		transform: translateY(-50%);
		width: 44px;
		height: 44px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(255, 255, 255, 0.1);
		border: none;
		border-radius: 50%;
		color: white;
		cursor: pointer;
		transition: background 0.15s;
	}

	.lightbox-prev {
		left: -4rem;
	}

	.lightbox-next {
		right: -4rem;
	}

	.lightbox-nav:hover {
		background: rgba(255, 255, 255, 0.2);
	}

	.lightbox-counter {
		position: absolute;
		bottom: -2.5rem;
		left: 50%;
		transform: translateX(-50%);
		color: rgba(255, 255, 255, 0.7);
		font-size: 0.875rem;
	}

	/* Audio Block */
	.audio-block {
		width: 100%;
	}

	.audio-player {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%);
		border-radius: 12px;
	}

	.audio-play-btn {
		width: 44px;
		height: 44px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #3b82f6;
		border: none;
		border-radius: 50%;
		color: white;
		cursor: pointer;
		flex-shrink: 0;
		transition: background 0.15s, transform 0.15s;
	}

	.audio-play-btn:hover {
		background: #2563eb;
		transform: scale(1.05);
	}

	.audio-waveform {
		color: #64748b;
		flex-shrink: 0;
	}

	.audio-progress-container {
		flex: 1;
		height: 8px;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 4px;
		cursor: pointer;
		overflow: hidden;
	}

	.audio-progress-bar {
		height: 100%;
		background: linear-gradient(90deg, #3b82f6, #60a5fa);
		border-radius: 4px;
		transition: width 0.1s;
	}

	.audio-time {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-size: 0.75rem;
		color: #94a3b8;
		font-variant-numeric: tabular-nums;
		min-width: 70px;
	}

	.audio-volume-container {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.audio-mute-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: none;
		color: #94a3b8;
		cursor: pointer;
		border-radius: 6px;
		transition: color 0.15s, background 0.15s;
	}

	.audio-mute-btn:hover {
		color: white;
		background: rgba(255, 255, 255, 0.1);
	}

	.audio-volume-slider {
		width: 80px;
		height: 4px;
		appearance: none;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 2px;
		cursor: pointer;
	}

	.audio-volume-slider::-webkit-slider-thumb {
		appearance: none;
		width: 14px;
		height: 14px;
		background: white;
		border-radius: 50%;
		cursor: pointer;
	}

	.audio-caption {
		margin: 0.75rem 0 0;
		font-size: 0.875rem;
		color: #6b7280;
	}

	.audio-placeholder {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
		padding: 3rem;
		background: #f9fafb;
		border: 2px dashed #d1d5db;
		border-radius: 12px;
		color: #9ca3af;
	}

	.audio-placeholder input[type="url"] {
		width: 100%;
		max-width: 400px;
		padding: 0.75rem 1rem;
		border: 1px solid #d1d5db;
		border-radius: 8px;
		font-size: 0.875rem;
	}

	.audio-placeholder input[type="file"] {
		padding: 0.5rem;
	}

	.audio-upload-divider {
		color: #9ca3af;
		font-size: 0.875rem;
	}

	/* File Block */
	.file-block {
		width: 100%;
	}

	.file-download-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem 1.25rem;
		background: #f8fafc;
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		text-decoration: none;
		color: inherit;
		transition: all 0.2s ease;
	}

	.file-download-card:hover {
		background: #f1f5f9;
		border-color: #d1d5db;
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
	}

	.file-icon {
		width: 48px;
		height: 48px;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 10px;
		color: white;
		flex-shrink: 0;
	}

	.file-icon-pdf { background: linear-gradient(135deg, #ef4444, #dc2626); }
	.file-icon-doc { background: linear-gradient(135deg, #3b82f6, #2563eb); }
	.file-icon-xls { background: linear-gradient(135deg, #22c55e, #16a34a); }
	.file-icon-zip { background: linear-gradient(135deg, #f59e0b, #d97706); }
	.file-icon-img { background: linear-gradient(135deg, #8b5cf6, #7c3aed); }
	.file-icon-other { background: linear-gradient(135deg, #6b7280, #4b5563); }

	.file-info {
		flex: 1;
		min-width: 0;
	}

	.file-name {
		display: block;
		font-weight: 600;
		color: #1f2937;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.file-size {
		display: block;
		font-size: 0.8125rem;
		color: #6b7280;
		margin-top: 0.125rem;
	}

	.file-download-icon {
		width: 44px;
		height: 44px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #3b82f6;
		border-radius: 10px;
		color: white;
		flex-shrink: 0;
		transition: background 0.15s;
	}

	.file-download-card:hover .file-download-icon {
		background: #2563eb;
	}

	.file-edit-controls {
		margin-top: 0.75rem;
	}

	.file-name-input {
		width: 100%;
		padding: 0.5rem 0.75rem;
		border: 1px solid #e5e7eb;
		border-radius: 6px;
		font-size: 0.875rem;
	}

	.file-placeholder {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
		padding: 3rem;
		background: #f9fafb;
		border: 2px dashed #d1d5db;
		border-radius: 12px;
		color: #9ca3af;
	}

	.file-placeholder input[type="url"] {
		width: 100%;
		max-width: 400px;
		padding: 0.75rem 1rem;
		border: 1px solid #d1d5db;
		border-radius: 8px;
		font-size: 0.875rem;
	}

	.file-placeholder input[type="file"] {
		padding: 0.5rem;
	}

	.file-upload-divider {
		color: #9ca3af;
		font-size: 0.875rem;
	}

	/* Embed Block */
	.embed-block {
		width: 100%;
	}

	.embed-wrapper {
		border-radius: 12px;
		overflow: hidden;
	}

	.embed-wrapper iframe {
		display: block;
		border-radius: 12px;
	}

	.embed-twitter,
	.embed-instagram,
	.embed-tiktok {
		min-height: 400px;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		background: #f8fafc;
		border-radius: 12px;
		padding: 1.5rem;
	}

	.embed-fallback {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1rem;
		color: #6b7280;
	}

	.embed-fallback a {
		color: #3b82f6;
		text-decoration: none;
		font-weight: 500;
	}

	.embed-fallback a:hover {
		text-decoration: underline;
	}

	.embed-placeholder {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
		padding: 3rem;
		background: #f9fafb;
		border: 2px dashed #d1d5db;
		border-radius: 12px;
		color: #9ca3af;
	}

	.embed-placeholder small {
		font-size: 0.8125rem;
		color: #9ca3af;
	}

	.embed-placeholder input[type="url"] {
		width: 100%;
		max-width: 400px;
		padding: 0.75rem 1rem;
		border: 1px solid #d1d5db;
		border-radius: 8px;
		font-size: 0.875rem;
	}

	.embed-platform-icons {
		display: flex;
		gap: 1rem;
		margin-top: 0.5rem;
		color: #9ca3af;
	}

	.embed-error {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 2rem;
		background: #fef2f2;
		border: 1px solid #fecaca;
		border-radius: 12px;
		color: #991b1b;
	}

	/* GIF Block */
	.gif-block {
		width: 100%;
	}

	.gif-container {
		position: relative;
		display: inline-block;
		border-radius: 12px;
		overflow: hidden;
	}

	.gif-image {
		display: block;
		max-width: 100%;
		height: auto;
		border-radius: 12px;
	}

	.gif-image.paused {
		filter: grayscale(50%);
	}

	.gif-play-toggle {
		position: absolute;
		bottom: 1rem;
		left: 1rem;
		width: 44px;
		height: 44px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 0, 0, 0.7);
		border: none;
		border-radius: 50%;
		color: white;
		cursor: pointer;
		transition: background 0.15s, transform 0.15s;
	}

	.gif-play-toggle:hover {
		background: rgba(0, 0, 0, 0.85);
		transform: scale(1.05);
	}

	.gif-badge {
		position: absolute;
		top: 1rem;
		left: 1rem;
		padding: 0.25rem 0.5rem;
		background: rgba(0, 0, 0, 0.7);
		border-radius: 4px;
		font-size: 0.6875rem;
		font-weight: 700;
		color: white;
		letter-spacing: 0.05em;
	}

	.gif-caption {
		margin: 0.75rem 0 0;
		font-size: 0.875rem;
		color: #6b7280;
	}

	.gif-placeholder {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
		padding: 3rem;
		background: #f9fafb;
		border: 2px dashed #d1d5db;
		border-radius: 12px;
		color: #9ca3af;
	}

	.gif-search-container {
		width: 100%;
		max-width: 400px;
	}

	.gif-search-input {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		background: white;
		border: 1px solid #d1d5db;
		border-radius: 8px;
	}

	.gif-search-input input {
		flex: 1;
		border: none;
		outline: none;
		font-size: 0.875rem;
	}

	.gif-placeholder input[type="url"] {
		width: 100%;
		max-width: 400px;
		padding: 0.75rem 1rem;
		border: 1px solid #d1d5db;
		border-radius: 8px;
		font-size: 0.875rem;
	}

	.gif-upload-divider {
		color: #9ca3af;
		font-size: 0.875rem;
	}

	/* Media Blocks Responsive */
	@media (max-width: 768px) {
		.gallery-grid {
			grid-template-columns: repeat(2, 1fr);
		}

		.lightbox-nav {
			position: fixed;
		}

		.lightbox-prev {
			left: 1rem;
		}

		.lightbox-next {
			right: 1rem;
		}

		.audio-player {
			flex-wrap: wrap;
		}

		.audio-progress-container {
			order: 5;
			width: 100%;
			flex: none;
			margin-top: 0.5rem;
		}

		.audio-volume-container {
			display: none;
		}

		.file-download-card {
			flex-wrap: wrap;
		}

		.file-info {
			width: calc(100% - 64px);
		}

		.file-download-icon {
			width: 100%;
			margin-top: 0.75rem;
		}
	}

	@media (max-width: 480px) {
		.gallery-grid {
			grid-template-columns: 1fr;
		}

		.audio-time {
			min-width: auto;
		}
	}

	/* Media Blocks Dark Mode */
	:global(.dark) .gallery-placeholder,
	:global(.dark) .audio-placeholder,
	:global(.dark) .file-placeholder,
	:global(.dark) .embed-placeholder,
	:global(.dark) .gif-placeholder {
		background: #1e293b;
		border-color: #334155;
		color: #94a3b8;
	}

	:global(.dark) .gallery-item-controls {
		background: #1e293b;
	}

	:global(.dark) .gallery-url-input,
	:global(.dark) .gallery-alt-input,
	:global(.dark) .file-name-input {
		background: #0f172a;
		border-color: #334155;
		color: #e2e8f0;
	}

	:global(.dark) .gallery-caption,
	:global(.dark) .audio-caption,
	:global(.dark) .gif-caption {
		color: #94a3b8;
	}

	:global(.dark) .file-download-card {
		background: #1e293b;
		border-color: #334155;
	}

	:global(.dark) .file-download-card:hover {
		background: #0f172a;
		border-color: #475569;
	}

	:global(.dark) .file-name {
		color: #f1f5f9;
	}

	:global(.dark) .file-size {
		color: #94a3b8;
	}

	:global(.dark) .embed-twitter,
	:global(.dark) .embed-instagram,
	:global(.dark) .embed-tiktok {
		background: #1e293b;
	}

	:global(.dark) .embed-fallback {
		color: #94a3b8;
	}

	:global(.dark) .embed-error {
		background: #450a0a;
		border-color: #7f1d1d;
		color: #fca5a5;
	}

	:global(.dark) .gif-search-input {
		background: #0f172a;
		border-color: #334155;
	}

	:global(.dark) .gif-search-input input {
		background: transparent;
		color: #e2e8f0;
	}

	:global(.dark) .audio-placeholder input[type="url"],
	:global(.dark) .file-placeholder input[type="url"],
	:global(.dark) .embed-placeholder input[type="url"],
	:global(.dark) .gif-placeholder input[type="url"] {
		background: #0f172a;
		border-color: #334155;
		color: #e2e8f0;
	}

	/* ==========================================================================
	   AI Block Styles
	   ========================================================================== */

	/* AI Block Common */
	.ai-generated-block, .ai-summary-block, .ai-translation-block {
		background: linear-gradient(135deg, #f8fafc 0%, #f1f5f9 100%);
		border: 1px solid #e2e8f0;
		border-radius: 12px;
		padding: 1.25rem;
		position: relative;
		overflow: hidden;
	}

	.ai-generated-block::before, .ai-summary-block::before, .ai-translation-block::before {
		content: '';
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		height: 3px;
		background: linear-gradient(90deg, #8b5cf6, #6366f1, #3b82f6);
	}

	.ai-block-header { display: flex; align-items: center; justify-content: space-between; gap: 1rem; margin-bottom: 1rem; flex-wrap: wrap; }

	.ai-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.75rem;
		background: linear-gradient(135deg, #8b5cf6, #6366f1);
		color: white;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.visually-hidden { position: absolute; width: 1px; height: 1px; padding: 0; margin: -1px; overflow: hidden; clip: rect(0, 0, 0, 0); white-space: nowrap; border: 0; }

	/* AI Model Selector */
	.ai-model-selector select { padding: 0.375rem 0.75rem; font-size: 0.875rem; border: 1px solid #d1d5db; border-radius: 6px; background: white; color: #374151; cursor: pointer; transition: border-color 0.15s, box-shadow 0.15s; }
	.ai-model-selector select:hover { border-color: #9ca3af; }
	.ai-model-selector select:focus { outline: none; border-color: #6366f1; box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1); }
	.ai-model-selector select:disabled { background: #f3f4f6; cursor: not-allowed; opacity: 0.7; }

	/* AI Prompt Section */
	.ai-prompt-section { display: flex; flex-direction: column; gap: 0.75rem; margin-bottom: 1rem; }
	.ai-prompt-label { display: flex; align-items: center; gap: 0.375rem; font-size: 0.875rem; font-weight: 500; color: #374151; }
	.ai-prompt-input { padding: 0.75rem; font-size: 0.9375rem; line-height: 1.5; border: 1px solid #d1d5db; border-radius: 8px; background: white; color: #1f2937; resize: vertical; transition: border-color 0.15s, box-shadow 0.15s; }
	.ai-prompt-input:focus { outline: none; border-color: #6366f1; box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1); }
	.ai-prompt-input:disabled { background: #f9fafb; cursor: not-allowed; }
	.ai-prompt-input::placeholder { color: #9ca3af; }

	/* AI Generate Button */
	.ai-generate-btn, .ai-translate-btn { display: inline-flex; align-items: center; justify-content: center; gap: 0.5rem; padding: 0.625rem 1.25rem; background: linear-gradient(135deg, #8b5cf6, #6366f1); color: white; font-size: 0.9375rem; font-weight: 600; border: none; border-radius: 8px; cursor: pointer; transition: all 0.2s; align-self: flex-start; }
	.ai-generate-btn:hover:not(:disabled), .ai-translate-btn:hover:not(:disabled) { background: linear-gradient(135deg, #7c3aed, #4f46e5); transform: translateY(-1px); box-shadow: 0 4px 12px rgba(99, 102, 241, 0.3); }
	.ai-generate-btn:active:not(:disabled), .ai-translate-btn:active:not(:disabled) { transform: translateY(0); }
	.ai-generate-btn:disabled, .ai-translate-btn:disabled { background: #9ca3af; cursor: not-allowed; transform: none; box-shadow: none; }

	/* AI Loading Skeleton */
	.ai-loading-skeleton { display: flex; flex-direction: column; gap: 0.625rem; padding: 1rem 0; }
	.skeleton-line { height: 1rem; background: linear-gradient(90deg, #e5e7eb 0%, #f3f4f6 50%, #e5e7eb 100%); background-size: 200% 100%; border-radius: 4px; animation: skeleton-shimmer 1.5s infinite; }
	.skeleton-line-full { width: 100%; }
	.skeleton-line-medium { width: 75%; }
	.skeleton-line-short { width: 50%; }
	@keyframes skeleton-shimmer { 0% { background-position: 200% 0; } 100% { background-position: -200% 0; } }

	/* AI Error */
	.ai-error { display: flex; align-items: center; gap: 0.5rem; padding: 0.75rem 1rem; background: #fef2f2; border: 1px solid #fecaca; border-radius: 8px; color: #dc2626; font-size: 0.875rem; margin-bottom: 1rem; }
	.ai-retry-btn { display: inline-flex; align-items: center; gap: 0.25rem; padding: 0.375rem 0.625rem; background: white; border: 1px solid #fecaca; border-radius: 4px; color: #dc2626; font-size: 0.8125rem; font-weight: 500; cursor: pointer; margin-left: auto; transition: all 0.15s; }
	.ai-retry-btn:hover { background: #fef2f2; border-color: #f87171; }

	/* AI Output Section */
	.ai-output-section { background: white; border: 1px solid #e5e7eb; border-radius: 8px; overflow: hidden; }
	.ai-output-header { display: flex; align-items: center; justify-content: space-between; padding: 0.625rem 1rem; background: #f9fafb; border-bottom: 1px solid #e5e7eb; }
	.ai-output-label { font-size: 0.8125rem; font-weight: 600; color: #6b7280; text-transform: uppercase; letter-spacing: 0.05em; }
	.ai-regenerate-btn, .ai-refresh-btn { display: inline-flex; align-items: center; gap: 0.25rem; padding: 0.375rem 0.625rem; background: white; border: 1px solid #d1d5db; border-radius: 4px; color: #4b5563; font-size: 0.8125rem; font-weight: 500; cursor: pointer; transition: all 0.15s; }
	.ai-regenerate-btn:hover:not(:disabled), .ai-refresh-btn:hover:not(:disabled) { background: #f3f4f6; border-color: #9ca3af; }
	.ai-regenerate-btn:disabled, .ai-refresh-btn:disabled { opacity: 0.5; cursor: not-allowed; }
	.ai-output-content { padding: 1rem; font-size: 0.9375rem; line-height: 1.7; color: #1f2937; white-space: pre-wrap; }
	.ai-output-content:focus { outline: none; background: #fffbeb; }

	/* AI Summary Block */
	.ai-summary-toggle { display: flex; align-items: center; gap: 0.375rem; background: none; border: none; padding: 0; cursor: pointer; color: inherit; }
	.ai-summary-toggle:hover { opacity: 0.8; }
	.ai-summary-controls { display: flex; align-items: center; gap: 0.5rem; }
	.ai-summary-controls select { padding: 0.25rem 0.5rem; font-size: 0.8125rem; border: 1px solid #d1d5db; border-radius: 4px; background: white; color: #374151; cursor: pointer; }
	.ai-summary-controls select:disabled { opacity: 0.7; cursor: not-allowed; }
	.ai-summary-content { padding-top: 0.5rem; overflow: hidden; transition: max-height 0.3s ease, opacity 0.3s ease; max-height: 500px; opacity: 1; }
	.ai-summary-content.collapsed { max-height: 0; opacity: 0; padding-top: 0; }
	.ai-summary-text { margin: 0; padding: 0.75rem 1rem; background: white; border-radius: 8px; font-size: 0.9375rem; line-height: 1.6; color: #374151; }
	.ai-summary-text:focus { outline: none; background: #fffbeb; }
	.ai-summary-placeholder { margin: 0; padding: 0.75rem 1rem; background: white; border-radius: 8px; font-size: 0.875rem; color: #9ca3af; font-style: italic; }

	/* AI Translation Block */
	.ai-translation-content { display: flex; flex-direction: column; gap: 1rem; }
	.ai-translation-block.side-by-side .ai-translation-content { flex-direction: row; align-items: stretch; }
	.ai-translation-block.side-by-side .ai-translation-arrow { transform: rotate(0deg); align-self: center; }
	.ai-translation-source, .ai-translation-target { flex: 1; display: flex; flex-direction: column; gap: 0.5rem; }
	.ai-translation-lang-header { display: flex; align-items: center; gap: 0.5rem; flex-wrap: wrap; }
	.ai-translation-lang-header label { font-size: 0.8125rem; font-weight: 600; color: #6b7280; text-transform: uppercase; letter-spacing: 0.05em; }
	.ai-translation-lang-header select { padding: 0.25rem 0.5rem; font-size: 0.8125rem; border: 1px solid #d1d5db; border-radius: 4px; background: white; color: #374151; cursor: pointer; }
	.ai-translation-lang-header select:disabled { opacity: 0.7; cursor: not-allowed; }
	.lang-display { font-size: 0.875rem; color: #374151; font-weight: 500; }
	.ai-translation-textarea { flex: 1; min-height: 100px; padding: 0.75rem; font-size: 0.9375rem; line-height: 1.5; border: 1px solid #d1d5db; border-radius: 8px; background: white; color: #1f2937; resize: vertical; transition: border-color 0.15s, box-shadow 0.15s; }
	.ai-translation-textarea:focus { outline: none; border-color: #6366f1; box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1); }
	.ai-translation-textarea:disabled { background: #f9fafb; cursor: not-allowed; }
	.ai-translation-arrow { display: flex; align-items: center; justify-content: center; color: #9ca3af; transform: rotate(90deg); padding: 0.5rem; }
	.ai-translation-output { flex: 1; min-height: 100px; padding: 0.75rem; font-size: 0.9375rem; line-height: 1.5; background: white; border: 1px solid #d1d5db; border-radius: 8px; color: #1f2937; white-space: pre-wrap; }
	.ai-translation-output:focus { outline: none; background: #fffbeb; }
	.ai-translation-placeholder { flex: 1; min-height: 100px; padding: 0.75rem; font-size: 0.875rem; background: #f9fafb; border: 1px dashed #d1d5db; border-radius: 8px; color: #9ca3af; display: flex; align-items: center; justify-content: center; font-style: italic; }
	.translation-skeleton { min-height: 100px; background: #f9fafb; border: 1px solid #e5e7eb; border-radius: 8px; padding: 0.75rem; }
	.ai-translation-actions { display: flex; justify-content: flex-end; margin-top: 0.5rem; }
	.ai-copy-btn { display: inline-flex; align-items: center; gap: 0.25rem; padding: 0.25rem 0.5rem; background: white; border: 1px solid #d1d5db; border-radius: 4px; color: #4b5563; font-size: 0.75rem; font-weight: 500; cursor: pointer; transition: all 0.15s; margin-left: auto; }
	.ai-copy-btn:hover { background: #f3f4f6; border-color: #9ca3af; }
	.ai-view-toggle { padding: 0.375rem 0.625rem; background: white; border: 1px solid #d1d5db; border-radius: 4px; color: #4b5563; font-size: 0.8125rem; font-weight: 500; cursor: pointer; transition: all 0.15s; }
	.ai-view-toggle:hover { background: #f3f4f6; border-color: #9ca3af; }

	/* Spin Animation */
	:global(.spin) { animation: spin 1s linear infinite; }
	@keyframes spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }

	/* AI Block Mobile Responsive */
	@media (max-width: 640px) {
		.ai-generated-block, .ai-summary-block, .ai-translation-block { padding: 1rem; }
		.ai-block-header { flex-direction: column; align-items: flex-start; gap: 0.75rem; }
		.ai-translation-block.side-by-side .ai-translation-content { flex-direction: column; }
		.ai-translation-block.side-by-side .ai-translation-arrow { transform: rotate(90deg); }
		.ai-generate-btn, .ai-translate-btn { width: 100%; }
		.ai-translation-actions { justify-content: stretch; }
		.ai-translation-actions .ai-translate-btn { flex: 1; }
	}

	/* AI Block Dark Mode */
	:global(.dark) .ai-generated-block, :global(.dark) .ai-summary-block, :global(.dark) .ai-translation-block { background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%); border-color: #334155; }
	:global(.dark) .ai-model-selector select, :global(.dark) .ai-summary-controls select, :global(.dark) .ai-translation-lang-header select { background: #1e293b; border-color: #475569; color: #e2e8f0; }
	:global(.dark) .ai-prompt-label { color: #e2e8f0; }
	:global(.dark) .ai-prompt-input, :global(.dark) .ai-translation-textarea { background: #1e293b; border-color: #475569; color: #e2e8f0; }
	:global(.dark) .ai-prompt-input::placeholder, :global(.dark) .ai-translation-textarea::placeholder { color: #64748b; }
	:global(.dark) .ai-output-section { background: #1e293b; border-color: #334155; }
	:global(.dark) .ai-output-header { background: #0f172a; border-color: #334155; }
	:global(.dark) .ai-output-label { color: #94a3b8; }
	:global(.dark) .ai-output-content { color: #e2e8f0; }
	:global(.dark) .ai-output-content:focus, :global(.dark) .ai-summary-text:focus, :global(.dark) .ai-translation-output:focus { background: #1e3a5f; }
	:global(.dark) .ai-regenerate-btn, :global(.dark) .ai-refresh-btn, :global(.dark) .ai-copy-btn, :global(.dark) .ai-view-toggle { background: #1e293b; border-color: #475569; color: #94a3b8; }
	:global(.dark) .ai-regenerate-btn:hover:not(:disabled), :global(.dark) .ai-refresh-btn:hover:not(:disabled), :global(.dark) .ai-copy-btn:hover, :global(.dark) .ai-view-toggle:hover { background: #334155; border-color: #64748b; }
	:global(.dark) .ai-error { background: #450a0a; border-color: #7f1d1d; color: #fca5a5; }
	:global(.dark) .ai-retry-btn { background: #1e293b; border-color: #7f1d1d; color: #fca5a5; }
	:global(.dark) .ai-retry-btn:hover { background: #450a0a; }
	:global(.dark) .skeleton-line { background: linear-gradient(90deg, #334155 0%, #475569 50%, #334155 100%); background-size: 200% 100%; }
	:global(.dark) .ai-summary-text { background: #1e293b; color: #e2e8f0; }
	:global(.dark) .ai-summary-placeholder { background: #1e293b; color: #64748b; }
	:global(.dark) .ai-translation-output { background: #1e293b; border-color: #475569; color: #e2e8f0; }
	:global(.dark) .ai-translation-placeholder { background: #0f172a; border-color: #334155; color: #64748b; }
	:global(.dark) .translation-skeleton { background: #0f172a; border-color: #334155; }
	:global(.dark) .lang-display { color: #e2e8f0; }
	:global(.dark) .ai-translation-arrow { color: #64748b; }
	:global(.dark) .ai-translation-lang-header label { color: #94a3b8; }
</style>
