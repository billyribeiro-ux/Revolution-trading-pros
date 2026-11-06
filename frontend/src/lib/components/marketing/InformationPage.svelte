<script lang="ts">
	import type { Snippet } from 'svelte';
	import { page } from '$app/state';
	let {
		title,
		category = 'Legal',
		description,
		metadata,
		actions,
		children
	}: {
		title: string;
		category?: 'Legal' | 'Support';
		description?: string;
		metadata?: string;
		actions?: Snippet;
		children: Snippet;
	} = $props();
	const headingId = $props.id();
	const links = [
		{ href: '/privacy', label: 'Privacy Policy' },
		{ href: '/terms', label: 'Terms of Service' },
		{ href: '/cookie-policy', label: 'Cookie Policy' },
		{ href: '/disclaimer', label: 'Risk Disclaimer' },
		{ href: '/refund', label: 'Refund Policy' },
		{ href: '/contact', label: 'Contact Support' }
	];
</script>

<article class="information-page" aria-labelledby={headingId}>
	<div class="information-inner">
		<header class="information-header">
			<p class="information-category">{category}</p>
			<h1 id={headingId}>{title}</h1>
			{#if description}<p class="information-description">{description}</p>{/if}
			{#if metadata}<p class="information-metadata">{metadata}</p>{/if}
			{#if actions}<div class="information-actions">{@render actions()}</div>{/if}
		</header>
		<nav class="information-nav" aria-label="Legal and support">
			{#each links as link (link.href)}
				<a href={link.href} aria-current={page.url.pathname === link.href ? 'page' : undefined}
					>{link.label}</a
				>
			{/each}
		</nav>
		<div class="information-prose">{@render children()}</div>
	</div>
</article>

<style>
	.information-page {
		width: 100%;
		min-width: 0;
		max-width: var(--rtp-content-max);
		margin-inline: auto;
		padding: clamp(2.5rem, 6vw, 5rem) clamp(1rem, 3vw, 2rem) clamp(4rem, 8vw, 7rem);
		font-family: var(--rtp-font-sans);
		color: var(--rtp-text);
		overflow-wrap: anywhere;
	}
	.information-inner {
		width: 100%;
		min-width: 0;
		max-width: var(--rtp-content-reading);
		margin-inline: auto;
	}
	.information-header {
		margin-block-end: 2rem;
	}
	.information-category {
		color: var(--rtp-blue-bright);
		font-size: 0.8125rem;
		font-weight: 700;
		letter-spacing: 0.12em;
		text-transform: uppercase;
		margin: 0 0 0.75rem;
	}
	h1 {
		font-family: var(--rtp-font-display);
		font-size: var(--rtp-information-title);
		font-weight: 800;
		line-height: 1.1;
		letter-spacing: -0.035em;
		margin: 0;
		text-wrap: balance;
	}
	.information-description {
		max-width: 65ch;
		color: var(--rtp-text-soft);
		font-size: 1.125rem;
		line-height: 1.7;
		margin: 1.25rem 0 0;
	}
	.information-metadata {
		color: var(--rtp-text-muted);
		font-size: 0.875rem;
		line-height: 1.6;
		margin: 1rem 0 0;
	}
	.information-actions {
		margin-block-start: 1.5rem;
	}
	.information-nav {
		display: flex;
		flex-wrap: wrap;
		gap: 0.375rem;
		border-block: 1px solid var(--rtp-border);
		padding-block: 1rem;
		margin-block-end: 2.5rem;
	}
	.information-nav a {
		display: inline-flex;
		align-items: center;
		min-height: 2.75rem;
		min-width: 0;
		max-width: 100%;
		padding: 0.625rem 0.875rem;
		border: 1px solid transparent;
		border-radius: var(--rtp-radius-sm);
		font-size: 0.875rem;
		line-height: 1.5;
		color: var(--rtp-text-muted);
		text-decoration: none;
	}
	.information-nav a:hover {
		color: var(--rtp-text);
		background: var(--rtp-surface);
	}
	.information-nav a[aria-current='page'] {
		color: var(--rtp-blue-bright);
		background: var(--rtp-surface);
		border-color: var(--rtp-border-strong);
		font-weight: 600;
	}
	.information-nav a:focus-visible,
	.information-prose :global(a:focus-visible),
	.information-prose :global(button:focus-visible),
	.information-actions :global(select:focus-visible) {
		outline: 2px solid var(--rtp-blue-bright);
		outline-offset: 4px;
	}
	.information-prose {
		min-width: 0;
		color: var(--rtp-text-muted);
		font-size: var(--rtp-information-body);
		line-height: 1.8;
	}
	.information-prose :global(h2) {
		font-family: var(--rtp-font-display);
		color: var(--rtp-text);
		font-size: 1.5rem;
		font-weight: 700;
		line-height: 1.35;
		letter-spacing: -0.02em;
		margin: 2.5rem 0 1rem;
	}
	.information-prose :global(h3) {
		color: var(--rtp-text);
		font-size: 1.125rem;
		font-weight: 700;
		line-height: 1.45;
		margin: 1.75rem 0 0.75rem;
	}
	.information-prose :global(p) {
		margin: 0 0 1.25rem;
	}
	.information-prose :global(ul),
	.information-prose :global(ol) {
		padding-inline-start: 1.5rem;
		margin: 0 0 1.5rem;
	}
	.information-prose :global(ul) {
		list-style: disc;
	}
	.information-prose :global(ol) {
		list-style: decimal;
	}
	.information-prose :global(li + li) {
		margin-block-start: 0.5rem;
	}
	.information-prose :global(strong) {
		color: var(--rtp-text);
		font-weight: 600;
	}
	.information-prose :global(a) {
		color: var(--rtp-blue-bright);
		text-decoration: underline;
		text-underline-offset: 0.2em;
	}
	.information-prose :global(a:hover) {
		color: #93c5fd;
	}
	.information-prose :global(.document-callout) {
		padding: 1.25rem;
		margin-block-end: 2rem;
		background: var(--rtp-surface);
		border: 1px solid var(--rtp-border-strong);
		border-inline-start: 3px solid var(--rtp-blue-bright);
		border-radius: var(--rtp-radius-md);
	}
	.information-prose :global(.document-callout--warning) {
		border-inline-start-color: var(--rtp-amber);
	}
	.information-prose :global(.document-callout h2) {
		margin-block-start: 0;
	}
	.information-prose :global(.document-callout p:last-child) {
		margin-block-end: 0;
	}
</style>
