#!/usr/bin/env node
/**
 * Comprehensive import fixer:
 * 1. Extract ALL $lib/icons imports from wherever they are (even inside other import blocks)
 * 2. Merge them into a single clean import
 * 3. Place it in the correct position
 * 4. Fix lines with excessive tabs (stripped imports)
 * 5. Fix duplicate identifier issues
 * 6. Add missing Icon import where <Icon> is used in template
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

	// Skip core icon files
	if (filePath.endsWith('lib/icons/index.ts') || filePath.endsWith('lib/icons/Icon.svelte')) {
		return false;
	}

	// Step 1: Find ALL $lib/icons import lines and extract their icon names
	const allIconNames = new Set();
	const iconImportRegex = /\t?import\s*\{([^}]+)\}\s*from\s*['"]\$lib\/icons['"];?\n?/g;
	let match;
	const importLinesToRemove = [];
	
	while ((match = iconImportRegex.exec(content)) !== null) {
		importLinesToRemove.push(match[0]);
		const names = match[1].split(',').map(s => s.trim()).filter(Boolean);
		for (const name of names) {
			allIconNames.add(name);
		}
	}

	if (importLinesToRemove.length === 0) {
		// No $lib/icons imports - just clean up tab lines
		const cleaned = content.replace(/\n\t{5,}\n/g, '\n').replace(/\t{10,}import/g, '\timport');
		if (cleaned !== original) {
			writeFileSync(filePath, cleaned);
			return true;
		}
		return false;
	}

	// Step 2: Remove all $lib/icons import lines from their current positions
	for (const line of importLinesToRemove) {
		content = content.replace(line, '');
	}

	// Step 3: Clean up excessive whitespace/tabs from stripped imports
	content = content.replace(/\t{10,}import/g, '\timport');
	content = content.replace(/\n\t{5,}\n/g, '\n');
	content = content.replace(/\n{3,}/g, '\n\n');

	// Step 4: Check if <Icon> is used in template and ensure Icon is in the set
	const scriptEndIdx = content.indexOf('</script>');
	if (scriptEndIdx > -1) {
		const templateContent = content.substring(scriptEndIdx);
		if (templateContent.includes('<Icon ') || templateContent.includes('<Icon\n') || templateContent.includes('<Icon\t')) {
			allIconNames.add('Icon');
		}
	}

	// Step 5: Create clean import line
	// Sort: Icon first, then alphabetically
	const sortedNames = [...allIconNames].sort((a, b) => {
		if (a === 'Icon') return -1;
		if (b === 'Icon') return 1;
		return a.localeCompare(b);
	});

	const cleanImport = `\timport { ${sortedNames.join(', ')} } from '$lib/icons';\n`;

	// Step 6: Find the right insertion point - after the last clean import line
	// Parse script content to find proper insertion point
	const lines = content.split('\n');
	let insertLineIdx = -1;
	let inMultiLineImport = false;
	
	for (let i = 0; i < lines.length; i++) {
		const line = lines[i];
		if (line.includes('</script>')) break;
		
		if (line.trim().startsWith('import ') || line.trim().startsWith('import\t')) {
			// Check if it's a single-line import
			if (line.includes(' from ') && (line.includes("';") || line.includes('";') || line.includes("'") || line.includes('"'))) {
				insertLineIdx = i;
				inMultiLineImport = false;
			} else {
				inMultiLineImport = true;
			}
		} else if (inMultiLineImport && line.includes('} from ')) {
			insertLineIdx = i;
			inMultiLineImport = false;
		}
	}

	if (insertLineIdx === -1) {
		// No imports found, insert at start of script
		const scriptStart = content.indexOf('<script');
		const scriptTagEnd = content.indexOf('>', scriptStart) + 1;
		content = content.substring(0, scriptTagEnd) + '\n' + cleanImport + content.substring(scriptTagEnd);
	} else {
		// Insert after the last import line
		lines.splice(insertLineIdx + 1, 0, cleanImport.trimEnd());
		content = lines.join('\n');
	}

	// Step 7: Fix any remaining duplicate icon={Icon} icon={IconFoo} patterns
	content = content.replace(/icon=\{Icon\}\s+icon=/g, 'icon=');

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
for (const f of fixed.slice(0, 50)) console.log(`  ${f}`);
if (fixed.length > 50) console.log(`  ... and ${fixed.length - 50} more`);
