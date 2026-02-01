<script lang="ts">
	/**
	 * MemberFormModal - Create/Edit Member Modal
	 * Revolution Trading Pros - Apple ICT 7 Principal Engineer Grade
	 *
	 * Enterprise-grade modal with comprehensive member management options.
	 * Features sturdier visual design and professional interaction patterns.
	 */
	import {
		membersApi,
		type Member,
		type CreateMemberRequest,
		type UpdateMemberRequest
	} from '$lib/api/members';
	import {
		IconX,
		IconUserPlus,
		IconUserEdit,
		IconEye,
		IconEyeOff,
		IconCopy,
		IconCheck
	} from '$lib/icons';

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

	// ============================================
	// FORM STATE - Core Fields
	// ============================================
	let name = $state('');
	let email = $state('');
	let password = $state('');
	let role = $state('user');
	let sendWelcomeEmail = $state(true);

	// ============================================
	// FORM STATE - Extended Profile
	// ============================================
	let phone = $state('');
	let company = $state('');
	let jobTitle = $state('');
	let bio = $state('');
	let timezone = $state('America/New_York');

	// ============================================
	// FORM STATE - Membership & Access
	// ============================================
	let subscriptionTier = $state('free');
	let accountStatus = $state('active');
	let expirationDate = $state('');
	let tags = $state<string[]>([]);
	let tagInput = $state('');

	// ============================================
	// FORM STATE - Preferences & Security
	// ============================================
	let enableTwoFactor = $state(false);
	let emailNotifications = $state(true);
	let smsNotifications = $state(false);
	let marketingEmails = $state(false);

	// ============================================
	// FORM STATE - Notes (Admin Only)
	// ============================================
	let adminNotes = $state('');

	// UI state
	let isLoading = $state(false);
	let error = $state('');
	let showPassword = $state(false);
	let temporaryPassword = $state<string | null>(null);
	let activeSection = $state<'basic' | 'profile' | 'membership' | 'preferences' | 'notes'>('basic');
	let copiedPassword = $state(false);

	const roles = [
		{ value: 'user', label: 'Member', description: 'Standard member access' },
		{ value: 'moderator', label: 'Moderator', description: 'Can moderate content' },
		{ value: 'admin', label: 'Admin', description: 'Full admin access' },
		{ value: 'super_admin', label: 'Super Admin', description: 'Unrestricted access' }
	];

	const subscriptionTiers = [
		{ value: 'free', label: 'Free', color: 'var(--color-text-muted)' },
		{ value: 'basic', label: 'Basic', color: 'var(--color-info)' },
		{ value: 'premium', label: 'Premium', color: 'var(--admin-accent-primary)' },
		{ value: 'elite', label: 'Elite', color: 'var(--color-watching)' },
		{ value: 'lifetime', label: 'Lifetime', color: 'var(--color-profit)' }
	];

	const accountStatuses = [
		{ value: 'active', label: 'Active', color: 'var(--color-profit)' },
		{ value: 'pending', label: 'Pending Verification', color: 'var(--color-watching)' },
		{ value: 'suspended', label: 'Suspended', color: 'var(--color-loss)' },
		{ value: 'banned', label: 'Banned', color: 'var(--color-loss-dark)' }
	];

	const timezones = [
		{ value: 'America/New_York', label: 'Eastern Time (ET)' },
		{ value: 'America/Chicago', label: 'Central Time (CT)' },
		{ value: 'America/Denver', label: 'Mountain Time (MT)' },
		{ value: 'America/Los_Angeles', label: 'Pacific Time (PT)' },
		{ value: 'America/Anchorage', label: 'Alaska Time (AKT)' },
		{ value: 'Pacific/Honolulu', label: 'Hawaii Time (HT)' },
		{ value: 'Europe/London', label: 'London (GMT/BST)' },
		{ value: 'Europe/Paris', label: 'Central European (CET)' },
		{ value: 'Asia/Tokyo', label: 'Japan (JST)' },
		{ value: 'Asia/Shanghai', label: 'China (CST)' },
		{ value: 'Asia/Dubai', label: 'Gulf (GST)' },
		{ value: 'Australia/Sydney', label: 'Sydney (AEST)' }
	];

	const sections = [
		{ id: 'basic', label: 'Basic Info', icon: '👤' },
		{ id: 'profile', label: 'Profile', icon: '📋' },
		{ id: 'membership', label: 'Membership', icon: '⭐' },
		{ id: 'preferences', label: 'Preferences', icon: '⚙️' },
		{ id: 'notes', label: 'Admin Notes', icon: '📝' }
	] as const;

	// Initialize form when member changes
	$effect(() => {
		if (isOpen && mode === 'edit' && member) {
			name = member.name || '';
			email = member.email || '';
			role = 'user';
			password = '';
			// Extended fields would be populated from member data
			phone = '';
			company = '';
			jobTitle = '';
			bio = '';
			timezone = 'America/New_York';
			subscriptionTier = 'free';
			accountStatus = 'active';
			expirationDate = '';
			tags = [];
			enableTwoFactor = false;
			emailNotifications = true;
			smsNotifications = false;
			marketingEmails = false;
			adminNotes = '';
		} else if (isOpen && mode === 'create') {
			name = '';
			email = '';
			password = '';
			role = 'user';
			sendWelcomeEmail = true;
			phone = '';
			company = '';
			jobTitle = '';
			bio = '';
			timezone = 'America/New_York';
			subscriptionTier = 'free';
			accountStatus = 'active';
			expirationDate = '';
			tags = [];
			tagInput = '';
			enableTwoFactor = false;
			emailNotifications = true;
			smsNotifications = false;
			marketingEmails = false;
			adminNotes = '';
		}
		temporaryPassword = null;
		error = '';
		activeSection = 'basic';
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
			activeSection = 'basic';
			return false;
		}
		if (!email.trim()) {
			error = 'Email is required';
			activeSection = 'basic';
			return false;
		}
		if (!email.includes('@') || !email.includes('.')) {
			error = 'Please enter a valid email address';
			activeSection = 'basic';
			return false;
		}
		if (mode === 'create' && password && password.length < 8) {
			error = 'Password must be at least 8 characters';
			activeSection = 'basic';
			return false;
		}
		if (phone && !/^[\d\s\-+()]+$/.test(phone)) {
			error = 'Please enter a valid phone number';
			activeSection = 'profile';
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
		const chars = 'ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnpqrstuvwxyz23456789!@#$%&*';
		password = Array.from(
			{ length: 20 },
			() => chars[Math.floor(Math.random() * chars.length)]
		).join('');
		showPassword = true;
	}

	async function copyPassword(text: string) {
		await navigator.clipboard.writeText(text);
		copiedPassword = true;
		setTimeout(() => (copiedPassword = false), 2000);
	}

	function closeAndClear() {
		temporaryPassword = null;
		onClose();
	}

	function addTag() {
		const newTag = tagInput.trim().toLowerCase();
		if (newTag && !tags.includes(newTag) && tags.length < 10) {
			tags = [...tags, newTag];
			tagInput = '';
		}
	}

	function removeTag(tag: string) {
		tags = tags.filter((t) => t !== tag);
	}

	function handleTagKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') {
			e.preventDefault();
			addTag();
		}
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
		<!-- Sturdy Layered Background -->
		<div class="modal-frame">
			<div class="modal-container">
				<!-- Header -->
				<div class="modal-header">
					<div class="header-content">
						<div class="header-icon">
							{#if mode === 'create'}
								<IconUserPlus size={24} />
							{:else}
								<IconUserEdit size={24} />
							{/if}
						</div>
						<div class="header-text">
							<h2 id="modal-title" class="modal-title">
								{mode === 'create' ? 'Create New Member' : 'Edit Member'}
							</h2>
							<p class="modal-subtitle">
								{mode === 'create'
									? 'Add a new member to the platform'
									: `Editing ${member?.name || 'member'}`}
							</p>
						</div>
					</div>
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
						<div class="success-animation">
							<div class="success-ring"></div>
							<div class="success-icon">
								<IconCheck size={40} />
							</div>
						</div>
						<h3>Member Created Successfully</h3>
						<p>
							A temporary password has been generated. Please share this securely with the member:
						</p>
						<div class="password-display">
							<code>{temporaryPassword}</code>
							<button
								type="button"
								class="btn-copy"
								onclick={() => copyPassword(temporaryPassword!)}
								title="Copy password"
							>
								{#if copiedPassword}
									<IconCheck size={18} />
								{:else}
									<IconCopy size={18} />
								{/if}
							</button>
						</div>
						<p class="password-warning">
							⚠️ This password will not be shown again. Make sure to save it securely.
						</p>
						<button type="button" class="btn-primary" onclick={closeAndClear}>Done</button>
					</div>
				{:else}
					<!-- Section Navigation -->
					<div class="section-nav">
						{#each sections as section (section.id)}
							<button
								type="button"
								class="section-tab"
								class:active={activeSection === section.id}
								onclick={() => (activeSection = section.id)}
							>
								<span class="tab-icon">{section.icon}</span>
								<span class="tab-label">{section.label}</span>
							</button>
						{/each}
					</div>

					<!-- Form -->
					<form
						class="modal-form"
						onsubmit={(e) => {
							e.preventDefault();
							handleSubmit();
						}}
					>
						{#if error}
							<div class="error-banner">
								<span class="error-icon">⚠️</span>
								{error}
							</div>
						{/if}

						<!-- Basic Info Section -->
						{#if activeSection === 'basic'}
							<div class="form-section" data-section="basic">
								<div class="section-header">
									<h3>Basic Information</h3>
									<p>Core member details and credentials</p>
								</div>

								<div class="form-grid">
									<div class="form-group full-width">
										<label for="name" class="form-label">
											Full Name <span class="required">*</span>
										</label>
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

									<div class="form-group full-width">
										<label for="email" class="form-label">
											Email Address <span class="required">*</span>
										</label>
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
										<span class="field-hint">
											{roles.find((r) => r.value === role)?.description}
										</span>
									</div>

									<div class="form-group">
										<label for="account-status" class="form-label">Account Status</label>
										<select
											id="account-status"
											class="form-select"
											bind:value={accountStatus}
											disabled={isLoading}
										>
											{#each accountStatuses as status (status.value)}
												<option value={status.value}>{status.label}</option>
											{/each}
										</select>
										<div class="status-indicator">
											<span
												class="status-dot"
												style="background: {accountStatuses.find((s) => s.value === accountStatus)
													?.color}"
											></span>
											<span class="status-text"
												>{accountStatuses.find((s) => s.value === accountStatus)?.label}</span
											>
										</div>
									</div>

									<div class="form-group full-width">
										<label for="password" class="form-label">
											{mode === 'create' ? 'Password' : 'New Password'}
											<span class="label-hint">
												{mode === 'create'
													? '(Leave blank to auto-generate)'
													: '(Leave blank to keep current)'}
											</span>
										</label>
										<div class="password-input-wrapper">
											<input
												id="password"
												type={showPassword ? 'text' : 'password'}
												class="form-input"
												placeholder={mode === 'create'
													? 'Auto-generated if empty'
													: 'Enter new password'}
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
											🔐 Generate Strong Password
										</button>
									</div>

									{#if mode === 'create'}
										<div class="form-group full-width checkbox-group">
											<label class="checkbox-label">
												<input
													type="checkbox"
													bind:checked={sendWelcomeEmail}
													disabled={isLoading}
												/>
												<span class="checkbox-box">
													{#if sendWelcomeEmail}<IconCheck size={14} />{/if}
												</span>
												<span class="checkbox-text">Send welcome email with login details</span>
											</label>
										</div>
									{/if}
								</div>
							</div>
						{/if}

						<!-- Profile Section -->
						{#if activeSection === 'profile'}
							<div class="form-section" data-section="profile">
								<div class="section-header">
									<h3>Profile Details</h3>
									<p>Extended member information</p>
								</div>

								<div class="form-grid">
									<div class="form-group">
										<label for="phone" class="form-label">Phone Number</label>
										<input
											id="phone"
											type="tel"
											class="form-input"
											placeholder="+1 (555) 123-4567"
											bind:value={phone}
											disabled={isLoading}
											autocomplete="tel"
										/>
									</div>

									<div class="form-group">
										<label for="timezone" class="form-label">Timezone</label>
										<select
											id="timezone"
											class="form-select"
											bind:value={timezone}
											disabled={isLoading}
										>
											{#each timezones as tz (tz.value)}
												<option value={tz.value}>{tz.label}</option>
											{/each}
										</select>
									</div>

									<div class="form-group">
										<label for="company" class="form-label">Company / Organization</label>
										<input
											id="company"
											type="text"
											class="form-input"
											placeholder="Enter company name"
											bind:value={company}
											disabled={isLoading}
											autocomplete="organization"
										/>
									</div>

									<div class="form-group">
										<label for="job-title" class="form-label">Job Title</label>
										<input
											id="job-title"
											type="text"
											class="form-input"
											placeholder="e.g. Day Trader, Portfolio Manager"
											bind:value={jobTitle}
											disabled={isLoading}
											autocomplete="organization-title"
										/>
									</div>

									<div class="form-group full-width">
										<label for="bio" class="form-label">
											Bio / About
											<span class="char-count">{bio.length}/500</span>
										</label>
										<textarea
											id="bio"
											class="form-textarea"
											placeholder="Brief description about the member..."
											bind:value={bio}
											disabled={isLoading}
											maxlength="500"
											rows="4"
										></textarea>
									</div>
								</div>
							</div>
						{/if}

						<!-- Membership Section -->
						{#if activeSection === 'membership'}
							<div class="form-section" data-section="membership">
								<div class="section-header">
									<h3>Membership & Access</h3>
									<p>Subscription tier and access control</p>
								</div>

								<div class="form-grid">
									<div class="form-group full-width">
										<span id="tier-label" class="form-label">Subscription Tier</span>
										<div class="tier-selector" role="group" aria-labelledby="tier-label">
											{#each subscriptionTiers as tier (tier.value)}
												<button
													type="button"
													class="tier-option"
													class:selected={subscriptionTier === tier.value}
													style="--tier-color: {tier.color}"
													onclick={() => (subscriptionTier = tier.value)}
													disabled={isLoading}
												>
													<span class="tier-badge" style="background: {tier.color}"></span>
													<span class="tier-name">{tier.label}</span>
												</button>
											{/each}
										</div>
									</div>

									<div class="form-group">
										<label for="expiration-date" class="form-label">
											Subscription Expiration
											<span class="label-hint">(Optional)</span>
										</label>
										<input
											id="expiration-date"
											type="date"
											class="form-input"
											bind:value={expirationDate}
											disabled={isLoading}
										/>
									</div>

									<div class="form-group">
										<span id="quick-exp-label" class="form-label">Quick Expiration</span>
										<div class="quick-actions" role="group" aria-labelledby="quick-exp-label">
											<button
												type="button"
												class="quick-btn"
												onclick={() => {
													const d = new Date();
													d.setMonth(d.getMonth() + 1);
													expirationDate = d.toISOString().split('T')[0];
												}}
											>
												+1 Month
											</button>
											<button
												type="button"
												class="quick-btn"
												onclick={() => {
													const d = new Date();
													d.setMonth(d.getMonth() + 3);
													expirationDate = d.toISOString().split('T')[0];
												}}
											>
												+3 Months
											</button>
											<button
												type="button"
												class="quick-btn"
												onclick={() => {
													const d = new Date();
													d.setFullYear(d.getFullYear() + 1);
													expirationDate = d.toISOString().split('T')[0];
												}}
											>
												+1 Year
											</button>
										</div>
									</div>

									<div class="form-group full-width">
										<label for="tags" class="form-label">
											Tags
											<span class="label-hint">(Max 10 tags)</span>
										</label>
										<div class="tags-input-wrapper">
											<div class="tags-display">
												{#each tags as tag (tag)}
													<span class="tag">
														{tag}
														<button
															type="button"
															class="tag-remove"
															onclick={() => removeTag(tag)}
															aria-label="Remove tag"
														>
															×
														</button>
													</span>
												{/each}
												{#if tags.length < 10}
													<input
														id="tags"
														type="text"
														class="tag-input"
														placeholder={tags.length === 0 ? 'Add tags...' : ''}
														bind:value={tagInput}
														onkeydown={handleTagKeydown}
														onblur={addTag}
														disabled={isLoading}
													/>
												{/if}
											</div>
										</div>
										<span class="field-hint">Press Enter to add a tag</span>
									</div>
								</div>
							</div>
						{/if}

						<!-- Preferences Section -->
						{#if activeSection === 'preferences'}
							<div class="form-section" data-section="preferences">
								<div class="section-header">
									<h3>Preferences & Security</h3>
									<p>Notification and security settings</p>
								</div>

								<div class="preferences-grid">
									<div class="preference-card">
										<div class="preference-header">
											<span class="preference-icon">🔐</span>
											<div class="preference-info">
												<h4>Two-Factor Authentication</h4>
												<p>Require 2FA for account login</p>
											</div>
										</div>
										<label class="toggle-switch">
											<input type="checkbox" bind:checked={enableTwoFactor} disabled={isLoading} />
											<span class="toggle-slider"></span>
										</label>
									</div>

									<div class="preference-card">
										<div class="preference-header">
											<span class="preference-icon">📧</span>
											<div class="preference-info">
												<h4>Email Notifications</h4>
												<p>Receive alerts and updates via email</p>
											</div>
										</div>
										<label class="toggle-switch">
											<input
												type="checkbox"
												bind:checked={emailNotifications}
												disabled={isLoading}
											/>
											<span class="toggle-slider"></span>
										</label>
									</div>

									<div class="preference-card">
										<div class="preference-header">
											<span class="preference-icon">📱</span>
											<div class="preference-info">
												<h4>SMS Notifications</h4>
												<p>Receive alerts via text message</p>
											</div>
										</div>
										<label class="toggle-switch">
											<input type="checkbox" bind:checked={smsNotifications} disabled={isLoading} />
											<span class="toggle-slider"></span>
										</label>
									</div>

									<div class="preference-card">
										<div class="preference-header">
											<span class="preference-icon">📰</span>
											<div class="preference-info">
												<h4>Marketing Emails</h4>
												<p>Receive promotional content</p>
											</div>
										</div>
										<label class="toggle-switch">
											<input type="checkbox" bind:checked={marketingEmails} disabled={isLoading} />
											<span class="toggle-slider"></span>
										</label>
									</div>
								</div>
							</div>
						{/if}

						<!-- Notes Section -->
						{#if activeSection === 'notes'}
							<div class="form-section" data-section="notes">
								<div class="section-header">
									<h3>Admin Notes</h3>
									<p>Internal notes (not visible to member)</p>
								</div>

								<div class="form-group full-width">
									<label for="admin-notes" class="form-label">
										Private Notes
										<span class="char-count">{adminNotes.length}/2000</span>
									</label>
									<textarea
										id="admin-notes"
										class="form-textarea large"
										placeholder="Add internal notes about this member. These notes are only visible to administrators..."
										bind:value={adminNotes}
										disabled={isLoading}
										maxlength="2000"
										rows="8"
									></textarea>
									<span class="field-hint">
										These notes are for internal use only and will not be shared with the member.
									</span>
								</div>
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
	</div>
{/if}

<style>
	/* ============================================
	   MODAL BACKDROP - Sturdy Layered Design
	   ============================================ */
	.modal-backdrop {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.85);
		backdrop-filter: blur(12px) saturate(180%);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: var(--z-modal);
		padding: var(--space-6);
		animation: backdropFadeIn var(--duration-normal) var(--ease-out);
	}

	@keyframes backdropFadeIn {
		from {
			opacity: 0;
			backdrop-filter: blur(0px) saturate(100%);
		}
		to {
			opacity: 1;
			backdrop-filter: blur(12px) saturate(180%);
		}
	}

	/* Sturdy Frame - Outer border effect */
	.modal-frame {
		position: relative;
		border-radius: var(--radius-xl);
		padding: 3px;
		background: linear-gradient(
			145deg,
			rgba(230, 184, 0, 0.4) 0%,
			rgba(230, 184, 0, 0.1) 30%,
			rgba(100, 116, 139, 0.2) 70%,
			rgba(230, 184, 0, 0.3) 100%
		);
		box-shadow:
			0 0 0 1px rgba(0, 0, 0, 0.5),
			0 30px 80px -20px rgba(0, 0, 0, 0.8),
			0 0 60px -15px rgba(230, 184, 0, 0.2),
			inset 0 1px 0 rgba(255, 255, 255, 0.05);
		animation: frameSlideIn 0.4s cubic-bezier(0.16, 1, 0.3, 1);
		max-height: 90vh;
	}

	@keyframes frameSlideIn {
		from {
			opacity: 0;
			transform: translateY(30px) scale(0.92);
		}
		to {
			opacity: 1;
			transform: translateY(0) scale(1);
		}
	}

	.modal-container {
		background: linear-gradient(
			165deg,
			var(--admin-surface-elevated) 0%,
			var(--admin-surface-primary) 40%,
			var(--admin-bg) 100%
		);
		border-radius: var(--radius-lg);
		max-width: 640px;
		width: 100%;
		max-height: calc(90vh - 6px);
		overflow: hidden;
		display: flex;
		flex-direction: column;
		position: relative;
	}

	.modal-container::before {
		content: '';
		position: absolute;
		inset: 0;
		background:
			radial-gradient(ellipse at 20% 0%, rgba(230, 184, 0, 0.08) 0%, transparent 50%),
			radial-gradient(ellipse at 80% 100%, rgba(100, 116, 139, 0.06) 0%, transparent 50%);
		pointer-events: none;
		border-radius: 1.125rem;
	}

	/* ============================================
	   HEADER
	   ============================================ */
	.modal-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-5) var(--space-6);
		border-bottom: 1px solid var(--admin-border-subtle);
		background: linear-gradient(
			180deg,
			var(--admin-surface-elevated) 0%,
			var(--admin-surface-primary) 100%
		);
		position: relative;
		z-index: 1;
	}

	.header-content {
		display: flex;
		align-items: center;
		gap: var(--space-4);
	}

	.header-icon {
		width: 48px;
		height: 48px;
		border-radius: var(--radius-lg);
		background: var(--admin-accent-bg);
		border: 1px solid var(--admin-accent-primary);
		color: var(--admin-accent-primary);
		display: flex;
		align-items: center;
		justify-content: center;
		box-shadow: var(--shadow-md);
	}

	.header-text {
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
	}

	.modal-title {
		font-family: var(--font-display);
		font-size: var(--text-xl);
		font-weight: var(--font-bold);
		color: var(--admin-text-primary);
		margin: 0;
		letter-spacing: var(--tracking-tight);
	}

	.modal-subtitle {
		font-size: var(--text-sm);
		color: var(--admin-text-muted);
		margin: 0;
	}

	.btn-close {
		background: var(--admin-surface-hover);
		border: 1px solid var(--admin-border-subtle);
		color: var(--admin-text-muted);
		cursor: pointer;
		padding: var(--space-3);
		border-radius: var(--radius-md);
		transition: var(--transition-all);
		min-width: 44px;
		min-height: 44px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.btn-close:hover:not(:disabled) {
		background: var(--color-loss-bg);
		border-color: var(--color-loss);
		color: var(--color-loss);
	}

	/* ============================================
	   SECTION NAVIGATION
	   ============================================ */
	.section-nav {
		display: flex;
		gap: var(--space-1);
		padding: var(--space-3) var(--space-4);
		background: var(--admin-surface-sunken);
		border-bottom: 1px solid var(--admin-border-subtle);
		overflow-x: auto;
		-webkit-overflow-scrolling: touch;
		scrollbar-width: none;
	}

	.section-nav::-webkit-scrollbar {
		display: none;
	}

	.section-tab {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-3) var(--space-4);
		background: transparent;
		border: 1px solid transparent;
		border-radius: var(--radius-md);
		color: var(--admin-text-muted);
		font-size: var(--text-sm);
		font-weight: var(--font-medium);
		cursor: pointer;
		transition: var(--transition-all);
		white-space: nowrap;
		min-height: 44px;
	}

	.section-tab:hover {
		background: var(--admin-surface-hover);
		color: var(--admin-text-secondary);
	}

	.section-tab.active {
		background: var(--admin-accent-bg);
		border-color: var(--admin-accent-primary);
		color: var(--admin-accent-primary);
	}

	.tab-icon {
		font-size: 1rem;
	}

	.tab-label {
		font-family: inherit;
	}

	/* ============================================
	   FORM STYLES
	   ============================================ */
	.modal-form {
		padding: var(--space-6);
		overflow-y: auto;
		flex: 1;
	}

	.form-section {
		animation: sectionFadeIn 0.25s ease;
	}

	@keyframes sectionFadeIn {
		from {
			opacity: 0;
			transform: translateX(10px);
		}
		to {
			opacity: 1;
			transform: translateX(0);
		}
	}

	.section-header {
		margin-bottom: var(--space-6);
	}

	.section-header h3 {
		font-family: var(--font-display);
		font-size: var(--text-base);
		font-weight: var(--font-semibold);
		color: var(--admin-text-primary);
		margin: 0 0 var(--space-1);
	}

	.section-header p {
		font-size: var(--text-sm);
		color: var(--admin-text-muted);
		margin: 0;
	}

	.error-banner {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		background: var(--color-loss-bg);
		border: 1px solid var(--color-loss);
		color: var(--color-loss);
		padding: var(--space-4);
		border-radius: var(--radius-md);
		margin-bottom: var(--space-5);
		font-size: var(--text-sm);
		font-weight: var(--font-medium);
	}

	.error-icon {
		font-size: 1rem;
	}

	.form-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: var(--space-5);
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}

	.form-group.full-width {
		grid-column: 1 / -1;
	}

	.form-label {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		font-family: var(--font-sans);
		font-size: var(--text-sm);
		font-weight: var(--font-semibold);
		color: var(--admin-text-secondary);
		text-transform: uppercase;
		letter-spacing: var(--tracking-wide);
	}

	.required {
		color: var(--color-loss);
	}

	.label-hint {
		font-weight: var(--font-normal);
		color: var(--admin-text-muted);
		text-transform: none;
		letter-spacing: normal;
	}

	.char-count {
		margin-left: auto;
		font-weight: var(--font-normal);
		color: var(--admin-text-muted);
		text-transform: none;
		letter-spacing: normal;
		font-size: var(--text-xs);
	}

	.field-hint {
		font-size: var(--text-xs);
		color: var(--admin-text-muted);
		margin-top: var(--space-1);
	}

	.form-input,
	.form-select,
	.form-textarea {
		width: 100%;
		padding: var(--space-3) var(--space-4);
		background: var(--admin-surface-sunken);
		border: 1px solid var(--admin-border-subtle);
		border-radius: var(--radius-md);
		color: var(--admin-text-primary);
		font-family: var(--font-sans);
		font-size: var(--text-base);
		transition: var(--transition-colors);
		min-height: 44px;
	}

	.form-textarea {
		resize: vertical;
		min-height: 100px;
	}

	.form-textarea.large {
		min-height: 180px;
	}

	.form-input:focus,
	.form-select:focus,
	.form-textarea:focus {
		outline: none;
		border-color: var(--admin-accent-primary);
		box-shadow: var(--admin-focus-ring);
		background: var(--admin-surface-primary);
	}

	.form-input::placeholder,
	.form-textarea::placeholder {
		color: var(--admin-text-muted);
	}

	.form-input:disabled,
	.form-select:disabled,
	.form-textarea:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* Status Indicator */
	.status-indicator {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-top: 0.375rem;
	}

	.status-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		box-shadow: 0 0 6px currentColor;
	}

	.status-text {
		font-size: 0.75rem;
		color: #94a3b8;
	}

	/* Password Input */
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
		transition: color 0.2s ease;
	}

	.btn-toggle-password:hover {
		color: #e6b800;
	}

	.btn-generate {
		margin-top: 0.5rem;
		background: linear-gradient(135deg, rgba(230, 184, 0, 0.1) 0%, rgba(230, 184, 0, 0.05) 100%);
		border: 1px dashed rgba(230, 184, 0, 0.4);
		color: #e6b800;
		padding: 0.625rem 1rem;
		border-radius: 0.5rem;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.btn-generate:hover:not(:disabled) {
		background: rgba(230, 184, 0, 0.15);
		border-style: solid;
		border-color: rgba(230, 184, 0, 0.6);
	}

	/* Checkbox Styles */
	.checkbox-group {
		margin-top: 0.5rem;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		cursor: pointer;
	}

	.checkbox-label input[type='checkbox'] {
		position: absolute;
		opacity: 0;
		width: 0;
		height: 0;
	}

	.checkbox-box {
		width: 22px;
		height: 22px;
		border: 2px solid rgba(148, 163, 184, 0.3);
		border-radius: 0.375rem;
		background: rgba(15, 23, 42, 0.6);
		display: flex;
		align-items: center;
		justify-content: center;
		color: #0d1117;
		transition: all 0.2s ease;
	}

	.checkbox-label input:checked + .checkbox-box {
		background: #e6b800;
		border-color: #e6b800;
	}

	.checkbox-text {
		font-size: 0.9375rem;
		color: #cbd5e1;
	}

	/* Tier Selector */
	.tier-selector {
		display: flex;
		flex-wrap: wrap;
		gap: 0.625rem;
	}

	.tier-option {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.15);
		border-radius: 0.5rem;
		color: #94a3b8;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.tier-option:hover {
		border-color: var(--tier-color);
		color: #f1f5f9;
	}

	.tier-option.selected {
		background: linear-gradient(135deg, rgba(var(--tier-color), 0.15) 0%, transparent 100%);
		border-color: var(--tier-color);
		color: #f1f5f9;
		box-shadow: 0 0 12px rgba(var(--tier-color), 0.2);
	}

	.tier-badge {
		width: 12px;
		height: 12px;
		border-radius: 50%;
	}

	.tier-name {
		font-family: inherit;
	}

	/* Quick Actions */
	.quick-actions {
		display: flex;
		gap: 0.5rem;
	}

	.quick-btn {
		padding: 0.5rem 0.75rem;
		background: rgba(100, 116, 139, 0.15);
		border: 1px solid rgba(100, 116, 139, 0.2);
		border-radius: 0.375rem;
		color: #94a3b8;
		font-size: 0.75rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.quick-btn:hover {
		background: rgba(230, 184, 0, 0.15);
		border-color: rgba(230, 184, 0, 0.3);
		color: #e6b800;
	}

	/* Tags Input */
	.tags-input-wrapper {
		background: rgba(15, 23, 42, 0.7);
		border: 1px solid rgba(148, 163, 184, 0.15);
		border-radius: 0.625rem;
		padding: 0.5rem;
		transition: all 0.2s ease;
	}

	.tags-input-wrapper:focus-within {
		border-color: rgba(230, 184, 0, 0.5);
		box-shadow: 0 0 0 3px rgba(230, 184, 0, 0.1);
	}

	.tags-display {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		align-items: center;
	}

	.tag {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.625rem;
		background: rgba(230, 184, 0, 0.15);
		border: 1px solid rgba(230, 184, 0, 0.3);
		border-radius: 0.375rem;
		color: #e6b800;
		font-size: 0.8125rem;
		font-weight: 500;
	}

	.tag-remove {
		background: none;
		border: none;
		color: rgba(230, 184, 0, 0.6);
		font-size: 1.125rem;
		line-height: 1;
		cursor: pointer;
		padding: 0;
		transition: color 0.2s ease;
	}

	.tag-remove:hover {
		color: #f87171;
	}

	.tag-input {
		flex: 1;
		min-width: 100px;
		background: transparent;
		border: none;
		color: #f1f5f9;
		font-size: 0.875rem;
		padding: 0.375rem;
		outline: none;
	}

	.tag-input::placeholder {
		color: #475569;
	}

	/* Preferences Grid */
	.preferences-grid {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.preference-card {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1rem 1.25rem;
		background: rgba(15, 23, 42, 0.5);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 0.75rem;
		transition: all 0.2s ease;
	}

	.preference-card:hover {
		background: rgba(15, 23, 42, 0.7);
		border-color: rgba(148, 163, 184, 0.2);
	}

	.preference-header {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.preference-icon {
		font-size: 1.5rem;
	}

	.preference-info h4 {
		font-size: 0.9375rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 0.125rem;
	}

	.preference-info p {
		font-size: 0.8125rem;
		color: #64748b;
		margin: 0;
	}

	/* Toggle Switch */
	.toggle-switch {
		position: relative;
		display: inline-block;
		width: 52px;
		height: 28px;
		cursor: pointer;
	}

	.toggle-switch input {
		opacity: 0;
		width: 0;
		height: 0;
	}

	.toggle-slider {
		position: absolute;
		inset: 0;
		background: rgba(100, 116, 139, 0.3);
		border-radius: 28px;
		transition: all 0.3s ease;
	}

	.toggle-slider::before {
		content: '';
		position: absolute;
		height: 22px;
		width: 22px;
		left: 3px;
		bottom: 3px;
		background: #f1f5f9;
		border-radius: 50%;
		transition: all 0.3s ease;
		box-shadow: 0 2px 6px rgba(0, 0, 0, 0.3);
	}

	.toggle-switch input:checked + .toggle-slider {
		background: linear-gradient(135deg, #e6b800, #b38f00);
	}

	.toggle-switch input:checked + .toggle-slider::before {
		transform: translateX(24px);
	}

	/* ============================================
	   MODAL ACTIONS
	   ============================================ */
	.modal-actions {
		display: flex;
		gap: 0.875rem;
		padding: 1.25rem 1.5rem;
		border-top: 1px solid rgba(148, 163, 184, 0.12);
		background: linear-gradient(180deg, rgba(15, 23, 36, 0.95) 0%, rgba(10, 15, 22, 1) 100%);
	}

	.btn-cancel,
	.btn-submit {
		flex: 1;
		padding: 0.875rem 1.5rem;
		border-radius: 0.625rem;
		font-family: inherit;
		font-size: 0.9375rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.25s ease;
	}

	.btn-cancel {
		background: rgba(100, 116, 139, 0.15);
		border: 1px solid rgba(100, 116, 139, 0.25);
		color: #cbd5e1;
	}

	.btn-cancel:hover:not(:disabled) {
		background: rgba(100, 116, 139, 0.25);
		border-color: rgba(100, 116, 139, 0.4);
		color: #f1f5f9;
	}

	.btn-submit {
		background: linear-gradient(135deg, #e6b800 0%, #b38f00 100%);
		border: none;
		color: #0d1117;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		box-shadow:
			0 4px 16px rgba(230, 184, 0, 0.25),
			inset 0 1px 0 rgba(255, 255, 255, 0.15);
	}

	.btn-submit:hover:not(:disabled) {
		background: linear-gradient(135deg, #f5c800 0%, #d4a600 100%);
		transform: translateY(-2px);
		box-shadow:
			0 8px 24px rgba(230, 184, 0, 0.35),
			inset 0 1px 0 rgba(255, 255, 255, 0.2);
	}

	.btn-cancel:disabled,
	.btn-submit:disabled {
		opacity: 0.5;
		cursor: not-allowed;
		transform: none;
	}

	.spinner {
		display: inline-block;
		width: 18px;
		height: 18px;
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

	/* ============================================
	   PASSWORD CREATED SUCCESS
	   ============================================ */
	.password-created {
		padding: 2.5rem 2rem;
		text-align: center;
	}

	.success-animation {
		position: relative;
		width: 80px;
		height: 80px;
		margin: 0 auto 1.5rem;
	}

	.success-ring {
		position: absolute;
		inset: 0;
		border: 3px solid rgba(16, 185, 129, 0.2);
		border-radius: 50%;
		animation: ringPulse 2s ease-in-out infinite;
	}

	@keyframes ringPulse {
		0%,
		100% {
			transform: scale(1);
			opacity: 1;
		}
		50% {
			transform: scale(1.1);
			opacity: 0.5;
		}
	}

	.success-icon {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, rgba(16, 185, 129, 0.2) 0%, rgba(16, 185, 129, 0.1) 100%);
		border-radius: 50%;
		color: #34d399;
		animation: iconBounce 0.5s cubic-bezier(0.68, -0.55, 0.265, 1.55);
	}

	@keyframes iconBounce {
		from {
			transform: scale(0);
		}
		to {
			transform: scale(1);
		}
	}

	.password-created h3 {
		font-family: 'Montserrat', sans-serif;
		font-size: 1.375rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.625rem;
	}

	.password-created p {
		font-size: 0.9375rem;
		color: #94a3b8;
		margin: 0 0 1.25rem;
		line-height: 1.5;
	}

	.password-display {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
		background: rgba(15, 23, 42, 0.7);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 0.625rem;
		padding: 1.125rem 1.25rem;
		margin-bottom: 1rem;
	}

	.password-display code {
		font-family: 'JetBrains Mono', 'SF Mono', monospace;
		font-size: 1.0625rem;
		color: #e6b800;
		word-break: break-all;
		letter-spacing: 0.025em;
	}

	.btn-copy {
		background: rgba(100, 116, 139, 0.2);
		border: 1px solid rgba(100, 116, 139, 0.2);
		color: #64748b;
		cursor: pointer;
		padding: 0.5rem;
		border-radius: 0.375rem;
		transition: all 0.2s ease;
	}

	.btn-copy:hover {
		background: rgba(230, 184, 0, 0.15);
		border-color: rgba(230, 184, 0, 0.3);
		color: #e6b800;
	}

	.password-warning {
		font-size: 0.875rem;
		color: #fbbf24;
		margin-bottom: 1.75rem;
		font-weight: 500;
	}

	.btn-primary {
		background: linear-gradient(135deg, #e6b800 0%, #b38f00 100%);
		border: none;
		color: #0d1117;
		padding: 0.875rem 2.5rem;
		border-radius: 0.625rem;
		font-size: 0.9375rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.25s ease;
		box-shadow:
			0 4px 16px rgba(230, 184, 0, 0.25),
			inset 0 1px 0 rgba(255, 255, 255, 0.15);
	}

	.btn-primary:hover {
		background: linear-gradient(135deg, #f5c800 0%, #d4a600 100%);
		transform: translateY(-2px);
		box-shadow:
			0 8px 24px rgba(230, 184, 0, 0.35),
			inset 0 1px 0 rgba(255, 255, 255, 0.2);
	}

	/* ============================================
	   RESPONSIVE - Tablet (< md: 768px)
	   ============================================ */
	@media (max-width: 768px) {
		.modal-backdrop {
			padding: var(--space-4);
			align-items: flex-start;
			padding-top: var(--space-8);
		}

		.modal-frame {
			max-height: calc(100vh - var(--space-12));
		}

		.modal-container {
			max-width: 100%;
		}

		/* Form grid single column on tablet */
		.form-grid {
			grid-template-columns: 1fr;
			gap: var(--space-4);
		}

		/* Tier selector wrap better */
		.tier-selector {
			flex-wrap: wrap;
		}

		.tier-option {
			flex: 1 1 calc(50% - 0.5rem);
			min-width: 120px;
		}

		/* Quick actions wrap */
		.quick-actions {
			flex-wrap: wrap;
		}

		.quick-btn {
			flex: 1;
			min-width: 80px;
			text-align: center;
		}
	}

	/* ============================================
	   RESPONSIVE - Mobile (< sm: 640px)
	   ============================================ */
	@media (max-width: 640px) {
		.modal-backdrop {
			padding: var(--space-3);
			padding-top: var(--space-4);
			align-items: flex-start;
		}

		.modal-frame {
			max-height: calc(100vh - var(--space-8));
			margin: 0;
		}

		.modal-container {
			max-width: 100%;
			max-height: calc(100vh - var(--space-8));
			border-radius: var(--radius-lg);
		}

		/* Reduced header padding on mobile */
		.modal-header {
			padding: var(--space-4);
		}

		.header-icon {
			width: 40px;
			height: 40px;
		}

		.modal-title {
			font-size: var(--text-lg);
		}

		.modal-subtitle {
			font-size: var(--text-xs);
		}

		/* Close button larger touch target */
		.btn-close {
			min-width: 48px;
			min-height: 48px;
			padding: var(--space-3);
		}

		/* Section nav compact on mobile */
		.section-nav {
			padding: var(--space-2) var(--space-3);
			gap: var(--space-1);
		}

		.section-tab {
			padding: var(--space-2) var(--space-3);
			min-height: 48px;
		}

		.tab-label {
			display: none;
		}

		.tab-icon {
			font-size: 1.25rem;
		}

		/* Form reduced padding on mobile */
		.modal-form {
			padding: var(--space-4);
		}

		.section-header {
			margin-bottom: var(--space-4);
		}

		.section-header h3 {
			font-size: var(--text-sm);
		}

		/* Form grid single column */
		.form-grid {
			grid-template-columns: 1fr;
			gap: var(--space-4);
		}

		/* Input fields full width with larger touch targets */
		.form-input,
		.form-select,
		.form-textarea {
			min-height: 48px;
			padding: var(--space-3) var(--space-4);
			font-size: 16px; /* Prevents iOS zoom on focus */
		}

		.form-label {
			font-size: var(--text-xs);
		}

		/* Tier selector stacked on mobile */
		.tier-selector {
			flex-direction: column;
			gap: var(--space-2);
		}

		.tier-option {
			width: 100%;
			justify-content: center;
			min-height: 48px;
		}

		/* Quick actions stacked */
		.quick-actions {
			flex-direction: column;
			gap: var(--space-2);
		}

		.quick-btn {
			width: 100%;
			min-height: 44px;
			justify-content: center;
		}

		/* Tags input larger touch targets */
		.tag {
			min-height: 36px;
			padding: var(--space-2) var(--space-3);
		}

		.tag-remove {
			min-width: 24px;
			min-height: 24px;
			display: flex;
			align-items: center;
			justify-content: center;
		}

		/* Preference cards stacked */
		.preference-card {
			flex-direction: column;
			align-items: flex-start;
			gap: var(--space-3);
			padding: var(--space-4);
		}

		.toggle-switch {
			align-self: flex-end;
		}

		/* Modal actions full width stacked */
		.modal-actions {
			flex-direction: column;
			padding: var(--space-4);
			gap: var(--space-3);
		}

		.btn-cancel,
		.btn-submit {
			width: 100%;
			min-height: 48px;
			padding: var(--space-4);
		}

		/* Password section */
		.btn-generate {
			width: 100%;
			min-height: 44px;
			justify-content: center;
		}

		.password-input-wrapper .form-input {
			padding-right: 3.5rem;
		}

		.btn-toggle-password {
			min-width: 44px;
			min-height: 44px;
			right: 0.25rem;
		}

		/* Checkbox larger touch target */
		.checkbox-box {
			width: 28px;
			height: 28px;
		}

		.checkbox-text {
			font-size: var(--text-sm);
		}

		/* Password created section */
		.password-created {
			padding: var(--space-5) var(--space-4);
		}

		.password-display {
			flex-direction: column;
			gap: var(--space-3);
		}

		.password-display code {
			font-size: var(--text-sm);
		}

		.btn-copy {
			width: 100%;
			min-height: 44px;
			justify-content: center;
		}
	}

	/* ============================================
	   RESPONSIVE - Extra Small Mobile (< 380px)
	   ============================================ */
	@media (max-width: 380px) {
		.modal-backdrop {
			padding: var(--space-2);
		}

		.modal-header {
			padding: var(--space-3);
		}

		.header-content {
			gap: var(--space-3);
		}

		.header-icon {
			width: 36px;
			height: 36px;
		}

		.modal-title {
			font-size: var(--text-base);
		}

		.section-nav {
			padding: var(--space-2);
		}

		.section-tab {
			padding: var(--space-2);
			min-width: 44px;
		}

		.modal-form {
			padding: var(--space-3);
		}

		.form-input,
		.form-select,
		.form-textarea {
			padding: var(--space-3);
		}

		.preference-info h4 {
			font-size: var(--text-sm);
		}

		.preference-info p {
			font-size: var(--text-xs);
		}
	}

	/* ============================================
	   TOUCH DEVICE OPTIMIZATIONS
	   ============================================ */
	@media (hover: none) and (pointer: coarse) {
		/* Larger touch targets for all interactive elements */
		.btn-close,
		.section-tab,
		.tier-option,
		.quick-btn,
		.tag-remove,
		.btn-toggle-password,
		.btn-generate,
		.btn-copy,
		.btn-cancel,
		.btn-submit {
			min-height: 44px;
			min-width: 44px;
		}

		.form-input,
		.form-select,
		.form-textarea {
			min-height: 48px;
			font-size: 16px; /* Prevents iOS zoom */
		}

		.checkbox-box {
			width: 28px;
			height: 28px;
		}

		.toggle-switch {
			width: 56px;
			height: 32px;
		}

		.toggle-slider::before {
			height: 26px;
			width: 26px;
		}

		.toggle-switch input:checked + .toggle-slider::before {
			transform: translateX(24px);
		}
	}

	/* ============================================
	   ACCESSIBILITY
	   ============================================ */
	@media (prefers-reduced-motion: reduce) {
		.modal-backdrop,
		.modal-frame,
		.form-section {
			animation: none;
		}

		.btn-close,
		.btn-cancel,
		.btn-submit,
		.section-tab,
		.form-input,
		.form-select,
		.form-textarea {
			transition: none;
		}
	}
</style>
