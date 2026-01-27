/**
 * CMS Comments Store - Enterprise Comment Management
 * ===================================================
 * SVELTE 5 RUNES VERSION
 *
 * Comprehensive comment system with:
 * - Thread hierarchy support
 * - @mention parsing and autocomplete
 * - Resolve/unresolve workflow
 * - Real-time notification management
 * - Inline text selection comments
 *
 * @version 2.0.0 - January 2026
 * @author Revolution Trading Pros
 */

import { browser } from '$app/environment';

// =============================================================================
// Type Definitions
// =============================================================================

export interface CmsUser {
	id: string;
	email: string;
	display_name: string;
	avatar_url?: string;
	role: string;
}

export interface CmsComment {
	id: string;
	content_id: string;
	parent_id: string | null;
	thread_id: string | null;
	body: string;
	block_id: string | null;
	selection_start: number | null;
	selection_end: number | null;
	is_resolved: boolean;
	resolved_by: string | null;
	resolved_at: string | null;
	mentioned_users: string[];
	deleted_at: string | null;
	created_at: string;
	updated_at: string;
	created_by: string | null;
	// Populated fields
	author?: CmsUser;
	resolver?: CmsUser;
	replies?: CmsComment[];
}

export interface CommentNotification {
	id: string;
	comment_id: string;
	user_id: string;
	notification_type: 'mention' | 'reply' | 'resolved';
	is_read: boolean;
	read_at: string | null;
	created_at: string;
	comment?: CmsComment;
}

export interface CreateCommentRequest {
	content_id: string;
	parent_id?: string;
	body: string;
	block_id?: string;
	selection_start?: number;
	selection_end?: number;
	mentioned_users?: string[];
}

export interface UpdateCommentRequest {
	body?: string;
	mentioned_users?: string[];
}

export type CommentFilter = 'all' | 'open' | 'resolved';

export interface TextSelection {
	blockId: string;
	start: number;
	end: number;
	text: string;
	rect?: DOMRect;
}

// =============================================================================
// Comments Store Class - Svelte 5 Runes
// =============================================================================

class CommentsStoreClass {
	// Core state
	private _comments = $state<CmsComment[]>([]);
	private _notifications = $state<CommentNotification[]>([]);
	private _users = $state<CmsUser[]>([]);
	private _isLoading = $state(false);
	private _isSaving = $state(false);
	private _error = $state<string | null>(null);

	// UI state
	private _filter = $state<CommentFilter>('all');
	private _activeCommentId = $state<string | null>(null);
	private _currentContentId = $state<string | null>(null);
	private _textSelection = $state<TextSelection | null>(null);
	private _mentionQuery = $state<string>('');
	private _showMentionDropdown = $state(false);

	// Subscribers for backward compatibility
	private subscribers = new Set<(value: CommentsStateSnapshot) => void>();

	// ==========================================================================
	// Getters for reactive state access
	// ==========================================================================

	get comments(): CmsComment[] {
		return this._comments;
	}
	get notifications(): CommentNotification[] {
		return this._notifications;
	}
	get users(): CmsUser[] {
		return this._users;
	}
	get isLoading(): boolean {
		return this._isLoading;
	}
	get isSaving(): boolean {
		return this._isSaving;
	}
	get error(): string | null {
		return this._error;
	}
	get filter(): CommentFilter {
		return this._filter;
	}
	get activeCommentId(): string | null {
		return this._activeCommentId;
	}
	get currentContentId(): string | null {
		return this._currentContentId;
	}
	get textSelection(): TextSelection | null {
		return this._textSelection;
	}
	get mentionQuery(): string {
		return this._mentionQuery;
	}
	get showMentionDropdown(): boolean {
		return this._showMentionDropdown;
	}

	// Derived getters
	get filteredComments(): CmsComment[] {
		const rootComments = this._comments.filter((c) => !c.parent_id && !c.deleted_at);

		switch (this._filter) {
			case 'open':
				return rootComments.filter((c) => !c.is_resolved);
			case 'resolved':
				return rootComments.filter((c) => c.is_resolved);
			default:
				return rootComments;
		}
	}

	get threadedComments(): CmsComment[] {
		// Build thread hierarchy
		const commentMap = new Map<string, CmsComment>();
		const rootComments: CmsComment[] = [];

		// First pass: create map
		for (const comment of this._comments) {
			if (!comment.deleted_at) {
				commentMap.set(comment.id, { ...comment, replies: [] });
			}
		}

		// Second pass: build hierarchy
		for (const comment of commentMap.values()) {
			if (comment.parent_id && commentMap.has(comment.parent_id)) {
				const parent = commentMap.get(comment.parent_id)!;
				parent.replies = parent.replies || [];
				parent.replies.push(comment);
			} else if (!comment.parent_id) {
				rootComments.push(comment);
			}
		}

		// Sort by created_at
		rootComments.sort(
			(a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
		);

		// Sort replies by created_at
		for (const comment of rootComments) {
			if (comment.replies) {
				comment.replies.sort(
					(a, b) => new Date(a.created_at).getTime() - new Date(b.created_at).getTime()
				);
			}
		}

		// Apply filter
		switch (this._filter) {
			case 'open':
				return rootComments.filter((c) => !c.is_resolved);
			case 'resolved':
				return rootComments.filter((c) => c.is_resolved);
			default:
				return rootComments;
		}
	}

	get unreadCount(): number {
		return this._notifications.filter((n) => !n.is_read).length;
	}

	get unreadNotifications(): CommentNotification[] {
		return this._notifications.filter((n) => !n.is_read);
	}

	get commentsByBlock(): Map<string, CmsComment[]> {
		const map = new Map<string, CmsComment[]>();
		for (const comment of this._comments) {
			if (comment.block_id && !comment.deleted_at) {
				const blockComments = map.get(comment.block_id) || [];
				blockComments.push(comment);
				map.set(comment.block_id, blockComments);
			}
		}
		return map;
	}

	get inlineComments(): CmsComment[] {
		return this._comments.filter(
			(c) =>
				c.block_id &&
				c.selection_start !== null &&
				c.selection_end !== null &&
				!c.deleted_at &&
				!c.parent_id
		);
	}

	get mentionSuggestions(): CmsUser[] {
		if (!this._mentionQuery) return this._users;
		const query = this._mentionQuery.toLowerCase();
		return this._users.filter(
			(u) =>
				u.display_name.toLowerCase().includes(query) || u.email.toLowerCase().includes(query)
		);
	}

	// ==========================================================================
	// Subscribe method for backward compatibility
	// ==========================================================================

	subscribe(fn: (value: CommentsStateSnapshot) => void): () => void {
		this.subscribers.add(fn);
		fn(this.getSnapshot());
		return () => {
			this.subscribers.delete(fn);
		};
	}

	private getSnapshot(): CommentsStateSnapshot {
		return {
			comments: this._comments,
			notifications: this._notifications,
			isLoading: this._isLoading,
			isSaving: this._isSaving,
			error: this._error,
			filter: this._filter,
			unreadCount: this.unreadCount,
			activeCommentId: this._activeCommentId
		};
	}

	private notifySubscribers(): void {
		const snapshot = this.getSnapshot();
		this.subscribers.forEach((fn) => fn(snapshot));
	}

	// ==========================================================================
	// API Methods
	// ==========================================================================

	/**
	 * Load comments for content
	 */
	async loadComments(contentId: string): Promise<void> {
		if (!browser) return;

		this._isLoading = true;
		this._error = null;
		this._currentContentId = contentId;
		this.notifySubscribers();

		try {
			const response = await fetch(`/api/cms/content/${contentId}/comments`);

			if (!response.ok) {
				throw new Error(`Failed to load comments: ${response.status}`);
			}

			const result = await response.json();
			this._comments = result.data || result.comments || [];

			if (import.meta.env.DEV) {
				console.debug('[Comments] Loaded', this._comments.length, 'comments');
			}
		} catch (error) {
			console.error('[Comments] Load failed:', error);
			this._error = error instanceof Error ? error.message : 'Failed to load comments';
		} finally {
			this._isLoading = false;
			this.notifySubscribers();
		}
	}

	/**
	 * Load CMS users for @mention autocomplete
	 */
	async loadUsers(): Promise<void> {
		if (!browser) return;

		try {
			const response = await fetch('/api/cms/users');

			if (!response.ok) {
				throw new Error(`Failed to load users: ${response.status}`);
			}

			const result = await response.json();
			this._users = result.data || result.users || [];
		} catch (error) {
			console.error('[Comments] Load users failed:', error);
		}
	}

	/**
	 * Load notifications for current user
	 */
	async loadNotifications(): Promise<void> {
		if (!browser) return;

		try {
			const response = await fetch('/api/cms/comments/notifications');

			if (!response.ok) {
				throw new Error(`Failed to load notifications: ${response.status}`);
			}

			const result = await response.json();
			this._notifications = result.data || result.notifications || [];
			this.notifySubscribers();
		} catch (error) {
			console.error('[Comments] Load notifications failed:', error);
		}
	}

	/**
	 * Create a new comment
	 */
	async createComment(request: CreateCommentRequest): Promise<CmsComment | null> {
		if (!browser) return null;

		this._isSaving = true;
		this._error = null;
		this.notifySubscribers();

		try {
			// Parse @mentions from body
			const mentionedUsers = this.parseMentions(request.body);
			const finalRequest = {
				...request,
				mentioned_users: [...(request.mentioned_users || []), ...mentionedUsers]
			};

			const response = await fetch(`/api/cms/content/${request.content_id}/comments`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(finalRequest)
			});

			if (!response.ok) {
				throw new Error(`Failed to create comment: ${response.status}`);
			}

			const result = await response.json();
			const newComment = result.data || result.comment || result;

			// Add to local state
			this._comments = [...this._comments, newComment];
			this.notifySubscribers();

			if (import.meta.env.DEV) {
				console.debug('[Comments] Created comment:', newComment.id);
			}

			return newComment;
		} catch (error) {
			console.error('[Comments] Create failed:', error);
			this._error = error instanceof Error ? error.message : 'Failed to create comment';
			return null;
		} finally {
			this._isSaving = false;
			this.notifySubscribers();
		}
	}

	/**
	 * Update a comment
	 */
	async updateComment(commentId: string, request: UpdateCommentRequest): Promise<boolean> {
		if (!browser) return false;

		this._isSaving = true;
		this._error = null;
		this.notifySubscribers();

		try {
			// Parse @mentions from body if updating body
			let finalRequest = { ...request };
			if (request.body) {
				const mentionedUsers = this.parseMentions(request.body);
				finalRequest.mentioned_users = [
					...(request.mentioned_users || []),
					...mentionedUsers
				];
			}

			const response = await fetch(`/api/cms/comments/${commentId}`, {
				method: 'PATCH',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(finalRequest)
			});

			if (!response.ok) {
				throw new Error(`Failed to update comment: ${response.status}`);
			}

			const result = await response.json();
			const updatedComment = result.data || result.comment || result;

			// Update local state
			this._comments = this._comments.map((c) =>
				c.id === commentId ? { ...c, ...updatedComment } : c
			);
			this.notifySubscribers();

			return true;
		} catch (error) {
			console.error('[Comments] Update failed:', error);
			this._error = error instanceof Error ? error.message : 'Failed to update comment';
			return false;
		} finally {
			this._isSaving = false;
			this.notifySubscribers();
		}
	}

	/**
	 * Delete a comment (soft delete)
	 */
	async deleteComment(commentId: string): Promise<boolean> {
		if (!browser) return false;

		this._isSaving = true;
		this._error = null;
		this.notifySubscribers();

		try {
			const response = await fetch(`/api/cms/comments/${commentId}`, {
				method: 'DELETE'
			});

			if (!response.ok) {
				throw new Error(`Failed to delete comment: ${response.status}`);
			}

			// Update local state
			this._comments = this._comments.map((c) =>
				c.id === commentId ? { ...c, deleted_at: new Date().toISOString() } : c
			);
			this.notifySubscribers();

			return true;
		} catch (error) {
			console.error('[Comments] Delete failed:', error);
			this._error = error instanceof Error ? error.message : 'Failed to delete comment';
			return false;
		} finally {
			this._isSaving = false;
			this.notifySubscribers();
		}
	}

	/**
	 * Resolve a comment
	 */
	async resolveComment(commentId: string): Promise<boolean> {
		if (!browser) return false;

		this._isSaving = true;
		this._error = null;
		this.notifySubscribers();

		try {
			const response = await fetch(`/api/cms/comments/${commentId}/resolve`, {
				method: 'POST'
			});

			if (!response.ok) {
				throw new Error(`Failed to resolve comment: ${response.status}`);
			}

			const result = await response.json();

			// Update local state
			this._comments = this._comments.map((c) =>
				c.id === commentId
					? {
							...c,
							is_resolved: true,
							resolved_at: new Date().toISOString(),
							resolved_by: result.resolved_by || null
						}
					: c
			);
			this.notifySubscribers();

			return true;
		} catch (error) {
			console.error('[Comments] Resolve failed:', error);
			this._error = error instanceof Error ? error.message : 'Failed to resolve comment';
			return false;
		} finally {
			this._isSaving = false;
			this.notifySubscribers();
		}
	}

	/**
	 * Unresolve a comment
	 */
	async unresolveComment(commentId: string): Promise<boolean> {
		if (!browser) return false;

		this._isSaving = true;
		this._error = null;
		this.notifySubscribers();

		try {
			const response = await fetch(`/api/cms/comments/${commentId}/unresolve`, {
				method: 'POST'
			});

			if (!response.ok) {
				throw new Error(`Failed to unresolve comment: ${response.status}`);
			}

			// Update local state
			this._comments = this._comments.map((c) =>
				c.id === commentId
					? {
							...c,
							is_resolved: false,
							resolved_at: null,
							resolved_by: null
						}
					: c
			);
			this.notifySubscribers();

			return true;
		} catch (error) {
			console.error('[Comments] Unresolve failed:', error);
			this._error = error instanceof Error ? error.message : 'Failed to unresolve comment';
			return false;
		} finally {
			this._isSaving = false;
			this.notifySubscribers();
		}
	}

	/**
	 * Mark notification as read
	 */
	async markNotificationRead(notificationId: string): Promise<void> {
		if (!browser) return;

		try {
			const response = await fetch(`/api/cms/comments/notifications/${notificationId}/read`, {
				method: 'POST'
			});

			if (!response.ok) {
				throw new Error(`Failed to mark notification as read: ${response.status}`);
			}

			// Update local state
			this._notifications = this._notifications.map((n) =>
				n.id === notificationId
					? { ...n, is_read: true, read_at: new Date().toISOString() }
					: n
			);
			this.notifySubscribers();
		} catch (error) {
			console.error('[Comments] Mark read failed:', error);
		}
	}

	/**
	 * Mark all notifications as read
	 */
	async markAllNotificationsRead(): Promise<void> {
		if (!browser) return;

		try {
			const response = await fetch('/api/cms/comments/notifications/read-all', {
				method: 'POST'
			});

			if (!response.ok) {
				throw new Error(`Failed to mark all notifications as read: ${response.status}`);
			}

			// Update local state
			const now = new Date().toISOString();
			this._notifications = this._notifications.map((n) => ({
				...n,
				is_read: true,
				read_at: now
			}));
			this.notifySubscribers();
		} catch (error) {
			console.error('[Comments] Mark all read failed:', error);
		}
	}

	// ==========================================================================
	// @Mention Parsing
	// ==========================================================================

	/**
	 * Parse @mentions from comment body
	 */
	parseMentions(text: string): string[] {
		const mentionPattern = /@\[([^\]]+)\]\(([^)]+)\)/g;
		const mentions: string[] = [];
		let match;

		while ((match = mentionPattern.exec(text)) !== null) {
			const userId = match[2];
			if (userId && !mentions.includes(userId)) {
				mentions.push(userId);
			}
		}

		return mentions;
	}

	/**
	 * Format mention for display
	 */
	formatMention(user: CmsUser): string {
		return `@[${user.display_name}](${user.id})`;
	}

	/**
	 * Render mentions as display names
	 */
	renderMentions(text: string): string {
		return text.replace(/@\[([^\]]+)\]\([^)]+\)/g, '<span class="mention">@$1</span>');
	}

	/**
	 * Set mention query for autocomplete
	 */
	setMentionQuery(query: string): void {
		this._mentionQuery = query;
		this._showMentionDropdown = query.length > 0;
		this.notifySubscribers();
	}

	/**
	 * Hide mention dropdown
	 */
	hideMentionDropdown(): void {
		this._showMentionDropdown = false;
		this._mentionQuery = '';
		this.notifySubscribers();
	}

	// ==========================================================================
	// UI State Management
	// ==========================================================================

	/**
	 * Set filter
	 */
	setFilter(filter: CommentFilter): void {
		this._filter = filter;
		this.notifySubscribers();
	}

	/**
	 * Set active comment
	 */
	setActiveComment(commentId: string | null): void {
		this._activeCommentId = commentId;
		this.notifySubscribers();
	}

	/**
	 * Set text selection for inline comment
	 */
	setTextSelection(selection: TextSelection | null): void {
		this._textSelection = selection;
		this.notifySubscribers();
	}

	/**
	 * Clear text selection
	 */
	clearTextSelection(): void {
		this._textSelection = null;
		this.notifySubscribers();
	}

	/**
	 * Get comment by ID
	 */
	getComment(commentId: string): CmsComment | undefined {
		return this._comments.find((c) => c.id === commentId);
	}

	/**
	 * Get comments for a specific block
	 */
	getBlockComments(blockId: string): CmsComment[] {
		return this._comments.filter((c) => c.block_id === blockId && !c.deleted_at);
	}

	/**
	 * Get inline comments for a block with selection range
	 */
	getInlineCommentsForBlock(blockId: string): CmsComment[] {
		return this._comments.filter(
			(c) =>
				c.block_id === blockId &&
				c.selection_start !== null &&
				c.selection_end !== null &&
				!c.deleted_at &&
				!c.parent_id
		);
	}

	/**
	 * Check if text range overlaps with any comment
	 */
	hasCommentInRange(blockId: string, start: number, end: number): CmsComment | null {
		return (
			this._comments.find(
				(c) =>
					c.block_id === blockId &&
					c.selection_start !== null &&
					c.selection_end !== null &&
					!c.deleted_at &&
					((start >= c.selection_start && start < c.selection_end) ||
						(end > c.selection_start && end <= c.selection_end) ||
						(start <= c.selection_start && end >= c.selection_end))
			) || null
		);
	}

	// ==========================================================================
	// Reset
	// ==========================================================================

	/**
	 * Reset store state
	 */
	reset(): void {
		this._comments = [];
		this._notifications = [];
		this._users = [];
		this._isLoading = false;
		this._isSaving = false;
		this._error = null;
		this._filter = 'all';
		this._activeCommentId = null;
		this._currentContentId = null;
		this._textSelection = null;
		this._mentionQuery = '';
		this._showMentionDropdown = false;
		this.notifySubscribers();
	}

	/**
	 * Clear error
	 */
	clearError(): void {
		this._error = null;
		this.notifySubscribers();
	}
}

// =============================================================================
// State Snapshot Interface
// =============================================================================

export interface CommentsStateSnapshot {
	comments: CmsComment[];
	notifications: CommentNotification[];
	isLoading: boolean;
	isSaving: boolean;
	error: string | null;
	filter: CommentFilter;
	unreadCount: number;
	activeCommentId: string | null;
}

// =============================================================================
// Export Store Instance
// =============================================================================

export const commentsStore = new CommentsStoreClass();

// =============================================================================
// Convenience Exports
// =============================================================================

export const comments = {
	get current() {
		return commentsStore.comments;
	},
	subscribe(fn: (value: CmsComment[]) => void) {
		return commentsStore.subscribe((snapshot) => fn(snapshot.comments));
	}
};

export const commentsUnreadCount = {
	get current() {
		return commentsStore.unreadCount;
	},
	subscribe(fn: (value: number) => void) {
		return commentsStore.subscribe((snapshot) => fn(snapshot.unreadCount));
	}
};

export const commentsFilter = {
	get current() {
		return commentsStore.filter;
	},
	subscribe(fn: (value: CommentFilter) => void) {
		return commentsStore.subscribe((snapshot) => fn(snapshot.filter));
	}
};

export const commentsIsLoading = {
	get current() {
		return commentsStore.isLoading;
	},
	subscribe(fn: (value: boolean) => void) {
		return commentsStore.subscribe((snapshot) => fn(snapshot.isLoading));
	}
};
