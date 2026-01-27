/**
 * CMS Content Editor Store - Apple ICT 7+ Principal Engineer Grade
 * ================================================================
 * SVELTE 5 RUNES VERSION
 *
 * Comprehensive editor state management with:
 * - Rich state tracking (content, dirty state, validation)
 * - Optimistic updates with rollback
 * - Real-time collaboration support
 * - Undo/redo history
 * - Autosave integration
 *
 * @version 2.0.0 - January 2026
 * @author Revolution Trading Pros
 */

import { browser } from '$app/environment';
import type { JsonValue } from '$lib/types/common';
import { autosaveStore } from './autosave.svelte';

// =============================================================================
// Type Definitions
// =============================================================================

export type ContentStatus = 'draft' | 'in_review' | 'approved' | 'scheduled' | 'published' | 'archived';

export type ContentType =
	| 'page'
	| 'blog_post'
	| 'alert_service'
	| 'trading_room'
	| 'indicator'
	| 'course'
	| 'lesson'
	| 'testimonial'
	| 'faq'
	| 'author'
	| 'topic_cluster'
	| 'weekly_watchlist'
	| 'resource';

export type EditorMode = 'richtext' | 'markdown' | 'html';

export interface ContentBlock {
	id: string;
	blockType: string;
	order: number;
	data: JsonValue;
	settings?: BlockSettings;
}

export interface BlockSettings {
	isHidden?: boolean;
	cssClass?: string;
	cssId?: string;
	marginTop?: string;
	marginBottom?: string;
	paddingTop?: string;
	paddingBottom?: string;
	backgroundColor?: string;
	backgroundImageId?: string;
}

export interface ContentMeta {
	metaTitle?: string;
	metaDescription?: string;
	metaKeywords?: string[];
	canonicalUrl?: string;
	robotsDirectives?: string;
	structuredData?: JsonValue;
	ogTitle?: string;
	ogDescription?: string;
	ogImageId?: string;
	twitterTitle?: string;
	twitterDescription?: string;
	twitterImageId?: string;
}

export interface EditorContent {
	id: string;
	contentType: ContentType;
	slug: string;
	locale: string;
	title: string;
	subtitle?: string;
	excerpt?: string;
	content?: string;
	contentBlocks?: ContentBlock[];
	featuredImageId?: string;
	authorId?: string;
	status: ContentStatus;
	publishedAt?: string;
	scheduledPublishAt?: string;
	scheduledUnpublishAt?: string;
	customFields?: JsonValue;
	template?: string;
	version: number;
	meta: ContentMeta;
}

export interface EditorHistoryEntry {
	content: EditorContent;
	timestamp: number;
	action: string;
}

export interface ValidationError {
	field: string;
	message: string;
}

// =============================================================================
// Editor Store Class - Svelte 5 Runes
// =============================================================================

class EditorStoreClass {
	// Core state
	private _content = $state<EditorContent | null>(null);
	private _originalContent = $state<EditorContent | null>(null);
	private _isDirty = $state(false);
	private _isLoading = $state(false);
	private _isSaving = $state(false);
	private _lastSavedAt = $state<Date | null>(null);
	private _errors = $state<ValidationError[]>([]);

	// Editor mode
	private _editorMode = $state<EditorMode>('richtext');
	private _isPreviewMode = $state(false);
	private _isFullscreen = $state(false);

	// Undo/Redo history
	private _history = $state<EditorHistoryEntry[]>([]);
	private _historyIndex = $state(-1);
	private _maxHistorySize = 50;

	// Active block tracking
	private _activeBlockId = $state<string | null>(null);
	private _selectedBlockIds = $state<string[]>([]);

	// Collaboration
	private _lockedBy = $state<string | null>(null);
	private _collaborators = $state<string[]>([]);

	// Subscribers for backward compatibility
	private subscribers = new Set<(value: EditorStateSnapshot) => void>();

	// ==========================================================================
	// Getters for reactive state access
	// ==========================================================================

	get content(): EditorContent | null {
		return this._content;
	}
	get originalContent(): EditorContent | null {
		return this._originalContent;
	}
	get isDirty(): boolean {
		return this._isDirty;
	}
	get isLoading(): boolean {
		return this._isLoading;
	}
	get isSaving(): boolean {
		return this._isSaving;
	}
	get lastSavedAt(): Date | null {
		return this._lastSavedAt;
	}
	get errors(): ValidationError[] {
		return this._errors;
	}
	get editorMode(): EditorMode {
		return this._editorMode;
	}
	get isPreviewMode(): boolean {
		return this._isPreviewMode;
	}
	get isFullscreen(): boolean {
		return this._isFullscreen;
	}
	get activeBlockId(): string | null {
		return this._activeBlockId;
	}
	get selectedBlockIds(): string[] {
		return this._selectedBlockIds;
	}
	get lockedBy(): string | null {
		return this._lockedBy;
	}
	get collaborators(): string[] {
		return this._collaborators;
	}

	// Derived getters
	get canUndo(): boolean {
		return this._historyIndex > 0;
	}

	get canRedo(): boolean {
		return this._historyIndex < this._history.length - 1;
	}

	get hasErrors(): boolean {
		return this._errors.length > 0;
	}

	get isValid(): boolean {
		return this._errors.length === 0;
	}

	get contentBlocks(): ContentBlock[] {
		return this._content?.contentBlocks ?? [];
	}

	get activeBlock(): ContentBlock | null {
		if (!this._activeBlockId || !this._content?.contentBlocks) return null;
		return this._content.contentBlocks.find((b) => b.id === this._activeBlockId) ?? null;
	}

	// ==========================================================================
	// Subscribe method for backward compatibility
	// ==========================================================================

	subscribe(fn: (value: EditorStateSnapshot) => void): () => void {
		this.subscribers.add(fn);
		fn(this.getSnapshot());
		return () => {
			this.subscribers.delete(fn);
		};
	}

	private getSnapshot(): EditorStateSnapshot {
		return {
			content: this._content,
			isDirty: this._isDirty,
			isLoading: this._isLoading,
			isSaving: this._isSaving,
			errors: this._errors,
			editorMode: this._editorMode,
			isPreviewMode: this._isPreviewMode,
			canUndo: this.canUndo,
			canRedo: this.canRedo
		};
	}

	private notifySubscribers(): void {
		const snapshot = this.getSnapshot();
		this.subscribers.forEach((fn) => fn(snapshot));
	}

	// ==========================================================================
	// Content Management
	// ==========================================================================

	/**
	 * Load content into the editor
	 */
	loadContent(content: EditorContent): void {
		this._content = structuredClone(content);
		this._originalContent = structuredClone(content);
		this._isDirty = false;
		this._errors = [];
		this._history = [{ content: structuredClone(content), timestamp: Date.now(), action: 'load' }];
		this._historyIndex = 0;
		this.notifySubscribers();

		// Check for autosaved draft
		if (browser && content.id) {
			autosaveStore.loadDraft(content.id);
		}
	}

	/**
	 * Create new content
	 */
	createNew(contentType: ContentType, defaults?: Partial<EditorContent>): void {
		const newContent: EditorContent = {
			id: crypto.randomUUID(),
			contentType,
			slug: '',
			locale: 'en',
			title: '',
			status: 'draft',
			version: 1,
			meta: {},
			...defaults
		};

		this._content = newContent;
		this._originalContent = structuredClone(newContent);
		this._isDirty = false;
		this._errors = [];
		this._history = [{ content: structuredClone(newContent), timestamp: Date.now(), action: 'create' }];
		this._historyIndex = 0;
		this.notifySubscribers();
	}

	/**
	 * Update content fields
	 */
	updateContent(updates: Partial<EditorContent>): void {
		if (!this._content) return;

		const previousContent = structuredClone(this._content);
		this._content = { ...this._content, ...updates };
		this._isDirty = true;

		this.pushHistory(previousContent, 'update');
		this.validate();
		this.notifySubscribers();

		// Trigger autosave
		if (browser && this._content.id) {
			autosaveStore.queueSave(this._content);
		}
	}

	/**
	 * Update a specific field
	 */
	updateField<K extends keyof EditorContent>(field: K, value: EditorContent[K]): void {
		if (!this._content) return;
		this.updateContent({ [field]: value } as Partial<EditorContent>);
	}

	/**
	 * Update meta fields
	 */
	updateMeta(updates: Partial<ContentMeta>): void {
		if (!this._content) return;
		this.updateContent({
			meta: { ...this._content.meta, ...updates }
		});
	}

	// ==========================================================================
	// Block Management
	// ==========================================================================

	/**
	 * Add a new block
	 */
	addBlock(blockType: string, data: JsonValue = {}, insertAfter?: string): ContentBlock {
		const newBlock: ContentBlock = {
			id: crypto.randomUUID(),
			blockType,
			order: this.contentBlocks.length,
			data
		};

		if (!this._content) return newBlock;

		const blocks = [...(this._content.contentBlocks ?? [])];

		if (insertAfter) {
			const insertIndex = blocks.findIndex((b) => b.id === insertAfter);
			if (insertIndex !== -1) {
				blocks.splice(insertIndex + 1, 0, newBlock);
				// Reorder blocks
				blocks.forEach((b, i) => (b.order = i));
			} else {
				blocks.push(newBlock);
			}
		} else {
			blocks.push(newBlock);
		}

		this.updateContent({ contentBlocks: blocks });
		this._activeBlockId = newBlock.id;
		return newBlock;
	}

	/**
	 * Update a block
	 */
	updateBlock(blockId: string, updates: Partial<ContentBlock>): void {
		if (!this._content?.contentBlocks) return;

		const blocks = this._content.contentBlocks.map((block) =>
			block.id === blockId ? { ...block, ...updates } : block
		);

		this.updateContent({ contentBlocks: blocks });
	}

	/**
	 * Update block data
	 */
	updateBlockData(blockId: string, data: JsonValue): void {
		this.updateBlock(blockId, { data });
	}

	/**
	 * Delete a block
	 */
	deleteBlock(blockId: string): void {
		if (!this._content?.contentBlocks) return;

		const blocks = this._content.contentBlocks
			.filter((b) => b.id !== blockId)
			.map((b, i) => ({ ...b, order: i }));

		this.updateContent({ contentBlocks: blocks });

		if (this._activeBlockId === blockId) {
			this._activeBlockId = null;
		}
		this._selectedBlockIds = this._selectedBlockIds.filter((id) => id !== blockId);
	}

	/**
	 * Move a block
	 */
	moveBlock(blockId: string, newIndex: number): void {
		if (!this._content?.contentBlocks) return;

		const blocks = [...this._content.contentBlocks];
		const currentIndex = blocks.findIndex((b) => b.id === blockId);
		if (currentIndex === -1) return;

		const [block] = blocks.splice(currentIndex, 1);
		blocks.splice(newIndex, 0, block);
		blocks.forEach((b, i) => (b.order = i));

		this.updateContent({ contentBlocks: blocks });
	}

	/**
	 * Duplicate a block
	 */
	duplicateBlock(blockId: string): ContentBlock | null {
		const block = this.contentBlocks.find((b) => b.id === blockId);
		if (!block) return null;

		return this.addBlock(block.blockType, structuredClone(block.data), blockId);
	}

	/**
	 * Set active block
	 */
	setActiveBlock(blockId: string | null): void {
		this._activeBlockId = blockId;
		this.notifySubscribers();
	}

	/**
	 * Toggle block selection
	 */
	toggleBlockSelection(blockId: string): void {
		if (this._selectedBlockIds.includes(blockId)) {
			this._selectedBlockIds = this._selectedBlockIds.filter((id) => id !== blockId);
		} else {
			this._selectedBlockIds = [...this._selectedBlockIds, blockId];
		}
		this.notifySubscribers();
	}

	/**
	 * Clear block selection
	 */
	clearBlockSelection(): void {
		this._selectedBlockIds = [];
		this.notifySubscribers();
	}

	// ==========================================================================
	// History (Undo/Redo)
	// ==========================================================================

	private pushHistory(previousContent: EditorContent, action: string): void {
		// Truncate future history
		this._history = this._history.slice(0, this._historyIndex + 1);

		// Add new entry
		this._history.push({
			content: structuredClone(previousContent),
			timestamp: Date.now(),
			action
		});

		// Trim if exceeds max size
		if (this._history.length > this._maxHistorySize) {
			this._history = this._history.slice(-this._maxHistorySize);
		}

		this._historyIndex = this._history.length - 1;
	}

	/**
	 * Undo last change
	 */
	undo(): void {
		if (!this.canUndo) return;

		this._historyIndex--;
		const entry = this._history[this._historyIndex];
		this._content = structuredClone(entry.content);
		this._isDirty = this._historyIndex > 0;
		this.notifySubscribers();
	}

	/**
	 * Redo last undone change
	 */
	redo(): void {
		if (!this.canRedo) return;

		this._historyIndex++;
		const entry = this._history[this._historyIndex];
		this._content = structuredClone(entry.content);
		this._isDirty = true;
		this.notifySubscribers();
	}

	// ==========================================================================
	// Validation
	// ==========================================================================

	/**
	 * Validate current content
	 */
	validate(): ValidationError[] {
		const errors: ValidationError[] = [];

		if (!this._content) {
			errors.push({ field: 'content', message: 'No content loaded' });
			this._errors = errors;
			return errors;
		}

		// Title validation
		if (!this._content.title?.trim()) {
			errors.push({ field: 'title', message: 'Title is required' });
		} else if (this._content.title.length > 200) {
			errors.push({ field: 'title', message: 'Title must be less than 200 characters' });
		}

		// Slug validation
		if (!this._content.slug?.trim()) {
			errors.push({ field: 'slug', message: 'Slug is required' });
		} else if (!/^[a-z0-9-]+$/.test(this._content.slug)) {
			errors.push({
				field: 'slug',
				message: 'Slug can only contain lowercase letters, numbers, and hyphens'
			});
		}

		// SEO validation
		if (this._content.meta.metaTitle && this._content.meta.metaTitle.length > 70) {
			errors.push({ field: 'meta.metaTitle', message: 'Meta title should be less than 70 characters' });
		}

		if (this._content.meta.metaDescription && this._content.meta.metaDescription.length > 160) {
			errors.push({
				field: 'meta.metaDescription',
				message: 'Meta description should be less than 160 characters'
			});
		}

		this._errors = errors;
		this.notifySubscribers();
		return errors;
	}

	/**
	 * Get error for specific field
	 */
	getFieldError(field: string): string | null {
		const error = this._errors.find((e) => e.field === field);
		return error?.message ?? null;
	}

	// ==========================================================================
	// Save Operations
	// ==========================================================================

	/**
	 * Save content
	 */
	async save(): Promise<boolean> {
		if (!this._content || this._isSaving) return false;

		const errors = this.validate();
		if (errors.length > 0) return false;

		this._isSaving = true;
		this.notifySubscribers();

		try {
			const response = await fetch(`/api/cms/content/${this._content.id}`, {
				method: 'PUT',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(this._content)
			});

			if (!response.ok) {
				throw new Error(`Save failed: ${response.status}`);
			}

			const result = await response.json();
			const savedContent = result.data || result;

			// Update with server response (includes new version)
			this._content = { ...this._content, ...savedContent };
			this._originalContent = structuredClone(this._content);
			this._isDirty = false;
			this._lastSavedAt = new Date();

			// Clear autosaved draft
			if (browser && this._content.id) {
				autosaveStore.clearDraft(this._content.id);
			}

			return true;
		} catch (error) {
			console.error('[Editor] Save failed:', error);
			this._errors = [{ field: 'save', message: 'Failed to save content' }];
			return false;
		} finally {
			this._isSaving = false;
			this.notifySubscribers();
		}
	}

	/**
	 * Discard changes and revert to original
	 */
	discardChanges(): void {
		if (!this._originalContent) return;

		this._content = structuredClone(this._originalContent);
		this._isDirty = false;
		this._errors = [];
		this._history = [
			{ content: structuredClone(this._originalContent), timestamp: Date.now(), action: 'discard' }
		];
		this._historyIndex = 0;

		if (browser && this._content?.id) {
			autosaveStore.clearDraft(this._content.id);
		}

		this.notifySubscribers();
	}

	// ==========================================================================
	// Editor Mode & View
	// ==========================================================================

	/**
	 * Set editor mode
	 */
	setEditorMode(mode: EditorMode): void {
		this._editorMode = mode;
		this.notifySubscribers();
	}

	/**
	 * Toggle preview mode
	 */
	togglePreview(): void {
		this._isPreviewMode = !this._isPreviewMode;
		this.notifySubscribers();
	}

	/**
	 * Toggle fullscreen
	 */
	toggleFullscreen(): void {
		this._isFullscreen = !this._isFullscreen;
		this.notifySubscribers();
	}

	// ==========================================================================
	// Collaboration
	// ==========================================================================

	/**
	 * Set lock status
	 */
	setLockedBy(userId: string | null): void {
		this._lockedBy = userId;
		this.notifySubscribers();
	}

	/**
	 * Update collaborators
	 */
	setCollaborators(userIds: string[]): void {
		this._collaborators = userIds;
		this.notifySubscribers();
	}

	// ==========================================================================
	// Reset
	// ==========================================================================

	/**
	 * Reset editor state
	 */
	reset(): void {
		this._content = null;
		this._originalContent = null;
		this._isDirty = false;
		this._isLoading = false;
		this._isSaving = false;
		this._lastSavedAt = null;
		this._errors = [];
		this._editorMode = 'richtext';
		this._isPreviewMode = false;
		this._isFullscreen = false;
		this._history = [];
		this._historyIndex = -1;
		this._activeBlockId = null;
		this._selectedBlockIds = [];
		this._lockedBy = null;
		this._collaborators = [];
		this.notifySubscribers();
	}
}

// =============================================================================
// State Snapshot Interface
// =============================================================================

export interface EditorStateSnapshot {
	content: EditorContent | null;
	isDirty: boolean;
	isLoading: boolean;
	isSaving: boolean;
	errors: ValidationError[];
	editorMode: EditorMode;
	isPreviewMode: boolean;
	canUndo: boolean;
	canRedo: boolean;
}

// =============================================================================
// Export Store Instance
// =============================================================================

export const editorStore = new EditorStoreClass();

// =============================================================================
// Convenience Exports
// =============================================================================

export const content = {
	get current() {
		return editorStore.content;
	},
	subscribe(fn: (value: EditorContent | null) => void) {
		return editorStore.subscribe((snapshot) => fn(snapshot.content));
	}
};

export const isDirty = {
	get current() {
		return editorStore.isDirty;
	},
	subscribe(fn: (value: boolean) => void) {
		return editorStore.subscribe((snapshot) => fn(snapshot.isDirty));
	}
};

export const isSaving = {
	get current() {
		return editorStore.isSaving;
	},
	subscribe(fn: (value: boolean) => void) {
		return editorStore.subscribe((snapshot) => fn(snapshot.isSaving));
	}
};

export const editorErrors = {
	get current() {
		return editorStore.errors;
	},
	subscribe(fn: (value: ValidationError[]) => void) {
		return editorStore.subscribe((snapshot) => fn(snapshot.errors));
	}
};
