/**
 * VirtualBlockList Types
 * ═══════════════════════════════════════════════════════════════════════════
 * Type definitions for the VirtualBlockList component
 */

import type { Block } from './types';

export interface VirtualBlockListProps {
	blocks: Block[];
	selectedBlockId: string | null;
	onSelectBlock: (id: string) => void;
	onUpdateBlock: (id: string, updates: Partial<Block>) => void;
	onDeleteBlock: (id: string) => void;
	onMoveBlock: (fromIndex: number, toIndex: number) => void;
	isEditing: boolean;
}

export interface VisibleRange {
	start: number;
	end: number;
}

export interface BlockMeasurement {
	height: number;
	measured: boolean;
}

export interface PerformanceMetrics {
	renderTime: number;
	visibleCount: number;
	totalBlocks: number;
	measuredBlocks: number;
	scrollPosition: number;
	fps: number;
}
