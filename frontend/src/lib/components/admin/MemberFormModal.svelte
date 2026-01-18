<script lang="ts">
	/**
	 * MemberFormModal - Create/Edit Member Modal
	 * Revolution Trading Pros - Apple ICT 11+ Principal Engineer Grade
	 *
	 * Enterprise-grade modal for member CRUD operations.
	 */
	import {
		membersApi,
		type Member,
		type CreateMemberRequest,
		type UpdateMemberRequest
	} from '$lib/api/members';
	import { IconX, IconUserPlus, IconUserEdit, IconEye, IconEyeOff, IconCopy } from '$lib/icons';

	interface Props {
		isOpen: boolean;
		mode?: 'create' | 'edit';
		member?: Member | null;
		onSave?: (member: Member, temporaryPassword?: string) => void;
		onSaved?: (member: Member, temporaryPassword?: string) => void;
		onClose: () => void;
	}

	let { isOpen, mode: modeProp, member = null, onSave, onSaved, onClose }: Props = $props();

	// Derive mode from props - if member provided, default to edit
	let mode = $derived(modeProp ?? (member ? 'edit' : 'create'));

	// Support both onSave and onSaved for flexibility
	const handleSaved = (member: Member, tempPassword?: string) => {
		onSave?.(member, tempPassword);
		onSaved?.(member, tempPassword);
	};

	// Form state
	let name = $state('');
	let email = $state('');
	let password = $state('');
	let role = $state('user');
	let sendWelcomeEmail = $state(true);

	// UI state
	let isLoading = $state(false);
	let error = $state('');
	let showPassword = $state(false);
	let temporaryPassword = $state<string | null>(null);

	const roles = [
		{ value: 'user', label: 'Member' },
		{ value: 'admin', label: 'Admin' },
		{ value: 'super_admin', label: 'Super Admin' }
	];

	// Initialize form when member changes
	$effect(() => {
		if (isOpen && mode === 'edit' && member) {
			name = member.name || '';
			email = member.email || '';
			role = 'user'; // Would need to fetch from member details
			password = '';
		} else if (isOpen && mode === 'create') {
			name = '';
			email = '';
			password = '';
			role = 'user';
			sendWelcomeEmail = true;
		}
		temporaryPassword = null;
		error = '';
	});

	$effect(() => {
		if (isOpen) {
			document.body.style.overflow = 'hidden';
		} else {
			document.body.style.overflow = '';
		}
		return () => {
			document.body.style.overflow = '';
		};
	});

	function validateForm(): boolean {
		if (!name.trim()) {
			error = 'Name is required';
			return false;
		}
		if (!email.trim()) {
			error = 'Email is required';
			return false;
		}
		if (!email.includes('@')) {
			error = 'Please enter a valid email address';
			return false;
		}
		if (mode === 'create' && password && password.length < 8) {
			error = 'Password must be at least 8 characters';
			return false;
		}
		return true;
	}

	async function handleSubmit() {
		if (!validateForm()) return;

		isLoading = true;
		error = '';

		try {
			if (mode === 'create') {
				const data: CreateMemberRequest = {
					name: name.trim(),
					email: email.trim(),
					role,
					send_welcome_email: sendWelcomeEmail
				};
				if (password) {
					data.password = password;
				}

				const result = await membersApi.createMember(data);
				temporaryPassword = result.temporary_password || null;

				if (temporaryPassword) {
					// Show the temporary password before closing
				} else {
					handleSaved(result.member);
					onClose();
				}
			} else if (mode === 'edit' && member) {
				const data: UpdateMemberRequest = {};
				if (name.trim() !== member.name) data.name = name.trim();
				if (email.trim() !== member.email) data.email = email.trim();
				if (password) data.password = password;

				const result = await membersApi.updateMember(member.id, data);
				handleSaved(result.member);
				onClose();
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'An error occurred';
		} finally {
			isLoading = false;
		}
	}

	function handleBackdropClick(e: MouseEvent) {
		if (e.target === e.currentTarget && !isLoading) {
			onClose();
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape' && !isLoading) {
			onClose();
		}
	}

	function generatePassword() {
		const chars = 'ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnpqrstuvwxyz23456789!@#$%';
		password = Array.from(
			{ length: 16 },
			() => chars[Math.floor(Math.random() * chars.length)]
		).join('');
		showPassword = true;
	}

	async function copyPassword(text: string) {
		await navigator.clipboard.writeText(text);
	}

	function closeAndClear() {
		temporaryPassword = null;
		onClose();
	}
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
	>
		<div class="modal-container">
			<!-- Header -->
			<div class="modal-header">
				<div class="header-icon">
					{#if mode === 'create'}
						<IconUserPlus size={24} />
					{:else}
						<IconUserEdit size={24} />
					{/if}
				</div>
				<h2 id="modal-title" class="modal-title">
					{mode === 'create' ? 'Create New Member' : 'Edit Member'}
				</h2>
				<button
					type="button"
					class="btn-close"
					onclick={closeAndClear}
					disabled={isLoading}
					aria-label="Close"
				>
					<IconX size={20} />
				</button>
			</div>

			{#if temporaryPassword}
				<!-- Temporary Password Display -->
				<div class="password-created">
					<div class="password-icon">
						<IconUserPlus size={32} />
					</div>
					<h3>Member Created Successfully</h3>
					<p>A temporary password has been generated. Please share this with the member:</p>
					<div class="password-display">
						<code>{temporaryPassword}</code>
						<button
							type="button"
							class="btn-copy"
							onclick={() => copyPassword(temporaryPassword!)}
							title="Copy password"
						>
							<IconCopy size={18} />
						</button>
					</div>
					<p class="password-warning">
						This password will not be shown again. Make sure to save it.
					</p>
					<button type="button" class="btn-primary" onclick={closeAndClear}>Done</button>
				</div>
			{:else}
				<!-- Form -->
				<form
					class="modal-form"
					onsubmit={(e) => {
						e.preventDefault();
						handleSubmit();
					}}
				>
					{#if error}
						<div class="error-banner">{error}</div>
					{/if}

					<div class="form-group">
						<label for="name" class="form-label">Full Name <span class="required">*</span></label>
						<input
							id="name"
							type="text"
							class="form-input"
							placeholder="Enter member's full name"
							bind:value={name}
							disabled={isLoading}
							autocomplete="name"
						/>
					</div>

					<div class="form-group">
						<label for="email" class="form-label"
							>Email Address <span class="required">*</span></label
						>
						<input
							id="email"
							type="email"
							class="form-input"
							placeholder="member@example.com"
							bind:value={email}
							disabled={isLoading}
							autocomplete="email"
						/>
					</div>

					<div class="form-group">
						<label for="role" class="form-label">Role</label>
						<select id="role" class="form-select" bind:value={role} disabled={isLoading}>
							{#each roles as r (r.value)}
								<option value={r.value}>{r.label}</option>
							{/each}
						</select>
					</div>

					<div class="form-group">
						<label for="password" class="form-label">
							{mode === 'create' ? 'Password' : 'New Password'}
							{#if mode === 'create'}
								<span class="label-hint">(Leave blank to auto-generate)</span>
							{:else}
								<span class="label-hint">(Leave blank to keep current)</span>
							{/if}
						</label>
						<div class="password-input-wrapper">
							<input
								id="password"
								type={showPassword ? 'text' : 'password'}
								class="form-input"
								placeholder={mode === 'create' ? 'Auto-generated if empty' : 'Enter new password'}
								bind:value={password}
								disabled={isLoading}
								autocomplete="new-password"
							/>
							<button
								type="button"
								class="btn-toggle-password"
								onclick={() => (showPassword = !showPassword)}
								title={showPassword ? 'Hide password' : 'Show password'}
							>
								{#if showPassword}
									<IconEyeOff size={18} />
								{:else}
									<IconEye size={18} />
								{/if}
							</button>
						</div>
						<button
							type="button"
							class="btn-generate"
							onclick={generatePassword}
							disabled={isLoading}
						>
							Generate Strong Password
						</button>
					</div>

					{#if mode === 'create'}
						<div class="form-group checkbox-group">
							<label class="checkbox-label">
								<input type="checkbox" bind:checked={sendWelcomeEmail} disabled={isLoading} />
								<span class="checkbox-text">Send welcome email with login details</span>
							</label>
						</div>
					{/if}

					<!-- Actions -->
					<div class="modal-actions">
						<button type="button" class="btn-cancel" onclick={closeAndClear} disabled={isLoading}>
							Cancel
						</button>
						<button type="submit" class="btn-submit" disabled={isLoading}>
							{#if isLoading}
								<span class="spinner"></span>
								{mode === 'create' ? 'Creating...' : 'Saving...'}
							{:else}
								{mode === 'create' ? 'Create Member' : 'Save Changes'}
							{/if}
						</button>
					</div>
				</form>
			{/if}
		</div>
	</div>
{/if}

<style>
	.modal-backdrop {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.8);
		backdrop-filter: blur(8px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 1rem;
		animation: fadeIn 0.2s ease;
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
		background: linear-gradient(145deg, #1e293b 0%, #0f172a 100%);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 1rem;
		max-width: 480px;
		width: 100%;
		max-height: 90vh;
		overflow-y: auto;
		box-shadow: 0 25px 60px -15px rgba(0, 0, 0, 0.7), 0 0 40px -10px rgba(230, 184, 0, 0.1);
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

	.modal-header {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1.25rem 1.5rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.15);
		background: rgba(15, 23, 42, 0.5);
	}

	.header-icon {
		width: 40px;
		height: 40px;
		border-radius: 0.5rem;
		background: rgba(230, 184, 0, 0.15);
		color: #e6b800;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.modal-title {
		flex: 1;
		font-family: 'Montserrat', sans-serif;
		font-size: 1.125rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.btn-close {
		background: transparent;
		border: none;
		color: #94a3b8;
		cursor: pointer;
		padding: 0.5rem;
		border-radius: 0.375rem;
		transition: all 0.2s ease;
	}

	.btn-close:hover:not(:disabled) {
		background: rgba(100, 116, 139, 0.2);
		color: #f1f5f9;
	}

	.modal-form {
		padding: 1.5rem;
	}

	.error-banner {
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #f87171;
		padding: 0.75rem 1rem;
		border-radius: 0.5rem;
		margin-bottom: 1.25rem;
		font-size: 0.875rem;
	}

	.form-group {
		margin-bottom: 1.25rem;
	}

	.form-label {
		display: block;
		font-family: inherit;
		font-size: 0.875rem;
		font-weight: 600;
		color: #e2e8f0;
		margin-bottom: 0.5rem;
	}

	.required {
		color: #f87171;
	}

	.label-hint {
		font-weight: 400;
		color: #64748b;
		font-size: 0.8125rem;
	}

	.form-input,
	.form-select {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 0.5rem;
		color: #f1f5f9;
		font-family: inherit;
		font-size: 0.9375rem;
		transition:
			border-color 0.2s ease,
			box-shadow 0.2s ease;
	}

	.form-input:focus,
	.form-select:focus {
		outline: none;
		border-color: #e6b800;
		box-shadow: 0 0 0 3px rgba(230, 184, 0, 0.15);
	}

	.form-input::placeholder {
		color: #64748b;
	}

	.form-input:disabled,
	.form-select:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.password-input-wrapper {
		position: relative;
		display: flex;
	}

	.password-input-wrapper .form-input {
		padding-right: 3rem;
	}

	.btn-toggle-password {
		position: absolute;
		right: 0.75rem;
		top: 50%;
		transform: translateY(-50%);
		background: none;
		border: none;
		color: #64748b;
		cursor: pointer;
		padding: 0.25rem;
	}

	.btn-toggle-password:hover {
		color: #cbd5e1;
	}

	.btn-generate {
		margin-top: 0.5rem;
		background: transparent;
		border: 1px dashed rgba(230, 184, 0, 0.4);
		color: #e6b800;
		padding: 0.5rem 1rem;
		border-radius: 0.5rem;
		font-size: 0.8125rem;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.btn-generate:hover:not(:disabled) {
		background: rgba(230, 184, 0, 0.1);
		border-color: #e6b800;
	}

	.checkbox-group {
		margin-top: 1rem;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.625rem;
		cursor: pointer;
	}

	.checkbox-label input[type='checkbox'] {
		width: 18px;
		height: 18px;
		accent-color: #e6b800;
	}

	.checkbox-text {
		font-size: 0.9375rem;
		color: #cbd5e1;
	}

	.modal-actions {
		display: flex;
		gap: 0.75rem;
		padding-top: 1rem;
		border-top: 1px solid rgba(148, 163, 184, 0.15);
		margin-top: 1.5rem;
	}

	.btn-cancel,
	.btn-submit {
		flex: 1;
		padding: 0.75rem 1.5rem;
		border-radius: 0.5rem;
		font-family: inherit;
		font-size: 0.9375rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.btn-cancel {
		background: rgba(100, 116, 139, 0.2);
		border: 1px solid rgba(100, 116, 139, 0.3);
		color: #cbd5e1;
	}

	.btn-cancel:hover:not(:disabled) {
		background: rgba(100, 116, 139, 0.3);
		color: #f1f5f9;
	}

	.btn-submit {
		background: linear-gradient(135deg, #e6b800, #b38f00);
		border: none;
		color: #0d1117;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		box-shadow: 0 4px 14px rgba(230, 184, 0, 0.3);
	}

	.btn-submit:hover:not(:disabled) {
		background: linear-gradient(135deg, #f5c800, #c9a000);
		transform: translateY(-2px);
		box-shadow: 0 8px 24px rgba(230, 184, 0, 0.4);
	}

	.btn-cancel:disabled,
	.btn-submit:disabled {
		opacity: 0.6;
		cursor: not-allowed;
		transform: none;
	}

	.spinner {
		display: inline-block;
		width: 16px;
		height: 16px;
		border: 2px solid transparent;
		border-top-color: currentColor;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Temporary Password Display */
	.password-created {
		padding: 2rem 1.5rem;
		text-align: center;
	}

	.password-icon {
		width: 64px;
		height: 64px;
		border-radius: 50%;
		background: rgba(16, 185, 129, 0.15);
		color: #34d399;
		display: flex;
		align-items: center;
		justify-content: center;
		margin: 0 auto 1rem;
	}

	.password-created h3 {
		font-family: 'Montserrat', sans-serif;
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 0.5rem;
	}

	.password-created p {
		font-size: 0.9375rem;
		color: #94a3b8;
		margin: 0 0 1rem;
	}

	.password-display {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 0.5rem;
		padding: 1rem;
		margin-bottom: 0.75rem;
	}

	.password-display code {
		font-family: 'JetBrains Mono', monospace;
		font-size: 1rem;
		color: #e6b800;
		word-break: break-all;
	}

	.btn-copy {
		background: transparent;
		border: none;
		color: #64748b;
		cursor: pointer;
		padding: 0.25rem;
		border-radius: 0.25rem;
		transition: all 0.2s ease;
	}

	.btn-copy:hover {
		background: rgba(100, 116, 139, 0.2);
		color: #e6b800;
	}

	.password-warning {
		font-size: 0.8125rem;
		color: #fbbf24;
		margin-bottom: 1.5rem;
	}

	.btn-primary {
		background: linear-gradient(135deg, #e6b800, #b38f00);
		border: none;
		color: #0d1117;
		padding: 0.75rem 2rem;
		border-radius: 0.5rem;
		font-size: 0.9375rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
		box-shadow: 0 4px 14px rgba(230, 184, 0, 0.3);
	}

	.btn-primary:hover {
		background: linear-gradient(135deg, #f5c800, #c9a000);
		transform: translateY(-2px);
		box-shadow: 0 8px 24px rgba(230, 184, 0, 0.4);
	}
</style>
