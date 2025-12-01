<script lang="ts">
	/**
	 * Table of Contents Component - Svelte 5
	 *
	 * Updated: December 2025 - Migrated to Svelte 5 runes ($props, $state, $derived, $effect)
	 */
	import { browser } from '$app/environment';

	// Svelte 5: Props using $props() rune
	interface Props {
		contentBlocks?: any[];
		title?: string;
		minHeadings?: number;
		maxDepth?: number;
		showNumbers?: boolean;
		collapsible?: boolean;
		defaultExpanded?: boolean;
		sticky?: boolean;
		showProgress?: boolean;
		smoothScroll?: boolean;
		highlightActive?: boolean;
		position?: 'inline' | 'sidebar' | 'floating';
	}

	let {
		contentBlocks = [],
		title = 'In This Article',
		minHeadings = 2,
		maxDepth = 4,
		showNumbers = true,
		collapsible = true,
		defaultExpanded = true,
		sticky = true,
		showProgress = true,
		smoothScroll = true,
		highlightActive = true,
		position = 'inline'
	}: Props = $props();

	interface TocItem {
		id: string;
		text: string;
		level: number;
		children: TocItem[];
		number?: string;
	}

	// Svelte 5: Reactive state using $state() rune
	let tocItems: TocItem[] = $state([]);
	let flatItems: TocItem[] = $state([]);
	let isExpanded = $state(defaultExpanded);
	let activeId = $state('');
	let readingProgress = $state(0);
	let isFloatingMinimized = $state(false);
	let contentElement: HTMLElement | null = $state(null);
	let observer: IntersectionObserver | null = null;
	let scrollListener: (() => void) | null = null;

	// Extract text from potentially HTML content
	function stripHtml(html: string): string {
		if (!browser) return html;
		const tmp = document.createElement('div');
		tmp.innerHTML = html;
		return tmp.textContent || tmp.innerText || '';
	}

	// Generate slug from text
	function generateSlug(text: string, index: number): string {
		const slug = stripHtml(text)
			.toLowerCase()
			.replace(/[^\w\s-]/g, '')
			.replace(/\s+/g, '-')
			.replace(/-+/g, '-')
			.trim();
		return `toc-${slug}-${index}`;
	}

	// Extract headings from content blocks
	function extractHeadings(): TocItem[] {
		const headings: { id: string; text: string; level: number }[] = [];
		let index = 0;

		contentBlocks.forEach((block) => {
			if (block.type === 'heading' && block.data?.level >= 2 && block.data?.level <= maxDepth) {
				const text = block.data?.text || '';
				const id = generateSlug(text, index);
				headings.push({
					id,
					text: stripHtml(text),
					level: block.data.level
				});
				index++;
			}
		});

		return buildHierarchy(headings);
	}

	// Build nested hierarchy from flat headings
	function buildHierarchy(
		headings: { id: string; text: string; level: number }[]
	): TocItem[] {
		const result: TocItem[] = [];
		const stack: { item: TocItem; level: number }[] = [];
		let counters: number[] = [0, 0, 0, 0, 0, 0];

		headings.forEach((heading) => {
			const level = heading.level;

			// Update counters
			counters[level - 1]++;
			for (let i = level; i < counters.length; i++) {
				counters[i] = 0;
			}

			// Generate number string
			const numberParts: number[] = [];
			for (let i = 1; i < level; i++) {
				if (counters[i] > 0) {
					numberParts.push(counters[i]);
				}
			}
			numberParts.push(counters[level - 1]);

			const item: TocItem = {
				id: heading.id,
				text: heading.text,
				level: heading.level,
				children: [],
				number: numberParts.join('.')
			};

			// Add to flat list for easy access
			flatItems.push(item);

			// Find parent
			while (stack.length > 0 && stack[stack.length - 1].level >= level) {
				stack.pop();
			}

			if (stack.length === 0) {
				result.push(item);
			} else {
				stack[stack.length - 1].item.children.push(item);
			}

			stack.push({ item, level });
		});

		return result;
	}

	// Scroll to heading
	function scrollToHeading(id: string) {
		if (!browser) return;

		const element = document.getElementById(id);
		if (element) {
			const offset = sticky ? 100 : 20;
			const elementPosition = element.getBoundingClientRect().top + window.scrollY;

			if (smoothScroll) {
				window.scrollTo({
					top: elementPosition - offset,
					behavior: 'smooth'
				});
			} else {
				window.scrollTo(0, elementPosition - offset);
			}

			// Update URL hash without jumping - use replaceState to prevent history pollution
			// Google Enterprise Pattern: Hash navigation shouldn't add history entries
			history.replaceState(null, '', `#${id}`);
			activeId = id;
		}
	}

	// Set up intersection observer for active section highlighting
	function setupObserver() {
		if (!browser || !highlightActive) return;

		observer = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting) {
						activeId = entry.target.id;
					}
				});
			},
			{
				rootMargin: '-20% 0% -60% 0%',
				threshold: 0
			}
		);

		// Observe all heading elements
		flatItems.forEach((item) => {
			const element = document.getElementById(item.id);
			if (element) {
				observer?.observe(element);
			}
		});
	}

	// Set up scroll listener for reading progress
	function setupScrollListener() {
		if (!browser || !showProgress) return;

		scrollListener = () => {
			const scrollTop = window.scrollY;
			const docHeight = document.documentElement.scrollHeight - window.innerHeight;
			readingProgress = Math.min(100, Math.max(0, (scrollTop / docHeight) * 100));
		};

		window.addEventListener('scroll', scrollListener, { passive: true });
	}

	// Inject IDs into DOM headings
	function injectHeadingIds() {
		if (!browser) return;

		// Find the content container
		contentElement = document.querySelector('.content');
		if (!contentElement) return;

		const headings = contentElement.querySelectorAll('h2, h3, h4, h5, h6');
		let itemIndex = 0;

		headings.forEach((heading) => {
			const level = parseInt(heading.tagName[1]);
			if (level >= 2 && level <= maxDepth && itemIndex < flatItems.length) {
				heading.id = flatItems[itemIndex].id;
				itemIndex++;
			}
		});
	}

	// Toggle expanded state
	function toggleExpanded() {
		if (collapsible) {
			isExpanded = !isExpanded;
		}
	}

	// Toggle floating minimized state
	function toggleMinimized() {
		isFloatingMinimized = !isFloatingMinimized;
	}

	// Svelte 5: Derived value for shouldShow
	let shouldShow = $derived(flatItems.length >= minHeadings);

	// Svelte 5: Effect for extracting headings when contentBlocks change
	$effect(() => {
		flatItems = [];
		tocItems = extractHeadings();
	});

	// Svelte 5: Effect for initialization and cleanup
	$effect(() => {
		if (!shouldShow || !browser) return;

		// Small delay to ensure DOM is ready
		const initTimeout = setTimeout(() => {
			injectHeadingIds();
			setupObserver();
			setupScrollListener();

			// Check for hash in URL
			if (window.location.hash) {
				const id = window.location.hash.slice(1);
				setTimeout(() => scrollToHeading(id), 100);
			}
		}, 100);

		// Svelte 5: Cleanup function returned from $effect
		return () => {
			clearTimeout(initTimeout);
			observer?.disconnect();
			if (scrollListener) {
				window.removeEventListener('scroll', scrollListener);
			}
		};
	});
</script>

{#if shouldShow}
	<nav
		class="toc-container"
		class:toc-inline={position === 'inline'}
		class:toc-sidebar={position === 'sidebar'}
		class:toc-floating={position === 'floating'}
		class:toc-sticky={sticky && position !== 'floating'}
		class:toc-minimized={isFloatingMinimized}
		aria-label="Table of contents"
	>
		<!-- Progress bar -->
		{#if showProgress}
			<div class="toc-progress-bar">
				<div class="toc-progress-fill" style="width: {readingProgress}%"></div>
			</div>
		{/if}

		<!-- Header -->
		<div class="toc-header">
			<button
				class="toc-title-button"
				onclick={toggleExpanded}
				aria-expanded={isExpanded}
				aria-controls="toc-list"
			>
				<svg class="toc-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<line x1="3" y1="6" x2="21" y2="6"></line>
					<line x1="3" y1="12" x2="15" y2="12"></line>
					<line x1="3" y1="18" x2="18" y2="18"></line>
				</svg>
				<span class="toc-title">{title}</span>
				<span class="toc-count">{flatItems.length} sections</span>
				{#if collapsible}
					<svg
						class="toc-chevron"
						class:rotated={isExpanded}
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<polyline points="6 9 12 15 18 9"></polyline>
					</svg>
				{/if}
			</button>

			{#if position === 'floating'}
				<button class="toc-minimize-btn" onclick={toggleMinimized} aria-label="Toggle TOC">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						{#if isFloatingMinimized}
							<polyline points="15 3 21 3 21 9"></polyline>
							<polyline points="9 21 3 21 3 15"></polyline>
							<line x1="21" y1="3" x2="14" y2="10"></line>
							<line x1="3" y1="21" x2="10" y2="14"></line>
						{:else}
							<polyline points="4 14 10 14 10 20"></polyline>
							<polyline points="20 10 14 10 14 4"></polyline>
							<line x1="14" y1="10" x2="21" y2="3"></line>
							<line x1="3" y1="21" x2="10" y2="14"></line>
						{/if}
					</svg>
				</button>
			{/if}
		</div>

		<!-- TOC List -->
		{#if isExpanded && !isFloatingMinimized}
			<div id="toc-list" class="toc-content">
				<ul class="toc-list" role="list">
					{#each tocItems as item}
						<li class="toc-item" class:active={activeId === item.id}>
							<button
								class="toc-link"
								onclick={() => scrollToHeading(item.id)}
								aria-current={activeId === item.id ? 'location' : undefined}
							>
								{#if showNumbers && item.number}
									<span class="toc-number">{item.number}</span>
								{/if}
								<span class="toc-text">{item.text}</span>
							</button>

							{#if item.children.length > 0}
								<ul class="toc-sublist" role="list">
									{#each item.children as child}
										<li class="toc-item toc-item-child" class:active={activeId === child.id}>
											<button
												class="toc-link"
												onclick={() => scrollToHeading(child.id)}
												aria-current={activeId === child.id ? 'location' : undefined}
											>
												{#if showNumbers && child.number}
													<span class="toc-number">{child.number}</span>
												{/if}
												<span class="toc-text">{child.text}</span>
											</button>

											{#if child.children.length > 0}
												<ul class="toc-sublist toc-sublist-deep" role="list">
													{#each child.children as grandchild}
														<li
															class="toc-item toc-item-grandchild"
															class:active={activeId === grandchild.id}
														>
															<button
																class="toc-link"
																onclick={() => scrollToHeading(grandchild.id)}
																aria-current={activeId === grandchild.id ? 'location' : undefined}
															>
																{#if showNumbers && grandchild.number}
																	<span class="toc-number">{grandchild.number}</span>
																{/if}
																<span class="toc-text">{grandchild.text}</span>
															</button>
														</li>
													{/each}
												</ul>
											{/if}
										</li>
									{/each}
								</ul>
							{/if}
						</li>
					{/each}
				</ul>

				<!-- Reading progress indicator -->
				{#if showProgress}
					<div class="toc-footer">
						<div class="toc-progress-text">
							<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
								<circle cx="12" cy="12" r="10"></circle>
								<polyline points="12 6 12 12 16 14"></polyline>
							</svg>
							<span>{Math.round(readingProgress)}% complete</span>
						</div>
					</div>
				{/if}
			</div>
		{/if}
	</nav>
{/if}

<style>
	.toc-container {
		--toc-bg: rgba(30, 41, 59, 0.95);
		--toc-border: rgba(148, 163, 184, 0.2);
		--toc-text: #e2e8f0;
		--toc-text-muted: #94a3b8;
		--toc-accent: #60a5fa;
		--toc-accent-bg: rgba(96, 165, 250, 0.1);
		--toc-hover-bg: rgba(96, 165, 250, 0.15);
		--toc-active-bg: rgba(96, 165, 250, 0.2);
		--toc-progress: linear-gradient(90deg, #60a5fa, #a78bfa);
		--toc-radius: 12px;

		font-family: inherit;
		background: var(--toc-bg);
		backdrop-filter: blur(12px);
		border: 1px solid var(--toc-border);
		border-radius: var(--toc-radius);
		overflow: hidden;
		transition: all 0.3s ease;
	}

	/* Position variants */
	.toc-inline {
		margin-bottom: 2rem;
	}

	.toc-sidebar {
		width: 280px;
		max-height: calc(100vh - 120px);
	}

	.toc-sidebar.toc-sticky {
		position: sticky;
		top: 100px;
	}

	.toc-floating {
		position: fixed;
		bottom: 24px;
		right: 24px;
		width: 320px;
		max-height: 70vh;
		z-index: 1000;
		box-shadow: 0 20px 40px rgba(0, 0, 0, 0.3);
	}

	.toc-floating.toc-minimized {
		width: auto;
		max-height: auto;
	}

	/* Progress bar */
	.toc-progress-bar {
		height: 3px;
		background: rgba(148, 163, 184, 0.1);
		overflow: hidden;
	}

	.toc-progress-fill {
		height: 100%;
		background: var(--toc-progress);
		transition: width 0.1s ease-out;
	}

	/* Header */
	.toc-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1rem 1.25rem;
		border-bottom: 1px solid var(--toc-border);
		background: rgba(15, 23, 42, 0.3);
	}

	.toc-title-button {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		background: none;
		border: none;
		color: var(--toc-text);
		cursor: pointer;
		padding: 0;
		flex: 1;
		text-align: left;
	}

	.toc-title-button:hover {
		color: var(--toc-accent);
	}

	.toc-icon {
		width: 20px;
		height: 20px;
		flex-shrink: 0;
	}

	.toc-title {
		font-size: 0.9375rem;
		font-weight: 700;
		letter-spacing: 0.01em;
	}

	.toc-count {
		font-size: 0.75rem;
		color: var(--toc-text-muted);
		background: var(--toc-accent-bg);
		padding: 0.25rem 0.5rem;
		border-radius: 12px;
		margin-left: auto;
	}

	.toc-chevron {
		width: 18px;
		height: 18px;
		transition: transform 0.3s ease;
		flex-shrink: 0;
		margin-left: 0.5rem;
	}

	.toc-chevron.rotated {
		transform: rotate(180deg);
	}

	.toc-minimize-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		background: var(--toc-accent-bg);
		border: 1px solid var(--toc-border);
		border-radius: 8px;
		color: var(--toc-text-muted);
		cursor: pointer;
		margin-left: 0.75rem;
		transition: all 0.2s ease;
	}

	.toc-minimize-btn:hover {
		background: var(--toc-hover-bg);
		color: var(--toc-accent);
	}

	.toc-minimize-btn svg {
		width: 16px;
		height: 16px;
	}

	/* Content */
	.toc-content {
		max-height: 400px;
		overflow-y: auto;
		scrollbar-width: thin;
		scrollbar-color: var(--toc-accent-bg) transparent;
	}

	.toc-content::-webkit-scrollbar {
		width: 6px;
	}

	.toc-content::-webkit-scrollbar-track {
		background: transparent;
	}

	.toc-content::-webkit-scrollbar-thumb {
		background: var(--toc-accent-bg);
		border-radius: 3px;
	}

	.toc-sidebar .toc-content,
	.toc-floating .toc-content {
		max-height: calc(100vh - 220px);
	}

	/* List styles */
	.toc-list {
		list-style: none;
		padding: 0.75rem 0;
		margin: 0;
	}

	.toc-sublist {
		list-style: none;
		padding: 0;
		margin: 0;
		padding-left: 1rem;
		border-left: 2px solid var(--toc-border);
		margin-left: 1.25rem;
	}

	.toc-sublist-deep {
		border-color: rgba(148, 163, 184, 0.1);
	}

	.toc-item {
		position: relative;
	}

	.toc-link {
		display: flex;
		align-items: flex-start;
		gap: 0.5rem;
		width: 100%;
		padding: 0.625rem 1.25rem;
		background: none;
		border: none;
		color: var(--toc-text-muted);
		text-align: left;
		cursor: pointer;
		transition: all 0.2s ease;
		font-size: 0.875rem;
		line-height: 1.5;
	}

	.toc-link:hover {
		color: var(--toc-text);
		background: var(--toc-hover-bg);
	}

	.toc-item.active > .toc-link {
		color: var(--toc-accent);
		background: var(--toc-active-bg);
		font-weight: 600;
	}

	.toc-item.active > .toc-link::before {
		content: '';
		position: absolute;
		left: 0;
		top: 50%;
		transform: translateY(-50%);
		width: 3px;
		height: 60%;
		background: var(--toc-progress);
		border-radius: 0 3px 3px 0;
	}

	.toc-item-child .toc-link {
		padding: 0.5rem 1rem;
		font-size: 0.8125rem;
	}

	.toc-item-grandchild .toc-link {
		padding: 0.375rem 0.75rem;
		font-size: 0.75rem;
	}

	.toc-number {
		font-weight: 600;
		color: var(--toc-accent);
		font-size: 0.75em;
		background: var(--toc-accent-bg);
		padding: 0.125rem 0.375rem;
		border-radius: 4px;
		flex-shrink: 0;
		min-width: 1.75rem;
		text-align: center;
	}

	.toc-text {
		flex: 1;
		overflow: hidden;
		text-overflow: ellipsis;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
	}

	/* Footer */
	.toc-footer {
		padding: 0.75rem 1.25rem;
		border-top: 1px solid var(--toc-border);
		background: rgba(15, 23, 42, 0.3);
	}

	.toc-progress-text {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.75rem;
		color: var(--toc-text-muted);
	}

	.toc-progress-text svg {
		width: 14px;
		height: 14px;
	}

	/* Responsive */
	@media (max-width: 1024px) {
		.toc-sidebar {
			position: static;
			width: 100%;
			max-height: none;
			margin-bottom: 2rem;
		}

		.toc-sidebar .toc-content {
			max-height: 300px;
		}
	}

	@media (max-width: 768px) {
		.toc-floating {
			left: 16px;
			right: 16px;
			bottom: 16px;
			width: auto;
			max-height: 60vh;
		}

		.toc-content {
			max-height: 250px;
		}

		.toc-header {
			padding: 0.875rem 1rem;
		}

		.toc-link {
			padding: 0.5rem 1rem;
		}

		.toc-title {
			font-size: 0.875rem;
		}

		.toc-count {
			display: none;
		}
	}

	/* Animation for expansion */
	.toc-content {
		animation: slideDown 0.3s ease;
	}

	@keyframes slideDown {
		from {
			opacity: 0;
			max-height: 0;
		}
		to {
			opacity: 1;
			max-height: 400px;
		}
	}

	/* Focus styles for accessibility */
	.toc-link:focus-visible,
	.toc-title-button:focus-visible,
	.toc-minimize-btn:focus-visible {
		outline: 2px solid var(--toc-accent);
		outline-offset: 2px;
	}
</style>
