<script lang="ts">
	/**
	 * AdminApprovalStatus Component (FluentForms 6.1.8 - December 2025)
	 *
	 * Displays and manages admin approval status for form submissions.
	 * Supports approval workflows with multiple statuses and notes.
	 */

	interface ApprovalLog {
		id: string;
		status: ApprovalStatus;
		note?: string;
		admin_name: string;
		admin_email?: string;
		created_at: string;
	}

	type ApprovalStatus = 'pending' | 'approved' | 'rejected' | 'needs_revision' | 'on_hold';

	interface Props {
		submissionId: string | number;
		currentStatus?: ApprovalStatus;
		logs?: ApprovalLog[];
		isAdmin?: boolean;
		allowStatusChange?: boolean;
		showHistory?: boolean;
		onStatusChange?: (status: ApprovalStatus, note: string) => Promise<void>;
		error?: string;
	}

	let {
		submissionId,
		currentStatus = 'pending',
		logs = [],
		isAdmin = false,
		allowStatusChange = false,
		showHistory = true,
		onStatusChange,
		error = ''
	}: Props = $props();

	let isUpdating = $state(false);
	let showNoteInput = $state(false);
	let selectedStatus = $state<ApprovalStatus | null>(null);
	let noteText = $state('');
	let updateError = $state('');

	const statusConfig: Record<ApprovalStatus, { label: string; color: string; bgColor: string; icon: string }> = {
		pending: {
			label: 'Pending Review',
			color: '#92400e',
			bgColor: '#fef3c7',
			icon: 'clock'
		},
		approved: {
			label: 'Approved',
			color: '#166534',
			bgColor: '#dcfce7',
			icon: 'check'
		},
		rejected: {
			label: 'Rejected',
			color: '#991b1b',
			bgColor: '#fee2e2',
			icon: 'x'
		},
		needs_revision: {
			label: 'Needs Revision',
			color: '#1e40af',
			bgColor: '#dbeafe',
			icon: 'edit'
		},
		on_hold: {
			label: 'On Hold',
			color: '#6b7280',
			bgColor: '#f3f4f6',
			icon: 'pause'
		}
	};

	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function initiateStatusChange(status: ApprovalStatus) {
		selectedStatus = status;
		showNoteInput = true;
		noteText = '';
		updateError = '';
	}

	function cancelStatusChange() {
		showNoteInput = false;
		selectedStatus = null;
		noteText = '';
		updateError = '';
	}

	async function confirmStatusChange() {
		if (!selectedStatus || !onStatusChange) return;

		isUpdating = true;
		updateError = '';

		try {
			await onStatusChange(selectedStatus, noteText);
			showNoteInput = false;
			selectedStatus = null;
			noteText = '';
		} catch (err) {
			updateError = err instanceof Error ? err.message : 'Failed to update status';
		} finally {
			isUpdating = false;
		}
	}

	const config = $derived(statusConfig[currentStatus]);
</script>

<div class="admin-approval-status" class:has-error={error || updateError}>
	<div class="status-card">
		<div class="current-status" style="--status-color: {config.color}; --status-bg: {config.bgColor}">
			<div class="status-icon">
				{#if config.icon === 'clock'}
					<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<circle cx="12" cy="12" r="10"></circle>
						<polyline points="12 6 12 12 16 14"></polyline>
					</svg>
				{:else if config.icon === 'check'}
					<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<polyline points="20 6 9 17 4 12"></polyline>
					</svg>
				{:else if config.icon === 'x'}
					<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<line x1="18" y1="6" x2="6" y2="18"></line>
						<line x1="6" y1="6" x2="18" y2="18"></line>
					</svg>
				{:else if config.icon === 'edit'}
					<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"></path>
						<path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"></path>
					</svg>
				{:else if config.icon === 'pause'}
					<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<rect x="6" y="4" width="4" height="16"></rect>
						<rect x="14" y="4" width="4" height="16"></rect>
					</svg>
				{/if}
			</div>
			<div class="status-info">
				<span class="status-label">{config.label}</span>
				<span class="submission-id">Submission #{submissionId}</span>
			</div>
		</div>

		{#if isAdmin && allowStatusChange}
			<div class="admin-actions">
				{#if showNoteInput}
					<div class="note-input-section">
						<label for="approval_note">Add a note (optional):</label>
						<textarea
							id="approval_note"
							bind:value={noteText}
							placeholder="Enter reason or instructions..."
							rows="3"
						></textarea>
						<div class="note-actions">
							<button
								type="button"
								class="btn btn-cancel"
								onclick={cancelStatusChange}
								disabled={isUpdating}
							>
								Cancel
							</button>
							<button
								type="button"
								class="btn btn-confirm"
								onclick={confirmStatusChange}
								disabled={isUpdating}
							>
								{#if isUpdating}
									<span class="spinner"></span>
								{/if}
								Confirm {selectedStatus ? statusConfig[selectedStatus].label : ''}
							</button>
						</div>
					</div>
				{:else}
					<div class="status-buttons">
						{#if currentStatus !== 'approved'}
							<button
								type="button"
								class="status-btn approve"
								onclick={() => initiateStatusChange('approved')}
							>
								<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
									<polyline points="20 6 9 17 4 12"></polyline>
								</svg>
								Approve
							</button>
						{/if}
						{#if currentStatus !== 'rejected'}
							<button
								type="button"
								class="status-btn reject"
								onclick={() => initiateStatusChange('rejected')}
							>
								<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
									<line x1="18" y1="6" x2="6" y2="18"></line>
									<line x1="6" y1="6" x2="18" y2="18"></line>
								</svg>
								Reject
							</button>
						{/if}
						{#if currentStatus !== 'needs_revision'}
							<button
								type="button"
								class="status-btn revision"
								onclick={() => initiateStatusChange('needs_revision')}
							>
								<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
									<path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"></path>
									<path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"></path>
								</svg>
								Request Revision
							</button>
						{/if}
						{#if currentStatus !== 'on_hold'}
							<button
								type="button"
								class="status-btn hold"
								onclick={() => initiateStatusChange('on_hold')}
							>
								<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
									<rect x="6" y="4" width="4" height="16"></rect>
									<rect x="14" y="4" width="4" height="16"></rect>
								</svg>
								Put On Hold
							</button>
						{/if}
					</div>
				{/if}
			</div>
		{/if}
	</div>

	{#if showHistory && logs.length > 0}
		<div class="approval-history">
			<h4 class="history-title">Approval History</h4>
			<div class="history-timeline">
				{#each logs as log}
					{@const logConfig = statusConfig[log.status]}
					<div class="history-item">
						<div class="timeline-dot" style="background-color: {logConfig.color}"></div>
						<div class="history-content">
							<div class="history-header">
								<span class="history-status" style="color: {logConfig.color}">
									{logConfig.label}
								</span>
								<span class="history-date">{formatDate(log.created_at)}</span>
							</div>
							<div class="history-admin">
								By {log.admin_name}
								{#if log.admin_email}
									<span class="admin-email">({log.admin_email})</span>
								{/if}
							</div>
							{#if log.note}
								<div class="history-note">
									<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
										<path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"></path>
									</svg>
									{log.note}
								</div>
							{/if}
						</div>
					</div>
				{/each}
			</div>
		</div>
	{/if}

	{#if error || updateError}
		<div class="error-message">
			<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<circle cx="12" cy="12" r="10"></circle>
				<line x1="12" y1="8" x2="12" y2="12"></line>
				<line x1="12" y1="16" x2="12.01" y2="16"></line>
			</svg>
			{error || updateError}
		</div>
	{/if}
</div>

<style>
	.admin-approval-status {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.status-card {
		background-color: white;
		border: 1px solid #e5e7eb;
		border-radius: 0.75rem;
		overflow: hidden;
	}

	.current-status {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem;
		background-color: var(--status-bg);
	}

	.status-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 48px;
		height: 48px;
		background-color: white;
		border-radius: 50%;
		color: var(--status-color);
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.status-info {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.status-label {
		font-size: 1.125rem;
		font-weight: 700;
		color: var(--status-color);
	}

	.submission-id {
		font-size: 0.8125rem;
		color: #6b7280;
	}

	.admin-actions {
		padding: 1rem;
		border-top: 1px solid #e5e7eb;
		background-color: #f9fafb;
	}

	.status-buttons {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.status-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 0.875rem;
		border: 1px solid;
		border-radius: 0.375rem;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s;
	}

	.status-btn.approve {
		background-color: #f0fdf4;
		border-color: #86efac;
		color: #166534;
	}

	.status-btn.approve:hover {
		background-color: #dcfce7;
	}

	.status-btn.reject {
		background-color: #fef2f2;
		border-color: #fecaca;
		color: #991b1b;
	}

	.status-btn.reject:hover {
		background-color: #fee2e2;
	}

	.status-btn.revision {
		background-color: #eff6ff;
		border-color: #bfdbfe;
		color: #1e40af;
	}

	.status-btn.revision:hover {
		background-color: #dbeafe;
	}

	.status-btn.hold {
		background-color: #f9fafb;
		border-color: #d1d5db;
		color: #6b7280;
	}

	.status-btn.hold:hover {
		background-color: #f3f4f6;
	}

	.note-input-section {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.note-input-section label {
		font-size: 0.8125rem;
		font-weight: 500;
		color: #374151;
	}

	.note-input-section textarea {
		padding: 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 0.5rem;
		font-size: 0.875rem;
		resize: vertical;
	}

	.note-input-section textarea:focus {
		outline: none;
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.note-actions {
		display: flex;
		gap: 0.5rem;
		justify-content: flex-end;
	}

	.btn {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 1rem;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-cancel {
		background-color: white;
		border: 1px solid #d1d5db;
		color: #6b7280;
	}

	.btn-cancel:hover {
		background-color: #f9fafb;
	}

	.btn-confirm {
		background-color: #3b82f6;
		border: 1px solid #3b82f6;
		color: white;
	}

	.btn-confirm:hover {
		background-color: #2563eb;
	}

	.btn:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.spinner {
		width: 14px;
		height: 14px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.approval-history {
		background-color: white;
		border: 1px solid #e5e7eb;
		border-radius: 0.75rem;
		padding: 1.25rem;
	}

	.history-title {
		font-size: 0.9375rem;
		font-weight: 600;
		color: #374151;
		margin: 0 0 1rem;
	}

	.history-timeline {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		padding-left: 1.5rem;
		border-left: 2px solid #e5e7eb;
	}

	.history-item {
		position: relative;
	}

	.timeline-dot {
		position: absolute;
		left: -1.75rem;
		top: 0.25rem;
		width: 12px;
		height: 12px;
		border-radius: 50%;
		border: 2px solid white;
	}

	.history-content {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.history-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.history-status {
		font-size: 0.875rem;
		font-weight: 600;
	}

	.history-date {
		font-size: 0.75rem;
		color: #9ca3af;
	}

	.history-admin {
		font-size: 0.8125rem;
		color: #6b7280;
	}

	.admin-email {
		color: #9ca3af;
	}

	.history-note {
		display: flex;
		align-items: flex-start;
		gap: 0.5rem;
		margin-top: 0.5rem;
		padding: 0.625rem;
		background-color: #f9fafb;
		border-radius: 0.375rem;
		font-size: 0.8125rem;
		color: #4b5563;
	}

	.history-note svg {
		flex-shrink: 0;
		margin-top: 0.125rem;
		color: #9ca3af;
	}

	.error-message {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem;
		background-color: #fef2f2;
		border: 1px solid #fecaca;
		border-radius: 0.5rem;
		color: #dc2626;
		font-size: 0.875rem;
	}

	/* Responsive */
	@media (max-width: 480px) {
		.status-buttons {
			flex-direction: column;
		}

		.status-btn {
			justify-content: center;
		}
	}
</style>
