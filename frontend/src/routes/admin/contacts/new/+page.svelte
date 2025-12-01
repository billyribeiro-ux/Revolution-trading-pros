<script lang="ts">
	import { goto } from '$app/navigation';
	import { IconUserPlus, IconArrowLeft } from '@tabler/icons-svelte';

	let formData = {
		name: '',
		email: '',
		phone: '',
		company: '',
		notes: ''
	};
	let isSubmitting = false;

	async function handleSubmit() {
		isSubmitting = true;
		// TODO: Implement API call
		setTimeout(() => {
			goto('/admin/contacts');
		}, 500);
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
		<div class="form-group">
			<label for="name">Full Name *</label>
			<input type="text" id="name" bind:value={formData.name} required />
		</div>

		<div class="form-group">
			<label for="email">Email Address *</label>
			<input type="email" id="email" bind:value={formData.email} required />
		</div>

		<div class="form-group">
			<label for="phone">Phone Number</label>
			<input type="tel" id="phone" bind:value={formData.phone} />
		</div>

		<div class="form-group">
			<label for="company">Company</label>
			<input type="text" id="company" bind:value={formData.company} />
		</div>

		<div class="form-group">
			<label for="notes">Notes</label>
			<textarea id="notes" bind:value={formData.notes} rows="4"></textarea>
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
		color: #64748b;
		text-decoration: none;
		font-size: 0.9rem;
		margin-bottom: 1rem;
	}

	.back-link:hover { color: #818cf8; }

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0;
	}

	.contact-form {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 16px;
		padding: 2rem;
	}

	.form-group {
		margin-bottom: 1.5rem;
	}

	.form-group label {
		display: block;
		font-size: 0.9rem;
		font-weight: 500;
		color: #94a3b8;
		margin-bottom: 0.5rem;
	}

	.form-group input,
	.form-group textarea {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.8);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 10px;
		color: #e2e8f0;
		font-size: 0.95rem;
	}

	.form-group input:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: rgba(99, 102, 241, 0.5);
	}

	.form-group textarea {
		resize: vertical;
		min-height: 100px;
	}

	.form-actions {
		display: flex;
		justify-content: flex-end;
		gap: 1rem;
		margin-top: 2rem;
		padding-top: 1.5rem;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
	}

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		color: white;
		border: none;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(99, 102, 241, 0.4);
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-secondary {
		padding: 0.75rem 1.5rem;
		background: rgba(99, 102, 241, 0.1);
		color: #94a3b8;
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 10px;
		font-weight: 600;
		text-decoration: none;
		transition: all 0.2s;
	}

	.btn-secondary:hover {
		background: rgba(99, 102, 241, 0.2);
		color: #e2e8f0;
	}
</style>
