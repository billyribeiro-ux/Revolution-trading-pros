<script lang="ts">
	/**
	 * Keyboard Shortcuts Help Modal - Apple ICT9+ Design
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * Displays all available keyboard shortcuts in a beautiful modal.
	 *
	 * @version 1.0.0
	 */

	import { fade, scale } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';
	import { keyboard } from '$lib/stores/keyboard';
	import IconKeyboard from '@tabler/icons-svelte/icons/keyboard';
	import IconX from '@tabler/icons-svelte/icons/x';

	interface Props {
		isOpen?: boolean;
		onclose?: () => void;
	}

	let { isOpen = $bindable(false), onclose }: Props = $props();

	let groupedShortcuts = $derived($keyboard.shortcuts
		.filter(s => s.enabled)
		.reduce((acc, shortcut) => {
			if (!acc[shortcut.category]) {
				acc[shortcut.category] = [];
			}
			acc[shortcut.category].push(shortcut);
			return acc;
		}, {} as Record<string, typeof $keyboard.shortcuts>));

	function close() {
		isOpen = false;
		keyboard.closeHelp();
		onclose?.();
	}

	function handleBackdrop(e: MouseEvent) {
		if (e.target === e.currentTarget) {
			close();
		}
	}

	// Sync with store
	$effect(() => {
		if ($keyboard.isHelpOpen !== isOpen) {
			isOpen = $keyboard.isHelpOpen;
		}
	});
</script>

{#if isOpen}
	<div
		class="shortcuts-overlay"
		transition:fade={{ duration: 150 }}
		onclick={handleBackdrop}
		onkeydown={(e) => e.key === 'Escape' && close()}
		role="dialog"
		aria-modal="true"
		tabindex="-1"
	>
		<div
			class="shortcuts-modal"
			transition:scale={{ duration: 200, start: 0.95, easing: quintOut }}
		>
			<!-- Header -->
			<div class="modal-header">
				<div class="header-title">
					<IconKeyboard size={24} />
					<h2>Keyboard Shortcuts</h2>
				</div>
				<button class="close-btn" onclick={close}>
					<IconX size={20} />
				</button>
			</div>

			<!-- Content -->
			<div class="modal-content">
				{#each Object.entries(groupedShortcuts) as [category, shortcuts], index}
					<div class="shortcut-group">
						<h3 class="group-title">{category}</h3>
						<div class="shortcuts-list">
							{#each shortcuts as shortcut}
								<div class="shortcut-item">
									<span class="shortcut-description">{shortcut.description}</span>
									<div class="shortcut-keys">
										{#each shortcut.keys as key, keyIndex}
											{#if keyIndex > 0}
												<span class="key-separator">+</span>
											{/if}
											<kbd>{keyboard.formatKeys([key])}</kbd>
										{/each}
									</div>
								</div>
							{/each}
						</div>
					</div>
				{/each}
			</div>

			<!-- Footer -->
			<div class="modal-footer">
				<span class="footer-hint">Press <kbd>?</kbd> to toggle this help</span>
			</div>
		</div>
	</div>
{/if}

<style>
	.shortcuts-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.7);
		backdrop-filter: blur(8px);
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 2rem;
		z-index: 9999;
	}

	.shortcuts-modal {
		width: 100%;
		max-width: 700px;
		max-height: 80vh;
		background: linear-gradient(135deg, rgba(30, 41, 59, 0.98), rgba(15, 23, 42, 0.98));
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 20px;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
		overflow: hidden;
		display: flex;
		flex-direction: column;
	}

	.modal-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1.25rem 1.5rem;
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
	}

	.header-title {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		color: #f1f5f9;
	}

	.header-title h2 {
		font-size: 1.25rem;
		font-weight: 600;
		margin: 0;
	}

	.close-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(99, 102, 241, 0.1);
		border: none;
		border-radius: 10px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.close-btn:hover {
		background: rgba(99, 102, 241, 0.2);
		color: #f1f5f9;
	}

	.modal-content {
		flex: 1;
		overflow-y: auto;
		padding: 1.5rem;
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1.5rem;
	}

	.modal-content::-webkit-scrollbar {
		width: 6px;
	}

	.modal-content::-webkit-scrollbar-track {
		background: transparent;
	}

	.modal-content::-webkit-scrollbar-thumb {
		background: rgba(99, 102, 241, 0.3);
		border-radius: 3px;
	}

	.shortcut-group {
		background: rgba(99, 102, 241, 0.05);
		border-radius: 12px;
		padding: 1rem;
	}

	.group-title {
		font-size: 0.6875rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: #818cf8;
		margin: 0 0 0.75rem 0;
	}

	.shortcuts-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.shortcut-item {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 0.75rem;
		padding: 0.5rem 0;
	}

	.shortcut-description {
		font-size: 0.875rem;
		color: #cbd5e1;
	}

	.shortcut-keys {
		display: flex;
		align-items: center;
		gap: 0.25rem;
	}

	.shortcut-keys kbd {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-width: 24px;
		padding: 0.25rem 0.5rem;
		background: rgba(99, 102, 241, 0.15);
		border: 1px solid rgba(99, 102, 241, 0.3);
		border-radius: 6px;
		color: #a5b4fc;
		font-size: 0.75rem;
		font-family: inherit;
		font-weight: 500;
	}

	.key-separator {
		color: #64748b;
		font-size: 0.75rem;
	}

	.modal-footer {
		padding: 1rem 1.5rem;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
		background: rgba(15, 23, 42, 0.5);
	}

	.footer-hint {
		font-size: 0.8125rem;
		color: #64748b;
	}

	.footer-hint kbd {
		padding: 0.125rem 0.375rem;
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 4px;
		color: #94a3b8;
		font-size: 0.75rem;
		font-family: inherit;
	}

	@media (max-width: 640px) {
		.shortcuts-overlay {
			padding: 1rem;
		}

		.modal-content {
			grid-template-columns: 1fr;
		}
	}
</style>
