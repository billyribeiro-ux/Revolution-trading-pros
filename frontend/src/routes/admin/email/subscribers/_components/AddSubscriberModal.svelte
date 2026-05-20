<script lang="ts">
	import { IconX, IconUserPlus } from '$lib/icons';

	export interface NewSubscriberForm {
		email: string;
		first_name: string;
		last_name: string;
		tags: string;
	}

	interface Props {
		newSubscriber: NewSubscriberForm;
		onclose: () => void;
		onsubmit: () => void;
	}

	let { newSubscriber = $bindable(), onclose, onsubmit }: Props = $props();
</script>

<div
	class="modal-overlay"
	role="button"
	tabindex="0"
	aria-label="Close modal"
	onclick={onclose}
	onkeydown={(e: KeyboardEvent) => {
		if (e.key === 'Escape' || e.key === 'Enter' || e.key === ' ') onclose();
	}}
>
	<div
		class="modal-content"
		role="dialog"
		aria-modal="true"
		tabindex="-1"
		onclick={(e: MouseEvent) => e.stopPropagation()}
		onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
	>
		<div class="modal-header">
			<h2>Add Subscriber</h2>
			<button class="close-btn" onclick={onclose}>
				<IconX size={20} />
			</button>
		</div>

		<div class="modal-body">
			<div class="form-group">
				<label for="email">Email Address *</label>
				<input
					id="email"
					name="email"
					autocomplete="email"
					type="email"
					bind:value={newSubscriber.email}
					placeholder="subscriber@example.com"
				/>
			</div>

			<div class="form-row">
				<div class="form-group">
					<label for="first_name">First Name</label>
					<input
						id="first_name"
						name="first_name"
						type="text"
						bind:value={newSubscriber.first_name}
					/>
				</div>
				<div class="form-group">
					<label for="last_name">Last Name</label>
					<input
						id="last_name"
						name="last_name"
						type="text"
						bind:value={newSubscriber.last_name}
					/>
				</div>
			</div>

			<div class="form-group">
				<label for="tags">Tags (comma-separated)</label>
				<input
					id="tags"
					name="tags"
					type="text"
					bind:value={newSubscriber.tags}
					placeholder="newsletter, vip"
				/>
			</div>
		</div>

		<div class="modal-footer">
			<button class="btn-secondary" onclick={onclose}>Cancel</button>
			<button class="btn-primary" onclick={onsubmit}>
				<IconUserPlus size={18} />
				Add Subscriber
			</button>
		</div>
	</div>
</div>

<style>
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.8);
		backdrop-filter: blur(4px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 50;
		padding: 2rem;
	}

	.modal-content {
		background: #1e293b;
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 20px;
		width: 100%;
		max-width: 500px;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.modal-header h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.close-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(148, 163, 184, 0.1);
		border: none;
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
	}

	.modal-body {
		padding: 1.5rem;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1.5rem;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.form-group label {
		font-size: 0.8125rem;
		font-weight: 500;
		color: #94a3b8;
	}

	.form-group input {
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 10px;
		color: #f1f5f9;
		font-size: 0.9375rem;
	}

	.form-group input:focus {
		outline: none;
		border-color: rgba(230, 184, 0, 0.5);
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}

	.btn-primary,
	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
		font-size: 0.875rem;
	}

	.btn-primary {
		background: linear-gradient(135deg, var(--primary-500) 0%, var(--primary-600) 100%);
		color: var(--bg-base);
	}

	.btn-secondary {
		background: rgba(148, 163, 184, 0.1);
		color: #94a3b8;
		border: 1px solid rgba(148, 163, 184, 0.2);
	}
</style>
