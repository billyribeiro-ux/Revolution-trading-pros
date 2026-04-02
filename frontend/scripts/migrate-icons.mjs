#!/usr/bin/env node
/**
 * Icon Migration Script: @tabler/icons-svelte-runes → @iconify/svelte
 * 
 * Phase 1: Rewrite $lib/icons/index.ts to export string constants
 * Phase 2: Update all consumer files to use <Icon icon={IconXxx} /> pattern
 */

import { readFileSync, writeFileSync, readdirSync, statSync } from 'fs';
import { join, relative } from 'path';

const SRC_DIR = join(process.cwd(), 'src');
const ICONS_INDEX = join(SRC_DIR, 'lib/icons/index.ts');

// ============================================================================
// Phase 1: Generate new index.ts with string constants
// ============================================================================

function generateNewBarrel() {
	const content = readFileSync(ICONS_INDEX, 'utf-8');
	const lines = content.split('\n');

	// Parse all tabler imports: import IconFoo from '@tabler/icons-svelte-runes/icons/foo-bar'
	const tablerImports = [];
	const localImports = [];
	const aliases = []; // e.g. IconArrowForwardUp as IconArrowForward

	for (const line of lines) {
		const tablerMatch = line.match(/^import\s+(\w+)\s+from\s+'@tabler\/icons-svelte-runes\/icons\/(.+?)';?$/);
		if (tablerMatch) {
			tablerImports.push({ name: tablerMatch[1], slug: tablerMatch[2] });
			continue;
		}
		const localMatch = line.match(/^import\s+(\w+)\s+from\s+'\.\/(.+?)';?$/);
		if (localMatch) {
			localImports.push({ name: localMatch[1], path: localMatch[2] });
		}
	}

	// Parse aliases from the export block
	const exportBlock = content.match(/export\s*\{[\s\S]*?\};/);
	if (exportBlock) {
		const aliasMatches = exportBlock[0].matchAll(/(\w+)\s+as\s+(\w+)/g);
		for (const m of aliasMatches) {
			aliases.push({ from: m[1], to: m[2] });
		}
	}

	// Generate new barrel file
	let output = `/**
 * Revolution Trading Pros - Icon System (Iconify)
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * All icons are string constants for use with the Icon component.
 * Icons use the Tabler icon set via @iconify/svelte.
 *
 * Usage:
 * import { Icon, IconUser, IconSettings } from '$lib/icons';
 * <Icon icon={IconUser} size={24} />
 */

// Icon wrapper component
export { default as Icon } from './Icon.svelte';

// =============================================================================
// Tabler Icon Constants
// =============================================================================

`;

	// Export string constants for each tabler icon
	for (const { name, slug } of tablerImports) {
		output += `export const ${name} = 'tabler:${slug}';\n`;
	}

	// Export aliases
	if (aliases.length > 0) {
		output += `\n// Aliases\n`;
		for (const { from, to } of aliases) {
			output += `export const ${to} = ${from};\n`;
		}
	}

	// Local SVG imports
	if (localImports.length > 0) {
		output += `\n// =============================================================================\n`;
		output += `// Custom Local SVG Icons (lightweight alternatives)\n`;
		output += `// =============================================================================\n\n`;
		for (const { name, path } of localImports) {
			output += `export { default as ${name} } from './${path}';\n`;
		}
	}

	// Type export
	output += `\nimport type { Component } from 'svelte';\n\n`;
	output += `/**\n * Type-safe icon component type\n * Use this for props that accept icon components\n */\n`;
	output += `export type IconComponent = Component<{ size?: number; color?: string; stroke?: number; class?: string }>;\n`;

	writeFileSync(ICONS_INDEX, output);
	console.log(`✅ Rewrote ${ICONS_INDEX}`);
	console.log(`   ${tablerImports.length} tabler icons → string constants`);
	console.log(`   ${localImports.length} local SVG icons preserved`);
	console.log(`   ${aliases.length} aliases preserved`);
}

// ============================================================================
// Phase 2: Migrate consumer files
// ============================================================================

function findSvelteAndTsFiles(dir, files = []) {
	for (const entry of readdirSync(dir)) {
		const fullPath = join(dir, entry);
		const stat = statSync(fullPath);
		if (stat.isDirectory() && !entry.startsWith('.') && entry !== 'node_modules') {
			findSvelteAndTsFiles(fullPath, files);
		} else if (entry.endsWith('.svelte') || entry.endsWith('.ts') || entry.endsWith('.js')) {
			files.push(fullPath);
		}
	}
	return files;
}

function migrateConsumerFile(filePath) {
	let content = readFileSync(filePath, 'utf-8');

	// Skip the barrel file itself and the Icon.svelte wrapper
	if (filePath.endsWith('lib/icons/index.ts') || filePath.endsWith('lib/icons/Icon.svelte')) {
		return false;
	}

	// Check if file imports from $lib/icons
	if (!content.includes("from '$lib/icons'") && !content.includes('from "$lib/icons"')) {
		return false;
	}

	let modified = false;

	// Step 1: Add Icon to the import if not already there
	// Match: import { IconFoo, IconBar } from '$lib/icons';
	const importRegex = /import\s*\{([^}]+)\}\s*from\s*['"](\$lib\/icons)['"]/g;
	content = content.replace(importRegex, (match, imports, path) => {
		const importList = imports.split(',').map(s => s.trim()).filter(Boolean);

		// Check if Icon is already imported
		const hasIcon = importList.some(i => i === 'Icon' || i.startsWith('Icon '));
		// Check if there are any tabler icon imports (Icon* but not just 'Icon')
		const hasTablerIcons = importList.some(i => /^Icon[A-Z]/.test(i));

		if (hasTablerIcons && !hasIcon) {
			importList.unshift('Icon');
			modified = true;
		}

		return `import { ${importList.join(', ')} } from '${path}'`;
	});

	// Step 2: Replace <IconXxx with <Icon icon={IconXxx} in templates
	// But NOT for local SVG icons (IconPlusLocal, IconXLocal, etc.)
	// And NOT for the Icon component itself
	const localIcons = [
		'IconPlusLocal', 'IconXLocal', 'IconChevronDownLocal', 'IconChevronLeftLocal',
		'IconChevronRightLocal', 'IconCheckLocal', 'IconMinusLocal', 'IconPhotoLocal',
		'IconVideoLocal', 'IconCodeLocal', 'IconListLocal', 'IconPlayerPlayLocal',
		'IconPlayerPauseLocal', 'IconVolumeLocal', 'IconVolumeOffLocal', 'IconMaximizeLocal',
		'IconCopyLocal', 'IconLinkLocal', 'IconLayoutGridLocal', 'IconColumnsLocal',
		'IconStarLocal', 'IconCircleLocal', 'IconSquareLocal', 'IconSparklesLocal'
	];

	// Replace <IconXxx (not followed by Local) with <Icon icon={IconXxx}
	// Pattern: <IconSomething followed by space, /, or >
	content = content.replace(/<(Icon[A-Z]\w*)([\s/>])/g, (match, iconName, after) => {
		// Skip local SVG icons
		if (localIcons.includes(iconName)) return match;
		// Skip if it's just "Icon" (our wrapper component)
		if (iconName === 'Icon') return match;
		// Skip IconComponent (it's a variable, not usage)
		if (iconName === 'IconComponent') return match;

		modified = true;
		return `<Icon icon={${iconName}}${after}`;
	});

	// Step 3: Fix closing tags - </IconXxx> → </Icon>
	content = content.replace(/<\/(Icon[A-Z]\w*)>/g, (match, iconName) => {
		if (localIcons.includes(iconName)) return match;
		if (iconName === 'Icon') return match;
		if (iconName === 'IconComponent') return match;
		modified = true;
		return '</Icon>';
	});

	if (modified) {
		writeFileSync(filePath, content);
		return true;
	}
	return false;
}

function migrateAllConsumers() {
	const files = findSvelteAndTsFiles(SRC_DIR);
	let count = 0;
	const migrated = [];

	for (const f of files) {
		if (migrateConsumerFile(f)) {
			count++;
			migrated.push(relative(SRC_DIR, f));
		}
	}

	console.log(`\n✅ Migrated ${count} consumer files`);
	if (migrated.length <= 30) {
		for (const f of migrated) console.log(`   ${f}`);
	} else {
		for (const f of migrated.slice(0, 20)) console.log(`   ${f}`);
		console.log(`   ... and ${migrated.length - 20} more`);
	}
}

// ============================================================================
// Run
// ============================================================================

console.log('🔄 Phase 1: Rewriting $lib/icons/index.ts...');
generateNewBarrel();

console.log('\n🔄 Phase 2: Migrating consumer files...');
migrateAllConsumers();

console.log('\n✅ Migration complete!');
