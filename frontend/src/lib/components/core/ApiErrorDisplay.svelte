<script lang="ts">
	/**
	 * API Error Display Component - Apple ICT9+ Error UX
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * Displays API errors with:
	 * - User-friendly messages
	 * - Validation error details
	 * - Retry actions
	 * - Severity-based styling
	 */

	import {
		IconAlertCircle,
		IconAlertTriangle,
		IconInfoCircle,
		IconRefresh,
		IconX
	} from '$lib/icons';
	import {
		isApiError,
		getUserFriendlyMessage,
		getValidationErrors
	} from '$lib/api/enterprise/errors';
	import type { EnterpriseApiError, ErrorSeverity } from '$lib/api/enterprise/types';

	// Props
	interface Props {
		/** The error to display */
		error: Error | EnterpriseApiError | null;
		/** Whether to show retry button */
		showRetry?: boolean;
		/** Whether the error is dismissible */
		dismissible?: boolean;
		/** Callback when retry is clicked */
		onRetry?: () => void;
		/** Callback when dismissed */
		onDismiss?: () => void;
		/** Custom title */
		title?: string;
		/** Variant style */
		variant?: 'inline' | 'banner' | 'toast';
	}

	let props: Props = $props();

	// Computed - props with defaults
	const error = $derived(props.error);
	const showRetry = $derived(props.showRetry ?? false);
	const dismissible = $derived(props.dismissible ?? true);
	const onRetry = $derived(props.onRetry);
	const onDismiss = $derived(props.onDismiss);
	const title = $derived(props.title);
	const variant = $derived(props.variant ?? 'inline');

	// Computed
	const severity = $derived<ErrorSeverity>(isApiError(error) ? error.severity : 'error');

	const message = $derived(getUserFriendlyMessage(error));

	const validationErrors = $derived(isApiError(error) ? getValidationErrors(error) : {});

	const hasValidationErrors = $derived(Object.keys(validationErrors).length > 0);

	const canRetry = $derived(showRetry && isApiError(error) && error.isRetryable);

	// Icon based on severity
	const IconComponent = $derived.by(() => {
		switch (severity) {
			case 'critical':
			case 'error':
				return IconAlertCircle;
			case 'warning':
				return IconAlertTriangle;
			default:
				return IconInfoCircle;
		}
	});
</script>

{#if error}
	{@const Icon = IconComponent}
	<div
		class="api-error {variant}"
		class:critical={severity === 'critical'}
		class:error={severity === 'error'}
		class:warning={severity === 'warning'}
		class:info={severity === 'info'}
		role="alert"
		aria-live="polite"
	>
		<div class="error-icon">
			<Icon size={20} />
		</div>

		<div class="error-body">
			{#if title}
				<h4 class="error-title">{title}</h4>
			{/if}

			<p class="error-message">{message}</p>

			{#if hasValidationErrors}
				<ul class="validation-errors">
					{#each Object.entries(validationErrors) as [field, messages]}
						<li class="validation-field">
							<strong>{field}:</strong>
							{#each messages as msg}
								<span>{msg}</span>
							{/each}
						</li>
					{/each}
				</ul>
			{/if}

			{#if isApiError(error) && import.meta.env.DEV}
				<div class="error-meta">
					<span class="error-code">{error.code}</span>
					{#if error.context?.traceId}
						<span class="trace-id">Trace: {error.context.traceId.slice(0, 8)}</span>
					{/if}
				</div>
			{/if}
		</div>

		<div class="error-actions">
			{#if canRetry}
				<button class="btn-retry" onclick={onRetry} aria-label="Retry">
					<IconRefresh size={16} />
				</button>
			{/if}

			{#if dismissible}
				<button class="btn-dismiss" onclick={onDismiss} aria-label="Dismiss">
					<IconX size={16} />
				</button>
			{/if}
		</div>
	</div>
{/if}

<style>
	.api-error {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		padding: 1rem;
		border-radius: 8px;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
	}

	.api-error.critical,
	.api-error.error {
		background: rgba(239, 68, 68, 0.1);
		border-color: rgba(239, 68, 68, 0.3);
	}

	.api-error.warning {
		background: rgba(245, 158, 11, 0.1);
		border-color: rgba(245, 158, 11, 0.3);
	}

	.api-error.info {
		background: rgba(59, 130, 246, 0.1);
		border-color: rgba(59, 130, 246, 0.3);
	}

	/* Banner variant */
	.api-error.banner {
		border-radius: 0;
		border-left: none;
		border-right: none;
		border-top: none;
	}

	/* Toast variant */
	.api-error.toast {
		position: fixed;
		bottom: 1.5rem;
		right: 1.5rem;
		max-width: 400px;
		z-index: 9999;
		box-shadow: 0 10px 40px rgba(0, 0, 0, 0.3);
		animation: slideIn 0.3s ease-out;
	}

	@keyframes slideIn {
		from {
			transform: translateX(100%);
			opacity: 0;
		}
		to {
			transform: translateX(0);
			opacity: 1;
		}
	}

	.error-icon {
		flex-shrink: 0;
		margin-top: 0.125rem;
	}

	.api-error.critical .error-icon,
	.api-error.error .error-icon {
		color: #f87171;
	}

	.api-error.warning .error-icon {
		color: #fbbf24;
	}

	.api-error.info .error-icon {
		color: #60a5fa;
	}

	.error-body {
		flex: 1;
		min-width: 0;
	}

	.error-title {
		font-size: 0.875rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 0.25rem 0;
	}

	.error-message {
		font-size: 0.875rem;
		color: #94a3b8;
		margin: 0;
		line-height: 1.5;
	}

	.validation-errors {
		list-style: none;
		padding: 0;
		margin: 0.75rem 0 0 0;
		font-size: 0.8125rem;
	}

	.validation-field {
		color: #94a3b8;
		margin-bottom: 0.25rem;
	}

	.validation-field strong {
		color: #cbd5e1;
		text-transform: capitalize;
	}

	.validation-field span {
		display: block;
		padding-left: 1rem;
		color: #f87171;
	}

	.error-meta {
		display: flex;
		gap: 0.75rem;
		margin-top: 0.5rem;
		font-size: 0.75rem;
		color: #64748b;
	}

	.error-code {
		font-family: monospace;
		background: rgba(0, 0, 0, 0.2);
		padding: 0.125rem 0.375rem;
		border-radius: 4px;
	}

	.trace-id {
		font-family: monospace;
	}

	.error-actions {
		display: flex;
		gap: 0.25rem;
		flex-shrink: 0;
	}

	.btn-retry,
	.btn-dismiss {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		background: transparent;
		border: none;
		border-radius: 6px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-retry:hover,
	.btn-dismiss:hover {
		background: rgba(148, 163, 184, 0.1);
		color: #f1f5f9;
	}
</style>
