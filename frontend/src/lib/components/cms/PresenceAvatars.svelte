<!--
/**
 * Presence Avatars Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Shows real-time collaborator avatars and cursors
 */
-->

<script lang="ts">
	import type { CollaboratorInfo } from '$lib/collaboration/yjs-provider';

	interface Props {
		collaborators: CollaboratorInfo[];
		maxVisible?: number;
		showNames?: boolean;
		size?: 'sm' | 'md' | 'lg';
	}

	let props: Props = $props();

	let maxVisible = $derived(props.maxVisible ?? 5);
	let showNames = $derived(props.showNames ?? false);
	let size = $derived(props.size ?? 'md');

	let visibleCollaborators = $derived(props.collaborators.slice(0, maxVisible));
	let hiddenCount = $derived(Math.max(0, props.collaborators.length - maxVisible));

	let sizeClasses = $derived(
		{
			sm: 'avatar-sm',
			md: 'avatar-md',
			lg: 'avatar-lg'
		}[size]
	);

	function getInitials(name: string): string {
		return name
			.split(' ')
			.map((n) => n[0])
			.join('')
			.toUpperCase()
			.slice(0, 2);
	}

	function getContrastColor(bgColor: string): string {
		const hex = bgColor.replace('#', '');
		const r = parseInt(hex.slice(0, 2), 16);
		const g = parseInt(hex.slice(2, 4), 16);
		const b = parseInt(hex.slice(4, 6), 16);
		const luminance = (0.299 * r + 0.587 * g + 0.114 * b) / 255;
		return luminance > 0.5 ? '#000000' : '#ffffff';
	}
</script>

{#if props.collaborators.length > 0}
	<div class="presence-avatars" role="group" aria-label="Active collaborators">
		<div class="avatar-stack">
			{#each visibleCollaborators as collaborator (collaborator.clientId)}
				<div
					class="avatar {sizeClasses}"
					style="background-color: {collaborator.color}; color: {getContrastColor(
						collaborator.color
					)}"
					title="{collaborator.name} is editing"
				>
					{getInitials(collaborator.name)}
					<span class="status-dot" aria-hidden="true"></span>
				</div>
			{/each}

			{#if hiddenCount > 0}
				<div class="avatar avatar-overflow {sizeClasses}" title="{hiddenCount} more collaborators">
					+{hiddenCount}
				</div>
			{/if}
		</div>

		{#if showNames && visibleCollaborators.length > 0}
			<span class="presence-text">
				{#if visibleCollaborators.length === 1}
					{visibleCollaborators[0].name} is editing
				{:else}
					{visibleCollaborators.length} people editing
				{/if}
			</span>
		{/if}
	</div>
{/if}

<style>
	.presence-avatars {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.avatar-stack {
		display: flex;
		flex-direction: row-reverse;
	}

	.avatar {
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 50%;
		font-weight: 600;
		border: 2px solid white;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
		position: relative;
		transition:
			transform 0.15s,
			z-index 0.15s;
	}

	.avatar:not(:last-child) {
		margin-left: -8px;
	}

	.avatar:hover {
		transform: translateY(-2px);
		z-index: 10;
	}

	.avatar-sm {
		width: 28px;
		height: 28px;
		font-size: 0.625rem;
	}

	.avatar-md {
		width: 36px;
		height: 36px;
		font-size: 0.75rem;
	}

	.avatar-lg {
		width: 44px;
		height: 44px;
		font-size: 0.875rem;
	}

	.avatar-overflow {
		background: #e2e8f0;
		color: #64748b;
	}

	.status-dot {
		position: absolute;
		bottom: 0;
		right: 0;
		width: 10px;
		height: 10px;
		background: #22c55e;
		border: 2px solid white;
		border-radius: 50%;
	}

	.avatar-sm .status-dot {
		width: 8px;
		height: 8px;
	}

	.presence-text {
		font-size: 0.8125rem;
		color: #64748b;
		font-weight: 500;
		white-space: nowrap;
	}

	:global(.dark) .avatar {
		border-color: #1e293b;
	}

	:global(.dark) .avatar-overflow {
		background: #334155;
		color: #94a3b8;
	}

	:global(.dark) .status-dot {
		border-color: #1e293b;
	}

	:global(.dark) .presence-text {
		color: #94a3b8;
	}
</style>
