#!/usr/bin/env node
/**
 * Fix {@const Icon = ...} patterns that shadow the Icon import.
 * Rename to {@const iconName = ...} and update the corresponding <Icon icon={Icon} to <Icon icon={iconName}
 * Also fix remaining type issues:
 * - icon: ComponentType → icon: string
 * - Remove unused ComponentType/Component imports
 * - Fix duplicate icon identifier declarations
 */

import { readFileSync, writeFileSync, readdirSync, statSync } from 'fs';
import { join, relative } from 'path';

const SRC_DIR = join(process.cwd(), 'src');

function findFiles(dir, files = []) {
	for (const entry of readdirSync(dir)) {
		const fullPath = join(dir, entry);
		const stat = statSync(fullPath);
		if (stat.isDirectory() && !entry.startsWith('.') && entry !== 'node_modules') {
			findFiles(fullPath, files);
		} else if (entry.endsWith('.svelte') || entry.endsWith('.ts') || entry.endsWith('.js')) {
			files.push(fullPath);
		}
	}
	return files;
}

function fixFile(filePath) {
	let content = readFileSync(filePath, 'utf-8');
	const original = content;

	if (filePath.endsWith('lib/icons/index.ts') || filePath.endsWith('lib/icons/Icon.svelte')) {
		return false;
	}

	// 1. Fix {@const Icon = expr} → {@const iconStr = expr}
	//    Then fix <Icon icon={Icon} → <Icon icon={iconStr}
	//    Also fix <Icon icon={IconComponent} patterns from earlier scripts
	
	// Replace all {@const Icon = ...} with {@const iconStr = ...}
	content = content.replace(/\{@const Icon = ([^}]+)\}/g, '{@const iconStr = $1}');
	
	// Replace <Icon icon={Icon} with <Icon icon={iconStr}  (only when it was the shadowed variable)
	// Also <Icon icon={Icon} size= etc
	content = content.replace(/<Icon icon=\{Icon\}/g, '<Icon icon={iconStr}');
	
	// Replace {@const IconComponent = ...} → {@const iconStr = ...}
	content = content.replace(/\{@const IconComponent = ([^}]+)\}/g, '{@const iconStr = $1}');
	content = content.replace(/<Icon icon=\{IconComponent\}/g, '<Icon icon={iconStr}');
	
	// Replace {@const SvelteComponent = ...} → {@const iconStr = ...}
	content = content.replace(/\{@const SvelteComponent = ([^}]+)\}/g, '{@const iconStr = $1}');
	content = content.replace(/<Icon icon=\{SvelteComponent\}/g, '<Icon icon={iconStr}');
	
	// Replace {@const SvelteComponent_1 = ...} → {@const iconStr2 = ...}
	content = content.replace(/\{@const SvelteComponent_1 = ([^}]+)\}/g, '{@const iconStr2 = $1}');
	content = content.replace(/<Icon icon=\{SvelteComponent_1\}/g, '<Icon icon={iconStr2}');

	// 2. Fix icon?: ComponentType → icon?: string (and icon: ComponentType)
	content = content.replace(/icon\??\s*:\s*ComponentType\b/g, (match) => {
		return match.replace('ComponentType', 'string');
	});
	
	// 3. Remove now-unused ComponentType import
	if (content.includes('ComponentType')) {
		// Check if ComponentType is actually used outside of comments
		const withoutComments = content.replace(/\/\/.*$/gm, '').replace(/\/\*[\s\S]*?\*\//g, '');
		const usageCount = (withoutComments.match(/ComponentType/g) || []).length;
		const importCount = (withoutComments.match(/import.*ComponentType/g) || []).length;
		if (usageCount <= importCount) {
			// Only in import, not used
			content = content.replace(/\timport type \{ ComponentType \} from 'svelte';\n/g, '');
			content = content.replace(/, ComponentType\b/g, '');
			content = content.replace(/ComponentType, /g, '');
			// Handle single import: import type { ComponentType } from 'svelte';
			content = content.replace(/import type \{ \} from 'svelte';\n/g, '');
		}
	}

	// 4. Fix duplicate identifier errors - IconBrandBing etc
	// Find lines that re-declare icons that are already in the $lib/icons import
	const iconsImportMatch = content.match(/import\s*\{([^}]+)\}\s*from\s*['"]\$lib\/icons['"]/);
	if (iconsImportMatch) {
		const importedNames = new Set(iconsImportMatch[1].split(',').map(s => s.trim()).filter(Boolean));
		// Find const declarations that duplicate these
		for (const name of importedNames) {
			// Remove duplicate const declarations like: const IconBrandBing = 'tabler:brand-bing';
			const dupeRegex = new RegExp(`\\n\\tconst ${name} = '[^']+';`, 'g');
			content = content.replace(dupeRegex, '');
		}
	}

	if (content !== original) {
		writeFileSync(filePath, content);
		return true;
	}
	return false;
}

const files = findFiles(SRC_DIR);
let count = 0;
const fixed = [];

for (const f of files) {
	if (fixFile(f)) {
		count++;
		fixed.push(relative(SRC_DIR, f));
	}
}

console.log(`Fixed ${count} files`);
for (const f of fixed) console.log(`  ${f}`);
