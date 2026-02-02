/**
 * Block State Management System
 * ═══════════════════════════════════════════════════════════════════════════
 * Centralized state for all block types with proper cleanup and reactivity
 */

import { getContext, setContext } from 'svelte';

// ============================================================================
// Types
// ============================================================================

export type BlockId = string & { readonly __brand: 'BlockId' };

export interface AudioState {
	playing: boolean;
	progress: number;
	volume: number;
	muted: boolean;
	duration: number;
	currentTime: number;
}

export interface LightboxState {
	open: boolean;
	blockId: BlockId | null;
	index: number;
	totalImages: number;
}

export interface NewsletterState {
	email: string;
	submitting: boolean;
	success: boolean;
	error: string | null;
}

export interface AIBlockState {
	loading: boolean;
	error: string | null;
	output: string | null;
	lastGenerated: number | null;
}

export interface CountdownState {
	days: number;
	hours: number;
	minutes: number;
	seconds: number;
	intervalId: ReturnType<typeof setInterval> | null;
}

export interface InteractiveState {
	accordion: Map<BlockId, Set<string>>; // blockId -> open item IDs
	tabs: Map<BlockId, string>; // blockId -> active tab ID
	toggles: Map<BlockId, boolean>; // blockId -> open state
	toc: Map<BlockId, { open: boolean; activeHeading: string }>;
}

export interface MediaState {
	audio: Map<BlockId, AudioState>;
	lightbox: LightboxState;
	gifs: Map<BlockId, boolean>; // blockId -> playing state
}

export interface FormState {
	newsletter: Map<BlockId, NewsletterState>;
}

export interface AIState {
	generated: Map<BlockId, AIBlockState>;
	summary: Map<BlockId, AIBlockState>;
	translation: Map<
		BlockId,
		AIBlockState & {
			sourceText: string;
			targetLanguage: string;
			view: 'stacked' | 'side-by-side';
		}
	>;
}

export interface SocialShareState {
	linkCopied: Map<BlockId, boolean>;
}

export interface ReusableComponentState {
	data: unknown[] | null;
	loading: boolean;
	error: string | null;
	version: number;
	syncedVersion: number;
	needsUpdate: boolean;
}

export interface BlockStateData {
	interactive: InteractiveState;
	media: MediaState;
	forms: FormState;
	ai: AIState;
	social: SocialShareState;
	countdown: Map<BlockId, CountdownState>;
	reusable: Map<BlockId, ReusableComponentState>;
}

// ============================================================================
// State Manager Class
// ============================================================================

export class BlockStateManager {
	private state = $state<BlockStateData>(this.createInitialState());
	private cleanupCallbacks = new Map<BlockId, Array<() => void>>();

	private createInitialState(): BlockStateData {
		return {
			interactive: {
				accordion: new Map(),
				tabs: new Map(),
				toggles: new Map(),
				toc: new Map()
			},
			media: {
				audio: new Map(),
				lightbox: {
					open: false,
					blockId: null,
					index: 0,
					totalImages: 0
				},
				gifs: new Map()
			},
			forms: {
				newsletter: new Map()
			},
			ai: {
				generated: new Map(),
				summary: new Map(),
				translation: new Map()
			},
			social: {
				linkCopied: new Map()
			},
			countdown: new Map(),
			reusable: new Map()
		};
	}

	// ========================================================================
	// Interactive Blocks
	// ========================================================================

	getAccordionState(blockId: BlockId): Set<string> {
		return this.state.interactive.accordion.get(blockId) ?? new Set();
	}

	toggleAccordionItem(blockId: BlockId, itemId: string, allowMultiple: boolean): void {
		const current = this.getAccordionState(blockId);
		const newSet = new Set(current);

		if (newSet.has(itemId)) {
			newSet.delete(itemId);
		} else {
			if (!allowMultiple) {
				newSet.clear();
			}
			newSet.add(itemId);
		}

		this.state.interactive.accordion.set(blockId, newSet);
	}

	getActiveTab(blockId: BlockId, defaultTabId?: string): string {
		return this.state.interactive.tabs.get(blockId) ?? defaultTabId ?? '';
	}

	setActiveTab(blockId: BlockId, tabId: string): void {
		this.state.interactive.tabs.set(blockId, tabId);
	}

	getToggleState(blockId: BlockId, defaultOpen = false): boolean {
		return this.state.interactive.toggles.get(blockId) ?? defaultOpen;
	}

	toggleToggle(blockId: BlockId): void {
		const current = this.getToggleState(blockId);
		this.state.interactive.toggles.set(blockId, !current);
	}

	getTocState(blockId: BlockId): { open: boolean; activeHeading: string } {
		return this.state.interactive.toc.get(blockId) ?? { open: true, activeHeading: '' };
	}

	setTocState(blockId: BlockId, updates: Partial<{ open: boolean; activeHeading: string }>): void {
		const current = this.getTocState(blockId);
		this.state.interactive.toc.set(blockId, { ...current, ...updates });
	}

	// ========================================================================
	// Media Blocks
	// ========================================================================

	getAudioState(blockId: BlockId): AudioState {
		return (
			this.state.media.audio.get(blockId) ?? {
				playing: false,
				progress: 0,
				volume: 1,
				muted: false,
				duration: 0,
				currentTime: 0
			}
		);
	}

	setAudioState(blockId: BlockId, updates: Partial<AudioState>): void {
		const current = this.getAudioState(blockId);
		this.state.media.audio.set(blockId, { ...current, ...updates });
	}

	getLightboxState(): LightboxState {
		return this.state.media.lightbox;
	}

	openLightbox(blockId: BlockId, index: number, totalImages: number): void {
		this.state.media.lightbox = {
			open: true,
			blockId,
			index,
			totalImages
		};
		if (typeof document !== 'undefined') {
			document.body.style.overflow = 'hidden';
		}
	}

	closeLightbox(): void {
		this.state.media.lightbox = {
			open: false,
			blockId: null,
			index: 0,
			totalImages: 0
		};
		if (typeof document !== 'undefined') {
			document.body.style.overflow = '';
		}
	}

	navigateLightbox(direction: 'next' | 'prev'): void {
		const { index, totalImages } = this.state.media.lightbox;
		if (direction === 'next') {
			this.state.media.lightbox.index = (index + 1) % totalImages;
		} else {
			this.state.media.lightbox.index = (index - 1 + totalImages) % totalImages;
		}
	}

	getGifPlaying(blockId: BlockId): boolean {
		return this.state.media.gifs.get(blockId) ?? true;
	}

	toggleGif(blockId: BlockId): void {
		const current = this.getGifPlaying(blockId);
		this.state.media.gifs.set(blockId, !current);
	}

	// ========================================================================
	// Form Blocks
	// ========================================================================

	getNewsletterState(blockId: BlockId): NewsletterState {
		return (
			this.state.forms.newsletter.get(blockId) ?? {
				email: '',
				submitting: false,
				success: false,
				error: null
			}
		);
	}

	setNewsletterState(blockId: BlockId, updates: Partial<NewsletterState>): void {
		const current = this.getNewsletterState(blockId);
		this.state.forms.newsletter.set(blockId, { ...current, ...updates });
	}

	// ========================================================================
	// AI Blocks
	// ========================================================================

	private createDefaultAIState(): AIBlockState {
		return {
			loading: false,
			error: null,
			output: null,
			lastGenerated: null
		};
	}

	getAIGeneratedState(blockId: BlockId): AIBlockState {
		return this.state.ai.generated.get(blockId) ?? this.createDefaultAIState();
	}

	setAIGeneratedState(blockId: BlockId, updates: Partial<AIBlockState>): void {
		const current = this.getAIGeneratedState(blockId);
		this.state.ai.generated.set(blockId, { ...current, ...updates });
	}

	getAISummaryState(blockId: BlockId): AIBlockState {
		return this.state.ai.summary.get(blockId) ?? this.createDefaultAIState();
	}

	setAISummaryState(blockId: BlockId, updates: Partial<AIBlockState>): void {
		const current = this.getAISummaryState(blockId);
		this.state.ai.summary.set(blockId, { ...current, ...updates });
	}

	getAITranslationState(blockId: BlockId) {
		return (
			this.state.ai.translation.get(blockId) ?? {
				...this.createDefaultAIState(),
				sourceText: '',
				targetLanguage: 'es',
				view: 'stacked' as const
			}
		);
	}

	setAITranslationState(
		blockId: BlockId,
		updates: Partial<ReturnType<typeof this.getAITranslationState>>
	): void {
		const current = this.getAITranslationState(blockId);
		this.state.ai.translation.set(blockId, { ...current, ...updates });
	}

	// ========================================================================
	// Social Share
	// ========================================================================

	getLinkCopied(blockId: BlockId): boolean {
		return this.state.social.linkCopied.get(blockId) ?? false;
	}

	setLinkCopied(blockId: BlockId, copied: boolean): void {
		this.state.social.linkCopied.set(blockId, copied);
	}

	// ========================================================================
	// Countdown
	// ========================================================================

	getCountdownState(blockId: BlockId): CountdownState {
		return (
			this.state.countdown.get(blockId) ?? {
				days: 0,
				hours: 0,
				minutes: 0,
				seconds: 0,
				intervalId: null
			}
		);
	}

	setCountdownState(blockId: BlockId, updates: Partial<CountdownState>): void {
		const current = this.getCountdownState(blockId);
		this.state.countdown.set(blockId, { ...current, ...updates });
	}

	startCountdown(blockId: BlockId, targetDate: string): void {
		// Clear existing interval
		const current = this.getCountdownState(blockId);
		if (current.intervalId) {
			clearInterval(current.intervalId);
		}

		const updateCountdown = () => {
			const target = new Date(targetDate).getTime();
			const now = new Date().getTime();
			const diff = target - now;

			if (diff <= 0) {
				this.setCountdownState(blockId, {
					days: 0,
					hours: 0,
					minutes: 0,
					seconds: 0,
					intervalId: null
				});
				const state = this.getCountdownState(blockId);
				if (state.intervalId) {
					clearInterval(state.intervalId);
				}
				return;
			}

			this.setCountdownState(blockId, {
				days: Math.floor(diff / (1000 * 60 * 60 * 24)),
				hours: Math.floor((diff % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60)),
				minutes: Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60)),
				seconds: Math.floor((diff % (1000 * 60)) / 1000)
			});
		};

		updateCountdown();
		const intervalId = setInterval(updateCountdown, 1000);
		this.setCountdownState(blockId, { intervalId });

		// Register cleanup
		this.registerCleanup(blockId, () => {
			if (intervalId) clearInterval(intervalId);
		});
	}

	// ========================================================================
	// Reusable Components
	// ========================================================================

	getReusableState(blockId: BlockId): ReusableComponentState {
		return (
			this.state.reusable.get(blockId) ?? {
				data: null,
				loading: false,
				error: null,
				version: 0,
				syncedVersion: 0,
				needsUpdate: false
			}
		);
	}

	setReusableState(blockId: BlockId, updates: Partial<ReusableComponentState>): void {
		const current = this.getReusableState(blockId);
		this.state.reusable.set(blockId, { ...current, ...updates });
	}

	// ========================================================================
	// Cleanup
	// ========================================================================

	registerCleanup(blockId: BlockId, callback: () => void): void {
		const callbacks = this.cleanupCallbacks.get(blockId) ?? [];
		callbacks.push(callback);
		this.cleanupCallbacks.set(blockId, callbacks);
	}

	cleanup(blockId: BlockId): void {
		// Run all cleanup callbacks
		const callbacks = this.cleanupCallbacks.get(blockId) ?? [];
		callbacks.forEach((cb) => cb());
		this.cleanupCallbacks.delete(blockId);

		// Clear state
		this.state.interactive.accordion.delete(blockId);
		this.state.interactive.tabs.delete(blockId);
		this.state.interactive.toggles.delete(blockId);
		this.state.interactive.toc.delete(blockId);
		this.state.media.audio.delete(blockId);
		this.state.media.gifs.delete(blockId);
		this.state.forms.newsletter.delete(blockId);
		this.state.ai.generated.delete(blockId);
		this.state.ai.summary.delete(blockId);
		this.state.ai.translation.delete(blockId);
		this.state.social.linkCopied.delete(blockId);
		this.state.reusable.delete(blockId);

		// Clear countdown
		const countdown = this.state.countdown.get(blockId);
		if (countdown?.intervalId) {
			clearInterval(countdown.intervalId);
		}
		this.state.countdown.delete(blockId);

		// Close lightbox if it belongs to this block
		if (this.state.media.lightbox.blockId === blockId) {
			this.closeLightbox();
		}
	}

	cleanupAll(): void {
		// Cleanup all blocks
		for (const blockId of this.cleanupCallbacks.keys()) {
			this.cleanup(blockId);
		}

		// Reset to initial state
		this.state = this.createInitialState();
	}
}

// ============================================================================
// Context API
// ============================================================================

const BLOCK_STATE_KEY = Symbol('BLOCK_STATE_MANAGER');

export function setBlockStateManager(manager: BlockStateManager): void {
	setContext(BLOCK_STATE_KEY, manager);
}

export function getBlockStateManager(): BlockStateManager {
	const manager = getContext<BlockStateManager | undefined>(BLOCK_STATE_KEY);
	if (!manager) {
		throw new Error(
			'BlockStateManager not found in context. Did you forget to call setBlockStateManager?'
		);
	}
	return manager;
}

// ============================================================================
// Utility Functions
// ============================================================================

export function createBlockId(id: string): BlockId {
	return id as BlockId;
}

export function isValidBlockId(id: unknown): id is BlockId {
	return typeof id === 'string' && id.length > 0;
}
