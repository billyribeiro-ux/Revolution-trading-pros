/**
 * CMS v2 Editor Components
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Advanced editor components for the CMS v2 system.
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 * @since January 2026
 */

// Components
export { default as AIAssistMenu } from './AIAssistMenu.svelte';
export { default as SlashCommands } from './SlashCommands.svelte';
export { default as FocusMode } from './FocusMode.svelte';
export { default as KeyboardShortcutsModal } from './KeyboardShortcutsModal.svelte';
export { default as RevisionDiffView } from './RevisionDiffView.svelte';
export { default as BlockGroup } from './BlockGroup.svelte';
export { default as ConflictResolutionModal } from './ConflictResolutionModal.svelte';
export { default as EditorToolbar } from './EditorToolbar.svelte';
export { default as MediaDropZone } from './MediaDropZone.svelte';
export { default as CommentsSidebar } from './CommentsSidebar.svelte';
export { default as InlineComment } from './InlineComment.svelte';

// Types
export type { SlashCommand, CommandCategory } from './SlashCommands.svelte';
export type { Revision } from './RevisionDiffView.svelte';
export type {
	LayoutType,
	PaddingSize,
	GapSize,
	BorderRadiusSize,
	GroupBlockData,
	Block,
	GroupBlock,
	ChildRenderProps
} from './BlockGroup.svelte';
export type { ContentData, ResolutionType } from './ConflictResolutionModal.svelte';
