#!/usr/bin/env node
/**
 * Final icon migration cleanup:
 * 1. Fix duplicate icon attributes: <Icon icon={Icon} icon={IconFoo} → <Icon icon={IconFoo}
 * 2. Add missing Icon imports to files that use <Icon> in template
 * 3. Fix leftover IconifyIcon references
 * 4. Fix broken "import {\nimport {" patterns (remaining)
 * 5. Clean up empty tab-only lines from stripped imports
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

	// === Fix 1: Duplicate icon attributes ===
	// <Icon icon={Icon} icon={IconFoo} → <Icon icon={IconFoo}
	content = content.replace(/icon=\{Icon\}\s+icon=/g, 'icon=');
	
	// === Fix 2: Broken import blocks - import { followed by import { ===
	// Pattern: \timport {\n\timport { ... } from '$lib/icons';\n
	const brokenImportRegex = /(\timport\s*\{)\n(\timport\s*\{[^}]+\}\s*from\s*['"]\$lib\/icons['"];?\n)/g;
	if (brokenImportRegex.test(content)) {
		content = content.replace(brokenImportRegex, (match, openImport, iconsImport) => {
			return iconsImport + openImport + '\n';
		});
	}

	// === Fix 3: Replace IconifyIcon with Icon from $lib/icons ===
	if (content.includes('IconifyIcon') && !filePath.endsWith('Icon.svelte')) {
		content = content.replace(/<IconifyIcon\b/g, '<Icon');
		content = content.replace(/<\/IconifyIcon>/g, '</Icon>');
		// Remove standalone IconifyIcon imports (already handled by fix-icon-patterns)
		content = content.replace(/\timport\s+IconifyIcon\s+from\s+'@iconify\/svelte';\n/g, '');
	}

	// === Fix 4: Clean up empty whitespace lines from stripped imports ===
	content = content.replace(/\n(\t{5,}\n){1,}/g, '\n');
	
	// === Fix 5: Ensure Icon is imported in files that use it ===
	const scriptEndIdx = content.indexOf('</script>');
	if (scriptEndIdx > -1) {
		const scriptContent = content.substring(0, scriptEndIdx);
		const templateContent = content.substring(scriptEndIdx);
		
		const usesIconInTemplate = templateContent.includes('<Icon ') || templateContent.includes('<Icon\n');
		
		if (usesIconInTemplate) {
			// Check if Icon is in any $lib/icons import
			const iconsImportMatch = scriptContent.match(/import\s*\{([^}]+)\}\s*from\s*['"]\$lib\/icons['"]/);
			
			if (iconsImportMatch) {
				const importedNames = iconsImportMatch[1].split(',').map(s => s.trim()).filter(Boolean);
				if (!importedNames.includes('Icon')) {
					importedNames.unshift('Icon');
					const newImport = `import { ${importedNames.join(', ')} } from '$lib/icons'`;
					content = content.replace(iconsImportMatch[0], newImport);
				}
			} else if (!scriptContent.includes("import { Icon }") && 
			           !scriptContent.includes("import { Icon,") &&
			           !scriptContent.includes("import Icon from")) {
				// No $lib/icons import at all, add one
				// Find a good insertion point after the last import
				const importLines = scriptContent.split('\n').filter(l => l.trim().startsWith('import '));
				if (importLines.length > 0) {
					const lastImport = importLines[importLines.length - 1];
					const lastImportIdx = content.lastIndexOf(lastImport);
					// Handle multi-line imports
					let insertAfter = content.indexOf('\n', lastImportIdx);
					// Check if next line is part of multi-line import (starts with tab but not 'import')
					let nextLine = content.substring(insertAfter + 1, content.indexOf('\n', insertAfter + 1));
					while (nextLine && (nextLine.match(/^\t+[^i\n]/) || nextLine.match(/^\t+\}/))) {
						// Part of multi-line import or closing brace
						if (nextLine.includes("} from")) {
							insertAfter = content.indexOf('\n', insertAfter + 1);
							break;
						}
						insertAfter = content.indexOf('\n', insertAfter + 1);
						nextLine = content.substring(insertAfter + 1, content.indexOf('\n', insertAfter + 1));
					}
					content = content.substring(0, insertAfter + 1) + 
					          "\timport { Icon } from '$lib/icons';\n" + 
					          content.substring(insertAfter + 1);
				}
			}
		}
	}

	// === Fix 6: Fix "Cannot find name" for specific icons that should be imported ===
	// Find all Icon* names used in the file but not imported
	const allIconUsages = new Set();
	const iconUsageRegex = /\bIcon([A-Z]\w*)\b/g;
	let m;
	while ((m = iconUsageRegex.exec(content)) !== null) {
		allIconUsages.add('Icon' + m[1]);
	}
	
	// Get currently imported icons from $lib/icons
	const currentImportMatch = content.match(/import\s*\{([^}]+)\}\s*from\s*['"]\$lib\/icons['"]/);
	const currentlyImported = new Set();
	if (currentImportMatch) {
		for (const name of currentImportMatch[1].split(',').map(s => s.trim()).filter(Boolean)) {
			currentlyImported.add(name);
		}
	}
	
	// Find missing icons (used but not imported)
	// Only consider icons that are string constants in the barrel
	const barrelContent = readFileSync(join(SRC_DIR, 'lib/icons/index.ts'), 'utf-8');
	const barrelExports = new Set();
	const exportRegex = /export const (Icon\w+)/g;
	while ((m = exportRegex.exec(barrelContent)) !== null) {
		barrelExports.add(m[1]);
	}
	// Also add local SVG exports
	const localExportRegex = /export \{ default as (Icon\w+)/g;
	while ((m = localExportRegex.exec(barrelContent)) !== null) {
		barrelExports.add(m[1]);
	}
	
	const missingIcons = [];
	for (const iconName of allIconUsages) {
		if (!currentlyImported.has(iconName) && barrelExports.has(iconName)) {
			// Check if it's actually used in template/script (not just in a comment)
			if (content.includes(`{${iconName}}`) || content.includes(`{${iconName},`) ||
			    content.includes(`${iconName},`) || content.includes(`${iconName} `) ||
			    content.includes(`= ${iconName}`) || content.includes(`|| ${iconName}`) ||
			    content.includes(`? ${iconName}`) || content.includes(`: ${iconName}`)) {
				missingIcons.push(iconName);
			}
		}
	}
	
	if (missingIcons.length > 0 && currentImportMatch) {
		const allImports = [...currentlyImported, ...missingIcons];
		const newImport = `import { ${allImports.join(', ')} } from '$lib/icons'`;
		content = content.replace(currentImportMatch[0], newImport);
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

console.log(`\nFixed ${count} files`);
for (const f of fixed.slice(0, 50)) console.log(`  ${f}`);
if (fixed.length > 50) console.log(`  ... and ${fixed.length - 50} more`);
