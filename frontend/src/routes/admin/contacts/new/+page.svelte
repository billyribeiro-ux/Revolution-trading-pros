<!--
	URL: /admin/contacts/new
-->

<script lang="ts">
	import { goto } from '$app/navigation';
	import { IconUserPlus, IconArrowLeft } from '#lib/icons/index.js';
	import { crmAPI } from '#lib/api/crm.js';

	let formData = $state({
		first_name: '',
		last_name: '',
		email: '',
		phone: '',
		job_title: ''
	});
	let isSubmitting = $state(false);
	let errorMessage = $state<string | null>(null);

	async function handleSubmit(event: Event) {
		event.preventDefault();
		isSubmitting = true;
		errorMessage = null;

		try {
			await crmAPI.createContact({
				first_name: formData.first_name,
				last_name: formData.last_name,
				email: formData.email,
				...(formData.phone && { phone: formData.phone }),
				...(formData.job_title && { job_title: formData.job_title })
			});
			goto('/admin/contacts');
		} catch (error) {
			console.error('Failed to create contact:', error);
			errorMessage = error instanceof Error ? error.message : 'Failed to create contact';
			isSubmitting = false;
		}
	}
</script>

<svelte:head>
	<title>New Contact - Admin</title>
</svelte:head>

<div class="new-contact-page">
	<div class="page-header">
		<a href="/admin/contacts" class="back-link">
			<IconArrowLeft size={18} />
			Back to Contacts
		</a>
		<h1>Add New Contact</h1>
	</div>

	<form class="contact-form" onsubmit={handleSubmit}>
		{#if errorMessage}
			<div class="error-banner">
				{errorMessage}
			</div>
		{/if}

		<div class="form-row">
			<div class="form-group">
				<label for="first_name">First Name *</label>
				<input
					type="text"
					id="first_name"
					name="first_name"
					autocomplete="given-name"
					bind:value={formData.first_name}
					required
				/>
			</div>
			<div class="form-group">
				<label for="last_name">Last Name *</label>
				<input
					type="text"
					id="last_name"
					name="last_name"
					autocomplete="family-name"
					bind:value={formData.last_name}
					required
				/>
			</div>
		</div>

		<div class="form-group">
			<label for="email">Email Address *</label>
			<input
				type="email"
				id="email"
				name="email"
				autocomplete="email"
				bind:value={formData.email}
				required
			/>
		</div>

		<div class="form-group">
			<label for="phone">Phone Number</label>
			<input type="tel" id="phone" name="phone" autocomplete="tel" bind:value={formData.phone} />
		</div>

		<div class="form-group">
			<label for="job_title">Job Title</label>
			<input autocomplete="off" type="text" id="job_title" name="job_title" bind:value={formData.job_title} />
		</div>

		<div class="form-actions">
			<a href="/admin/contacts" class="btn-secondary">Cancel</a>
			<button type="submit" class="btn-primary" disabled={isSubmitting}>
				<IconUserPlus size={18} />
				{isSubmitting ? 'Creating...' : 'Create Contact'}
			</button>
		</div>
	</form>
</div>

<style>
	.new-contact-page {
		max-width: 600px;
	}

	.page-header {
		margin-bottom: 2rem;
	}

	.back-link {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		color: var(--text-tertiary, #6e7681);
		text-decoration: none;
		font-size: 0.9rem;
		margin-bottom: 1rem;
	}

	.back-link:hover {
		color: var(--primary-500);
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: var(--text-primary, #f0f6fc);
		margin: 0;
	}

	.contact-form {
		background: var(--bg-base, #0d1117);
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 16px;
		padding: 2rem;
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}

	.form-group {
		margin-bottom: 1.5rem;
	}

	.form-group label {
		display: block;
		font-size: 0.9rem;
		font-weight: 500;
		color: var(--text-secondary, #8b949e);
		margin-bottom: 0.5rem;
	}

	.form-group input {
		width: 100%;
		padding: 0.75rem 1rem;
		background: var(--bg-base, #0d1117);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 10px;
		color: var(--text-primary, #f0f6fc);
		font-size: 0.95rem;
	}

	.form-group input:focus {
		outline: none;
		border-color: rgba(230, 184, 0, 0.5);
	}

	.form-actions {
		display: flex;
		justify-content: flex-end;
		gap: 1rem;
		margin-top: 2rem;
		padding-top: 1.5rem;
		border-top: 1px solid rgba(230, 184, 0, 0.1);
	}

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: var(--primary-500);
		color: var(--bg-base);
		border: none;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}


	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-secondary {
		padding: 0.75rem 1.5rem;
		background: rgba(230, 184, 0, 0.1);
		color: var(--text-secondary, #8b949e);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 10px;
		font-weight: 600;
		text-decoration: none;
		transition: all 0.2s;
	}

	.btn-secondary:hover {
		background: rgba(230, 184, 0, 0.2);
		color: var(--text-primary, #f0f6fc);
	}

	.error-banner {
		background: rgba(248, 81, 73, 0.1);
		border: 1px solid rgba(248, 81, 73, 0.3);
		color: var(--error-emphasis, #f85149);
		padding: 0.75rem 1rem;
		border-radius: 10px;
		margin-bottom: 1.5rem;
		font-size: 0.9rem;
	}
</style>
