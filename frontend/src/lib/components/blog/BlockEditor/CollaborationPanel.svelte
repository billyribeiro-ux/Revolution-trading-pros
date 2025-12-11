<script lang="ts">
	/**
	 * Collaboration Panel - Enterprise-Grade Team Features
	 * =====================================================
	 * Comments, annotations, suggestions, and real-time
	 * collaboration for team content editing.
	 *
	 * @version 1.0.0
	 * @author Revolution Trading Pros
	 */

	interface Comment {
		id: string;
		blockId: string | null; // null for general comments
		author: {
			id: string;
			name: string;
			avatar?: string;
			email: string;
		};
		content: string;
		status: 'open' | 'resolved' | 'pending';
		type: 'comment' | 'suggestion' | 'question' | 'approval';
		replies: Reply[];
		selection?: {
			start: number;
			end: number;
			text: string;
		};
		createdAt: Date;
		updatedAt: Date;
		resolvedAt?: Date;
		resolvedBy?: string;
	}

	interface Reply {
		id: string;
		author: {
			id: string;
			name: string;
			avatar?: string;
		};
		content: string;
		createdAt: Date;
	}

	interface Props {
		comments: Comment[];
		currentUser: { id: string; name: string; avatar?: string; email: string };
		onAddComment: (comment: Omit<Comment, 'id' | 'createdAt' | 'updatedAt' | 'replies'>) => void;
		onResolve: (commentId: string) => void;
		onReply: (commentId: string, content: string) => void;
		onDelete: (commentId: string) => void;
		selectedBlockId?: string | null;
	}

	let {
		comments,
		currentUser,
		onAddComment,
		onResolve,
		onReply,
		onDelete,
		selectedBlockId = null
	}: Props = $props();

	// State
	let activeTab = $state<'all' | 'open' | 'resolved'>('all');
	let showNewComment = $state(false);
	let newCommentContent = $state('');
	let newCommentType = $state<Comment['type']>('comment');
	let replyingTo = $state<string | null>(null);
	let replyContent = $state('');
	let expandedComments = $state<Set<string>>(new Set());

	// Filter comments
	let filteredComments = $derived(() => {
		let filtered = [...comments];

		if (activeTab === 'open') {
			filtered = filtered.filter(c => c.status === 'open' || c.status === 'pending');
		} else if (activeTab === 'resolved') {
			filtered = filtered.filter(c => c.status === 'resolved');
		}

		// Sort by date (newest first)
		filtered.sort((a, b) => b.createdAt.getTime() - a.createdAt.getTime());

		return filtered;
	});

	// Stats
	let stats = $derived(() => ({
		total: comments.length,
		open: comments.filter(c => c.status === 'open').length,
		pending: comments.filter(c => c.status === 'pending').length,
		resolved: comments.filter(c => c.status === 'resolved').length
	}));

	// Format relative time
	function formatRelativeTime(date: Date): string {
		const now = new Date();
		const diff = now.getTime() - date.getTime();
		const seconds = Math.floor(diff / 1000);
		const minutes = Math.floor(seconds / 60);
		const hours = Math.floor(minutes / 60);
		const days = Math.floor(hours / 24);

		if (seconds < 60) return 'Just now';
		if (minutes < 60) return `${minutes}m ago`;
		if (hours < 24) return `${hours}h ago`;
		if (days < 7) return `${days}d ago`;
		return date.toLocaleDateString();
	}

	// Get type icon
	function getTypeIcon(type: Comment['type']): string {
		switch (type) {
			case 'comment': return '💬';
			case 'suggestion': return '💡';
			case 'question': return '❓';
			case 'approval': return '✅';
			default: return '💬';
		}
	}

	// Get type label
	function getTypeLabel(type: Comment['type']): string {
		switch (type) {
			case 'comment': return 'Comment';
			case 'suggestion': return 'Suggestion';
			case 'question': return 'Question';
			case 'approval': return 'Approval Request';
			default: return 'Comment';
		}
	}

	// Get status color
	function getStatusColor(status: Comment['status']): string {
		switch (status) {
			case 'open': return '#f59e0b';
			case 'pending': return '#3b82f6';
			case 'resolved': return '#10b981';
			default: return '#6b7280';
		}
	}

	// Get initials
	function getInitials(name: string): string {
		return name.split(' ').map(n => n[0]).join('').toUpperCase().slice(0, 2);
	}

	// Submit new comment
	function submitComment(): void {
		if (!newCommentContent.trim()) return;

		onAddComment({
			blockId: selectedBlockId,
			author: currentUser,
			content: newCommentContent.trim(),
			status: newCommentType === 'approval' ? 'pending' : 'open',
			type: newCommentType
		});

		newCommentContent = '';
		newCommentType = 'comment';
		showNewComment = false;
	}

	// Submit reply
	function submitReply(commentId: string): void {
		if (!replyContent.trim()) return;

		onReply(commentId, replyContent.trim());
		replyContent = '';
		replyingTo = null;
	}

	// Toggle expand
	function toggleExpand(commentId: string): void {
		if (expandedComments.has(commentId)) {
			expandedComments.delete(commentId);
		} else {
			expandedComments.add(commentId);
		}
		expandedComments = new Set(expandedComments);
	}

	// Delete comment with confirmation
	function handleDelete(commentId: string): void {
		if (confirm('Are you sure you want to delete this comment?')) {
			onDelete(commentId);
		}
	}
</script>

<div class="collaboration-panel">
	<!-- Header -->
	<div class="panel-header">
		<h3>Comments & Feedback</h3>
		<button class="new-comment-btn" onclick={() => showNewComment = true}>
			+ New
		</button>
	</div>

	<!-- Stats -->
	<div class="stats-bar">
		<div class="stat" class:active={activeTab === 'all'}>
			<span class="stat-value">{stats().total}</span>
			<span class="stat-label">Total</span>
		</div>
		<div class="stat" class:active={activeTab === 'open'}>
			<span class="stat-value">{stats().open + stats().pending}</span>
			<span class="stat-label">Open</span>
		</div>
		<div class="stat" class:active={activeTab === 'resolved'}>
			<span class="stat-value">{stats().resolved}</span>
			<span class="stat-label">Resolved</span>
		</div>
	</div>

	<!-- Tabs -->
	<div class="tabs">
		<button
			class="tab"
			class:active={activeTab === 'all'}
			onclick={() => activeTab = 'all'}
		>
			All
		</button>
		<button
			class="tab"
			class:active={activeTab === 'open'}
			onclick={() => activeTab = 'open'}
		>
			Open
			{#if stats().open + stats().pending > 0}
				<span class="badge">{stats().open + stats().pending}</span>
			{/if}
		</button>
		<button
			class="tab"
			class:active={activeTab === 'resolved'}
			onclick={() => activeTab = 'resolved'}
		>
			Resolved
		</button>
	</div>

	<!-- New Comment Form -->
	{#if showNewComment}
		<div class="new-comment-form">
			<div class="form-header">
				<span>New {getTypeLabel(newCommentType)}</span>
				<button class="close-btn" onclick={() => showNewComment = false}>✕</button>
			</div>
			<div class="type-selector">
				<button
					class:selected={newCommentType === 'comment'}
					onclick={() => newCommentType = 'comment'}
				>
					💬 Comment
				</button>
				<button
					class:selected={newCommentType === 'suggestion'}
					onclick={() => newCommentType = 'suggestion'}
				>
					💡 Suggestion
				</button>
				<button
					class:selected={newCommentType === 'question'}
					onclick={() => newCommentType = 'question'}
				>
					❓ Question
				</button>
				<button
					class:selected={newCommentType === 'approval'}
					onclick={() => newCommentType = 'approval'}
				>
					✅ Approval
				</button>
			</div>
			<textarea
				bind:value={newCommentContent}
				placeholder="Add your comment..."
				rows="3"
				aria-label="Comment content"
			></textarea>
			{#if selectedBlockId}
				<div class="context-info">
					<span class="context-icon">📎</span>
					<span>Attached to selected block</span>
				</div>
			{/if}
			<div class="form-actions">
				<button class="cancel-btn" onclick={() => showNewComment = false}>
					Cancel
				</button>
				<button
					class="submit-btn"
					onclick={submitComment}
					disabled={!newCommentContent.trim()}
				>
					Add {getTypeLabel(newCommentType)}
				</button>
			</div>
		</div>
	{/if}

	<!-- Comments List -->
	<div class="comments-list">
		{#if filteredComments().length === 0}
			<div class="empty-state">
				<span class="empty-icon">💬</span>
				<p>No comments yet</p>
				<button onclick={() => showNewComment = true}>Start a discussion</button>
			</div>
		{:else}
			{#each filteredComments() as comment}
				<div
					class="comment-card"
					class:resolved={comment.status === 'resolved'}
					class:expanded={expandedComments.has(comment.id)}
				>
					<!-- Comment Header -->
					<div class="comment-header">
						<div class="author-info">
							{#if comment.author.avatar}
								<img
									src={comment.author.avatar}
									alt={comment.author.name}
									class="avatar"
								/>
							{:else}
								<div class="avatar-placeholder">
									{getInitials(comment.author.name)}
								</div>
							{/if}
							<div class="author-details">
								<span class="author-name">{comment.author.name}</span>
								<span class="comment-time">{formatRelativeTime(comment.createdAt)}</span>
							</div>
						</div>
						<div class="comment-meta">
							<span class="type-badge" title={getTypeLabel(comment.type)}>
								{getTypeIcon(comment.type)}
							</span>
							<span
								class="status-dot"
								style="background-color: {getStatusColor(comment.status)}"
								title={comment.status}
							></span>
						</div>
					</div>

					<!-- Selected Text (if any) -->
					{#if comment.selection}
						<div class="selection-context">
							<span class="quote-icon">"</span>
							<span class="selected-text">{comment.selection.text}</span>
						</div>
					{/if}

					<!-- Comment Content -->
					<div class="comment-content">
						{comment.content}
					</div>

					<!-- Comment Actions -->
					<div class="comment-actions">
						{#if comment.status !== 'resolved'}
							<button
								class="action-btn reply-btn"
								onclick={() => {
									replyingTo = replyingTo === comment.id ? null : comment.id;
									toggleExpand(comment.id);
								}}
							>
								↩ Reply
							</button>
							<button
								class="action-btn resolve-btn"
								onclick={() => onResolve(comment.id)}
							>
								✓ Resolve
							</button>
						{:else}
							<span class="resolved-info">
								Resolved {comment.resolvedAt ? formatRelativeTime(comment.resolvedAt) : ''}
							</span>
						{/if}
						{#if comment.author.id === currentUser.id}
							<button
								class="action-btn delete-btn"
								onclick={() => handleDelete(comment.id)}
							>
								🗑
							</button>
						{/if}
					</div>

					<!-- Replies -->
					{#if comment.replies.length > 0}
						<button
							class="show-replies-btn"
							onclick={() => toggleExpand(comment.id)}
						>
							{expandedComments.has(comment.id) ? '▼' : '▶'}
							{comment.replies.length} {comment.replies.length === 1 ? 'reply' : 'replies'}
						</button>

						{#if expandedComments.has(comment.id)}
							<div class="replies">
								{#each comment.replies as reply}
									<div class="reply">
										<div class="reply-header">
											{#if reply.author.avatar}
												<img
													src={reply.author.avatar}
													alt={reply.author.name}
													class="reply-avatar"
												/>
											{:else}
												<div class="reply-avatar-placeholder">
													{getInitials(reply.author.name)}
												</div>
											{/if}
											<span class="reply-author">{reply.author.name}</span>
											<span class="reply-time">{formatRelativeTime(reply.createdAt)}</span>
										</div>
										<div class="reply-content">
											{reply.content}
										</div>
									</div>
								{/each}
							</div>
						{/if}
					{/if}

					<!-- Reply Form -->
					{#if replyingTo === comment.id}
						<div class="reply-form">
							<textarea
								bind:value={replyContent}
								placeholder="Write a reply..."
								rows="2"
								aria-label="Reply content"
							></textarea>
							<div class="reply-actions">
								<button class="cancel-btn" onclick={() => replyingTo = null}>
									Cancel
								</button>
								<button
									class="submit-btn"
									onclick={() => submitReply(comment.id)}
									disabled={!replyContent.trim()}
								>
									Reply
								</button>
							</div>
						</div>
					{/if}
				</div>
			{/each}
		{/if}
	</div>
</div>

<style>
	.collaboration-panel {
		display: flex;
		flex-direction: column;
		height: 100%;
		background: var(--bg-primary, #ffffff);
	}

	.panel-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem;
		border-bottom: 1px solid var(--border-color, #e5e7eb);
	}

	.panel-header h3 {
		margin: 0;
		font-size: 1rem;
		font-weight: 600;
		color: var(--text-primary, #1f2937);
	}

	.new-comment-btn {
		padding: 0.375rem 0.75rem;
		background: var(--primary, #3b82f6);
		color: white;
		border: none;
		border-radius: 0.375rem;
		font-size: 0.75rem;
		font-weight: 500;
		cursor: pointer;
		transition: background 0.2s;
	}

	.new-comment-btn:hover {
		background: var(--primary-hover, #2563eb);
	}

	.stats-bar {
		display: flex;
		padding: 0.75rem 1rem;
		gap: 1rem;
		background: var(--bg-secondary, #f9fafb);
		border-bottom: 1px solid var(--border-color, #e5e7eb);
	}

	.stat {
		flex: 1;
		text-align: center;
		padding: 0.5rem;
		border-radius: 0.375rem;
		transition: background 0.2s;
	}

	.stat.active {
		background: var(--bg-primary, #ffffff);
	}

	.stat-value {
		display: block;
		font-size: 1.25rem;
		font-weight: 700;
		color: var(--text-primary, #1f2937);
	}

	.stat-label {
		font-size: 0.6875rem;
		color: var(--text-secondary, #6b7280);
		text-transform: uppercase;
		letter-spacing: 0.025em;
	}

	.tabs {
		display: flex;
		padding: 0 1rem;
		border-bottom: 1px solid var(--border-color, #e5e7eb);
	}

	.tab {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.75rem 1rem;
		background: none;
		border: none;
		font-size: 0.8125rem;
		font-weight: 500;
		color: var(--text-secondary, #6b7280);
		cursor: pointer;
		border-bottom: 2px solid transparent;
		margin-bottom: -1px;
		transition: all 0.2s;
	}

	.tab:hover {
		color: var(--text-primary, #1f2937);
	}

	.tab.active {
		color: var(--primary, #3b82f6);
		border-bottom-color: var(--primary, #3b82f6);
	}

	.badge {
		padding: 0.125rem 0.375rem;
		background: var(--primary, #3b82f6);
		color: white;
		border-radius: 9999px;
		font-size: 0.625rem;
		font-weight: 600;
	}

	.new-comment-form {
		padding: 1rem;
		border-bottom: 1px solid var(--border-color, #e5e7eb);
		background: #eff6ff;
	}

	.form-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.75rem;
	}

	.form-header span {
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--text-primary, #1f2937);
	}

	.close-btn {
		padding: 0.25rem 0.5rem;
		background: none;
		border: none;
		cursor: pointer;
		color: var(--text-secondary, #6b7280);
	}

	.type-selector {
		display: flex;
		gap: 0.25rem;
		margin-bottom: 0.75rem;
	}

	.type-selector button {
		flex: 1;
		padding: 0.375rem 0.5rem;
		background: var(--bg-primary, #ffffff);
		border: 1px solid var(--border-color, #e5e7eb);
		border-radius: 0.25rem;
		font-size: 0.6875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.type-selector button.selected {
		background: var(--primary, #3b82f6);
		color: white;
		border-color: var(--primary, #3b82f6);
	}

	.new-comment-form textarea,
	.reply-form textarea {
		width: 100%;
		padding: 0.625rem;
		border: 1px solid var(--border-color, #e5e7eb);
		border-radius: 0.375rem;
		font-size: 0.875rem;
		resize: vertical;
		outline: none;
	}

	.new-comment-form textarea:focus,
	.reply-form textarea:focus {
		border-color: var(--primary, #3b82f6);
	}

	.context-info {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		margin-top: 0.5rem;
		font-size: 0.75rem;
		color: var(--text-secondary, #6b7280);
	}

	.form-actions,
	.reply-actions {
		display: flex;
		justify-content: flex-end;
		gap: 0.5rem;
		margin-top: 0.75rem;
	}

	.cancel-btn,
	.submit-btn {
		padding: 0.5rem 1rem;
		border-radius: 0.375rem;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.cancel-btn {
		background: var(--bg-primary, #ffffff);
		border: 1px solid var(--border-color, #e5e7eb);
		color: var(--text-secondary, #6b7280);
	}

	.submit-btn {
		background: var(--primary, #3b82f6);
		border: none;
		color: white;
	}

	.submit-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.comments-list {
		flex: 1;
		overflow-y: auto;
		padding: 0.75rem;
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem 1rem;
		text-align: center;
	}

	.empty-icon {
		font-size: 2.5rem;
		margin-bottom: 0.75rem;
	}

	.empty-state p {
		margin: 0 0 1rem;
		color: var(--text-secondary, #6b7280);
	}

	.empty-state button {
		padding: 0.5rem 1rem;
		background: var(--primary, #3b82f6);
		color: white;
		border: none;
		border-radius: 0.375rem;
		cursor: pointer;
	}

	.comment-card {
		background: var(--bg-primary, #ffffff);
		border: 1px solid var(--border-color, #e5e7eb);
		border-radius: 0.5rem;
		padding: 0.875rem;
		margin-bottom: 0.75rem;
		transition: all 0.2s;
	}

	.comment-card:hover {
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
	}

	.comment-card.resolved {
		opacity: 0.7;
		background: var(--bg-secondary, #f9fafb);
	}

	.comment-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 0.625rem;
	}

	.author-info {
		display: flex;
		align-items: center;
		gap: 0.625rem;
	}

	.avatar {
		width: 32px;
		height: 32px;
		border-radius: 50%;
		object-fit: cover;
	}

	.avatar-placeholder {
		width: 32px;
		height: 32px;
		border-radius: 50%;
		background: var(--primary, #3b82f6);
		color: white;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 0.6875rem;
		font-weight: 600;
	}

	.author-details {
		display: flex;
		flex-direction: column;
	}

	.author-name {
		font-size: 0.8125rem;
		font-weight: 600;
		color: var(--text-primary, #1f2937);
	}

	.comment-time {
		font-size: 0.6875rem;
		color: var(--text-tertiary, #9ca3af);
	}

	.comment-meta {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.type-badge {
		font-size: 1rem;
	}

	.status-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
	}

	.selection-context {
		display: flex;
		align-items: flex-start;
		gap: 0.25rem;
		padding: 0.5rem 0.75rem;
		margin-bottom: 0.625rem;
		background: var(--bg-secondary, #f9fafb);
		border-left: 3px solid var(--primary, #3b82f6);
		border-radius: 0 0.25rem 0.25rem 0;
	}

	.quote-icon {
		font-size: 1.25rem;
		color: var(--primary, #3b82f6);
		line-height: 1;
	}

	.selected-text {
		font-size: 0.8125rem;
		font-style: italic;
		color: var(--text-secondary, #6b7280);
	}

	.comment-content {
		font-size: 0.875rem;
		color: var(--text-primary, #1f2937);
		line-height: 1.5;
		margin-bottom: 0.625rem;
	}

	.comment-actions {
		display: flex;
		gap: 0.5rem;
		padding-top: 0.5rem;
		border-top: 1px solid var(--border-color, #e5e7eb);
	}

	.action-btn {
		padding: 0.25rem 0.5rem;
		background: none;
		border: none;
		font-size: 0.75rem;
		color: var(--text-secondary, #6b7280);
		cursor: pointer;
		transition: color 0.2s;
	}

	.action-btn:hover {
		color: var(--text-primary, #1f2937);
	}

	.resolve-btn:hover {
		color: #10b981;
	}

	.delete-btn:hover {
		color: #ef4444;
	}

	.resolved-info {
		font-size: 0.75rem;
		color: #10b981;
	}

	.show-replies-btn {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0;
		background: none;
		border: none;
		font-size: 0.75rem;
		color: var(--text-secondary, #6b7280);
		cursor: pointer;
	}

	.replies {
		margin-top: 0.75rem;
		padding-left: 1rem;
		border-left: 2px solid var(--border-color, #e5e7eb);
	}

	.reply {
		padding: 0.625rem 0;
	}

	.reply:not(:last-child) {
		border-bottom: 1px solid var(--border-color, #e5e7eb);
	}

	.reply-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 0.375rem;
	}

	.reply-avatar {
		width: 20px;
		height: 20px;
		border-radius: 50%;
		object-fit: cover;
	}

	.reply-avatar-placeholder {
		width: 20px;
		height: 20px;
		border-radius: 50%;
		background: var(--primary, #3b82f6);
		color: white;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 0.5rem;
		font-weight: 600;
	}

	.reply-author {
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--text-primary, #1f2937);
	}

	.reply-time {
		font-size: 0.625rem;
		color: var(--text-tertiary, #9ca3af);
	}

	.reply-content {
		font-size: 0.8125rem;
		color: var(--text-secondary, #6b7280);
		line-height: 1.4;
	}

	.reply-form {
		margin-top: 0.75rem;
		padding-top: 0.75rem;
		border-top: 1px solid var(--border-color, #e5e7eb);
	}

	.reply-form textarea {
		margin-bottom: 0.5rem;
	}
</style>
