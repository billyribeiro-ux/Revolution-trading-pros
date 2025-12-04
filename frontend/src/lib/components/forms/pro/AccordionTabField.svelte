<script lang="ts">
	/**
	 * AccordionTabField Component (FluentForms 6.1.5 - November 2025)
	 *
	 * Long, complex forms can overwhelm users. This component solves this with
	 * two elegant modes:
	 * - Accordion Mode: expandable sections that open one at a time
	 * - Tab Mode: horizontal tabs for quick navigation between sections
	 */

	interface Section {
		id: string;
		title: string;
		icon?: string;
		content?: any;
	}

	interface Props {
		sections: Section[];
		mode?: 'accordion' | 'tabs';
		defaultOpen?: string;
		allowMultiple?: boolean;
		animated?: boolean;
		required?: boolean;
		error?: string;
		disabled?: boolean;
	}

	let {
		sections = [],
		mode = 'accordion',
		defaultOpen = '',
		allowMultiple = false,
		animated = true,
		required = false,
		error = '',
		disabled = false
	}: Props = $props();

	let openSections = $state<Set<string>>(new Set(defaultOpen ? [defaultOpen] : []));
	let activeTab = $state(defaultOpen || sections[0]?.id || '');

	function toggleSection(sectionId: string) {
		if (disabled) return;

		if (mode === 'tabs') {
			activeTab = sectionId;
			return;
		}

		const newOpenSections = new Set(openSections);

		if (newOpenSections.has(sectionId)) {
			newOpenSections.delete(sectionId);
		} else {
			if (!allowMultiple) {
				newOpenSections.clear();
			}
			newOpenSections.add(sectionId);
		}

		openSections = newOpenSections;
	}

	function isOpen(sectionId: string): boolean {
		if (mode === 'tabs') {
			return activeTab === sectionId;
		}
		return openSections.has(sectionId);
	}
</script>

{#if mode === 'accordion'}
	<div class="accordion-field" class:disabled class:has-error={error}>
		{#each sections as section}
			<div class="accordion-item" class:open={isOpen(section.id)}>
				<button
					type="button"
					class="accordion-header"
					onclick={() => toggleSection(section.id)}
					aria-expanded={isOpen(section.id)}
					aria-controls="section-{section.id}"
					{disabled}
				>
					{#if section.icon}
						<span class="section-icon">{section.icon}</span>
					{/if}
					<span class="section-title">{section.title}</span>
					{#if required}
						<span class="required-marker">*</span>
					{/if}
					<svg
						class="chevron"
						width="16"
						height="16"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<polyline points="6 9 12 15 18 9"></polyline>
					</svg>
				</button>
				<div
					id="section-{section.id}"
					class="accordion-content"
					class:animated
					role="region"
					aria-labelledby="header-{section.id}"
				>
					<div class="content-inner">
						<slot name={section.id}>
							{#if section.content}
								{@html section.content}
							{/if}
						</slot>
					</div>
				</div>
			</div>
		{/each}
	</div>
{:else}
	<div class="tabs-field" class:disabled class:has-error={error}>
		<div class="tabs-header" role="tablist">
			{#each sections as section}
				<button
					type="button"
					class="tab-button"
					class:active={activeTab === section.id}
					onclick={() => toggleSection(section.id)}
					role="tab"
					aria-selected={activeTab === section.id}
					aria-controls="panel-{section.id}"
					{disabled}
				>
					{#if section.icon}
						<span class="section-icon">{section.icon}</span>
					{/if}
					<span class="tab-title">{section.title}</span>
					{#if required}
						<span class="required-marker">*</span>
					{/if}
				</button>
			{/each}
		</div>
		<div class="tabs-content">
			{#each sections as section}
				<div
					id="panel-{section.id}"
					class="tab-panel"
					class:active={activeTab === section.id}
					class:animated
					role="tabpanel"
					aria-labelledby="tab-{section.id}"
				>
					<slot name={section.id}>
						{#if section.content}
							{@html section.content}
						{/if}
					</slot>
				</div>
			{/each}
		</div>
	</div>
{/if}

{#if error}
	<div class="error-message">{error}</div>
{/if}

<style>
	/* Accordion Mode Styles */
	.accordion-field {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.accordion-item {
		border: 1px solid #e5e7eb;
		border-radius: 0.5rem;
		overflow: hidden;
		background-color: white;
	}

	.accordion-item.open {
		border-color: #3b82f6;
	}

	.accordion-header {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		width: 100%;
		padding: 1rem;
		background-color: #f9fafb;
		border: none;
		cursor: pointer;
		text-align: left;
		font-size: 0.9375rem;
		font-weight: 500;
		color: #374151;
		transition: all 0.2s;
	}

	.accordion-header:hover:not(:disabled) {
		background-color: #f3f4f6;
	}

	.accordion-item.open .accordion-header {
		background-color: #eff6ff;
		color: #1d4ed8;
	}

	.section-icon {
		font-size: 1.125rem;
	}

	.section-title {
		flex: 1;
	}

	.required-marker {
		color: #ef4444;
		font-weight: 600;
	}

	.chevron {
		transition: transform 0.2s;
		color: #9ca3af;
	}

	.accordion-item.open .chevron {
		transform: rotate(180deg);
		color: #3b82f6;
	}

	.accordion-content {
		display: grid;
		grid-template-rows: 0fr;
		transition: grid-template-rows 0.3s ease-out;
	}

	.accordion-content.animated {
		transition: grid-template-rows 0.3s ease-out;
	}

	.accordion-item.open .accordion-content {
		grid-template-rows: 1fr;
	}

	.content-inner {
		overflow: hidden;
	}

	.accordion-item.open .content-inner {
		padding: 1rem;
		border-top: 1px solid #e5e7eb;
	}

	/* Tabs Mode Styles */
	.tabs-field {
		display: flex;
		flex-direction: column;
	}

	.tabs-header {
		display: flex;
		border-bottom: 2px solid #e5e7eb;
		overflow-x: auto;
	}

	.tab-button {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: none;
		border: none;
		border-bottom: 2px solid transparent;
		margin-bottom: -2px;
		cursor: pointer;
		font-size: 0.9375rem;
		font-weight: 500;
		color: #6b7280;
		white-space: nowrap;
		transition: all 0.2s;
	}

	.tab-button:hover:not(:disabled) {
		color: #374151;
		background-color: #f9fafb;
	}

	.tab-button.active {
		color: #3b82f6;
		border-bottom-color: #3b82f6;
	}

	.tab-title {
		display: inline;
	}

	.tabs-content {
		position: relative;
	}

	.tab-panel {
		display: none;
		padding: 1.5rem;
	}

	.tab-panel.active {
		display: block;
	}

	.tab-panel.animated {
		animation: fadeIn 0.3s ease-out;
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
			transform: translateY(-5px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	/* Disabled State */
	.disabled {
		opacity: 0.6;
		pointer-events: none;
	}

	.accordion-header:disabled,
	.tab-button:disabled {
		cursor: not-allowed;
		opacity: 0.5;
	}

	/* Error State */
	.has-error .accordion-item {
		border-color: #fca5a5;
	}

	.has-error .tabs-header {
		border-color: #fca5a5;
	}

	.error-message {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-top: 0.5rem;
		padding: 0.5rem 0.75rem;
		background-color: #fef2f2;
		color: #dc2626;
		border-radius: 0.375rem;
		font-size: 0.875rem;
	}
</style>
