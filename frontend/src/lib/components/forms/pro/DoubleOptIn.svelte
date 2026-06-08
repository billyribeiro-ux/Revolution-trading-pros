<script lang="ts">
	/**
	 * DoubleOptIn Component (FluentForms 6.1.8 - December 2025)
	 *
	 * Double opt-in confirmation system for form submissions.
	 * Sends verification email and tracks confirmation status.
	 * GDPR compliant email subscription management.
	 */

	import { onMount } from 'svelte';
	import { SvelteDate } from 'svelte/reactivity';
	import Icon from '$lib/components/Icon.svelte';

	type OptInStatus = 'pending' | 'sent' | 'confirmed' | 'expired' | 'failed';

	interface OptInData {
		status: OptInStatus;
		email: string;
		sentAt?: string;
		confirmedAt?: string;
		expiresAt?: string;
		attempts?: number;
	}

	interface Props {
		submissionId: string | number;
		formId: string | number;
		email: string;
		data?: OptInData;
		showStatus?: boolean;
		allowResend?: boolean;
		resendCooldown?: number; // seconds
		expirationHours?: number;
		onResend?: () => Promise<void>;
		onConfirm?: (token: string) => Promise<void>;
		error?: string;
	}

	let {
		submissionId: _submissionId,
		formId: _formId,
		email,
		data = { status: 'pending', email: '' },
		showStatus = true,
		allowResend = true,
		resendCooldown = 60,
		expirationHours: _expirationHours = 24,
		onResend,
		onConfirm: _onConfirm,
		error = ''
	}: Props = $props();

	let isResending = $state(false);
	let resendDeadline = $state(0);
	let localError = $state('');
	const now = new SvelteDate();

	onMount(() => {
		const interval = setInterval(() => {
			now.setTime(Date.now());
		}, 1000);

		return () => clearInterval(interval);
	});

	const statusConfig: Record<
		OptInStatus,
		{ label: string; description: string; color: string; bgColor: string }
	> = {
		pending: {
			label: 'Pending Verification',
			description: 'Awaiting email confirmation',
			color: '#92400e',
			bgColor: '#fef3c7'
		},
		sent: {
			label: 'Verification Email Sent',
			description: 'Please check your inbox and click the confirmation link',
			color: '#1e40af',
			bgColor: '#dbeafe'
		},
		confirmed: {
			label: 'Email Confirmed',
			description: 'Your email has been verified successfully',
			color: '#166534',
			bgColor: '#dcfce7'
		},
		expired: {
			label: 'Confirmation Expired',
			description: 'The confirmation link has expired. Please request a new one.',
			color: '#dc2626',
			bgColor: '#fee2e2'
		},
		failed: {
			label: 'Verification Failed',
			description: 'Unable to send verification email. Please try again.',
			color: '#991b1b',
			bgColor: '#fee2e2'
		}
	};

	function formatDate(dateString: string | undefined): string {
		if (!dateString) return '';
		return new Date(dateString).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function getTimeRemaining(): string {
		if (!data.expiresAt) return '';
		const now = new Date();
		const expires = new Date(data.expiresAt);
		const diff = expires.getTime() - now.getTime();

		if (diff <= 0) return 'Expired';

		const hours = Math.floor(diff / (1000 * 60 * 60));
		const minutes = Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60));

		if (hours > 0) {
			return `${hours}h ${minutes}m remaining`;
		}
		return `${minutes}m remaining`;
	}

	function maskEmail(emailStr: string): string {
		const [local, domain] = emailStr.split('@');
		if (!domain) return emailStr;
		const maskedLocal =
			local.length > 2 ? local[0] + '*'.repeat(local.length - 2) + local[local.length - 1] : local;
		return `${maskedLocal}@${domain}`;
	}

	async function handleResend() {
		if (isResending || resendTimer > 0 || !onResend) return;

		isResending = true;
		localError = '';

		try {
			await onResend();
			resendDeadline = Date.now() + resendCooldown * 1000;
		} catch (err) {
			localError = err instanceof Error ? err.message : 'Failed to resend verification email';
		} finally {
			isResending = false;
		}
	}

	const config = $derived(statusConfig[data.status]);
	const resendTimer = $derived(Math.max(0, Math.ceil((resendDeadline - now.getTime()) / 1000)));
	const canResend = $derived(
		allowResend && ['sent', 'expired', 'failed'].includes(data.status) && resendTimer === 0
	);
</script>

<div class={['double-opt-in', { 'has-error': error || localError }]}>
	{#if showStatus}
		<div class="status-card" style:--status-color={config.color} style:--status-bg={config.bgColor}>
			<div class="status-icon">
				{#if data.status === 'pending'}
					<Icon name="IconClock" size={24} />
				{:else if data.status === 'sent'}
					<Icon name="IconSend" size={24} />
				{:else if data.status === 'confirmed'}
					<Icon name="IconCircleCheck" size={24} />
				{:else if data.status === 'expired'}
					<Icon name="IconAlertCircle" size={24} />
				{:else}
					<Icon name="IconCircleX" size={24} />
				{/if}
			</div>

			<div class="status-content">
				<h3 class="status-title">{config.label}</h3>
				<p class="status-description">{config.description}</p>

				<div class="email-info">
					<Icon name="IconMail" size={14} />
					<span>{maskEmail(email || data?.email || '')}</span>
				</div>

				{#if data.status === 'sent' && data.expiresAt}
					<div class="expiry-info">
						<Icon name="IconClock" size={14} />
						<span>{getTimeRemaining()}</span>
					</div>
				{/if}

				{#if data.sentAt && data.status === 'sent'}
					<div class="sent-info">
						Sent on {formatDate(data.sentAt)}
						{#if data.attempts && data.attempts > 1}
							<span class="attempts-badge">Attempt #{data.attempts}</span>
						{/if}
					</div>
				{/if}

				{#if data.confirmedAt && data.status === 'confirmed'}
					<div class="confirmed-info">
						Confirmed on {formatDate(data.confirmedAt)}
					</div>
				{/if}
			</div>
		</div>
	{/if}

	{#if canResend || isResending}
		<div class="resend-section">
			<p class="resend-hint">
				Didn't receive the email? Check your spam folder or request a new one.
			</p>
			<button
				type="button"
				class="resend-btn"
				onclick={handleResend}
				disabled={isResending || resendTimer > 0}
			>
				{#if isResending}
					<span class="spinner"></span>
					Sending...
				{:else if resendTimer > 0}
					<Icon name="IconClock" size={16} />
					Resend in {resendTimer}s
				{:else}
					<Icon name="IconRotateClockwise" size={16} />
					Resend Verification Email
				{/if}
			</button>
		</div>
	{/if}

	{#if data.status === 'confirmed'}
		<div class="success-banner">
			<Icon name="IconCheck" size={20} />
			<span>Your subscription is now active!</span>
		</div>
	{/if}

	<div class="gdpr-notice">
		<Icon name="IconShield" size={14} />
		<span>Your email is protected and will only be used as per our privacy policy.</span>
	</div>

	<!-- Hidden inputs for form state -->
	<input type="hidden" name="double_opt_in_status" value={data.status} />
	<input type="hidden" name="double_opt_in_email" value={email || data?.email || ''} />

	{#if error || localError}
		<div class="error-message">
			<Icon name="IconAlertCircle" size={14} />
			{error || localError}
		</div>
	{/if}
</div>

<style>
	.double-opt-in {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.status-card {
		display: flex;
		gap: 1rem;
		padding: 1.25rem;
		background-color: var(--status-bg);
		border: 1px solid color-mix(in srgb, var(--status-color) 30%, transparent);
		border-radius: 0.75rem;
	}

	.status-icon {
		display: flex;
		align-items: flex-start;
		justify-content: center;
		flex-shrink: 0;
		width: 48px;
		height: 48px;
		background-color: white;
		border-radius: 50%;
		color: var(--status-color);
		padding: 0.75rem;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.status-content {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
		flex: 1;
	}

	.status-title {
		font-size: 1rem;
		font-weight: 700;
		color: var(--status-color);
		margin: 0;
	}

	.status-description {
		font-size: 0.875rem;
		color: #6b7280;
		margin: 0;
	}

	.email-info,
	.expiry-info {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.8125rem;
		color: #4b5563;
		margin-top: 0.5rem;
	}

	.sent-info,
	.confirmed-info {
		font-size: 0.75rem;
		color: #9ca3af;
		margin-top: 0.25rem;
	}

	.attempts-badge {
		display: inline-flex;
		padding: 0.125rem 0.375rem;
		background-color: #f3f4f6;
		border-radius: 0.25rem;
		font-size: 0.6875rem;
		margin-left: 0.5rem;
	}

	.resend-section {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		background-color: #f9fafb;
		border-radius: 0.5rem;
	}

	.resend-hint {
		font-size: 0.8125rem;
		color: #6b7280;
		margin: 0;
		text-align: center;
	}

	.resend-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1.25rem;
		background-color: white;
		border: 1px solid #d1d5db;
		border-radius: 0.5rem;
		font-size: 0.875rem;
		font-weight: 500;
		color: #374151;
		cursor: pointer;
		transition: all 0.15s;
	}

	.resend-btn:hover:not(:disabled) {
		background-color: #f9fafb;
		border-color: #9ca3af;
	}

	.resend-btn:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.spinner {
		width: 16px;
		height: 16px;
		border: 2px solid #d1d5db;
		border-top-color: #374151;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.success-banner {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.75rem;
		background-color: #dcfce7;
		border: 1px solid #86efac;
		border-radius: 0.5rem;
		color: #166534;
		font-size: 0.9375rem;
		font-weight: 500;
	}

	.gdpr-notice {
		display: flex;
		align-items: flex-start;
		gap: 0.5rem;
		padding: 0.75rem;
		background-color: #f9fafb;
		border-radius: 0.375rem;
		font-size: 0.75rem;
		color: #6b7280;
	}

	.gdpr-notice :global(svg) {
		flex-shrink: 0;
		margin-top: 0.125rem;
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
	@media (max-width: 479.98px) {
		.status-card {
			flex-direction: column;
			align-items: center;
			text-align: center;
		}

		.email-info,
		.expiry-info {
			justify-content: center;
		}
	}
</style>
