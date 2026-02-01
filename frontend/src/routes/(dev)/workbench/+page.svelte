<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * Component Workbench - Main UI
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Zero-config component explorer for Svelte 5
	 * @version 1.0.0 - January 2026
	 * @standards Apple Principal Engineer ICT Level 7+
	 *
	 * Features:
	 * - Auto-discovers all components from $lib/components
	 * - Live props editing with type-aware inputs
	 * - Viewport controls for responsive testing
	 * - Theme/background customization
	 * - Source code viewer
	 * - Favorites and recent components
	 */
	import type { PageData } from './$types';
	import type { ComponentInfo } from './+page.server';
	import ComponentTree from './ComponentTree.svelte';
	import ComponentRenderer from './ComponentRenderer.svelte';
	import PropsEditor from './PropsEditor.svelte';
	import SnippetEditor from './SnippetEditor.svelte';

	let { data }: { data: PageData } = $props();

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let selectedComponent = $state<ComponentInfo | null>(null);
	let propValues = $state<Record<string, unknown>>({});
	let snippetContent = $state('');

	// Viewport state
	let viewportWidth = $state(1280);
	let viewportHeight = $state<number | null>(null);
	let zoom = $state(100);

	// Theme state
	let background = $state('#ffffff');
	let showBorder = $state(true);
	let padding = $state(16);

	// Panel state
	let showSource = $state(false);
	let rightPanelTab = $state<'props' | 'source'>('props');

	// ═══════════════════════════════════════════════════════════════════════════
	// VIEWPORT PRESETS
	// ═══════════════════════════════════════════════════════════════════════════

	const viewportPresets = [
		{ name: 'iPhone SE', width: 375, height: 667 },
		{ name: 'iPhone 14', width: 390, height: 844 },
		{ name: 'iPad', width: 768, height: 1024 },
		{ name: 'Desktop', width: 1280, height: null },
		{ name: 'Wide', width: 1920, height: null }
	];

	const zoomLevels = [50, 75, 100, 125, 150, 200];

	const backgroundPresets = [
		{ name: 'White', value: '#ffffff' },
		{ name: 'Light', value: '#f4f4f5' },
		{ name: 'Dark', value: '#18181b' },
		{ name: 'Black', value: '#0a0a0a' },
		{ name: 'Checker', value: 'repeating-conic-gradient(#808080 0% 25%, transparent 0% 50%) 50% / 20px 20px' }
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// HANDLERS
	// ═══════════════════════════════════════════════════════════════════════════

	function handleSelectComponent(component: ComponentInfo) {
		selectedComponent = component;
		propValues = {};
		snippetContent = '';

		// Initialize default values for props
		for (const prop of component.props) {
			if (prop.defaultValue) {
				try {
					propValues[prop.name] = JSON.parse(prop.defaultValue);
				} catch {
					propValues[prop.name] = prop.defaultValue.replace(/^['"]|['"]$/g, '');
				}
			}
		}

		// Update URL with component path
		if (typeof window !== 'undefined') {
			const url = new URL(window.location.href);
			url.searchParams.set('component', component.relativePath);
			window.history.replaceState({}, '', url.toString());
		}
	}

	function handlePropChange(name: string, value: unknown) {
		propValues = { ...propValues, [name]: value };
	}

	function handleSnippetChange(value: string) {
		snippetContent = value;
	}

	function setViewport(preset: (typeof viewportPresets)[0]) {
		viewportWidth = preset.width;
		viewportHeight = preset.height;
	}

	function copyPermalink() {
		if (!selectedComponent) return;

		const url = new URL(window.location.href);
		url.searchParams.set('component', selectedComponent.relativePath);
		url.searchParams.set('props', JSON.stringify(propValues));

		navigator.clipboard.writeText(url.toString());
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// LIFECYCLE - Load component from URL
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		if (typeof window === 'undefined') return;

		const url = new URL(window.location.href);
		const componentPath = url.searchParams.get('component');

		if (componentPath && !selectedComponent) {
			// Find component by path
			for (const components of Object.values(data.tree)) {
				const found = components.find((c) => c.relativePath === componentPath);
				if (found) {
					handleSelectComponent(found);
					break;
				}
			}
		}
	});
</script>

<svelte:head>
	<title>Component Workbench | {selectedComponent?.name ?? 'Select a component'}</title>
</svelte:head>

<div class="workbench">
	<!-- Sidebar: Component Tree -->
	<ComponentTree
		tree={data.tree}
		selectedComponent={selectedComponent?.relativePath ?? null}
		onSelect={handleSelectComponent}
	/>

	<!-- Main Content Area -->
	<div class="main-area">
		<!-- Toolbar -->
		<header class="toolbar">
			<!-- Viewport Controls -->
			<div class="toolbar-group">
				<span class="toolbar-label">Viewport:</span>
				{#each viewportPresets as preset}
					<button
						class="toolbar-btn"
						class:active={viewportWidth === preset.width}
						onclick={() => setViewport(preset)}
					>
						{preset.name}
					</button>
				{/each}
				<input
					type="number"
					class="viewport-input"
					bind:value={viewportWidth}
					min="320"
					max="2560"
					title="Custom width"
				/>
			</div>

			<!-- Zoom Controls -->
			<div class="toolbar-group">
				<span class="toolbar-label">Zoom:</span>
				<select class="toolbar-select" bind:value={zoom}>
					{#each zoomLevels as level}
						<option value={level}>{level}%</option>
					{/each}
				</select>
			</div>

			<!-- Background Controls -->
			<div class="toolbar-group">
				<span class="toolbar-label">Background:</span>
				{#each backgroundPresets as preset}
					<button
						class="bg-btn"
						class:active={background === preset.value}
						onclick={() => (background = preset.value)}
						title={preset.name}
						style="background: {preset.value};"
					></button>
				{/each}
			</div>

			<!-- Options -->
			<div class="toolbar-group">
				<label class="toolbar-toggle">
					<input type="checkbox" bind:checked={showBorder} />
					<span>Border</span>
				</label>
				<label class="toolbar-toggle">
					<span>Padding:</span>
					<input
						type="number"
						class="padding-input"
						bind:value={padding}
						min="0"
						max="100"
					/>
				</label>
			</div>

			<!-- Actions -->
			<div class="toolbar-group toolbar-group--right">
				{#if selectedComponent}
					<button class="action-btn" onclick={copyPermalink} title="Copy permalink">
						🔗
					</button>
				{/if}
				<span class="component-count">
					{data.totalCount} components
				</span>
			</div>
		</header>

		<!-- Preview Area -->
		<ComponentRenderer
			component={selectedComponent}
			{propValues}
			{snippetContent}
			viewport={{ width: viewportWidth, height: viewportHeight }}
			{background}
			{showBorder}
			{padding}
			{zoom}
		/>
	</div>

	<!-- Right Panel: Props & Source -->
	<aside class="right-panel">
		<!-- Tab Switcher -->
		<div class="panel-tabs">
			<button
				class="panel-tab"
				class:active={rightPanelTab === 'props'}
				onclick={() => (rightPanelTab = 'props')}
			>
				Props
			</button>
			<button
				class="panel-tab"
				class:active={rightPanelTab === 'source'}
				onclick={() => (rightPanelTab = 'source')}
			>
				Source
			</button>
		</div>

		{#if rightPanelTab === 'props'}
			<!-- Props Editor -->
			{#if selectedComponent}
				<PropsEditor
					props={selectedComponent.props}
					values={propValues}
					onChange={handlePropChange}
				/>

				<!-- Snippet Editor -->
				<SnippetEditor
					value={snippetContent}
					onChange={handleSnippetChange}
					hasSnippets={selectedComponent.hasSnippets}
				/>
			{:else}
				<div class="no-component">
					<p>Select a component to edit its props</p>
				</div>
			{/if}
		{:else}
			<!-- Source Viewer -->
			{#if selectedComponent}
				<div class="source-viewer">
					<pre class="source-code">{selectedComponent.source}</pre>
				</div>
			{:else}
				<div class="no-component">
					<p>Select a component to view its source</p>
				</div>
			{/if}
		{/if}
	</aside>
</div>

<style>
	.workbench {
		display: flex;
		height: calc(100vh - 52px);
		overflow: hidden;
	}

	.main-area {
		flex: 1;
		display: flex;
		flex-direction: column;
		min-width: 0;
	}

	/* Toolbar */
	.toolbar {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 0.5rem 1rem;
		background: #111;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
		flex-wrap: wrap;
	}

	.toolbar-group {
		display: flex;
		align-items: center;
		gap: 0.375rem;
	}

	.toolbar-group--right {
		margin-left: auto;
	}

	.toolbar-label {
		font-size: 0.6875rem;
		color: #71717a;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.toolbar-btn {
		padding: 0.375rem 0.625rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.25rem;
		color: #a1a1aa;
		font-size: 0.75rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.toolbar-btn:hover {
		background: rgba(255, 255, 255, 0.1);
		color: #fafafa;
	}

	.toolbar-btn.active {
		background: rgba(139, 92, 246, 0.2);
		border-color: rgba(139, 92, 246, 0.4);
		color: #c4b5fd;
	}

	.toolbar-select {
		padding: 0.375rem 0.5rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.25rem;
		color: #fafafa;
		font-size: 0.75rem;
		cursor: pointer;
	}

	.viewport-input,
	.padding-input {
		width: 60px;
		padding: 0.375rem 0.5rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.25rem;
		color: #fafafa;
		font-size: 0.75rem;
		text-align: center;
	}

	.padding-input {
		width: 40px;
	}

	.bg-btn {
		width: 20px;
		height: 20px;
		border: 1px solid rgba(255, 255, 255, 0.2);
		border-radius: 0.25rem;
		cursor: pointer;
		transition: transform 0.15s;
	}

	.bg-btn:hover {
		transform: scale(1.1);
	}

	.bg-btn.active {
		outline: 2px solid #8b5cf6;
		outline-offset: 1px;
	}

	.toolbar-toggle {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-size: 0.75rem;
		color: #a1a1aa;
		cursor: pointer;
	}

	.toolbar-toggle input[type='checkbox'] {
		accent-color: #8b5cf6;
	}

	.action-btn {
		padding: 0.375rem 0.5rem;
		background: transparent;
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.25rem;
		font-size: 0.875rem;
		cursor: pointer;
		transition: background 0.15s;
	}

	.action-btn:hover {
		background: rgba(255, 255, 255, 0.1);
	}

	.component-count {
		font-size: 0.6875rem;
		color: #52525b;
	}

	/* Right Panel */
	.right-panel {
		width: 320px;
		min-width: 320px;
		display: flex;
		flex-direction: column;
		background: #111;
		border-left: 1px solid rgba(255, 255, 255, 0.1);
		overflow: hidden;
	}

	.panel-tabs {
		display: flex;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
	}

	.panel-tab {
		flex: 1;
		padding: 0.75rem;
		background: transparent;
		border: none;
		color: #71717a;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s;
	}

	.panel-tab:hover {
		background: rgba(255, 255, 255, 0.05);
		color: #a1a1aa;
	}

	.panel-tab.active {
		background: rgba(139, 92, 246, 0.1);
		color: #c4b5fd;
		border-bottom: 2px solid #8b5cf6;
	}

	.no-component {
		padding: 2rem;
		text-align: center;
		color: #52525b;
		font-size: 0.875rem;
	}

	.source-viewer {
		flex: 1;
		overflow: auto;
		padding: 1rem;
	}

	.source-code {
		margin: 0;
		font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
		font-size: 0.75rem;
		line-height: 1.6;
		color: #a1a1aa;
		white-space: pre-wrap;
		word-break: break-all;
	}
</style>
