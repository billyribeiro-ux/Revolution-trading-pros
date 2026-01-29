<!--
  CMS Comments Sidebar Component
  ==============================
  Enterprise-grade comment management sidebar for CMS editor.

  Features:
  - Thread hierarchy display
  - Filter by All/Open/Resolved
  - @mention autocomplete
  - Resolve/unresolve workflow
  - Unread notification badge
  - Accessible with ARIA attributes

  @version 2.0.0 - January 2026
  @author Revolution Trading Pros
-->
<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import {
		commentsStore,
		type CmsComment,
		type CmsUser,
		type CommentFilter
	} from '$lib/stores/comments.svelte';

	// ==========================================================================
	// Props
	// ==========================================================================

	interface Props {
		contentId: string;
		currentBlockId?: string | null;
		onCommentClick?: (comment: CmsComment) => void;
		onClose?: () => void;
	}

	let { contentId, currentBlockId = null, onCommentClick, onClose }: Props = $props();

	// ==========================================================================
	// Local State
	// ==========================================================================

	let newCommentBody = $state('');
	let replyingTo = $state<string | null>(null);
	let replyBody = $state('');
	let editingId = $state<string | null>(null);
	let editBody = $state('');
	let mentionCursorPosition = $state(0);
	let activeTextarea = $state<HTMLTextAreaElement | null>(null);
	let isExpanded = $state<Set<string>>(new Set());

	// Derived state from store
	let comments = $derived(commentsStore.threadedComments);
	let filter = $derived(commentsStore.filter);
	let unreadCount = $derived(commentsStore.unreadCount);
	let isLoading = $derived(commentsStore.isLoading);
	let isSaving = $derived(commentsStore.isSaving);
	let users = $derived(commentsStore.users);
	let mentionSuggestions = $derived(commentsStore.mentionSuggestions);
	let showMentionDropdown = $derived(commentsStore.showMentionDropdown);

	// ==========================================================================
	// Lifecycle
	// ==========================================================================

	onMount(async () => {
		await Promise.all([commentsStore.loadComments(contentId), commentsStore.loadUsers()]);
		await commentsStore.loadNotifications();
	});

	onDestroy(() => {
		commentsStore.hideMentionDropdown();
	});

	// ==========================================================================
	// Handlers
	// ==========================================================================

	function handleFilterChange(newFilter: CommentFilter) {
		commentsStore.setFilter(newFilter);
	}

	async function handleSubmitComment() {
		if (!newCommentBody.trim()) return;

		const comment = await commentsStore.createComment({
			content_id: contentId,
			body: newCommentBody,
			block_id: currentBlockId || undefined
		});

		if (comment) {
			newCommentBody = '';
		}
	}

	async function handleSubmitReply(parentId: string) {
		if (!replyBody.trim()) return;

		const parent = commentsStore.getComment(parentId);
		const comment = await commentsStore.createComment({
			content_id: contentId,
			parent_id: parentId,
			body: replyBody
		});

		if (comment) {
			replyBody = '';
			replyingTo = null;
		}
	}

	async function handleUpdateComment(commentId: string) {
		if (!editBody.trim()) return;

		const success = await commentsStore.updateComment(commentId, {
			body: editBody
		});

		if (success) {
			editBody = '';
			editingId = null;
		}
	}

	async function handleDeleteComment(commentId: string) {
		if (!confirm('Are you sure you want to delete this comment?')) return;
		await commentsStore.deleteComment(commentId);
	}

	async function handleResolve(commentId: string) {
		await commentsStore.resolveComment(commentId);
	}

	async function handleUnresolve(commentId: string) {
		await commentsStore.unresolveComment(commentId);
	}

	function handleCommentClick(comment: CmsComment) {
		commentsStore.setActiveComment(comment.id);
		onCommentClick?.(comment);
	}

	function startReply(commentId: string) {
		replyingTo = commentId;
		replyBody = '';
	}

	function cancelReply() {
		replyingTo = null;
		replyBody = '';
	}

	function startEdit(comment: CmsComment) {
		editingId = comment.id;
		editBody = comment.body;
	}

	function cancelEdit() {
		editingId = null;
		editBody = '';
	}

	function toggleExpand(commentId: string) {
		const newExpanded = new Set(isExpanded);
		if (newExpanded.has(commentId)) {
			newExpanded.delete(commentId);
		} else {
			newExpanded.add(commentId);
		}
		isExpanded = newExpanded;
	}

	// ==========================================================================
	// @Mention Handlers
	// ==========================================================================

	function handleTextareaInput(event: Event, textareaRef: HTMLTextAreaElement) {
		const target = event.target as HTMLTextAreaElement;
		const value = target.value;
		const cursorPos = target.selectionStart;
		activeTextarea = textareaRef;
		mentionCursorPosition = cursorPos;

		// Check for @ trigger
		const textBeforeCursor = value.substring(0, cursorPos);
		const mentionMatch = textBeforeCursor.match(/@(\w*)$/);

		if (mentionMatch) {
			commentsStore.setMentionQuery(mentionMatch[1]);
		} else {
			commentsStore.hideMentionDropdown();
		}
	}

	function insertMention(user: CmsUser, setBody: (value: string) => void, currentBody: string) {
		if (!activeTextarea) return;

		const textBeforeCursor = currentBody.substring(0, mentionCursorPosition);
		const textAfterCursor = currentBody.substring(mentionCursorPosition);

		// Find the @ position
		const mentionStart = textBeforeCursor.lastIndexOf('@');
		const newText =
			textBeforeCursor.substring(0, mentionStart) +
			commentsStore.formatMention(user) +
			' ' +
			textAfterCursor;

		setBody(newText);
		commentsStore.hideMentionDropdown();

		// Focus back on textarea
		setTimeout(() => {
			activeTextarea?.focus();
		}, 0);
	}

	// ==========================================================================
	// Utilities
	// ==========================================================================

	function formatDate(dateString: string): string {
		const date = new Date(dateString);
		const now = new Date();
		const diff = now.getTime() - date.getTime();

		if (diff < 60000) return 'Just now';
		if (diff < 3600000) return `${Math.floor(diff / 60000)}m ago`;
		if (diff < 86400000) return `${Math.floor(diff / 3600000)}h ago`;
		if (diff < 604800000) return `${Math.floor(diff / 86400000)}d ago`;

		return date.toLocaleDateString();
	}

	function getUserInitials(user?: CmsUser): string {
		if (!user?.display_name) return '?';
		return user.display_name
			.split(' ')
			.map((n) => n[0])
			.join('')
			.toUpperCase()
			.slice(0, 2);
	}

	function renderCommentBody(body: string): string {
		return commentsStore.renderMentions(body);
	}
</script>

<aside
	class="flex h-full w-80 flex-col border-l border-gray-200 bg-white"
	aria-label="Comments sidebar"
>
	<!-- Header -->
	<header class="flex items-center justify-between border-b border-gray-200 px-4 py-3">
		<div class="flex items-center gap-2">
			<h2 class="text-lg font-semibold text-gray-900">Comments</h2>
			{#if unreadCount > 0}
				<span
					class="flex h-5 min-w-5 items-center justify-center rounded-full bg-blue-600 px-1.5 text-xs font-medium text-white"
					aria-label="{unreadCount} unread notifications"
				>
					{unreadCount}
				</span>
			{/if}
		</div>
		{#if onClose}
			<button
				type="button"
				onclick={onClose}
				class="rounded-lg p-1.5 text-gray-500 transition-colors hover:bg-gray-100 hover:text-gray-700"
				aria-label="Close comments sidebar"
			>
				<svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M6 18L18 6M6 6l12 12"
					/>
				</svg>
			</button>
		{/if}
	</header>

	<!-- Filters -->
	<div class="border-b border-gray-200 px-4 py-2" role="tablist" aria-label="Comment filters">
		<div class="flex gap-1">
			{#each ['all', 'open', 'resolved'] as filterOption}
				<button
					type="button"
					role="tab"
					aria-selected={filter === filterOption}
					onclick={() => handleFilterChange(filterOption as CommentFilter)}
					class="rounded-md px-3 py-1.5 text-sm font-medium capitalize transition-colors
						{filter === filterOption
						? 'bg-blue-100 text-blue-700'
						: 'text-gray-600 hover:bg-gray-100 hover:text-gray-900'}"
				>
					{filterOption}
				</button>
			{/each}
		</div>
	</div>

	<!-- New Comment Form -->
	<div class="border-b border-gray-200 p-4">
		<div class="relative">
			<textarea
				bind:value={newCommentBody}
				oninput={(e) => handleTextareaInput(e, e.target as HTMLTextAreaElement)}
				placeholder="Add a comment... Use @ to mention"
				class="w-full resize-none rounded-lg border border-gray-300 p-3 text-sm transition-colors
					placeholder:text-gray-400 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
				rows="3"
				aria-label="New comment"
			></textarea>

			<!-- Mention Dropdown -->
			{#if showMentionDropdown && mentionSuggestions.length > 0}
				<div
					class="absolute left-0 right-0 z-10 mt-1 max-h-48 overflow-auto rounded-lg border border-gray-200 bg-white shadow-lg"
					role="listbox"
					aria-label="User suggestions"
				>
					{#each mentionSuggestions as user}
						<button
							type="button"
							role="option"
							aria-selected="false"
							onclick={() => insertMention(user, (v) => (newCommentBody = v), newCommentBody)}
							class="flex w-full items-center gap-2 px-3 py-2 text-left text-sm transition-colors hover:bg-gray-50"
						>
							<span
								class="flex h-6 w-6 items-center justify-center rounded-full bg-gray-200 text-xs font-medium text-gray-600"
							>
								{getUserInitials(user)}
							</span>
							<span class="text-gray-900">{user.display_name}</span>
							<span class="text-gray-500">({user.email})</span>
						</button>
					{/each}
				</div>
			{/if}
		</div>
		<div class="mt-2 flex justify-end">
			<button
				type="button"
				onclick={handleSubmitComment}
				disabled={!newCommentBody.trim() || isSaving}
				class="rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white transition-colors
					hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2
					disabled:cursor-not-allowed disabled:opacity-50"
			>
				{isSaving ? 'Posting...' : 'Post Comment'}
			</button>
		</div>
	</div>

	<!-- Comments List -->
	<div class="flex-1 overflow-y-auto" role="list" aria-label="Comments list">
		{#if isLoading}
			<div class="flex items-center justify-center py-8">
				<div
					class="h-8 w-8 animate-spin rounded-full border-2 border-gray-300 border-t-blue-600"
				></div>
			</div>
		{:else if comments.length === 0}
			<div class="py-8 text-center text-gray-500">
				<svg
					class="mx-auto h-12 w-12 text-gray-300"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="1.5"
						d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"
					/>
				</svg>
				<p class="mt-2">No comments yet</p>
				<p class="text-sm">Be the first to add a comment</p>
			</div>
		{:else}
			{#each comments as comment (comment.id)}
				<article
					class="border-b border-gray-100 p-4 transition-colors hover:bg-gray-50
						{commentsStore.activeCommentId === comment.id ? 'bg-blue-50' : ''}"
					role="listitem"
					aria-labelledby="comment-{comment.id}"
				>
					<!-- Comment Header -->
					<div class="flex items-start justify-between gap-2">
						<button
							type="button"
							onclick={() => handleCommentClick(comment)}
							class="flex items-center gap-2 text-left"
						>
							<span
								class="flex h-8 w-8 items-center justify-center rounded-full bg-gray-200 text-xs font-medium text-gray-600"
							>
								{getUserInitials(comment.author)}
							</span>
							<div>
								<p id="comment-{comment.id}" class="text-sm font-medium text-gray-900">
									{comment.author?.display_name || 'Unknown User'}
								</p>
								<p class="text-xs text-gray-500">{formatDate(comment.created_at)}</p>
							</div>
						</button>

						<!-- Status Badge -->
						{#if comment.is_resolved}
							<span
								class="rounded-full bg-green-100 px-2 py-0.5 text-xs font-medium text-green-700"
							>
								Resolved
							</span>
						{/if}
					</div>

					<!-- Comment Body -->
					{#if editingId === comment.id}
						<div class="mt-3">
							<textarea
								bind:value={editBody}
								class="w-full resize-none rounded-lg border border-gray-300 p-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
								rows="3"
								aria-label="Edit comment"
							></textarea>
							<div class="mt-2 flex justify-end gap-2">
								<button
									type="button"
									onclick={cancelEdit}
									class="rounded-lg px-3 py-1.5 text-sm text-gray-600 hover:bg-gray-100"
								>
									Cancel
								</button>
								<button
									type="button"
									onclick={() => handleUpdateComment(comment.id)}
									disabled={!editBody.trim() || isSaving}
									class="rounded-lg bg-blue-600 px-3 py-1.5 text-sm text-white hover:bg-blue-700 disabled:opacity-50"
								>
									Save
								</button>
							</div>
						</div>
					{:else}
						<div class="mt-2 text-sm text-gray-700" role="article">
							{@html renderCommentBody(comment.body)}
						</div>

						<!-- Block Reference -->
						{#if comment.block_id}
							<div class="mt-2 flex items-center gap-1 text-xs text-gray-500">
								<svg class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1"
									/>
								</svg>
								<span>Attached to block</span>
							</div>
						{/if}

						<!-- Comment Actions -->
						<div class="mt-3 flex items-center gap-3">
							<button
								type="button"
								onclick={() => startReply(comment.id)}
								class="text-xs text-gray-500 hover:text-gray-700"
								aria-label="Reply to comment"
							>
								Reply
							</button>
							<button
								type="button"
								onclick={() => startEdit(comment)}
								class="text-xs text-gray-500 hover:text-gray-700"
								aria-label="Edit comment"
							>
								Edit
							</button>
							<button
								type="button"
								onclick={() => handleDeleteComment(comment.id)}
								class="text-xs text-gray-500 hover:text-red-600"
								aria-label="Delete comment"
							>
								Delete
							</button>
							<div class="flex-1"></div>
							{#if comment.is_resolved}
								<button
									type="button"
									onclick={() => handleUnresolve(comment.id)}
									class="flex items-center gap-1 text-xs text-gray-500 hover:text-orange-600"
									aria-label="Reopen comment"
								>
									<svg class="h-3.5 w-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
										/>
									</svg>
									Reopen
								</button>
							{:else}
								<button
									type="button"
									onclick={() => handleResolve(comment.id)}
									class="flex items-center gap-1 text-xs text-gray-500 hover:text-green-600"
									aria-label="Resolve comment"
								>
									<svg class="h-3.5 w-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M5 13l4 4L19 7"
										/>
									</svg>
									Resolve
								</button>
							{/if}
						</div>
					{/if}

					<!-- Reply Form -->
					{#if replyingTo === comment.id}
						<div class="mt-3 rounded-lg bg-gray-50 p-3" role="form" aria-label="Reply form">
							<textarea
								bind:value={replyBody}
								oninput={(e) => handleTextareaInput(e, e.target as HTMLTextAreaElement)}
								placeholder="Write a reply..."
								class="w-full resize-none rounded-lg border border-gray-300 bg-white p-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
								rows="2"
								aria-label="Reply text"
							></textarea>

							<!-- Mention Dropdown for Reply -->
							{#if showMentionDropdown && mentionSuggestions.length > 0}
								<div
									class="mt-1 max-h-32 overflow-auto rounded-lg border border-gray-200 bg-white shadow-lg"
									role="listbox"
								>
									{#each mentionSuggestions as user}
										<button
											type="button"
											role="option"
											aria-selected="false"
											onclick={() => insertMention(user, (v) => (replyBody = v), replyBody)}
											class="flex w-full items-center gap-2 px-3 py-2 text-left text-sm hover:bg-gray-50"
										>
											<span
												class="flex h-5 w-5 items-center justify-center rounded-full bg-gray-200 text-xs font-medium text-gray-600"
											>
												{getUserInitials(user)}
											</span>
											<span class="text-gray-900">{user.display_name}</span>
										</button>
									{/each}
								</div>
							{/if}

							<div class="mt-2 flex justify-end gap-2">
								<button
									type="button"
									onclick={cancelReply}
									class="rounded-lg px-3 py-1.5 text-sm text-gray-600 hover:bg-gray-200"
								>
									Cancel
								</button>
								<button
									type="button"
									onclick={() => handleSubmitReply(comment.id)}
									disabled={!replyBody.trim() || isSaving}
									class="rounded-lg bg-blue-600 px-3 py-1.5 text-sm text-white hover:bg-blue-700 disabled:opacity-50"
								>
									Reply
								</button>
							</div>
						</div>
					{/if}

					<!-- Replies Thread -->
					{#if comment.replies && comment.replies.length > 0}
						<div class="mt-3 border-l-2 border-gray-200 pl-4">
							{#if !isExpanded.has(comment.id) && comment.replies.length > 2}
								<button
									type="button"
									onclick={() => toggleExpand(comment.id)}
									class="mb-2 text-xs font-medium text-blue-600 hover:text-blue-700"
								>
									Show {comment.replies.length} replies
								</button>
							{:else}
								{#each comment.replies as reply (reply.id)}
									<div class="mb-3 last:mb-0" role="article">
										<div class="flex items-center gap-2">
											<span
												class="flex h-6 w-6 items-center justify-center rounded-full bg-gray-200 text-xs font-medium text-gray-600"
											>
												{getUserInitials(reply.author)}
											</span>
											<span class="text-sm font-medium text-gray-900">
												{reply.author?.display_name || 'Unknown User'}
											</span>
											<span class="text-xs text-gray-500">{formatDate(reply.created_at)}</span>
										</div>
										<div class="mt-1 pl-8 text-sm text-gray-700">
											{@html renderCommentBody(reply.body)}
										</div>
									</div>
								{/each}
								{#if comment.replies.length > 2}
									<button
										type="button"
										onclick={() => toggleExpand(comment.id)}
										class="text-xs font-medium text-blue-600 hover:text-blue-700"
									>
										Hide replies
									</button>
								{/if}
							{/if}
						</div>
					{/if}
				</article>
			{/each}
		{/if}
	</div>
</aside>

<style>
	/* Mention highlighting */
	:global(.mention) {
		background-color: rgb(219 234 254);
		color: rgb(29 78 216);
		padding: 0.125rem 0.25rem;
		border-radius: 0.25rem;
		font-weight: 500;
	}

	/* Smooth transitions */
	aside {
		transition: transform 0.3s ease-in-out;
	}

	/* Custom scrollbar */
	.overflow-y-auto::-webkit-scrollbar {
		width: 6px;
	}

	.overflow-y-auto::-webkit-scrollbar-track {
		background: transparent;
	}

	.overflow-y-auto::-webkit-scrollbar-thumb {
		background-color: rgb(209 213 219);
		border-radius: 3px;
	}

	.overflow-y-auto::-webkit-scrollbar-thumb:hover {
		background-color: rgb(156 163 175);
	}
</style>
