<!--
/**
 * Newsletter Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Email signup form with customizable styling
 */
-->

<script lang="ts">
	import { IconMail, IconLoader2, IconCheck } from '$lib/icons';
	import { getBlockStateManager, type BlockId } from '$lib/stores/blockState.svelte';
	import type { Block, BlockContent } from '../types';

	interface Props {
		block: Block;
		blockId: BlockId;
		isSelected: boolean;
		isEditing: boolean;
		onUpdate: (updates: Partial<Block>) => void;
		onError?: (error: Error) => void;
	}

	const props: Props = $props();
	const stateManager = getBlockStateManager();

	let email = $state('');
	let status = $state<'idle' | 'loading' | 'success' | 'error'>('idle');
	let errorMessage = $state('');

	let title = $derived(props.block.content.newsletterTitle || 'Subscribe to our newsletter');
	let description = $derived(props.block.content.newsletterDescription || 'Get the latest trading insights delivered to your inbox.');
	let buttonText = $derived(props.block.content.newsletterButtonText || 'Subscribe');
	let placeholder = $derived(props.block.content.newsletterPlaceholder || 'Enter your email');

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	async function handleSubmit(e: Event): Promise<void> {
		e.preventDefault();
		if (!email.trim() || !email.includes('@')) {
			status = 'error';
			errorMessage = 'Please enter a valid email address';
			return;
		}

		status = 'loading';

		try {
			// Simulated API call - replace with actual newsletter service
			await new Promise(resolve => setTimeout(resolve, 1500));
			status = 'success';
			email = '';
		} catch (error) {
			status = 'error';
			errorMessage = 'Failed to subscribe. Please try again.';
		}
	}

	function handlePaste(e: ClipboardEvent): void {
		e.preventDefault();
		document.execCommand('insertText', false, e.clipboardData?.getData('text/plain') || '');
	}
</script>

<div class="newsletter-block" role="region" aria-label="Newsletter signup">
	<div class="newsletter-icon">
		<IconMail size={24} aria-hidden="true" />
	</div>

	{#if props.isEditing}
		<h3
			contenteditable="true"
			class="newsletter-title"
			oninput={(e) => updateContent({ newsletterTitle: (e.target as HTMLElement).textContent || '' })}
			onpaste={handlePaste}
		>{title}</h3>
		<p
			contenteditable="true"
			class="newsletter-description"
			oninput={(e) => updateContent({ newsletterDescription: (e.target as HTMLElement).textContent || '' })}
			onpaste={handlePaste}
		>{description}</p>
	{:else}
		<h3 class="newsletter-title">{title}</h3>
		<p class="newsletter-description">{description}</p>
	{/if}

	{#if status === 'success'}
		<div class="newsletter-success">
			<IconCheck size={24} aria-hidden="true" />
			<span>Thanks for subscribing!</span>
		</div>
	{:else}
		<form class="newsletter-form" onsubmit={handleSubmit}>
			<div class="form-group">
				<input
					type="email"
					bind:value={email}
					placeholder={placeholder}
					disabled={status === 'loading' || props.isEditing}
					aria-label="Email address"
				/>
				<button type="submit" disabled={status === 'loading' || props.isEditing}>
					{#if status === 'loading'}
						<IconLoader2 size={18} class="spinning" aria-hidden="true" />
					{:else}
						{buttonText}
					{/if}
				</button>
			</div>
			{#if status === 'error'}
				<p class="form-error" role="alert">{errorMessage}</p>
			{/if}
		</form>
	{/if}

	{#if props.isEditing && props.isSelected}
		<div class="newsletter-settings">
			<label>
				<span>Button Text:</span>
				<input type="text" value={buttonText} oninput={(e) => updateContent({ newsletterButtonText: (e.target as HTMLInputElement).value })} />
			</label>
			<label>
				<span>Placeholder:</span>
				<input type="text" value={placeholder} oninput={(e) => updateContent({ newsletterPlaceholder: (e.target as HTMLInputElement).value })} />
			</label>
		</div>
	{/if}
</div>

<style>
	.newsletter-block {
		padding: 2rem;
		background: linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%);
		border-radius: 16px;
		text-align: center;
		color: white;
	}

	.newsletter-icon {
		width: 56px;
		height: 56px;
		margin: 0 auto 1rem;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(255,255,255,0.2);
		border-radius: 14px;
	}

	.newsletter-title {
		margin: 0 0 0.5rem;
		font-size: 1.5rem;
		font-weight: 700;
		outline: none;
	}

	.newsletter-description {
		margin: 0 0 1.5rem;
		font-size: 1rem;
		opacity: 0.9;
		outline: none;
	}

	.newsletter-form { max-width: 400px; margin: 0 auto; }

	.form-group {
		display: flex;
		gap: 0.5rem;
		background: white;
		padding: 0.375rem;
		border-radius: 12px;
	}

	.form-group input {
		flex: 1;
		padding: 0.75rem 1rem;
		border: none;
		background: transparent;
		font-size: 1rem;
		color: #1e293b;
	}

	.form-group input::placeholder { color: #94a3b8; }
	.form-group input:focus { outline: none; }

	.form-group button {
		padding: 0.75rem 1.5rem;
		background: #1d4ed8;
		border: none;
		border-radius: 8px;
		color: white;
		font-size: 1rem;
		font-weight: 600;
		cursor: pointer;
		transition: background 0.15s;
		display: flex;
		align-items: center;
		justify-content: center;
		min-width: 120px;
	}

	.form-group button:hover:not(:disabled) { background: #1e40af; }
	.form-group button:disabled { opacity: 0.7; cursor: not-allowed; }
	.form-group button :global(.spinning) { animation: spin 1s linear infinite; }
	@keyframes spin { to { transform: rotate(360deg); } }

	.form-error {
		margin: 0.75rem 0 0;
		font-size: 0.875rem;
		color: #fecaca;
	}

	.newsletter-success {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
		padding: 1rem;
		background: rgba(255,255,255,0.2);
		border-radius: 12px;
		font-weight: 500;
	}

	.newsletter-settings {
		display: flex;
		gap: 1rem;
		flex-wrap: wrap;
		margin-top: 1.5rem;
		padding-top: 1.5rem;
		border-top: 1px solid rgba(255,255,255,0.2);
	}

	.newsletter-settings label {
		flex: 1;
		min-width: 150px;
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
		text-align: left;
	}

	.newsletter-settings span {
		font-size: 0.75rem;
		font-weight: 600;
		opacity: 0.9;
	}

	.newsletter-settings input {
		padding: 0.5rem 0.75rem;
		border: 1px solid rgba(255,255,255,0.3);
		border-radius: 6px;
		background: rgba(255,255,255,0.1);
		color: white;
		font-size: 0.875rem;
	}

	@media (max-width: 480px) {
		.form-group { flex-direction: column; }
		.form-group button { width: 100%; }
	}

	:global(.dark) .newsletter-block {
		background: linear-gradient(135deg, #1e3a8a 0%, #1e40af 100%);
	}
</style>
