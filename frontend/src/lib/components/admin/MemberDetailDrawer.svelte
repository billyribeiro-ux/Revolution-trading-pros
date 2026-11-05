<script lang="ts">
	import type { Attachment } from 'svelte/attachments';
	import type { MemberProfileRecord } from '#lib/api/member-profile.js';
	import MemberProfileView from './MemberProfileView.svelte';
	let {
		isOpen,
		memberId,
		onClose,
		onEdit,
		onEmail,
		onRefresh
	}: {
		isOpen: boolean;
		memberId: number | null;
		onClose: () => void;
		onEdit?: (member: MemberProfileRecord) => void;
		onEmail?: (member: MemberProfileRecord) => void;
		onRefresh?: () => void;
	} = $props();
	const openDrawer: Attachment<HTMLDialogElement> = (element) => {
		const previousOverflow = document.body.style.overflow;
		document.body.style.overflow = 'hidden';
		element.showModal();
		return () => {
			element.close();
			document.body.style.overflow = previousOverflow;
		};
	};
</script>

{#if isOpen && memberId}
	<dialog
		class="drawer"
		aria-label="Member details"
		{@attach openDrawer}
		oncancel={(event) => {
			event.preventDefault();
			onClose();
		}}
		onclick={(event) => {
			if (event.target !== event.currentTarget) return;
			const rect = event.currentTarget.getBoundingClientRect();
			if (
				event.clientX < rect.left ||
				event.clientX > rect.right ||
				event.clientY < rect.top ||
				event.clientY > rect.bottom
			)
				onClose();
		}}
	>
		<header>
			<h2>Member Details</h2>
			<a href={`/admin/members/${memberId}`}>Open full profile</a><button
				type="button"
				aria-label="Close member details"
				onclick={onClose}>×</button
			>
		</header>
		<div class="drawer-content">
			<MemberProfileView {memberId} compact {onEdit} {onEmail} {onRefresh} />
		</div>
	</dialog>
{/if}

<style>
	.drawer {
		width: min(1050px, 95vw);
		height: 100dvh;
		max-width: none;
		max-height: none;
		margin: 0 0 0 auto;
		padding: 0;
		border: 0;
		background: var(--bg-base, #0d1117);
		color: var(--text-primary, #f0f6fc);
		overflow: auto;
		box-shadow: -10px 0 50px #0005;
	}
	.drawer::backdrop {
		background: #0009;
	}
	header {
		position: sticky;
		top: 0;
		background: var(--bg-elevated, #161b22);
		border-bottom: 1px solid var(--border-default, #30363d);
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem 1.5rem;
		z-index: 10;
	}
	h2 {
		font-size: 1.15rem;
		margin: 0;
		flex: 1;
	}
	a {
		color: var(--primary-500, #e6b800);
	}
	button {
		background: var(--bg-surface, #1c2128);
		color: inherit;
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
		width: 36px;
		height: 36px;
		font-size: 1.6rem;
		cursor: pointer;
	}
	.drawer-content {
		padding: 1.5rem;
	}
	@media (max-width: 600px) {
		.drawer {
			width: 100vw;
		}
		.drawer-content {
			padding: 1rem;
		}
		header {
			padding: 1rem;
		}
	}
</style>
