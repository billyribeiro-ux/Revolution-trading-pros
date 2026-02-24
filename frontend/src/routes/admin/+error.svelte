<script lang="ts">
import { logger } from '$lib/utils/logger';
	/**
	 * Admin Error Boundary - Enterprise L8 Pattern
	 *
	 * Specialized error handling for admin section with:
	 * - Admin-specific error messages
	 * - Quick navigation to admin dashboard
	 * - Debug information for developers
	 * - Role-based error recovery
	 *
	 * @version 1.0.0 - L8 Principal Engineer
	 */
	import { page } from '$app/state';
	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import { authStore } from '$lib/stores/auth.svelte';
	import {
		IconAlertTriangle,
		IconLock,
		IconServer,
		IconSearch,
		IconHome,
		IconRefresh,
		IconArrowLeft
	} from '$lib/icons';

	// Error details from SvelteKit
	let status = $derived(page.status);
	let message = $derived(page.error?.message || 'An unexpected error occurred');

	// Admin error configuration
	const errorConfig: Record<
		number,
		{ title: string; description: string; color: string; showDebug: boolean }
	> = {
		400: {
			title: 'Invalid Request',
			description: 'The request parameters are invalid. Please check your input and try again.',
			color: '#eab308',
			showDebug: true
		},
		401: {
			title: 'Authentication Required',
			description: 'Your session has expired. Please sign in again to continue.',
			color: '#f97316',
			showDebug: false
		},
		403: {
			title: 'Access Denied',
			description: 'You do not have sufficient permissions to access this admin resource.',
			color: '#ef4444',
			showDebug: true
		},
		404: {
			title: 'Admin Page Not Found',
			description: 'This admin page does not exist. It may have been moved or removed.',
			color: '#E6B800',
			showDebug: false
		},
		500: {
			title: 'Internal Server Error',
			description: 'An error occurred while processing your request. Our team has been notified.',
			color: '#ef4444',
			showDebug: true
		},
		502: {
			title: 'Backend Unavailable',
			description: 'The API server is temporarily unavailable. Please try again in a few moments.',
			color: '#f97316',
			showDebug: true
		},
		503: {
			title: 'Service Maintenance',
			description: 'The admin panel is currently undergoing maintenance. Please try again shortly.',
			color: '#B38F00',
			showDebug: false
		}
	};

	// Get config for current error
	let config = $derived(
		errorConfig[status] || {
			title: 'Admin Error',
			description: message,
			color: '#ef4444',
			showDebug: true
		}
	);

	// Determine icon based on status
	let ErrorIcon = $derived(
		status === 401
			? IconLock
			: status === 403
				? IconLock
				: status === 404
					? IconSearch
					: status >= 500
						? IconServer
						: IconAlertTriangle
	);

	// Log admin errors
	$effect(() => {
		if (browser && status >= 400) {
			logger.error(`[Admin Error ${status}]`, {
				message,
				path: page.url.pathname,
				timestamp: new Date().toISOString()
			});
		}
	});

	async function handleRetry() {
		if (browser) {
			window.location.reload();
		}
	}

	async function handleGoBack() {
		if (browser) {
			window.history.back();
		}
	}

	async function handleLogin() {
		authStore.clearAuth();
		await goto(`/login?redirect=${encodeURIComponent(page.url.pathname)}`);
	}
</script>

<div class="admin-error-page">
	<div class="error-card">
		<!-- Error Header -->
		<div class="error-header" style="--accent-color: {config.color}">
			<div class="error-icon-wrapper">
				<ErrorIcon size={48} stroke={1.5} />
			</div>
			<div class="error-status-badge">
				{status}
			</div>
		</div>

		<!-- Error Content -->
		<div class="error-content">
			<h1 class="error-title">{config.title}</h1>
			<p class="error-description">{config.description}</p>

			<!-- Debug Information (dev mode only) -->
			{#if import.meta.env.DEV && config.showDebug}
				<div class="debug-panel">
					<div class="debug-header">
						<span class="debug-label">Debug Information</span>
					</div>
					<div class="debug-content">
						<div class="debug-row">
							<span class="debug-key">Status:</span>
							<span class="debug-value">{status}</span>
						</div>
						<div class="debug-row">
							<span class="debug-key">Path:</span>
							<span class="debug-value">{page.url.pathname}</span>
						</div>
						<div class="debug-row">
							<span class="debug-key">Message:</span>
							<span class="debug-value error-message">{message}</span>
						</div>
						<div class="debug-row">
							<span class="debug-key">Time:</span>
							<span class="debug-value">{new Date().toISOString()}</span>
						</div>
					</div>
				</div>
			{/if}
		</div>

		<!-- Actions -->
		<div class="error-actions">
			{#if status === 401}
				<button type="button" class="btn btn-primary" onclick={handleLogin}>
					<IconLock size={18} />
					Sign In
				</button>
			{:else if status === 403}
				<a href="/admin" class="btn btn-primary">
					<IconHome size={18} />
					Admin Dashboard
				</a>
				<button type="button" class="btn btn-secondary" onclick={handleGoBack}>
					<IconArrowLeft size={18} />
					Go Back
				</button>
			{:else if status >= 500}
				<button type="button" class="btn btn-primary" onclick={handleRetry}>
					<IconRefresh size={18} />
					Retry
				</button>
				<a href="/admin" class="btn btn-secondary">
					<IconHome size={18} />
					Dashboard
				</a>
			{:else}
				<a href="/admin" class="btn btn-primary">
					<IconHome size={18} />
					Admin Dashboard
				</a>
				<button type="button" class="btn btn-secondary" onclick={handleGoBack}>
					<IconArrowLeft size={18} />
					Go Back
				</button>
			{/if}
		</div>

		<!-- Footer -->
		<div class="error-footer">
			<p>
				If this problem persists,
				<a href="mailto:support@revolutiontradingpros.com">contact technical support</a>.
			</p>
		</div>
	</div>
</div>

<style>
	.admin-error-page {
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--bg-base);
		padding: 2rem;
	}

	.error-card {
		max-width: 500px;
		width: 100%;
		background: var(--bg-elevated);
		border-radius: 1rem;
		overflow: hidden;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
	}

	.error-header {
		background: linear-gradient(
			135deg,
			var(--accent-color),
			color-mix(in srgb, var(--accent-color) 70%, #000)
		);
		padding: 2.5rem 2rem;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1rem;
	}

	.error-icon-wrapper {
		width: 80px;
		height: 80px;
		background: rgba(255, 255, 255, 0.15);
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #fff;
	}

	.error-status-badge {
		font-size: 3rem;
		font-weight: 800;
		color: #fff;
		text-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
	}

	.error-content {
		padding: 2rem;
	}

	.error-title {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--text-primary);
		margin-bottom: 0.75rem;
		text-align: center;
	}

	.error-description {
		font-size: 1rem;
		color: var(--text-secondary);
		text-align: center;
		line-height: 1.6;
		margin-bottom: 1.5rem;
	}

	.debug-panel {
		background: var(--bg-base);
		border-radius: 0.5rem;
		overflow: hidden;
		margin-top: 1rem;
	}

	.debug-header {
		background: var(--bg-elevated);
		padding: 0.75rem 1rem;
		border-bottom: 1px solid var(--border-default);
	}

	.debug-label {
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--text-tertiary);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.debug-content {
		padding: 1rem;
	}

	.debug-row {
		display: flex;
		gap: 0.75rem;
		padding: 0.5rem 0;
		border-bottom: 1px solid var(--bg-elevated);
	}

	.debug-row:last-child {
		border-bottom: none;
	}

	.debug-key {
		font-size: 0.8125rem;
		font-weight: 500;
		color: var(--text-tertiary);
		min-width: 70px;
	}

	.debug-value {
		font-size: 0.8125rem;
		color: var(--text-primary);
		font-family: 'SF Mono', Monaco, monospace;
		word-break: break-all;
	}

	.error-message {
		color: var(--error-emphasis);
	}

	.error-actions {
		display: flex;
		gap: 0.75rem;
		padding: 0 2rem 2rem;
		justify-content: center;
	}

	.btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		font-size: 0.9375rem;
		font-weight: 600;
		text-decoration: none;
		border-radius: 0.5rem;
		cursor: pointer;
		transition: all 0.2s ease;
		border: none;
	}

	.btn-primary {
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		color: var(--bg-base);
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 10px 20px rgba(230, 184, 0, 0.3);
	}

	.btn-secondary {
		background: var(--bg-surface);
		color: var(--text-primary);
	}

	.btn-secondary:hover {
		background: var(--bg-hover);
	}

	.error-footer {
		padding: 1rem 2rem;
		background: var(--bg-base);
		text-align: center;
	}

	.error-footer p {
		font-size: 0.8125rem;
		color: var(--text-tertiary);
		margin: 0;
	}

	.error-footer a {
		color: var(--primary-500);
		text-decoration: none;
	}

	.error-footer a:hover {
		text-decoration: underline;
	}

	@media (max-width: 480px) {
		.error-actions {
			flex-direction: column;
		}

		.btn {
			width: 100%;
			justify-content: center;
		}
	}
</style>
