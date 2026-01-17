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
			buttonHoverBg: '#dc2626'
		},
		warning: {
			icon: IconAlertTriangle,
			iconColor: 'var(--admin-warning)',
			iconBg: 'var(--admin-warning-bg)',
			buttonBg: 'var(--admin-warning)',
			buttonHoverBg: '#d97706'
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
			buttonHoverBg: '#16a34a'
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
			<button type="button" class="btn-close" onclick={onCancel} disabled={isLoading} aria-label="Close">
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
		z-index: var(--z-modal, 1000);
		padding: 1rem;
		animation: fadeIn 0.2s ease;
	}

	@keyframes fadeIn {
		from { opacity: 0; }
		to { opacity: 1; }
	}

	.modal-container {
		position: relative;
		background: var(--admin-surface-primary);
		border: 1px solid var(--admin-border-subtle);
		border-radius: var(--radius-xl, 1rem);
		padding: 2rem;
		max-width: 420px;
		width: 100%;
		display: flex;
		flex-direction: column;
		align-items: center;
		text-align: center;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
		animation: slideUp 0.3s cubic-bezier(0.16, 1, 0.3, 1);
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
		border-radius: 50%;
		background: var(--icon-bg);
		color: var(--icon-color);
		display: flex;
		align-items: center;
		justify-content: center;
		margin-bottom: 1.25rem;
	}

	.modal-content {
		margin-bottom: 1.5rem;
	}

	.modal-title {
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--admin-text-primary);
		margin: 0 0 0.5rem;
	}

	.modal-message {
		font-family: var(--font-body), 'Roboto', sans-serif;
		font-size: 0.9375rem;
		color: var(--admin-text-secondary);
		margin: 0;
		line-height: 1.5;
	}

	.modal-input-group {
		margin-top: 1rem;
		text-align: left;
		width: 100%;
	}

	.modal-input-label {
		display: block;
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--admin-text-secondary);
		margin-bottom: 0.5rem;
	}

	.modal-input {
		width: 100%;
		padding: 0.75rem 1rem;
		background: var(--admin-surface-sunken);
		border: 1px solid var(--admin-border-subtle);
		border-radius: var(--radius-md, 0.5rem);
		color: var(--admin-text-primary);
		font-size: 0.9375rem;
		transition: border-color 0.2s ease, box-shadow 0.2s ease;
	}

	.modal-input:focus {
		outline: none;
		border-color: var(--admin-accent-primary);
		box-shadow: 0 0 0 3px rgba(230, 184, 0, 0.15);
	}

	.modal-input::placeholder {
		color: var(--admin-text-muted);
	}

	.modal-actions {
		display: flex;
		gap: 0.75rem;
		width: 100%;
	}

	.btn-cancel,
	.btn-confirm {
		flex: 1;
		padding: 0.75rem 1.5rem;
		border-radius: var(--radius-md, 0.5rem);
		font-family: var(--font-body), 'Roboto', sans-serif;
		font-size: 0.9375rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
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
		color: #0D1117;
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
		top: 1rem;
		right: 1rem;
		background: transparent;
		border: none;
		color: var(--admin-text-muted);
		cursor: pointer;
		padding: 0.25rem;
		border-radius: var(--radius-sm, 0.25rem);
		transition: all 0.2s ease;
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
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
		margin-right: 0.5rem;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}
</style>
