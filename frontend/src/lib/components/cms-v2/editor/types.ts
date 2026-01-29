/**
 * CMS v2 Editor Types
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Shared type definitions for the CMS v2 editor system.
 * These types are exported from this file to ensure proper TypeScript support
 * when importing from the index.ts barrel file.
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 * @since January 2026
 */

import type { PageBlock } from '$lib/page-builder/types';

// ==========================================================================
// SlashCommands Types
// ==========================================================================

export type CommandCategory = 'formatting' | 'blocks' | 'trading' | 'embeds' | 'actions';

export interface SlashCommand {
	id: string;
	label: string;
	description: string;
	category: CommandCategory;
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	icon: any;
	keywords: string[];
	shortcut?: string;
}

// ==========================================================================
// RevisionDiffView Types
// ==========================================================================

interface RevisionData {
	title?: string;
	content?: string;
	content_blocks?: PageBlock[];
	[key: string]: unknown;
}

export interface Revision {
	id: string;
	revision_number: number;
	created_at: string;
	created_by: string | null;
	change_summary: string | null;
	changed_fields: string[] | null;
	data?: RevisionData;
}

// ==========================================================================
// BlockGroup Types
// ==========================================================================

export type LayoutType = 'stack' | 'columns';
export type PaddingSize = 'none' | 'sm' | 'md' | 'lg' | 'xl';
export type GapSize = 'none' | 'sm' | 'md' | 'lg' | 'xl';
export type BorderRadiusSize = 'none' | 'sm' | 'md' | 'lg' | 'xl' | 'full';

export interface GroupBlockData {
	layout: LayoutType;
	columns: 1 | 2 | 3 | 4 | 'auto';
	gap: GapSize;
	backgroundColor?: string;
	backgroundImage?: string;
	backgroundOverlay?: string;
	padding: PaddingSize;
	borderRadius: BorderRadiusSize;
	[key: string]: unknown;
}

export interface Block {
	id: string;
	blockType: string;
	data: Record<string, unknown>;
	children?: Block[];
}

export interface GroupBlock extends Block {
	blockType: 'group';
	data: GroupBlockData;
	children: Block[];
}

export interface ChildRenderProps {
	block: Block;
	isNested: boolean;
	nestingLevel: number;
	onUpdate: (updated: Block) => void;
}

// ==========================================================================
// ConflictResolutionModal Types
// ==========================================================================

export interface ContentData {
	id: string;
	title: string;
	slug?: string;
	content?: string;
	excerpt?: string;
	updatedAt: string;
	version?: number;
	status?: string;
	[key: string]: unknown;
}

export type ResolutionType = 'mine' | 'theirs' | 'merge';
