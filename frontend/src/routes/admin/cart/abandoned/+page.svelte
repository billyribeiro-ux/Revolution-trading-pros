<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import abandonedCartsApi, {
		type AbandonedCart,
		type DashboardStats,
		type CartStatus,
		STATUS_LABELS
	} from '$lib/api/abandoned-carts';
	import {
		IconArrowLeft,
		IconShoppingCart,
		IconMail,
		IconSend,
		IconSearch,
		IconFilter,
		IconRefresh,
		IconChevronLeft,
		IconChevronRight,
		IconX,
		IconCurrencyDollar,
		IconCheck,
		IconClock,
		IconAlertCircle,
		IconGift,
		IconPercentage,
		IconChartBar,
		IconUsers,
		IconExternalLink
	} from '$lib/icons';
	import { toastStore } from '$lib/stores/toast.svelte';

	// State
	let loading = $state(true);
	let stats = $state<DashboardStats | null>(null);
	let carts = $state<AbandonedCart[]>([]);
	let pagination = $state({ current_page: 1, last_page: 1, per_page: 20, total: 0 });
	let searchQuery = $state('');
	let statusFilter = $state<CartStatus | ''>('');
	let selectedCarts = $state<Set<number>>(new Set());

	// Modal state
	let showRecoveryModal = $state(false);
	let recoveryTemplate = $state<'reminder_1' | 'reminder_2' | 'final_discount' | 'custom'>(
		'reminder_1'
	);
	let customSubject = $state('');
	let customBody = $state('');
	let discountCode = $state('');
	let discountPercent = $state(15);
	let sending = $state(false);

	onMount(async () => {
		await loadDashboard();
	});

	async function loadDashboard() {
		loading = true;
		try {
			const [statsData, cartsData] = await Promise.all([
				abandonedCartsApi.getDashboardStats(),
				abandonedCartsApi.getAbandonedCarts()
			]);
			stats = statsData;
			carts = cartsData.data;
			pagination = {
				current_page: cartsData.current_page,
				last_page: cartsData.last_page,
				per_page: cartsData.per_page,
				total: cartsData.total
			};
		} catch (error) {
			console.error('Failed to load dashboard:', error);
			toastStore.error('Failed to load abandoned carts data');
		} finally {
			loading = false;
		}
	}

	async function loadCarts(page: number = 1) {
		try {
			const filters: { status?: CartStatus; search?: string } = {};
			if (statusFilter) filters.status = statusFilter;
			if (searchQuery) filters.search = searchQuery;

			const result = await abandonedCartsApi.getAbandonedCarts(page, pagination.per_page, filters);
			carts = result.data;
			pagination = {
				current_page: result.current_page,
				last_page: result.last_page,
				per_page: result.per_page,
				total: result.total
			};
		} catch (error) {
			console.error('Failed to load carts:', error);
		}
	}

	async function handleSearch() {
		await loadCarts();
	}

	async function handleStatusFilter(status: CartStatus | '') {
		statusFilter = status;
		await loadCarts();
	}

	function toggleCartSelection(id: number) {
		if (selectedCarts.has(id)) {
			selectedCarts.delete(id);
		} else {
			selectedCarts.add(id);
		}
		selectedCarts = selectedCarts;
	}

	function selectAllCarts() {
		if (selectedCarts.size === carts.length) {
			selectedCarts.clear();
		} else {
			carts.forEach((c) => selectedCarts.add(c.id));
		}
		selectedCarts = selectedCarts;
	}

	async function handleSendRecovery() {
		if (selectedCarts.size === 0) return;

		sending = true;
		try {
			const options: import('$lib/api/abandoned-carts').RecoveryEmailOptions = {
				template: recoveryTemplate
			};
			if (recoveryTemplate === 'custom' && customSubject) options.custom_subject = customSubject;
			if (recoveryTemplate === 'custom' && customBody) options.custom_body = customBody;
			if (recoveryTemplate === 'final_discount' && discountCode)
				options.discount_code = discountCode;
			if (recoveryTemplate === 'final_discount') options.discount_percent = discountPercent;

			const result = await abandonedCartsApi.sendBulkRecovery(Array.from(selectedCarts), options);

			toastStore.success(result.message);
			showRecoveryModal = false;
			selectedCarts.clear();
			selectedCarts = selectedCarts;
			await loadDashboard();
		} catch (error) {
			toastStore.error(error instanceof Error ? error.message : 'Failed to send recovery emails');
		} finally {
			sending = false;
		}
	}

	async function handleMarkRecovered(cartId: number) {
		try {
			await abandonedCartsApi.markAsRecovered(cartId);
			toastStore.success('Cart marked as recovered');
			await loadDashboard();
		} catch (error) {
			toastStore.error('Failed to mark as recovered');
		}
	}

	function formatCurrency(amount: number): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD',
			minimumFractionDigits: 0,
			maximumFractionDigits: 2
		}).format(amount);
	}

	function formatDate(dateString: string | null): string {
		if (!dateString) return '-';
		return new Date(dateString).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function getTimeSince(dateString: string): string {
		const diff = Date.now() - new Date(dateString).getTime();
		const hours = Math.floor(diff / (1000 * 60 * 60));
		const days = Math.floor(hours / 24);

		if (days > 0) return `${days}d ago`;
		if (hours > 0) return `${hours}h ago`;
		return 'Just now';
	}

	function getStatusClass(status: CartStatus): string {
		const colorMap: Record<string, string> = {
			pending: 'status-warning',
			email_sent: 'status-info',
			clicked: 'status-primary',
			recovered: 'status-success',
			expired: 'status-neutral',
			unsubscribed: 'status-neutral'
		};
		return colorMap[status] || 'status-neutral';
	}

	function getTemplateInfo(template: string): {
		name: string;
		description: string;
		timing: string;
	} {
		switch (template) {
			case 'reminder_1':
				return {
					name: 'First Reminder',
					description: 'Gentle nudge about items left behind',
					timing: 'Best sent 1 hour after abandonment'
				};
			case 'reminder_2':
				return {
					name: 'Second Reminder',
					description: 'Highlight product benefits and urgency',
					timing: 'Best sent 24 hours after abandonment'
				};
			case 'final_discount':
				return {
					name: 'Final Discount',
					description: 'Last chance offer with exclusive discount',
					timing: 'Best sent 72 hours after abandonment'
				};
			default:
				return {
					name: 'Custom Email',
					description: 'Your personalized recovery message',
					timing: 'Send anytime'
				};
		}
	}

	let pendingCount = $derived(stats?.by_status?.pending || 0);
	let emailSentCount = $derived(stats?.by_status?.email_sent || 0);
	let clickedCount = $derived(stats?.by_status?.clicked || 0);
</script>

<svelte:head>
	<title>Abandoned Cart Recovery | Revolution Trading Pros</title>
</svelte:head>

<div class="abandoned-cart-page">
	<!-- Header -->
	<div class="page-header">
		<button class="back-btn" onclick={() => goto('/admin')}>
			<IconArrowLeft size={20} />
			Back to Admin
		</button>

		<div class="header-content">
			<div class="header-title">
				<div class="title-icon">
					<IconShoppingCart size={32} />
				</div>
				<div>
					<h1>Cart Recovery Center</h1>
					<p>Recover lost revenue with automated email sequences</p>
				</div>
			</div>
			<button class="btn-refresh" onclick={loadDashboard}>
				<IconRefresh size={18} />
				Refresh
			</button>
		</div>
	</div>

	{#if loading}
		<div class="loading-state">
			<div class="loader"></div>
			<p>Loading recovery dashboard...</p>
		</div>
	{:else if stats}
		<!-- Key Metrics -->
		<div class="metrics-grid">
			<div class="metric-card total">
				<div class="metric-icon">
					<IconShoppingCart size={28} />
				</div>
				<div class="metric-content">
					<div class="metric-label">Total Abandoned</div>
					<div class="metric-value">{stats.total_abandoned}</div>
					<div class="metric-secondary">{formatCurrency(stats.total_value)} value</div>
				</div>
			</div>

			<div class="metric-card recovered">
				<div class="metric-icon">
					<IconCheck size={28} />
				</div>
				<div class="metric-content">
					<div class="metric-label">Recovered</div>
					<div class="metric-value">{stats.recovered_count}</div>
					<div class="metric-secondary">{formatCurrency(stats.recovered_value)} recovered</div>
				</div>
				<div class="metric-badge success">{stats.recovery_rate.toFixed(1)}%</div>
			</div>

			<div class="metric-card pending">
				<div class="metric-icon">
					<IconClock size={28} />
				</div>
				<div class="metric-content">
					<div class="metric-label">Pending Recovery</div>
					<div class="metric-value">{stats.pending_recovery}</div>
					<div class="metric-secondary">Awaiting action</div>
				</div>
			</div>

			<div class="metric-card average">
				<div class="metric-icon">
					<IconCurrencyDollar size={28} />
				</div>
				<div class="metric-content">
					<div class="metric-label">Avg Cart Value</div>
					<div class="metric-value">{formatCurrency(stats.avg_cart_value)}</div>
					<div class="metric-secondary">Per abandoned cart</div>
				</div>
			</div>
		</div>

		<!-- Recovery Pipeline -->
		<div class="pipeline-section">
			<div class="section-header">
				<IconChartBar size={20} />
				<h3>Recovery Pipeline</h3>
			</div>
			<div class="pipeline-stages">
				<div class="pipeline-stage">
					<div class="stage-icon pending">
						<IconAlertCircle size={24} />
					</div>
					<div class="stage-info">
						<div class="stage-count">{pendingCount}</div>
						<div class="stage-label">Pending</div>
					</div>
					<div class="stage-bar">
						<div
							class="stage-fill pending"
							style="--width: {(pendingCount / stats.total_abandoned) * 100}%"
						></div>
					</div>
				</div>

				<div class="pipeline-connector">
					<IconChevronRight size={20} />
				</div>

				<div class="pipeline-stage">
					<div class="stage-icon sent">
						<IconMail size={24} />
					</div>
					<div class="stage-info">
						<div class="stage-count">{emailSentCount}</div>
						<div class="stage-label">Email Sent</div>
					</div>
					<div class="stage-bar">
						<div
							class="stage-fill sent"
							style="--width: {(emailSentCount / stats.total_abandoned) * 100}%"
						></div>
					</div>
				</div>

				<div class="pipeline-connector">
					<IconChevronRight size={20} />
				</div>

				<div class="pipeline-stage">
					<div class="stage-icon clicked">
						<IconExternalLink size={24} />
					</div>
					<div class="stage-info">
						<div class="stage-count">{clickedCount}</div>
						<div class="stage-label">Clicked</div>
					</div>
					<div class="stage-bar">
						<div
							class="stage-fill clicked"
							style="--width: {(clickedCount / stats.total_abandoned) * 100}%"
						></div>
					</div>
				</div>

				<div class="pipeline-connector">
					<IconChevronRight size={20} />
				</div>

				<div class="pipeline-stage">
					<div class="stage-icon recovered">
						<IconCheck size={24} />
					</div>
					<div class="stage-info">
						<div class="stage-count">{stats.recovered_count}</div>
						<div class="stage-label">Recovered</div>
					</div>
					<div class="stage-bar">
						<div
							class="stage-fill recovered"
							style="--width: {(stats.recovered_count / stats.total_abandoned) * 100}%"
						></div>
					</div>
				</div>
			</div>
		</div>

		<!-- Quick Actions -->
		<div class="quick-actions">
			<button
				class="action-card reminder"
				onclick={() => {
					recoveryTemplate = 'reminder_1';
					selectedCarts = new Set(carts.filter((c) => c.status === 'pending').map((c) => c.id));
					showRecoveryModal = true;
				}}
			>
				<div class="action-icon">
					<IconMail size={24} />
				</div>
				<div class="action-content">
					<h4>Send First Reminder</h4>
					<p>Gentle nudge to pending carts</p>
				</div>
				<div class="action-count">{pendingCount} carts</div>
			</button>

			<button
				class="action-card urgency"
				onclick={() => {
					recoveryTemplate = 'reminder_2';
					selectedCarts = new Set(carts.filter((c) => c.status === 'email_sent').map((c) => c.id));
					showRecoveryModal = true;
				}}
			>
				<div class="action-icon">
					<IconClock size={24} />
				</div>
				<div class="action-content">
					<h4>Send Urgency Email</h4>
					<p>Follow up with sent carts</p>
				</div>
				<div class="action-count">{emailSentCount} carts</div>
			</button>

			<button
				class="action-card discount"
				onclick={() => {
					recoveryTemplate = 'final_discount';
					selectedCarts = new Set(
						carts
							.filter((c) => c.status === 'email_sent' || c.status === 'clicked')
							.map((c) => c.id)
					);
					showRecoveryModal = true;
				}}
			>
				<div class="action-icon">
					<IconPercentage size={24} />
				</div>
				<div class="action-content">
					<h4>Final Discount Offer</h4>
					<p>Last chance with special pricing</p>
				</div>
				<div class="action-count">{emailSentCount + clickedCount} carts</div>
			</button>
		</div>

		<!-- Toolbar -->
		<div class="toolbar">
			<div class="search-box">
				<IconSearch size={18} />
				<input
					type="text"
					placeholder="Search by email..."
					bind:value={searchQuery}
					onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && handleSearch()}
				/>
			</div>

			<div class="filter-group">
				<IconFilter size={16} />
				<select bind:value={statusFilter} onchange={() => handleStatusFilter(statusFilter)}>
					<option value="">All Status</option>
					<option value="pending">Pending</option>
					<option value="email_sent">Email Sent</option>
					<option value="clicked">Clicked</option>
					<option value="recovered">Recovered</option>
					<option value="expired">Expired</option>
				</select>
			</div>

			{#if selectedCarts.size > 0}
				<button class="btn-selected" onclick={() => (showRecoveryModal = true)}>
					<IconSend size={18} />
					Send Recovery ({selectedCarts.size})
				</button>
			{/if}
		</div>

		<!-- Carts Table -->
		<div class="carts-table-container">
			{#if carts.length === 0}
				<div class="empty-state">
					<IconShoppingCart size={64} stroke={1} />
					<h3>No abandoned carts</h3>
					<p>Great! No abandoned carts matching your filters.</p>
				</div>
			{:else}
				<table class="carts-table">
					<thead>
						<tr>
							<th class="checkbox-col">
								<input
									type="checkbox"
									checked={selectedCarts.size === carts.length && carts.length > 0}
									onchange={selectAllCarts}
								/>
							</th>
							<th>Customer</th>
							<th>Cart Value</th>
							<th>Items</th>
							<th>Status</th>
							<th>Abandoned</th>
							<th>Attempts</th>
							<th>Actions</th>
						</tr>
					</thead>
					<tbody>
						{#each carts as cart}
							<tr class:selected={selectedCarts.has(cart.id)}>
								<td class="checkbox-col">
									<input
										type="checkbox"
										checked={selectedCarts.has(cart.id)}
										onchange={() => toggleCartSelection(cart.id)}
									/>
								</td>
								<td>
									<div class="customer-info">
										<div class="customer-avatar">
											{cart.user_email ? cart.user_email.slice(0, 2).toUpperCase() : 'G'}
										</div>
										<div class="customer-details">
											<div class="customer-name">{cart.user_name || 'Guest'}</div>
											<div class="customer-email">{cart.user_email || 'No email'}</div>
										</div>
									</div>
								</td>
								<td>
									<span class="cart-value">{formatCurrency(cart.cart_value)}</span>
								</td>
								<td>
									<span class="item-count">{cart.cart_data?.items?.length || 0} items</span>
								</td>
								<td>
									<span class="status-badge {getStatusClass(cart.status)}">
										{STATUS_LABELS[cart.status]}
									</span>
								</td>
								<td>
									<div class="abandoned-info">
										<span class="time-ago">{getTimeSince(cart.abandoned_at)}</span>
										<span class="date-full">{formatDate(cart.abandoned_at)}</span>
									</div>
								</td>
								<td>
									<span class="attempts-badge">{cart.recovery_attempts}</span>
								</td>
								<td>
									<div class="actions">
										{#if cart.status !== 'recovered'}
											<button
												class="action-btn send"
												title="Send Recovery Email"
												onclick={() => {
													selectedCarts.clear();
													selectedCarts.add(cart.id);
													selectedCarts = selectedCarts;
													showRecoveryModal = true;
												}}
											>
												<IconMail size={16} />
											</button>
											<button
												class="action-btn recover"
												title="Mark as Recovered"
												onclick={() => handleMarkRecovered(cart.id)}
											>
												<IconCheck size={16} />
											</button>
										{/if}
									</div>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>

				<!-- Pagination -->
				<div class="pagination">
					<div class="pagination-info">
						Showing {(pagination.current_page - 1) * pagination.per_page + 1} to {Math.min(
							pagination.current_page * pagination.per_page,
							pagination.total
						)} of {pagination.total}
					</div>
					<div class="pagination-controls">
						<button
							class="page-btn"
							disabled={pagination.current_page === 1}
							onclick={() => loadCarts(pagination.current_page - 1)}
						>
							<IconChevronLeft size={18} />
						</button>
						<span class="page-indicator"
							>Page {pagination.current_page} of {pagination.last_page}</span
						>
						<button
							class="page-btn"
							disabled={pagination.current_page === pagination.last_page}
							onclick={() => loadCarts(pagination.current_page + 1)}
						>
							<IconChevronRight size={18} />
						</button>
					</div>
				</div>
			{/if}
		</div>
	{/if}
</div>

<!-- Recovery Email Modal -->
{#if showRecoveryModal}
	<div
		class="modal-overlay"
		role="button"
		tabindex="0"
		aria-label="Close modal"
		onclick={() => (showRecoveryModal = false)}
		onkeydown={(e: KeyboardEvent) => {
			if (e.key === 'Escape' || e.key === 'Enter' || e.key === ' ') showRecoveryModal = false;
		}}
	>
		<div
			class="modal-content"
			role="dialog"
			aria-modal="true"
			tabindex="-1"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
		>
			<div class="modal-header">
				<div>
					<h2>Send Recovery Email</h2>
					<p>Sending to {selectedCarts.size} abandoned cart{selectedCarts.size > 1 ? 's' : ''}</p>
				</div>
				<button class="close-btn" onclick={() => (showRecoveryModal = false)}>
					<IconX size={20} />
				</button>
			</div>

			<div class="modal-body">
				<!-- Template Selection -->
				<div class="template-selector">
					<span class="selector-label">Choose Email Type</span>
					<div class="template-grid">
						<button
							class="template-option"
							class:active={recoveryTemplate === 'reminder_1'}
							onclick={() => (recoveryTemplate = 'reminder_1')}
						>
							<IconMail size={24} />
							<span class="template-name">First Reminder</span>
							<span class="template-timing">1 hour</span>
						</button>
						<button
							class="template-option"
							class:active={recoveryTemplate === 'reminder_2'}
							onclick={() => (recoveryTemplate = 'reminder_2')}
						>
							<IconClock size={24} />
							<span class="template-name">Urgency</span>
							<span class="template-timing">24 hours</span>
						</button>
						<button
							class="template-option"
							class:active={recoveryTemplate === 'final_discount'}
							onclick={() => (recoveryTemplate = 'final_discount')}
						>
							<IconGift size={24} />
							<span class="template-name">Final Discount</span>
							<span class="template-timing">72 hours</span>
						</button>
						<button
							class="template-option"
							class:active={recoveryTemplate === 'custom'}
							onclick={() => (recoveryTemplate = 'custom')}
						>
							<IconUsers size={24} />
							<span class="template-name">Custom</span>
							<span class="template-timing">Anytime</span>
						</button>
					</div>
				</div>

				<!-- Template Preview -->
				<div class="template-preview">
					<div class="preview-header">Template Info</div>
					<div class="preview-name">{getTemplateInfo(recoveryTemplate).name}</div>
					<div class="preview-desc">{getTemplateInfo(recoveryTemplate).description}</div>
					<div class="preview-timing">{getTemplateInfo(recoveryTemplate).timing}</div>
				</div>

				<!-- Discount Settings -->
				{#if recoveryTemplate === 'final_discount'}
					<div class="discount-settings">
						<div class="form-group">
							<label for="discount-percent">Discount Percentage</label>
							<div class="discount-slider">
								<input
									id="discount-percent"
									type="range"
									min="5"
									max="30"
									step="5"
									bind:value={discountPercent}
								/>
								<span class="discount-value">{discountPercent}%</span>
							</div>
						</div>
						<div class="form-group">
							<label for="discount-code">Discount Code</label>
							<input
								id="discount-code"
								type="text"
								placeholder="e.g., COMEBACK15"
								bind:value={discountCode}
							/>
						</div>
					</div>
				{/if}

				<!-- Custom Template Fields -->
				{#if recoveryTemplate === 'custom'}
					<div class="form-group">
						<label for="custom-subject">Subject Line</label>
						<input
							id="custom-subject"
							type="text"
							placeholder="Enter email subject..."
							bind:value={customSubject}
						/>
					</div>
					<div class="form-group">
						<label for="custom-body">Email Body</label>
						<textarea
							id="custom-body"
							rows="8"
							placeholder={'Enter email body... Use {{name}}, {{cart_items}}, {{cart_total}} for personalization'}
							bind:value={customBody}
						></textarea>
					</div>
				{/if}
			</div>

			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showRecoveryModal = false)}>Cancel</button>
				<button
					class="btn-primary"
					onclick={handleSendRecovery}
					disabled={sending || (recoveryTemplate === 'custom' && (!customSubject || !customBody))}
				>
					<IconSend size={18} />
					{sending
						? 'Sending...'
						: `Send to ${selectedCarts.size} Cart${selectedCarts.size > 1 ? 's' : ''}`}
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.abandoned-cart-page {
		padding: 2rem;
		max-width: 1600px;
		margin: 0 auto;
	}

	/* Header */
	.back-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0;
		color: #94a3b8;
		background: none;
		border: none;
		cursor: pointer;
		font-size: 0.875rem;
		margin-bottom: 1rem;
		transition: color 0.2s;
	}

	.back-btn:hover {
		color: #e6b800;
	}

	.header-content {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 2rem;
	}

	.header-title {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.title-icon {
		width: 64px;
		height: 64px;
		background: linear-gradient(135deg, #f59e0b, #d97706);
		border-radius: 20px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
		box-shadow: 0 8px 32px rgba(245, 158, 11, 0.3);
	}

	.header-title h1 {
		font-size: 2rem;
		font-weight: 800;
		background: linear-gradient(135deg, #f1f5f9, #cbd5e1);
		-webkit-background-clip: text;
		background-clip: text;
		-webkit-text-fill-color: transparent;
		margin: 0;
	}

	.header-title p {
		color: #64748b;
		margin: 0.25rem 0 0;
	}

	.btn-refresh {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 12px;
		color: #94a3b8;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-refresh:hover {
		border-color: rgba(245, 158, 11, 0.3);
		color: #fbbf24;
	}

	/* Loading State */
	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 6rem 2rem;
		color: #64748b;
	}

	.loader {
		width: 48px;
		height: 48px;
		border: 4px solid rgba(245, 158, 11, 0.2);
		border-top-color: #f59e0b;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Metrics Grid */
	.metrics-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1.5rem;
		margin-bottom: 2rem;
	}

	.metric-card {
		position: relative;
		display: flex;
		align-items: flex-start;
		gap: 1rem;
		padding: 1.5rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 20px;
	}

	.metric-icon {
		width: 56px;
		height: 56px;
		border-radius: 16px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.metric-card.total .metric-icon {
		background: rgba(245, 158, 11, 0.15);
		color: #fbbf24;
	}
	.metric-card.recovered .metric-icon {
		background: rgba(16, 185, 129, 0.15);
		color: #34d399;
	}
	.metric-card.pending .metric-icon {
		background: rgba(239, 68, 68, 0.15);
		color: #f87171;
	}
	.metric-card.average .metric-icon {
		background: rgba(230, 184, 0, 0.15);
		color: #ffd11a;
	}

	.metric-label {
		font-size: 0.75rem;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		font-weight: 600;
	}

	.metric-value {
		font-size: 2rem;
		font-weight: 800;
		color: #f1f5f9;
		line-height: 1.2;
	}

	.metric-secondary {
		font-size: 0.8125rem;
		color: #94a3b8;
		margin-top: 0.25rem;
	}

	.metric-badge {
		position: absolute;
		top: 1rem;
		right: 1rem;
		padding: 0.375rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 700;
	}

	.metric-badge.success {
		background: rgba(16, 185, 129, 0.15);
		color: #34d399;
	}

	/* Pipeline Section */
	.pipeline-section {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 20px;
		padding: 1.5rem;
		margin-bottom: 2rem;
	}

	.section-header {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		color: #f1f5f9;
		margin-bottom: 1.5rem;
	}

	.section-header h3 {
		font-size: 1.125rem;
		font-weight: 600;
		margin: 0;
	}

	.pipeline-stages {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.pipeline-stage {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.75rem;
	}

	.stage-icon {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.stage-icon.pending {
		background: rgba(245, 158, 11, 0.15);
		color: #fbbf24;
	}
	.stage-icon.sent {
		background: rgba(56, 189, 248, 0.15);
		color: #38bdf8;
	}
	.stage-icon.clicked {
		background: rgba(230, 184, 0, 0.15);
		color: #ffd11a;
	}
	.stage-icon.recovered {
		background: rgba(16, 185, 129, 0.15);
		color: #34d399;
	}

	.stage-info {
		text-align: center;
	}

	.stage-count {
		font-size: 1.5rem;
		font-weight: 800;
		color: #f1f5f9;
	}

	.stage-label {
		font-size: 0.75rem;
		color: #94a3b8;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.stage-bar {
		width: 100%;
		height: 6px;
		background: rgba(148, 163, 184, 0.1);
		border-radius: 3px;
		overflow: hidden;
	}

	.stage-fill {
		height: 100%;
		width: var(--width);
		border-radius: 3px;
		transition: width 0.5s ease-out;
	}

	.stage-fill.pending {
		background: #fbbf24;
	}
	.stage-fill.sent {
		background: #38bdf8;
	}
	.stage-fill.clicked {
		background: #ffd11a;
	}
	.stage-fill.recovered {
		background: #34d399;
	}

	.pipeline-connector {
		color: #64748b;
		flex-shrink: 0;
	}

	/* Quick Actions */
	.quick-actions {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.action-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem 1.5rem;
		background: rgba(30, 41, 59, 0.6);
		border: 2px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
		cursor: pointer;
		transition: all 0.3s;
		text-align: left;
	}

	.action-card:hover {
		transform: translateY(-2px);
	}

	.action-card.reminder:hover {
		border-color: rgba(56, 189, 248, 0.5);
	}
	.action-card.urgency:hover {
		border-color: rgba(245, 158, 11, 0.5);
	}
	.action-card.discount:hover {
		border-color: rgba(16, 185, 129, 0.5);
	}

	.action-icon {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.action-card.reminder .action-icon {
		background: rgba(56, 189, 248, 0.15);
		color: #38bdf8;
	}
	.action-card.urgency .action-icon {
		background: rgba(245, 158, 11, 0.15);
		color: #fbbf24;
	}
	.action-card.discount .action-icon {
		background: rgba(16, 185, 129, 0.15);
		color: #34d399;
	}

	.action-content h4 {
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.action-content p {
		font-size: 0.8125rem;
		color: #64748b;
		margin: 0.25rem 0 0;
	}

	.action-count {
		margin-left: auto;
		padding: 0.375rem 0.75rem;
		background: rgba(148, 163, 184, 0.1);
		border-radius: 8px;
		font-size: 0.75rem;
		font-weight: 600;
		color: #94a3b8;
	}

	/* Toolbar */
	.toolbar {
		display: flex;
		align-items: center;
		gap: 1rem;
		margin-bottom: 1rem;
	}

	.search-box {
		flex: 1;
		max-width: 400px;
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 12px;
		color: #94a3b8;
	}

	.search-box input {
		flex: 1;
		background: none;
		border: none;
		color: #f1f5f9;
		font-size: 0.9375rem;
		outline: none;
	}

	.search-box input::placeholder {
		color: #64748b;
	}

	.filter-group {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0 1rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 10px;
		color: #94a3b8;
	}

	.filter-group select {
		background: none;
		border: none;
		color: #f1f5f9;
		font-size: 0.875rem;
		padding: 0.75rem 0;
		cursor: pointer;
	}

	.btn-selected {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: linear-gradient(135deg, #f59e0b, #d97706);
		border: none;
		border-radius: 10px;
		color: white;
		font-weight: 600;
		cursor: pointer;
		box-shadow: 0 4px 14px rgba(245, 158, 11, 0.3);
	}

	/* Table */
	.carts-table-container {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
		overflow: hidden;
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		color: #64748b;
	}

	.empty-state h3 {
		color: #f1f5f9;
		margin: 1rem 0 0.5rem;
	}

	.carts-table {
		width: 100%;
		border-collapse: collapse;
	}

	.carts-table thead {
		background: rgba(15, 23, 42, 0.6);
	}

	.carts-table th {
		padding: 1rem 1.5rem;
		text-align: left;
		font-size: 0.75rem;
		font-weight: 600;
		color: #94a3b8;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.carts-table tbody tr {
		border-top: 1px solid rgba(148, 163, 184, 0.1);
		transition: background 0.2s;
	}

	.carts-table tbody tr:hover {
		background: rgba(245, 158, 11, 0.03);
	}

	.carts-table tbody tr.selected {
		background: rgba(245, 158, 11, 0.08);
	}

	.carts-table td {
		padding: 1rem 1.5rem;
		color: #cbd5e1;
	}

	.checkbox-col {
		width: 48px;
	}

	.checkbox-col input {
		width: 18px;
		height: 18px;
		cursor: pointer;
	}

	.customer-info {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.customer-avatar {
		width: 40px;
		height: 40px;
		border-radius: 50%;
		background: linear-gradient(135deg, #f59e0b, #d97706);
		display: flex;
		align-items: center;
		justify-content: center;
		font-weight: 700;
		font-size: 0.875rem;
		color: white;
	}

	.customer-name {
		font-weight: 600;
		color: #f1f5f9;
	}

	.customer-email {
		font-size: 0.8125rem;
		color: #64748b;
	}

	.cart-value {
		font-weight: 700;
		color: #34d399;
	}

	.item-count {
		font-size: 0.875rem;
		color: #94a3b8;
	}

	.status-badge {
		display: inline-flex;
		padding: 0.25rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 600;
		border: 1px solid;
	}

	.status-warning {
		background: rgba(245, 158, 11, 0.15);
		color: #fbbf24;
		border-color: rgba(245, 158, 11, 0.3);
	}

	.status-info {
		background: rgba(56, 189, 248, 0.15);
		color: #38bdf8;
		border-color: rgba(56, 189, 248, 0.3);
	}

	.status-primary {
		background: rgba(230, 184, 0, 0.15);
		color: #ffd11a;
		border-color: rgba(230, 184, 0, 0.3);
	}

	.status-success {
		background: rgba(16, 185, 129, 0.15);
		color: #34d399;
		border-color: rgba(16, 185, 129, 0.3);
	}

	.status-neutral {
		background: rgba(148, 163, 184, 0.15);
		color: #94a3b8;
		border-color: rgba(148, 163, 184, 0.3);
	}

	.abandoned-info {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.time-ago {
		font-weight: 600;
		color: #f87171;
	}

	.date-full {
		font-size: 0.75rem;
		color: #64748b;
	}

	.attempts-badge {
		display: inline-flex;
		width: 28px;
		height: 28px;
		align-items: center;
		justify-content: center;
		background: rgba(148, 163, 184, 0.1);
		border-radius: 8px;
		font-size: 0.875rem;
		font-weight: 600;
		color: #94a3b8;
	}

	.actions {
		display: flex;
		gap: 0.5rem;
	}

	.action-btn {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 6px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.action-btn.send:hover {
		background: rgba(56, 189, 248, 0.15);
		border-color: rgba(56, 189, 248, 0.3);
		color: #38bdf8;
	}

	.action-btn.recover:hover {
		background: rgba(16, 185, 129, 0.15);
		border-color: rgba(16, 185, 129, 0.3);
		color: #34d399;
	}

	/* Pagination */
	.pagination {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem 1.5rem;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
	}

	.pagination-info {
		font-size: 0.875rem;
		color: #64748b;
	}

	.pagination-controls {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.page-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.page-btn:hover:not(:disabled) {
		background: rgba(245, 158, 11, 0.15);
		border-color: rgba(245, 158, 11, 0.3);
		color: #fbbf24;
	}

	.page-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.page-indicator {
		font-size: 0.875rem;
		color: #94a3b8;
	}

	/* Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.85);
		backdrop-filter: blur(8px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 50;
		padding: 2rem;
	}

	.modal-content {
		background: linear-gradient(135deg, #1e293b, #0f172a);
		border: 1px solid rgba(245, 158, 11, 0.2);
		border-radius: 24px;
		width: 100%;
		max-width: 600px;
		max-height: 90vh;
		overflow-y: auto;
		box-shadow: 0 24px 48px rgba(0, 0, 0, 0.4);
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		padding: 1.5rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.modal-header h2 {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0;
	}

	.modal-header p {
		font-size: 0.875rem;
		color: #64748b;
		margin: 0.25rem 0 0;
	}

	.close-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(148, 163, 184, 0.1);
		border: none;
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
	}

	.modal-body {
		padding: 1.5rem;
	}

	/* Template Selector */
	.template-selector {
		margin-bottom: 1.5rem;
	}

	.selector-label {
		display: block;
		font-size: 0.75rem;
		font-weight: 600;
		color: #94a3b8;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-bottom: 0.75rem;
	}

	.template-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 0.75rem;
	}

	.template-option {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;
		padding: 1rem;
		background: rgba(30, 41, 59, 0.6);
		border: 2px solid rgba(148, 163, 184, 0.1);
		border-radius: 12px;
		cursor: pointer;
		transition: all 0.2s;
		color: #94a3b8;
	}

	.template-option:hover {
		border-color: rgba(245, 158, 11, 0.3);
	}

	.template-option.active {
		border-color: #f59e0b;
		background: rgba(245, 158, 11, 0.1);
		color: #fbbf24;
	}

	.template-name {
		font-weight: 600;
		font-size: 0.8125rem;
		color: #f1f5f9;
	}

	.template-timing {
		font-size: 0.6875rem;
		color: #64748b;
	}

	/* Template Preview */
	.template-preview {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 12px;
		padding: 1rem 1.25rem;
		margin-bottom: 1.5rem;
	}

	.preview-header {
		font-size: 0.6875rem;
		font-weight: 600;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.1em;
		margin-bottom: 0.5rem;
	}

	.preview-name {
		font-weight: 600;
		color: #f1f5f9;
		margin-bottom: 0.25rem;
	}

	.preview-desc {
		font-size: 0.8125rem;
		color: #94a3b8;
		margin-bottom: 0.5rem;
	}

	.preview-timing {
		font-size: 0.75rem;
		color: #fbbf24;
		font-style: italic;
	}

	/* Discount Settings */
	.discount-settings {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
		margin-bottom: 1rem;
	}

	.form-group {
		margin-bottom: 1rem;
	}

	.form-group label {
		display: block;
		font-size: 0.8125rem;
		font-weight: 500;
		color: #94a3b8;
		margin-bottom: 0.5rem;
	}

	.form-group input,
	.form-group textarea {
		width: 100%;
		padding: 0.875rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 12px;
		color: #f1f5f9;
		font-size: 0.9375rem;
		font-family: inherit;
		resize: vertical;
		transition: border-color 0.2s;
	}

	.form-group input:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: rgba(245, 158, 11, 0.5);
	}

	.discount-slider {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.discount-slider input[type='range'] {
		flex: 1;
		height: 8px;
		background: rgba(245, 158, 11, 0.2);
		border-radius: 4px;
		-webkit-appearance: none;
		appearance: none;
	}

	.discount-slider input[type='range']::-webkit-slider-thumb {
		-webkit-appearance: none;
		width: 20px;
		height: 20px;
		background: #f59e0b;
		border-radius: 50%;
		cursor: pointer;
	}

	.discount-value {
		min-width: 48px;
		padding: 0.5rem 0.75rem;
		background: rgba(245, 158, 11, 0.15);
		border-radius: 8px;
		font-weight: 700;
		color: #fbbf24;
		text-align: center;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1.5rem;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
	}

	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.875rem 1.5rem;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 12px;
		color: #94a3b8;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover {
		background: rgba(148, 163, 184, 0.2);
	}

	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.875rem 1.5rem;
		background: linear-gradient(135deg, #f59e0b, #d97706);
		border: none;
		border-radius: 12px;
		color: white;
		font-weight: 600;
		cursor: pointer;
		box-shadow: 0 4px 14px rgba(245, 158, 11, 0.3);
		transition: all 0.2s;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 8px 24px rgba(245, 158, 11, 0.4);
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
		transform: none;
	}

	/* Responsive */
	@media (max-width: 1200px) {
		.metrics-grid {
			grid-template-columns: repeat(2, 1fr);
		}

		.quick-actions {
			grid-template-columns: 1fr;
		}

		.template-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 768px) {
		.abandoned-cart-page {
			padding: 1rem;
		}

		.header-content {
			flex-direction: column;
			align-items: flex-start;
			gap: 1rem;
		}

		.metrics-grid {
			grid-template-columns: 1fr;
		}

		.pipeline-stages {
			flex-direction: column;
		}

		.pipeline-connector {
			transform: rotate(90deg);
		}

		.toolbar {
			flex-direction: column;
		}

		.search-box {
			max-width: 100%;
		}

		.discount-settings {
			grid-template-columns: 1fr;
		}

		.carts-table-container {
			overflow-x: auto;
		}

		.carts-table {
			min-width: 800px;
		}
	}
</style>
