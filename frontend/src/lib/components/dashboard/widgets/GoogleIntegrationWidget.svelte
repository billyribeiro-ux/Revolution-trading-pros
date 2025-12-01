<script lang="ts">
	import { createEventDispatcher, onMount } from 'svelte';
	import { fly, scale, fade } from 'svelte/transition';
	import { cubicOut, backOut, elasticOut } from 'svelte/easing';
	import { spring, tweened } from 'svelte/motion';

	// Config available for widget customization
	export const config: Record<string, unknown> = {};

	const dispatch = createEventDispatcher();

	interface Integration {
		id: string;
		name: string;
		description: string;
		icon: string;
		color: string;
		connected: boolean;
		lastSync?: string;
		status: 'disconnected' | 'connected' | 'syncing' | 'error';
		errorMessage?: string;
	}

	let mounted = false;
	let selectedIntegration: Integration | null = null;
	let showConnectModal = false;

	let integrations: Integration[] = [
		{
			id: 'google_search_console',
			name: 'Google Search Console',
			description: 'Track keyword rankings, impressions, and search performance',
			icon: 'search-console',
			color: '#4285f4',
			connected: false,
			status: 'disconnected'
		},
		{
			id: 'google_analytics',
			name: 'Google Analytics 4',
			description: 'Monitor visitor traffic, behavior, and conversion metrics',
			icon: 'analytics',
			color: '#f4b400',
			connected: false,
			status: 'disconnected'
		},
		{
			id: 'google_tag_manager',
			name: 'Google Tag Manager',
			description: 'Manage and deploy marketing tags without code changes',
			icon: 'tag-manager',
			color: '#4285f4',
			connected: false,
			status: 'disconnected'
		},
		{
			id: 'google_ads',
			name: 'Google Ads',
			description: 'Track ad performance, conversions, and ROI',
			icon: 'ads',
			color: '#34a853',
			connected: false,
			status: 'disconnected'
		}
	];

	const progressValue = tweened(0, { duration: 2000, easing: cubicOut });

	onMount(async () => {
		mounted = true;
		// Fetch actual integration status from API
		await fetchIntegrationStatus();
	});

	async function fetchIntegrationStatus() {
		try {
			const response = await fetch('/api/integrations/status');
			if (response.ok) {
				const data = await response.json();
				integrations = integrations.map((int) => ({
					...int,
					connected: data[int.id]?.connected || false,
					status: data[int.id]?.connected ? 'connected' : 'disconnected',
					lastSync: data[int.id]?.last_sync
				}));
			}
		} catch (error) {
			console.error('Failed to fetch integration status:', error);
		}
	}

	async function handleConnect(integration: Integration) {
		selectedIntegration = integration;

		if (integration.connected) {
			// Show disconnect confirmation
			showConnectModal = true;
		} else {
			// Initiate OAuth flow
			integration.status = 'syncing';
			integrations = [...integrations];

			try {
				const response = await fetch(`/api/integrations/${integration.id}/auth-url`);
				const data = await response.json();

				if (data.auth_url) {
					// Open OAuth popup
					const popup = window.open(
						data.auth_url,
						'google-auth',
						'width=600,height=700,scrollbars=yes'
					);

					// Listen for OAuth callback
					window.addEventListener('message', (event) => {
						if (event.data.type === 'oauth-callback' && event.data.provider === integration.id) {
							if (event.data.success) {
								integration.connected = true;
								integration.status = 'connected';
								integration.lastSync = new Date().toISOString();
							} else {
								integration.status = 'error';
								integration.errorMessage = event.data.error;
							}
							integrations = [...integrations];
						}
					});
				}
			} catch (error) {
				integration.status = 'error';
				integration.errorMessage = 'Failed to connect';
				integrations = [...integrations];
			}
		}
	}

	async function handleDisconnect() {
		if (!selectedIntegration) return;

		try {
			await fetch(`/api/integrations/${selectedIntegration.id}/disconnect`, { method: 'POST' });
			selectedIntegration.connected = false;
			selectedIntegration.status = 'disconnected';
			selectedIntegration.lastSync = undefined;
			integrations = [...integrations];
		} catch (error) {
			console.error('Failed to disconnect:', error);
		}

		showConnectModal = false;
		selectedIntegration = null;
	}

	function getIconPath(icon: string): string {
		switch (icon) {
			case 'search-console':
				return `<path d="M15.5 14h-.79l-.28-.27C15.41 12.59 16 11.11 16 9.5 16 5.91 13.09 3 9.5 3S3 5.91 3 9.5 5.91 16 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z"/>`;
			case 'analytics':
				return `<path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM9 17H7v-7h2v7zm4 0h-2V7h2v10zm4 0h-2v-4h2v4z"/>`;
			case 'tag-manager':
				return `<path d="M21 7L9 19l-5.5-5.5 1.41-1.41L9 16.17 19.59 5.59 21 7z"/>`;
			case 'ads':
				return `<path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1.41 16.09V20h-2.67v-1.93c-1.71-.36-3.16-1.46-3.27-3.4h1.96c.1 1.05.82 1.87 2.65 1.87 1.96 0 2.4-.98 2.4-1.59 0-.83-.44-1.61-2.67-2.14-2.48-.6-4.18-1.62-4.18-3.67 0-1.72 1.39-2.84 3.11-3.21V4h2.67v1.95c1.86.45 2.79 1.86 2.85 3.39H14.3c-.05-1.11-.64-1.87-2.22-1.87-1.5 0-2.4.68-2.4 1.64 0 .84.65 1.39 2.67 1.91s4.18 1.39 4.18 3.91c-.01 1.83-1.38 2.83-3.12 3.16z"/>`;
			default:
				return '';
		}
	}

	$: connectedCount = integrations.filter((i) => i.connected).length;
</script>

<div class="google-integration-widget">
	<!-- Header with Progress -->
	<div class="widget-header">
		<div class="header-content">
			<div class="google-logo">
				<svg viewBox="0 0 24 24" class="google-g">
					<path
						fill="#4285F4"
						d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"
					/>
					<path
						fill="#34A853"
						d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"
					/>
					<path
						fill="#FBBC05"
						d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"
					/>
					<path
						fill="#EA4335"
						d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"
					/>
				</svg>
			</div>
			<div class="header-text">
				<h3>Google Integrations</h3>
				<p>Connect your Google tools for enhanced analytics</p>
			</div>
		</div>

		<div class="progress-indicator">
			<div class="progress-ring">
				<svg viewBox="0 0 36 36">
					<circle class="progress-bg" cx="18" cy="18" r="16" />
					<circle
						class="progress-fill"
						cx="18"
						cy="18"
						r="16"
						style="stroke-dashoffset: {100 - (connectedCount / integrations.length) * 100}"
					/>
				</svg>
				<span class="progress-text">{connectedCount}/{integrations.length}</span>
			</div>
		</div>
	</div>

	<!-- Integration Cards -->
	<div class="integrations-grid">
		{#each integrations as integration, i}
			{#if mounted}
				<div
					class="integration-card"
					class:connected={integration.connected}
					class:syncing={integration.status === 'syncing'}
					class:error={integration.status === 'error'}
					in:fly={{ y: 20, duration: 500, delay: 100 + i * 80, easing: cubicOut }}
					role="button"
					tabindex="0"
					onclick={() => handleConnect(integration)}
					onkeypress={(e) => e.key === 'Enter' && handleConnect(integration)}
				>
					<!-- Status Indicator -->
					<div class="status-dot" class:connected={integration.connected}>
						{#if integration.status === 'syncing'}
							<div class="syncing-indicator"></div>
						{/if}
					</div>

					<!-- Icon -->
					<div class="integration-icon" style="background: {integration.color}15">
						<svg
							viewBox="0 0 24 24"
							fill={integration.color}
							width="24"
							height="24"
						>
							{@html getIconPath(integration.icon)}
						</svg>
					</div>

					<!-- Info -->
					<div class="integration-info">
						<h4>{integration.name}</h4>
						<p>{integration.description}</p>
						{#if integration.lastSync}
							<span class="last-sync">
								Last synced: {new Date(integration.lastSync).toLocaleDateString()}
							</span>
						{/if}
						{#if integration.errorMessage}
							<span class="error-message">{integration.errorMessage}</span>
						{/if}
					</div>

					<!-- Action Button -->
					<button
						class="connect-button"
						class:connected={integration.connected}
						style="--accent: {integration.color}"
					>
						{#if integration.status === 'syncing'}
							<div class="button-spinner"></div>
						{:else if integration.connected}
							<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
								<polyline points="20 6 9 17 4 12" />
							</svg>
							Connected
						{:else}
							<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
								<line x1="12" y1="5" x2="12" y2="19" />
								<line x1="5" y1="12" x2="19" y2="12" />
							</svg>
							Connect
						{/if}
					</button>

					<!-- Hover Effect -->
					<div class="card-glow" style="background: {integration.color}"></div>
				</div>
			{/if}
		{/each}
	</div>

	<!-- Benefits Section -->
	<div class="benefits-section">
		<h4>Why Connect?</h4>
		<div class="benefits-grid">
			<div class="benefit-item">
				<div class="benefit-icon">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<polyline points="22 12 18 12 15 21 9 3 6 12 2 12" />
					</svg>
				</div>
				<span>Real-time data sync</span>
			</div>
			<div class="benefit-item">
				<div class="benefit-icon">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<rect x="3" y="3" width="18" height="18" rx="2" />
						<path d="M3 9h18" />
						<path d="M9 21V9" />
					</svg>
				</div>
				<span>Unified dashboard</span>
			</div>
			<div class="benefit-item">
				<div class="benefit-icon">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" />
					</svg>
				</div>
				<span>Secure OAuth 2.0</span>
			</div>
			<div class="benefit-item">
				<div class="benefit-icon">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<circle cx="12" cy="12" r="10" />
						<polyline points="12 6 12 12 16 14" />
					</svg>
				</div>
				<span>Auto-refresh data</span>
			</div>
		</div>
	</div>
</div>

<!-- Disconnect Modal -->
{#if showConnectModal && selectedIntegration}
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="modal-overlay" onclick={() => (showConnectModal = false)} transition:fade>
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div
			class="modal-content"
			onclick={(e) => e.stopPropagation()}
			in:scale={{ start: 0.9, duration: 300, easing: backOut }}
		>
			<h3>Disconnect {selectedIntegration.name}?</h3>
			<p>
				This will stop syncing data from {selectedIntegration.name}. Your historical data will be preserved.
			</p>
			<div class="modal-actions">
				<button class="modal-button secondary" onclick={() => (showConnectModal = false)}>
					Cancel
				</button>
				<button class="modal-button danger" onclick={handleDisconnect}> Disconnect </button>
			</div>
		</div>
	</div>
{/if}

<style>
	.google-integration-widget {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.widget-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding-bottom: 1rem;
		border-bottom: 1px solid rgba(0, 0, 0, 0.06);
	}

	.header-content {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.google-logo {
		width: 48px;
		height: 48px;
		background: white;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
	}

	.google-g {
		width: 28px;
		height: 28px;
	}

	.header-text h3 {
		font-size: 1.125rem;
		font-weight: 600;
		color: #1d1d1f;
		margin: 0;
	}

	.header-text p {
		font-size: 0.8rem;
		color: #86868b;
		margin: 0.25rem 0 0;
	}

	.progress-indicator {
		display: flex;
		align-items: center;
	}

	.progress-ring {
		position: relative;
		width: 48px;
		height: 48px;
	}

	.progress-ring svg {
		transform: rotate(-90deg);
	}

	.progress-bg {
		fill: none;
		stroke: #e5e7eb;
		stroke-width: 3;
	}

	.progress-fill {
		fill: none;
		stroke: #34a853;
		stroke-width: 3;
		stroke-linecap: round;
		stroke-dasharray: 100;
		transition: stroke-dashoffset 1s cubic-bezier(0.4, 0, 0.2, 1);
	}

	.progress-text {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		font-size: 0.7rem;
		font-weight: 600;
		color: #1d1d1f;
	}

	/* Integration Cards */
	.integrations-grid {
		display: flex;
		flex-direction: column;
		gap: 0.875rem;
	}

	.integration-card {
		position: relative;
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem 1.25rem;
		background: #f9fafb;
		border-radius: 16px;
		cursor: pointer;
		transition: all 0.3s cubic-bezier(0.25, 0.1, 0.25, 1);
		overflow: hidden;
	}

	.integration-card:hover {
		background: white;
		box-shadow: 0 4px 16px rgba(0, 0, 0, 0.08);
		transform: translateY(-2px);
	}

	.integration-card.connected {
		background: linear-gradient(135deg, #f0fdf4 0%, #dcfce7 100%);
	}

	.integration-card.error {
		background: linear-gradient(135deg, #fef2f2 0%, #fee2e2 100%);
	}

	.status-dot {
		position: absolute;
		top: 12px;
		right: 12px;
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: #d1d5db;
	}

	.status-dot.connected {
		background: #22c55e;
		box-shadow: 0 0 8px rgba(34, 197, 94, 0.5);
	}

	.syncing-indicator {
		width: 100%;
		height: 100%;
		border-radius: 50%;
		background: #f59e0b;
		animation: pulse 1.5s ease-in-out infinite;
	}

	@keyframes pulse {
		0%,
		100% {
			transform: scale(1);
			opacity: 1;
		}
		50% {
			transform: scale(1.5);
			opacity: 0.5;
		}
	}

	.integration-icon {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.integration-info {
		flex: 1;
		min-width: 0;
	}

	.integration-info h4 {
		font-size: 0.9rem;
		font-weight: 600;
		color: #1d1d1f;
		margin: 0 0 0.25rem;
	}

	.integration-info p {
		font-size: 0.75rem;
		color: #6b7280;
		margin: 0;
		line-height: 1.4;
	}

	.last-sync {
		display: block;
		font-size: 0.65rem;
		color: #22c55e;
		margin-top: 0.25rem;
	}

	.error-message {
		display: block;
		font-size: 0.65rem;
		color: #ef4444;
		margin-top: 0.25rem;
	}

	.connect-button {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		border-radius: 980px;
		font-size: 0.75rem;
		font-weight: 600;
		background: white;
		color: #1d1d1f;
		border: 1px solid #e5e7eb;
		cursor: pointer;
		transition: all 0.3s ease;
		flex-shrink: 0;
	}

	.connect-button:hover {
		background: var(--accent);
		color: white;
		border-color: var(--accent);
	}

	.connect-button.connected {
		background: #22c55e;
		color: white;
		border-color: #22c55e;
	}

	.connect-button svg {
		width: 14px;
		height: 14px;
	}

	.button-spinner {
		width: 14px;
		height: 14px;
		border: 2px solid rgba(0, 0, 0, 0.1);
		border-top-color: currentColor;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.card-glow {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		opacity: 0;
		pointer-events: none;
		transition: opacity 0.3s ease;
	}

	.integration-card:hover .card-glow {
		opacity: 0.03;
	}

	/* Benefits Section */
	.benefits-section {
		padding-top: 1rem;
		border-top: 1px solid rgba(0, 0, 0, 0.06);
	}

	.benefits-section h4 {
		font-size: 0.8rem;
		font-weight: 600;
		color: #86868b;
		margin: 0 0 0.75rem;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.benefits-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 0.75rem;
	}

	.benefit-item {
		display: flex;
		align-items: center;
		gap: 0.625rem;
	}

	.benefit-icon {
		width: 28px;
		height: 28px;
		border-radius: 6px;
		background: #f3f4f6;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #6b7280;
	}

	.benefit-icon svg {
		width: 14px;
		height: 14px;
	}

	.benefit-item span {
		font-size: 0.75rem;
		color: #4b5563;
	}

	/* Modal */
	.modal-overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: rgba(0, 0, 0, 0.5);
		backdrop-filter: blur(4px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
	}

	.modal-content {
		background: white;
		border-radius: 20px;
		padding: 2rem;
		max-width: 400px;
		width: 90%;
		text-align: center;
	}

	.modal-content h3 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #1d1d1f;
		margin: 0 0 0.5rem;
	}

	.modal-content p {
		font-size: 0.875rem;
		color: #6b7280;
		margin: 0 0 1.5rem;
	}

	.modal-actions {
		display: flex;
		gap: 0.75rem;
		justify-content: center;
	}

	.modal-button {
		padding: 0.75rem 1.5rem;
		border-radius: 980px;
		font-size: 0.875rem;
		font-weight: 600;
		cursor: pointer;
		border: none;
		transition: all 0.2s ease;
	}

	.modal-button.secondary {
		background: #f3f4f6;
		color: #1d1d1f;
	}

	.modal-button.secondary:hover {
		background: #e5e7eb;
	}

	.modal-button.danger {
		background: #ef4444;
		color: white;
	}

	.modal-button.danger:hover {
		background: #dc2626;
	}

	/* Dark mode */
	@media (prefers-color-scheme: dark) {
		.header-text h3,
		.integration-info h4 {
			color: #f5f5f7;
		}

		.integration-card {
			background: #2c2c2e;
		}

		.integration-card:hover {
			background: #3a3a3c;
		}

		.modal-content {
			background: #2c2c2e;
		}

		.modal-content h3 {
			color: #f5f5f7;
		}
	}
</style>
