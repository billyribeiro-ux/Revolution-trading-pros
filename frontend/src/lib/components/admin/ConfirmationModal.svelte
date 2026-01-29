<script lang="ts">
	/**
	 * ConfirmationModal - Enterprise Confirmation Dialog
	 * Revolution Trading Pros - Apple ICT 11+ Principal Engineer Grade
	 *
	 * Custom modal to replace browser confirm() dialogs.
	 * Supports different variants: danger, warning, info, success
	 */
	import { IconX, IconAlertTriangle, IconTrash, IconInfoCircle, IconCheck } from '$lib/icons';

	interface Props {
		isOpen: boolean;
		title: string;
		message: string;
		confirmText?: string;
		confirmLabel?: string; // Alias for confirmText
		cancelText?: string;
		variant?: 'danger' | 'warning' | 'info' | 'success';
		isLoading?: boolean;
		showInput?: boolean;
		inputLabel?: string;
		inputPlaceholder?: string;
		inputValue?: string;
		onConfirm: (inputValue?: string) => void | Promise<void>;
		onCancel: () => void;
	}

	let {
		isOpen,
		title,
		message,
		confirmText: confirmTextProp,
		confirmLabel,
		cancelText = 'Cancel',
		variant = 'danger',
		isLoading = false,
		showInput = false,
		inputLabel = '',
		inputPlaceholder = '',
		inputValue = $bindable(''),
		onConfirm,
		onCancel
	}: Props = $props();

	// Support both confirmText and confirmLabel for flexibility
	let confirmText = $derived(confirmTextProp ?? confirmLabel ?? 'Confirm');

	let modalRef = $state<HTMLDivElement | null>(null);

	const variantConfig = {
		danger: {
			icon: IconTrash,
			iconColor: 'var(--admin-error)',
			iconBg: 'var(--admin-error-bg)',
			buttonBg: 'var(--admin-error)',
			buttonHoverBg: 'var(--color-loss)'
		},
		warning: {
			icon: IconAlertTriangle,
			iconColor: 'var(--admin-warning)',
			iconBg: 'var(--admin-warning-bg)',
			buttonBg: 'var(--admin-warning)',
			buttonHoverBg: 'var(--color-watching)'
		},
		info: {
			icon: IconInfoCircle,
			iconColor: 'var(--admin-accent-primary)',
			iconBg: 'var(--admin-accent-bg)',
			buttonBg: 'var(--admin-accent-primary)',
			buttonHoverBg: 'var(--admin-accent-primary-hover)'
		},
		success: {
			icon: IconCheck,
			iconColor: 'var(--admin-success)',
			iconBg: 'var(--admin-success-bg)',
			buttonBg: 'var(--admin-success)',
			buttonHoverBg: 'var(--color-profit)'
		}
	};

	let config = $derived(variantConfig[variant]);
	let VariantIcon = $derived(config.icon);

	function handleBackdropClick(e: MouseEvent) {
		if (e.target === e.currentTarget && !isLoading) {
			onCancel();
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape' && !isLoading) {
			onCancel();
		}
	}

	async function handleConfirm() {
		await onConfirm(showInput ? inputValue : undefined);
	}

	$effect(() => {
		if (isOpen) {
			document.body.style.overflow = 'hidden';
			modalRef?.focus();
		} else {
			document.body.style.overflow = '';
		}
		return () => {
			document.body.style.overflow = '';
		};
	});
</script>

{#if isOpen}
	<div
		class="modal-backdrop"
		role="dialog"
		aria-modal="true"
		aria-labelledby="modal-title"
		onclick={handleBackdropClick}
		onkeydown={handleKeydown}
		tabindex="-1"
		bind:this={modalRef}
	>
		<div class="modal-container">
			<!-- Icon -->
			<div class="modal-icon" style="--icon-color: {config.iconColor}; --icon-bg: {config.iconBg}">
				<VariantIcon size={28} />
			</div>

			<!-- Content -->
			<div class="modal-content">
				<h3 id="modal-title" class="modal-title">{title}</h3>
				<p class="modal-message">{message}</p>

				{#if showInput}
					<div class="modal-input-group">
						{#if inputLabel}
							<label for="modal-input" class="modal-input-label">{inputLabel}</label>
						{/if}
						<input
							id="modal-input"
							type="text"
							class="modal-input"
							placeholder={inputPlaceholder}
							bind:value={inputValue}
							disabled={isLoading}
						/>
					</div>
				{/if}
			</div>

			<!-- Actions -->
			<div class="modal-actions">
				<button type="button" class="btn-cancel" onclick={onCancel} disabled={isLoading}>
					{cancelText}
				</button>
				<button
					type="button"
					class="btn-confirm"
					style="--btn-bg: {config.buttonBg}; --btn-hover-bg: {config.buttonHoverBg}"
					onclick={handleConfirm}
					disabled={isLoading}
				>
					{#if isLoading}
						<span class="spinner"></span>
						Processing...
					{:else}
						{confirmText}
					{/if}
				</button>
			</div>

			<!-- Close button -->
			<button
				type="button"
				class="btn-close"
				onclick={onCancel}
				disabled={isLoading}
				aria-label="Close"
			>
				<IconX size={20} />
			</button>
		</div>
	</div>
{/if}

<style>
	.modal-backdrop {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.6);
		backdrop-filter: blur(4px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: var(--z-modal);
		padding: var(--space-4);
		animation: fadeIn var(--duration-fast) var(--ease-default);
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	.modal-container {
		position: relative;
		background: var(--admin-surface-primary);
		border: 1px solid var(--admin-border-subtle);
		border-radius: var(--radius-xl);
		padding: var(--space-8);
		max-width: 420px;
		width: 100%;
		display: flex;
		flex-direction: column;
		align-items: center;
		text-align: center;
		box-shadow: var(--shadow-xl);
		animation: slideUp var(--duration-normal) var(--ease-out);
	}

	@keyframes slideUp {
		from {
			opacity: 0;
			transform: translateY(20px) scale(0.95);
		}
		to {
			opacity: 1;
			transform: translateY(0) scale(1);
		}
	}

	.modal-icon {
		width: 60px;
		height: 60px;
		border-radius: var(--radius-full);
		background: var(--icon-bg);
		color: var(--icon-color);
		display: flex;
		align-items: center;
		justify-content: center;
		margin-bottom: var(--space-5);
	}

	.modal-content {
		margin-bottom: var(--space-6);
	}

	.modal-title {
		font-family: var(--font-display);
		font-size: var(--text-lg);
		font-weight: var(--font-semibold);
		color: var(--admin-text-primary);
		margin: 0 0 var(--space-2);
	}

	.modal-message {
		font-family: var(--font-sans);
		font-size: var(--text-base);
		color: var(--admin-text-secondary);
		margin: 0;
		line-height: var(--leading-normal);
	}

	.modal-input-group {
		margin-top: var(--space-4);
		text-align: left;
		width: 100%;
	}

	.modal-input-label {
		display: block;
		font-size: var(--text-sm);
		font-weight: var(--font-medium);
		color: var(--admin-text-secondary);
		margin-bottom: var(--space-2);
	}

	.modal-input {
		width: 100%;
		padding: var(--space-3) var(--space-4);
		background: var(--admin-surface-sunken);
		border: 1px solid var(--admin-border-subtle);
		border-radius: var(--radius-md);
		color: var(--admin-text-primary);
		font-size: var(--text-base);
		transition:
			var(--transition-colors),
			box-shadow var(--duration-fast) var(--ease-default);
	}

	.modal-input:focus {
		outline: none;
		border-color: var(--admin-accent-primary);
		box-shadow: var(--admin-focus-ring);
	}

	.modal-input::placeholder {
		color: var(--admin-text-muted);
	}

	.modal-actions {
		display: flex;
		gap: var(--space-3);
		width: 100%;
	}

	.btn-cancel,
	.btn-confirm {
		flex: 1;
		padding: var(--space-3) var(--space-6);
		border-radius: var(--radius-md);
		font-family: var(--font-sans);
		font-size: var(--text-base);
		font-weight: var(--font-medium);
		cursor: pointer;
		transition: var(--transition-all);
		min-height: 44px;
	}

	.btn-cancel {
		background: transparent;
		border: 1px solid var(--admin-border-subtle);
		color: var(--admin-text-secondary);
	}

	.btn-cancel:hover:not(:disabled) {
		background: var(--admin-surface-hover);
		border-color: var(--admin-border-light);
	}

	.btn-confirm {
		background: var(--btn-bg);
		border: none;
		color: var(--color-text-primary);
	}

	.btn-confirm:hover:not(:disabled) {
		background: var(--btn-hover-bg);
	}

	.btn-cancel:disabled,
	.btn-confirm:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-confirm:focus-visible,
	.btn-cancel:focus-visible {
		box-shadow: var(--admin-focus-ring);
		outline: none;
	}

	.btn-close {
		position: absolute;
		top: var(--space-4);
		right: var(--space-4);
		background: transparent;
		border: none;
		color: var(--admin-text-muted);
		cursor: pointer;
		padding: var(--space-1);
		border-radius: var(--radius-sm);
		transition: var(--transition-all);
		min-width: 44px;
		min-height: 44px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.btn-close:hover:not(:disabled) {
		background: var(--admin-surface-hover);
		color: var(--admin-text-primary);
	}

	.spinner {
		display: inline-block;
		width: 16px;
		height: 16px;
		border: 2px solid transparent;
		border-top-color: currentColor;
		border-radius: var(--radius-full);
		animation: spin 0.8s linear infinite;
		margin-right: var(--space-2);
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE - Mobile (< sm: 640px)
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: calc(var(--breakpoint-sm) - 1px)) {
		.modal-container {
			padding: var(--space-6);
			max-width: 100%;
		}

		.modal-actions {
			flex-direction: column;
		}

		.btn-cancel,
		.btn-confirm {
			width: 100%;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ACCESSIBILITY
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		.modal-backdrop,
		.modal-container {
			animation: none;
		}
	}
</style>
