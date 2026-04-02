#!/usr/bin/env node
/**
 * Fix Svelte 5 `state_referenced_locally` warnings.
 * Pattern: $state(props.xxx ?? default) → $state(default)
 * These files already have $effect() blocks to sync from props.
 * 
 * Also fixes non-reactive props references passed to function calls
 * by wrapping them in $derived or $effect patterns.
 */

import { readFileSync, writeFileSync } from 'fs';
import { join } from 'path';

const SRC = join(process.cwd(), 'src');

// Files with $state(props.xxx ?? default) pattern
const STATE_PROP_FILES = [
	// card/*
	'lib/components/ui/card/card.svelte',
	'lib/components/ui/card/card-action.svelte',
	'lib/components/ui/card/card-content.svelte',
	'lib/components/ui/card/card-description.svelte',
	'lib/components/ui/card/card-footer.svelte',
	'lib/components/ui/card/card-header.svelte',
	'lib/components/ui/card/card-title.svelte',
	// dialog/*
	'lib/components/ui/dialog/dialog.svelte',
	'lib/components/ui/dialog/dialog-close.svelte',
	'lib/components/ui/dialog/dialog-content.svelte',
	'lib/components/ui/dialog/dialog-description.svelte',
	'lib/components/ui/dialog/dialog-footer.svelte',
	'lib/components/ui/dialog/dialog-header.svelte',
	'lib/components/ui/dialog/dialog-overlay.svelte',
	'lib/components/ui/dialog/dialog-title.svelte',
	'lib/components/ui/dialog/dialog-trigger.svelte',
	// dropdown-menu/*
	'lib/components/ui/dropdown-menu/dropdown-menu.svelte',
	'lib/components/ui/dropdown-menu/dropdown-menu-checkbox-group.svelte',
	'lib/components/ui/dropdown-menu/dropdown-menu-checkbox-item.svelte',
	'lib/components/ui/dropdown-menu/dropdown-menu-content.svelte',
	'lib/components/ui/dropdown-menu/dropdown-menu-group.svelte',
	'lib/components/ui/dropdown-menu/dropdown-menu-group-heading.svelte',
	'lib/components/ui/dropdown-menu/dropdown-menu-item.svelte',
	'lib/components/ui/dropdown-menu/dropdown-menu-label.svelte',
	'lib/components/ui/dropdown-menu/dropdown-menu-radio-group.svelte',
	'lib/components/ui/dropdown-menu/dropdown-menu-radio-item.svelte',
	'lib/components/ui/dropdown-menu/dropdown-menu-separator.svelte',
	'lib/components/ui/dropdown-menu/dropdown-menu-shortcut.svelte',
	'lib/components/ui/dropdown-menu/dropdown-menu-sub.svelte',
	'lib/components/ui/dropdown-menu/dropdown-menu-sub-content.svelte',
	'lib/components/ui/dropdown-menu/dropdown-menu-sub-trigger.svelte',
	'lib/components/ui/dropdown-menu/dropdown-menu-trigger.svelte',
	// select/*
	'lib/components/ui/select/select.svelte',
	'lib/components/ui/select/select-trigger.svelte',
	// table/*
	'lib/components/ui/table/table.svelte',
	'lib/components/ui/table/table-row.svelte',
	// Modal
	'lib/components/ui/Modal.svelte',
	// MobileResponsiveTable
	'lib/components/ui/MobileResponsiveTable.svelte',
];

let count = 0;

for (const relPath of STATE_PROP_FILES) {
	const filePath = join(SRC, relPath);
	let content;
	try {
		content = readFileSync(filePath, 'utf-8');
	} catch {
		console.log(`  SKIP (not found): ${relPath}`);
		continue;
	}
	const original = content;

	// Pattern 1: $state(props.xxx ?? default) → $state(default)
	// Matches: $state(props.ref ?? null), $state(props.open ?? false), $state<type>(props.xxx ?? val)
	content = content.replace(
		/\$state(<[^>]+>)?\(props\.\w+\s*\?\?\s*([^)]+)\)/g,
		(match, generic, defaultVal) => {
			return `$state${generic || ''}(${defaultVal.trim()})`;
		}
	);

	// Pattern 2: $state(props.xxx) without ?? (just capturing props value)
	// e.g., $state(props.sortBy) — more rare but can happen
	// Don't replace if it's inside $effect or $derived
	
	if (content !== original) {
		writeFileSync(filePath, content);
		count++;
		console.log(`  Fixed: ${relPath}`);
	}
}

// Special files that need different treatment
// AudioBlock: props.blockId and props.onError passed to function call
const audioPath = join(SRC, 'lib/components/cms/blocks/media/AudioBlock.svelte');
try {
	let audio = readFileSync(audioPath, 'utf-8');
	const audioOrig = audio;
	
	// Wrap the useMediaControls call in a reactive pattern
	// Change: const controls = useMediaControls({ blockId: props.blockId, onError: props.onError, ... })
	// To: use $derived for blockId and onError, then pass them
	audio = audio.replace(
		/const controls = useMediaControls\(\{\n\s*blockId: props\.blockId,\n\s*onError: props\.onError,/,
		'const controls = useMediaControls({\n\t\tget blockId() { return props.blockId; },\n\t\tget onError() { return props.onError; },'
	);
	
	if (audio !== audioOrig) {
		writeFileSync(audioPath, audio);
		count++;
		console.log('  Fixed: AudioBlock.svelte');
	}
} catch { /* skip */ }

// VideoBlock
const videoPath = join(SRC, 'lib/components/cms/blocks/media/VideoBlock.svelte');
try {
	let video = readFileSync(videoPath, 'utf-8');
	const videoOrig = video;
	// Similar pattern - find $state(props.xxx) 
	video = video.replace(
		/\$state(<[^>]+>)?\(props\.\w+\s*\?\?\s*([^)]+)\)/g,
		(match, generic, defaultVal) => `$state${generic || ''}(${defaultVal.trim()})`
	);
	if (video !== videoOrig) {
		writeFileSync(videoPath, video);
		count++;
		console.log('  Fixed: VideoBlock.svelte');
	}
} catch { /* skip */ }

// TickerBlock
const tickerPath = join(SRC, 'lib/components/cms/blocks/trading/TickerBlock.svelte');
try {
	let ticker = readFileSync(tickerPath, 'utf-8');
	const tickerOrig = ticker;
	ticker = ticker.replace(
		/\$state(<[^>]+>)?\(props\.\w+\s*\?\?\s*([^)]+)\)/g,
		(match, generic, defaultVal) => `$state${generic || ''}(${defaultVal.trim()})`
	);
	if (ticker !== tickerOrig) {
		writeFileSync(tickerPath, ticker);
		count++;
		console.log('  Fixed: TickerBlock.svelte');
	}
} catch { /* skip */ }

console.log(`\nFixed ${count} files total`);
