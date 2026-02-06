<!--
/**
 * Risk Disclaimer Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Prominent risk warning display for trading content
 * Supports multiple styles, preset disclaimers, and acknowledgment flow
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */
-->

<script lang="ts">
	import {
		IconAlertTriangle,
		IconCircleX,
		IconInfoCircle,
		IconChevronDown,
		IconChevronUp,
		IconShieldCheck,
		IconEdit,
		IconSquare,
		IconSquareCheck
	} from '$lib/icons';
	import type { Block, BlockContent } from '../types';
	import type { BlockId } from '$lib/stores/blockState.svelte';
	import { onMount } from 'svelte';

	// ═══════════════════════════════════════════════════════════════════════════
	// Props Interface
	// ═══════════════════════════════════════════════════════════════════════════

	interface Props {
		block: Block;
		blockId: BlockId;
		isSelected: boolean;
		isEditing: boolean;
		onUpdate: (updates: Partial<Block>) => void;
		onError?: (error: Error) => void;
	}

	let props: Props = $props();

	// ═══════════════════════════════════════════════════════════════════════════
	// Content Interfaces
	// ═══════════════════════════════════════════════════════════════════════════

	interface RiskDisclaimerContent {
		text: string;
		expandedText?: string;
		style: 'warning' | 'danger' | 'info';
		preset?: 'general' | 'trading' | 'investment' | 'custom';
		requireAcknowledgment?: boolean;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Preset Disclaimers
	// ═══════════════════════════════════════════════════════════════════════════

	const PRESET_DISCLAIMERS: Record<
		string,
		{ text: string; expandedText: string; defaultStyle: RiskDisclaimerContent['style'] }
	> = {
		general: {
			text: 'Trading involves substantial risk of loss and is not suitable for all investors.',
			expandedText:
				'The high degree of leverage can work against you as well as for you. Before deciding to invest, you should carefully consider your investment objectives, level of experience, and risk appetite. The possibility exists that you could sustain a loss of some or all of your initial investment and therefore you should not invest money that you cannot afford to lose. You should be aware of all the risks associated with trading and seek advice from an independent financial advisor if you have any doubts.',
			defaultStyle: 'warning'
		},
		trading: {
			text: 'Past performance is not indicative of future results. Trading carries significant risk.',
			expandedText:
				'Trading financial instruments carries a high level of risk to your capital with the possibility of losing more than your initial investment. Trading is not suitable for all investors. Please ensure that you fully understand the risks involved, taking into account your investment objectives and level of experience, before trading. If necessary, seek independent advice. Historical results are not a reliable indicator of future performance. No representation is being made that any account will or is likely to achieve profits or losses similar to those shown. All trading strategies are used at your own risk.',
			defaultStyle: 'danger'
		},
		investment: {
			text: 'This content does not constitute financial advice. Always consult a qualified professional.',
			expandedText:
				'The information provided on this platform is for educational and informational purposes only. It should not be considered as investment advice or a recommendation to buy, sell, or hold any financial instrument. The content is not tailored to the specific investment needs, objectives, or financial situation of any individual user. Before making any investment decisions, you should conduct your own research and consult with a licensed financial advisor or other qualified professional who can provide personalized advice based on your individual circumstances.',
			defaultStyle: 'info'
		}
	};

	// ═══════════════════════════════════════════════════════════════════════════
	// State Management
	// ═══════════════════════════════════════════════════════════════════════════

	let isExpanded = $state(false);
	let isAcknowledged = $state(false);
	let animateCheckbox = $state(false);

	// ═══════════════════════════════════════════════════════════════════════════
	// Derived State
	// ═══════════════════════════════════════════════════════════════════════════

	let style = $derived<RiskDisclaimerContent['style']>(
		(props.block.content.disclaimerStyle as RiskDisclaimerContent['style']) || 'warning'
	);
	let preset = $derived<NonNullable<RiskDisclaimerContent['preset']>>(
		(props.block.content.disclaimerPreset as RiskDisclaimerContent['preset']) || 'general'
	);
	let text = $derived(
		props.block.content.disclaimerText ||
			(preset && preset !== 'custom' ? PRESET_DISCLAIMERS[preset]?.text : '') ||
			''
	);
	let expandedText = $derived(
		props.block.content.disclaimerExpandedText ||
			(preset && preset !== 'custom' ? PRESET_DISCLAIMERS[preset]?.expandedText : '') ||
			''
	);
	let requireAcknowledgment = $derived(props.block.content.disclaimerRequireAck === true);

	// Style configurations
	let styleConfig = $derived.by(() => {
		const configs = {
			warning: {
				icon: IconAlertTriangle,
				bgLight: 'linear-gradient(135deg, #fefce8 0%, #fef3c7 100%)',
				bgDark: 'linear-gradient(135deg, #422006 0%, #1c1917 100%)',
				borderLight: '#fbbf24',
				borderDark: '#b45309',
				textLight: '#92400e',
				textDark: '#fcd34d',
				accentLight: '#f59e0b',
				accentDark: '#f59e0b',
				iconBg: '#fef3c7',
				iconBgDark: '#78350f',
				role: 'note' as const
			},
			danger: {
				icon: IconCircleX,
				bgLight: 'linear-gradient(135deg, #fef2f2 0%, #fee2e2 100%)',
				bgDark: 'linear-gradient(135deg, #450a0a 0%, #1c1917 100%)',
				borderLight: '#f87171',
				borderDark: '#991b1b',
				textLight: '#991b1b',
				textDark: '#fca5a5',
				accentLight: '#ef4444',
				accentDark: '#ef4444',
				iconBg: '#fee2e2',
				iconBgDark: '#7f1d1d',
				role: 'alert' as const
			},
			info: {
				icon: IconInfoCircle,
				bgLight: 'linear-gradient(135deg, #eff6ff 0%, #dbeafe 100%)',
				bgDark: 'linear-gradient(135deg, #1e3a8a 0%, #1e293b 100%)',
				borderLight: '#60a5fa',
				borderDark: '#1e40af',
				textLight: '#1e40af',
				textDark: '#93c5fd',
				accentLight: '#3b82f6',
				accentDark: '#3b82f6',
				iconBg: '#dbeafe',
				iconBgDark: '#1e3a8a',
				role: 'note' as const
			}
		};
		return configs[style];
	});

	let IconComponent = $derived(styleConfig.icon);
	let ariaRole = $derived(styleConfig.role);

	// Check if there's expanded content to show
	let hasExpandedContent = $derived(expandedText && expandedText.length > 0);

	// Generate unique IDs for accessibility
	let contentId = $derived(`disclaimer-content-${props.blockId}`);
	let expandedId = $derived(`disclaimer-expanded-${props.blockId}`);

	// ═══════════════════════════════════════════════════════════════════════════
	// Lifecycle
	// ═══════════════════════════════════════════════════════════════════════════

	onMount(() => {
		// Check if acknowledgment was previously stored (e.g., in sessionStorage)
		const storageKey = `risk-ack-${props.blockId}`;
		const wasAcknowledged = sessionStorage.getItem(storageKey);
		if (wasAcknowledged === 'true') {
			isAcknowledged = true;
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// Update Functions
	// ═══════════════════════════════════════════════════════════════════════════

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	function handlePresetChange(e: Event): void {
		const select = e.target as HTMLSelectElement;
		const newPreset = select.value as NonNullable<RiskDisclaimerContent['preset']>;

		if (newPreset === 'custom') {
			updateContent({ disclaimerPreset: newPreset });
		} else {
			const presetData = PRESET_DISCLAIMERS[newPreset];
			if (presetData) {
				updateContent({
					disclaimerPreset: newPreset,
					disclaimerText: presetData.text,
					disclaimerExpandedText: presetData.expandedText,
					disclaimerStyle: presetData.defaultStyle
				});
			}
		}
	}

	function handleStyleChange(e: Event): void {
		const select = e.target as HTMLSelectElement;
		updateContent({ disclaimerStyle: select.value as 'warning' | 'danger' | 'info' });
	}

	function handleTextChange(e: Event): void {
		const textarea = e.target as HTMLTextAreaElement;
		updateContent({
			disclaimerText: textarea.value,
			disclaimerPreset: 'custom'
		});
	}

	function handleExpandedTextChange(e: Event): void {
		const textarea = e.target as HTMLTextAreaElement;
		updateContent({
			disclaimerExpandedText: textarea.value,
			disclaimerPreset: 'custom'
		});
	}

	function handleAckToggle(e: Event): void {
		const checkbox = e.target as HTMLInputElement;
		updateContent({ disclaimerRequireAck: checkbox.checked });
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Interaction Handlers
	// ═══════════════════════════════════════════════════════════════════════════

	function toggleExpanded(): void {
		isExpanded = !isExpanded;
	}

	function handleAcknowledge(): void {
		isAcknowledged = !isAcknowledged;
		animateCheckbox = true;

		// Store acknowledgment in session
		const storageKey = `risk-ack-${props.blockId}`;
		if (isAcknowledged) {
			sessionStorage.setItem(storageKey, 'true');
		} else {
			sessionStorage.removeItem(storageKey);
		}

		// Reset animation after it completes
		setTimeout(() => {
			animateCheckbox = false;
		}, 300);
	}

	function handleKeyDown(e: KeyboardEvent): void {
		if (e.key === 'Enter' || e.key === ' ') {
			e.preventDefault();
			toggleExpanded();
		}
	}

	function handleCheckboxKeyDown(e: KeyboardEvent): void {
		if (e.key === 'Enter' || e.key === ' ') {
			e.preventDefault();
			handleAcknowledge();
		}
	}
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════ -->
<!-- Template -->
<!-- ═══════════════════════════════════════════════════════════════════════════ -->

<div
	class="risk-disclaimer-block style-{style}"
	role={ariaRole}
	aria-labelledby={contentId}
	style="
		--bg-gradient: {styleConfig.bgLight};
		--border-color: {styleConfig.borderLight};
		--text-color: {styleConfig.textLight};
		--accent-color: {styleConfig.accentLight};
		--icon-bg: {styleConfig.iconBg};
	"
>
	<!-- Settings Panel (Edit Mode) -->
	{#if props.isEditing && props.isSelected}
		<div class="disclaimer-settings">
			<div class="settings-header">
				<IconEdit size={16} aria-hidden="true" />
				<span>Disclaimer Settings</span>
			</div>

			<div class="settings-row">
				<label class="setting-field">
					<span>Preset:</span>
					<select value={preset} onchange={handlePresetChange}>
						<option value="general">General Risk</option>
						<option value="trading">Trading Risk</option>
						<option value="investment">Investment Advice</option>
						<option value="custom">Custom</option>
					</select>
				</label>

				<label class="setting-field">
					<span>Style:</span>
					<select value={style} onchange={handleStyleChange}>
						<option value="warning">Warning (Yellow)</option>
						<option value="danger">Danger (Red)</option>
						<option value="info">Info (Blue)</option>
					</select>
				</label>

				<label class="setting-field checkbox-field">
					<input type="checkbox" checked={requireAcknowledgment} onchange={handleAckToggle} />
					<span>Require acknowledgment</span>
				</label>
			</div>

			<div class="settings-row text-fields">
				<label class="setting-field full-width">
					<span>Main Warning Text:</span>
					<textarea
						value={text}
						oninput={handleTextChange}
						placeholder="Enter the main warning message..."
						rows="2"
					></textarea>
				</label>
			</div>

			<div class="settings-row text-fields">
				<label class="setting-field full-width">
					<span>Expanded Details (optional):</span>
					<textarea
						value={expandedText}
						oninput={handleExpandedTextChange}
						placeholder="Enter detailed disclaimer text for the expandable section..."
						rows="4"
					></textarea>
				</label>
			</div>
		</div>
	{/if}

	<!-- Disclaimer Display -->
	<div class="disclaimer-container">
		<!-- Icon -->
		<div class="disclaimer-icon" aria-hidden="true">
			<IconComponent size={24} />
		</div>

		<!-- Content -->
		<div class="disclaimer-content">
			<!-- Main Text -->
			<div id={contentId} class="disclaimer-main">
				{#if props.isEditing && !props.isSelected}
					<p class="main-text">{text}</p>
				{:else if !props.isEditing}
					<p class="main-text">{text}</p>
				{:else}
					<p class="main-text editing-preview">{text || 'Enter warning text...'}</p>
				{/if}
			</div>

			<!-- Expandable Section -->
			{#if hasExpandedContent}
				<div class="expandable-section">
					<button
						type="button"
						class="expand-toggle"
						onclick={toggleExpanded}
						onkeydown={handleKeyDown}
						aria-expanded={isExpanded}
						aria-controls={expandedId}
					>
						<span>{isExpanded ? 'Hide details' : 'Read more'}</span>
						{#if isExpanded}
							<IconChevronUp size={16} aria-hidden="true" />
						{:else}
							<IconChevronDown size={16} aria-hidden="true" />
						{/if}
					</button>

					<div
						id={expandedId}
						class="expanded-content"
						class:visible={isExpanded}
						aria-hidden={!isExpanded}
					>
						<div class="expanded-inner">
							<p>{expandedText}</p>
						</div>
					</div>
				</div>
			{/if}

			<!-- Acknowledgment Checkbox -->
			{#if requireAcknowledgment && !props.isEditing}
				<div class="acknowledgment-section">
					<div class="divider"></div>
					<label
						class="acknowledgment-checkbox"
						class:checked={isAcknowledged}
						class:animate={animateCheckbox}
					>
						<button
							type="button"
							class="checkbox-button"
							role="checkbox"
							aria-checked={isAcknowledged}
							onclick={handleAcknowledge}
							onkeydown={handleCheckboxKeyDown}
						>
							{#if isAcknowledged}
								<IconSquareCheck size={22} aria-hidden="true" />
							{:else}
								<IconSquare size={22} aria-hidden="true" />
							{/if}
						</button>
						<span class="checkbox-label"> I understand and acknowledge the risks involved </span>
					</label>

					{#if isAcknowledged}
						<div class="acknowledgment-confirmed">
							<IconShieldCheck size={16} aria-hidden="true" />
							<span>Risk acknowledged</span>
						</div>
					{/if}
				</div>
			{/if}

			{#if requireAcknowledgment && props.isEditing}
				<div class="acknowledgment-preview">
					<IconSquare size={18} aria-hidden="true" />
					<span>I understand and acknowledge the risks involved</span>
					<em>(checkbox shown in preview)</em>
				</div>
			{/if}
		</div>
	</div>

	<!-- Style Indicator Badge -->
	{#if props.isEditing}
		<div class="style-badge">
			<IconComponent size={14} aria-hidden="true" />
			<span>{style.charAt(0).toUpperCase() + style.slice(1)}</span>
		</div>
	{/if}
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════ -->
<!-- Styles -->
<!-- ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	.risk-disclaimer-block {
		position: relative;
		border-radius: 12px;
		background: var(--bg-gradient);
		border: 2px solid var(--border-color);
		overflow: hidden;
		transition: all 0.3s ease;
	}

	/* Settings Panel */
	.disclaimer-settings {
		padding: 1rem;
		background: rgba(255, 255, 255, 0.9);
		border-bottom: 1px solid rgba(0, 0, 0, 0.1);
	}

	.settings-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 0.875rem;
		padding-bottom: 0.5rem;
		border-bottom: 1px solid rgba(0, 0, 0, 0.1);
		font-size: 0.8125rem;
		font-weight: 600;
		color: #374151;
	}

	.settings-row {
		display: flex;
		flex-wrap: wrap;
		gap: 1rem;
		align-items: flex-start;
		margin-bottom: 0.875rem;
	}

	.settings-row:last-child {
		margin-bottom: 0;
	}

	.setting-field {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
		font-size: 0.8125rem;
	}

	.setting-field span {
		color: #4b5563;
		font-weight: 500;
	}

	.setting-field select,
	.setting-field textarea {
		padding: 0.5rem 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 6px;
		font-size: 0.875rem;
		background: #ffffff;
	}

	.setting-field select {
		min-width: 140px;
	}

	.setting-field textarea {
		resize: vertical;
		min-height: 60px;
		font-family: inherit;
		line-height: 1.5;
	}

	.setting-field.full-width {
		flex: 1;
		min-width: 100%;
	}

	.setting-field.full-width textarea {
		width: 100%;
	}

	.checkbox-field {
		flex-direction: row;
		align-items: center;
		gap: 0.5rem;
		padding-top: 1.5rem;
	}

	.checkbox-field input {
		width: 16px;
		height: 16px;
		cursor: pointer;
	}

	/* Main Disclaimer Container */
	.disclaimer-container {
		display: flex;
		gap: 1rem;
		padding: 1.25rem;
	}

	/* Icon */
	.disclaimer-icon {
		flex-shrink: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		width: 48px;
		height: 48px;
		background: var(--icon-bg);
		border-radius: 12px;
		color: var(--accent-color);
	}

	/* Content Area */
	.disclaimer-content {
		flex: 1;
		min-width: 0;
	}

	.disclaimer-main {
		margin-bottom: 0.75rem;
	}

	.main-text {
		margin: 0;
		font-size: 0.9375rem;
		font-weight: 600;
		line-height: 1.6;
		color: var(--text-color);
	}

	.main-text.editing-preview {
		opacity: 0.6;
		font-style: italic;
	}

	/* Expandable Section */
	.expandable-section {
		margin-top: 0.5rem;
	}

	.expand-toggle {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.75rem;
		background: rgba(0, 0, 0, 0.05);
		border: none;
		border-radius: 6px;
		font-size: 0.8125rem;
		font-weight: 500;
		color: var(--text-color);
		cursor: pointer;
		transition: all 0.15s;
	}

	.expand-toggle:hover {
		background: rgba(0, 0, 0, 0.1);
	}

	.expand-toggle:focus-visible {
		outline: 2px solid var(--accent-color);
		outline-offset: 2px;
	}

	.expanded-content {
		max-height: 0;
		overflow: hidden;
		transition:
			max-height 0.3s ease-out,
			margin-top 0.3s ease-out;
	}

	.expanded-content.visible {
		max-height: 500px;
		margin-top: 0.75rem;
	}

	.expanded-inner {
		padding: 1rem;
		background: rgba(255, 255, 255, 0.6);
		border-radius: 8px;
		border-left: 3px solid var(--accent-color);
	}

	.expanded-inner p {
		margin: 0;
		font-size: 0.875rem;
		line-height: 1.7;
		color: var(--text-color);
		opacity: 0.9;
	}

	/* Acknowledgment Section */
	.acknowledgment-section {
		margin-top: 1rem;
	}

	.divider {
		height: 1px;
		background: rgba(0, 0, 0, 0.1);
		margin-bottom: 1rem;
	}

	.acknowledgment-checkbox {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		cursor: pointer;
		user-select: none;
	}

	.checkbox-button {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0;
		background: transparent;
		border: none;
		color: var(--accent-color);
		cursor: pointer;
		transition: transform 0.2s;
	}

	.checkbox-button:focus-visible {
		outline: 2px solid var(--accent-color);
		outline-offset: 2px;
		border-radius: 4px;
	}

	.acknowledgment-checkbox.animate .checkbox-button {
		animation: checkPulse 0.3s ease-out;
	}

	@keyframes checkPulse {
		0% {
			transform: scale(1);
		}
		50% {
			transform: scale(1.2);
		}
		100% {
			transform: scale(1);
		}
	}

	.checkbox-label {
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--text-color);
	}

	.acknowledgment-checkbox.checked .checkbox-label {
		color: var(--accent-color);
	}

	.acknowledgment-confirmed {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-top: 0.75rem;
		padding: 0.5rem 0.75rem;
		background: rgba(34, 197, 94, 0.15);
		border-radius: 6px;
		font-size: 0.8125rem;
		font-weight: 600;
		color: #16a34a;
		animation: fadeIn 0.3s ease-out;
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
			transform: translateY(-4px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.acknowledgment-preview {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-top: 0.75rem;
		padding-top: 0.75rem;
		border-top: 1px solid rgba(0, 0, 0, 0.1);
		font-size: 0.8125rem;
		color: var(--text-color);
		opacity: 0.7;
	}

	.acknowledgment-preview em {
		font-size: 0.75rem;
		opacity: 0.6;
	}

	/* Style Badge */
	.style-badge {
		position: absolute;
		top: 0.75rem;
		right: 0.75rem;
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.25rem 0.625rem;
		background: var(--icon-bg);
		border-radius: 6px;
		font-size: 0.6875rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		color: var(--accent-color);
	}

	/* Warning Style Specific */
	.style-warning {
		--bg-gradient: linear-gradient(135deg, #fefce8 0%, #fef3c7 100%);
		--border-color: #fbbf24;
		--text-color: #92400e;
		--accent-color: #f59e0b;
		--icon-bg: #fef3c7;
	}

	/* Danger Style Specific */
	.style-danger {
		--bg-gradient: linear-gradient(135deg, #fef2f2 0%, #fee2e2 100%);
		--border-color: #f87171;
		--text-color: #991b1b;
		--accent-color: #ef4444;
		--icon-bg: #fee2e2;
	}

	/* Info Style Specific */
	.style-info {
		--bg-gradient: linear-gradient(135deg, #eff6ff 0%, #dbeafe 100%);
		--border-color: #60a5fa;
		--text-color: #1e40af;
		--accent-color: #3b82f6;
		--icon-bg: #dbeafe;
	}

	/* Dark Mode */
	:global(.dark) .risk-disclaimer-block {
		border-width: 1px;
	}

	:global(.dark) .style-warning {
		--bg-gradient: linear-gradient(135deg, #422006 0%, #1c1917 100%);
		--border-color: #b45309;
		--text-color: #fcd34d;
		--accent-color: #f59e0b;
		--icon-bg: #78350f;
	}

	:global(.dark) .style-danger {
		--bg-gradient: linear-gradient(135deg, #450a0a 0%, #1c1917 100%);
		--border-color: #991b1b;
		--text-color: #fca5a5;
		--accent-color: #ef4444;
		--icon-bg: #7f1d1d;
	}

	:global(.dark) .style-info {
		--bg-gradient: linear-gradient(135deg, #1e3a8a 0%, #1e293b 100%);
		--border-color: #1e40af;
		--text-color: #93c5fd;
		--accent-color: #3b82f6;
		--icon-bg: #1e3a8a;
	}

	:global(.dark) .disclaimer-settings {
		background: rgba(30, 41, 59, 0.95);
		border-color: rgba(255, 255, 255, 0.1);
	}

	:global(.dark) .settings-header {
		color: #e2e8f0;
		border-color: rgba(255, 255, 255, 0.1);
	}

	:global(.dark) .setting-field span {
		color: #94a3b8;
	}

	:global(.dark) .setting-field select,
	:global(.dark) .setting-field textarea {
		background: #0f172a;
		border-color: #475569;
		color: #e2e8f0;
	}

	:global(.dark) .expand-toggle {
		background: rgba(255, 255, 255, 0.1);
	}

	:global(.dark) .expand-toggle:hover {
		background: rgba(255, 255, 255, 0.15);
	}

	:global(.dark) .expanded-inner {
		background: rgba(0, 0, 0, 0.3);
	}

	:global(.dark) .divider {
		background: rgba(255, 255, 255, 0.1);
	}

	:global(.dark) .acknowledgment-confirmed {
		background: rgba(34, 197, 94, 0.2);
		color: #86efac;
	}

	:global(.dark) .acknowledgment-preview {
		border-color: rgba(255, 255, 255, 0.1);
	}

	/* Responsive Adjustments */
	@media (max-width: 640px) {
		.disclaimer-container {
			flex-direction: column;
			gap: 0.75rem;
		}

		.disclaimer-icon {
			width: 40px;
			height: 40px;
		}

		.settings-row {
			flex-direction: column;
		}

		.setting-field.full-width {
			min-width: 0;
		}

		.checkbox-field {
			padding-top: 0;
		}
	}
</style>
