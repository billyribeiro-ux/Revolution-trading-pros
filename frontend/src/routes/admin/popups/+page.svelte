<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import {
		IconPlus,
		IconEdit,
		IconTrash,
		IconCopy,
		IconEye,
		IconEyeOff,
		IconChartBar,
		IconSettings
	} from '$lib/icons';
	import { getAllPopups, deletePopup, togglePopupStatus, duplicatePopup } from '$lib/api/popups';
	import type { Popup } from '$lib/stores/popups';

	let popups = $state<Popup[]>([]);
	let loading = $state(true);
	let selectedTab = $state<'active' | 'inactive' | 'all'>('all');

	onMount(async () => {
		await loadPopups();
		if (browser) {
			const { gsap } = await import('gsap');
			gsap.from('.popup-card', {
				opacity: 0,
				y: 20,
				duration: 0.5,
				stagger: 0.1,
				ease: 'power3.out'
			});
		}
	});

	async function loadPopups() {
		loading = true;
		try {
			popups = await getAllPopups();
		} catch (error) {
			console.error('Error loading popups:', error);
			popups = [];
		} finally {
			loading = false;
		}
	}

	async function handleToggleStatus(popup: Popup) {
		try {
			await togglePopupStatus(popup.id, !popup.isActive);
			await loadPopups();
		} catch (error) {
			console.error('Error toggling popup status:', error);
		}
	}

	async function handleDelete(popupId: string) {
		if (!confirm('Are you sure you want to delete this popup?')) return;

		try {
			await deletePopup(popupId);
			await loadPopups();
		} catch (error) {
			console.error('Error deleting popup:', error);
		}
	}

	async function handleDuplicate(popupId: string) {
		try {
			await duplicatePopup(popupId);
			await loadPopups();
		} catch (error) {
			console.error('Error duplicating popup:', error);
		}
	}

	function getConversionRate(popup: Popup): string {
		if (popup.impressions === 0) return '0%';
		return ((popup.conversions / popup.impressions) * 100).toFixed(1) + '%';
	}

	let filteredPopups = $derived(popups.filter((popup) => {
		if (selectedTab === 'active') return popup.isActive;
		if (selectedTab === 'inactive') return !popup.isActive;
		return true;
	}));
</script>

<div class="popups-page">
	<!-- Header -->
	<div class="page-header">
		<div>
			<h1 class="page-title">Popup Manager</h1>
			<p class="page-subtitle">Create and manage custom popups for your site</p>
		</div>
		<a href="/admin/popups/create" class="btn-primary">
			<IconPlus size={20} />
			<span>Create Popup</span>
		</a>
	</div>

	<!-- Tabs -->
	<div class="tabs">
		<button
			class="tab"
			class:active={selectedTab === 'all'}
			onclick={() => (selectedTab = 'all')}
			role="tab"
			aria-selected={selectedTab === 'all'}
		>
			All Popups ({popups.length})
		</button>
		<button
			class="tab"
			class:active={selectedTab === 'active'}
			onclick={() => (selectedTab = 'active')}
			role="tab"
			aria-selected={selectedTab === 'active'}
		>
			Active ({popups.filter((p) => p.isActive).length})
		</button>
		<button
			class="tab"
			class:active={selectedTab === 'inactive'}
			onclick={() => (selectedTab = 'inactive')}
			role="tab"
			aria-selected={selectedTab === 'inactive'}
		>
			Inactive ({popups.filter((p) => !p.isActive).length})
		</button>
	</div>

	<!-- Popups Grid -->
	{#if loading}
		<div class="loading-state">
			<div class="spinner"></div>
			<p>Loading popups...</p>
		</div>
	{:else if filteredPopups.length === 0}
		<div class="empty-state">
			<IconSettings size={64} class="empty-icon" />
			<h3>No popups found</h3>
			<p>Create your first popup to get started</p>
			<a href="/admin/popups/create" class="btn-primary">
				<IconPlus size={20} />
				<span>Create Popup</span>
			</a>
		</div>
	{:else}
		<div class="popups-grid">
			{#each filteredPopups as popup}
				<div class="popup-card">
					<!-- Status Badge -->
					<div class="card-header">
						<div class="status-badge" class:active={popup.isActive}>
							{popup.isActive ? 'Active' : 'Inactive'}
						</div>
						<button
							class="icon-btn"
							onclick={() => handleToggleStatus(popup)}
							title={popup.isActive ? 'Deactivate' : 'Activate'}
						>
							{#if popup.isActive}
								<IconEye size={18} />
							{:else}
								<IconEyeOff size={18} />
							{/if}
						</button>
					</div>

					<!-- Popup Info -->
					<div class="card-body">
						<h3 class="popup-name">{popup.name}</h3>
						{#if popup.title}
							<p class="popup-title">"{popup.title}"</p>
						{/if}

						<!-- Stats -->
						<div class="popup-stats">
							<div class="stat">
								<span class="stat-label">Impressions</span>
								<span class="stat-value">{popup.impressions.toLocaleString()}</span>
							</div>
							<div class="stat">
								<span class="stat-label">Conversions</span>
								<span class="stat-value">{popup.conversions.toLocaleString()}</span>
							</div>
							<div class="stat">
								<span class="stat-label">CVR</span>
								<span class="stat-value">{getConversionRate(popup)}</span>
							</div>
						</div>

						<!-- Display Rules Preview -->
						<div class="rules-preview">
							<div class="rule-tag">
								{popup.displayRules.frequency}
							</div>
							<div class="rule-tag">
								{popup.displayRules.delaySeconds}s delay
							</div>
							{#if popup.displayRules.showOnScroll}
								<div class="rule-tag">
									Scroll {popup.displayRules.scrollPercentage}%
								</div>
							{/if}
						</div>
					</div>

					<!-- Actions -->
					<div class="card-actions">
						<a href="/admin/popups/{popup.id}/edit" class="action-btn">
							<IconEdit size={16} />
							<span>Edit</span>
						</a>
						<button class="action-btn" onclick={() => handleDuplicate(popup.id)}>
							<IconCopy size={16} />
							<span>Duplicate</span>
						</button>
						<a href="/admin/popups/{popup.id}/analytics" class="action-btn">
							<IconChartBar size={16} />
							<span>Analytics</span>
						</a>
						<button class="action-btn danger" onclick={() => handleDelete(popup.id)}>
							<IconTrash size={16} />
							<span>Delete</span>
						</button>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.popups-page {
		max-width: 1400px;
		margin: 0 auto;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 2rem;
		gap: 2rem;
	}

	.page-title {
		font-size: 2rem;
		font-weight: 700;
		color: #f1f5f9;
		margin-bottom: 0.5rem;
	}

	.page-subtitle {
		font-size: 1.125rem;
		color: #64748b;
	}

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		color: white;
		font-weight: 600;
		border-radius: 12px;
		border: none;
		cursor: pointer;
		text-decoration: none;
		transition: all 0.3s;
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 10px 25px rgba(99, 102, 241, 0.4);
	}

	/* Tabs */
	.tabs {
		display: flex;
		gap: 1rem;
		margin-bottom: 2rem;
		border-bottom: 2px solid #1e293b;
	}

	.tab {
		padding: 0.75rem 1.5rem;
		background: none;
		border: none;
		color: #94a3b8;
		font-weight: 600;
		cursor: pointer;
		border-bottom: 2px solid transparent;
		margin-bottom: -2px;
		transition: all 0.2s;
	}

	.tab:hover {
		color: #e2e8f0;
	}

	.tab.active {
		color: #6366f1;
		border-bottom-color: #6366f1;
	}

	/* Popups Grid */
	.popups-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
		gap: 1.5rem;
	}

	.popup-card {
		background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 16px;
		overflow: hidden;
		transition: all 0.3s;
	}

	.popup-card:hover {
		border-color: rgba(99, 102, 241, 0.3);
		transform: translateY(-4px);
		box-shadow: 0 10px 30px rgba(0, 0, 0, 0.3);
	}

	.card-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem 1.5rem;
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
	}

	.status-badge {
		padding: 0.375rem 0.75rem;
		border-radius: 6px;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.status-badge.active {
		background: rgba(34, 197, 94, 0.2);
		color: #4ade80;
	}

	.status-badge:not(.active) {
		background: rgba(100, 116, 139, 0.2);
		color: #94a3b8;
	}

	.icon-btn {
		background: rgba(99, 102, 241, 0.1);
		border: none;
		border-radius: 8px;
		padding: 0.5rem;
		color: #a5b4fc;
		cursor: pointer;
		transition: all 0.2s;
		display: flex;
		align-items: center;
	}

	.icon-btn:hover {
		background: rgba(99, 102, 241, 0.2);
	}

	.card-body {
		padding: 1.5rem;
	}

	.popup-name {
		font-size: 1.25rem;
		font-weight: 700;
		color: #f1f5f9;
		margin-bottom: 0.5rem;
	}

	.popup-title {
		font-size: 0.875rem;
		color: #94a3b8;
		font-style: italic;
		margin-bottom: 1rem;
	}

	.popup-stats {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 1rem;
		margin-bottom: 1rem;
		padding: 1rem;
		background: rgba(0, 0, 0, 0.2);
		border-radius: 8px;
	}

	.stat {
		display: flex;
		flex-direction: column;
		align-items: center;
		text-align: center;
	}

	.stat-label {
		font-size: 0.75rem;
		color: #94a3b8;
		margin-bottom: 0.25rem;
	}

	.stat-value {
		font-size: 1.125rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.rules-preview {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.rule-tag {
		padding: 0.25rem 0.75rem;
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 6px;
		font-size: 0.75rem;
		color: #a5b4fc;
	}

	.card-actions {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1px;
		background: rgba(99, 102, 241, 0.1);
	}

	.action-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.75rem;
		background: #1e293b;
		border: none;
		color: #94a3b8;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
	}

	.action-btn:hover {
		background: #334155;
		color: #e2e8f0;
	}

	.action-btn.danger:hover {
		background: rgba(239, 68, 68, 0.2);
		color: #f87171;
	}

	/* Loading State */
	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem;
		color: #94a3b8;
	}

	.spinner {
		width: 48px;
		height: 48px;
		border: 4px solid rgba(99, 102, 241, 0.1);
		border-top-color: #6366f1;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 1rem;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Empty State */
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem;
		text-align: center;
	}

	.empty-state h3 {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
		margin-bottom: 0.5rem;
	}

	.empty-state p {
		color: #94a3b8;
		margin-bottom: 2rem;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.page-header {
			flex-direction: column;
			align-items: flex-start;
		}

		.popups-grid {
			grid-template-columns: 1fr;
		}

		.tabs {
			overflow-x: auto;
			-webkit-overflow-scrolling: touch;
		}
	}
</style>
