#!/usr/bin/env node
/**
 * Fix broken imports caused by the direct-tabler migration script.
 * The script inserted $lib/icons imports inside multi-line import blocks.
 * 
 * Pattern to fix:
 *   import {
 *   import { Icon, IconFoo } from '$lib/icons';
 *       something,
 *       something2
 *   } from 'some/module';
 * 
 * Should become:
 *   import { Icon, IconFoo } from '$lib/icons';
 *   import {
 *       something,
 *       something2
 *   } from 'some/module';
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

	// Pattern 1: import {\n\timport { Icon, ... } from '$lib/icons';\n\t\treal imports...
	// The $lib/icons line was inserted inside a multi-line import block
	const brokenPattern = /(\timport\s*\{)\n(\timport\s*\{[^}]+\}\s*from\s*['"]\$lib\/icons['"];?\n)/g;
	
	if (brokenPattern.test(content)) {
		content = content.replace(brokenPattern, (match, openImport, iconsImport) => {
			// Move the icons import BEFORE the original import statement
			return iconsImport + openImport + '\n';
		});
	}

	// Pattern 2: Lines that are just tabs/whitespace (leftover from stripped tabler imports)
	// Multiple consecutive lines of only whitespace/tabs
	content = content.replace(/(\n\t{5,}\n){2,}/g, '\n');
	// Single lines of just many tabs
	content = content.replace(/\n\t{10,}\n/g, '\n');

	// Pattern 3: Ensure files that use <Icon in template have Icon imported
	// Check if file uses <Icon in template but doesn't have Icon in imports
	const templateSection = content.split('</script>').slice(1).join('</script>');
	const scriptSection = content.split('</script>')[0] || '';
	
	if (templateSection && templateSection.includes('<Icon ') && 
	    !scriptSection.includes("import { Icon") && 
	    !scriptSection.includes("import {Icon") &&
	    !scriptSection.includes(", Icon,") &&
	    !scriptSection.includes(", Icon }") &&
	    !scriptSection.includes("{ Icon,") &&
	    !scriptSection.includes("{ Icon }")) {
		
		// Check if there's an existing $lib/icons import to add Icon to
		const existingImport = content.match(/import\s*\{([^}]+)\}\s*from\s*['"]\$lib\/icons['"]/);
		if (existingImport) {
			const icons = existingImport[1].split(',').map(s => s.trim()).filter(Boolean);
			if (!icons.includes('Icon')) {
				icons.unshift('Icon');
				const newImport = `import { ${icons.join(', ')} } from '$lib/icons'`;
				content = content.replace(existingImport[0], newImport);
			}
		} else {
			// Add a new import for Icon
			// Find the last import line in the script
			const scriptEnd = content.indexOf('</script>');
			const scriptContent = content.substring(0, scriptEnd);
			const lastImportIdx = scriptContent.lastIndexOf('\timport ');
			if (lastImportIdx !== -1) {
				const lineEnd = content.indexOf('\n', lastImportIdx);
				// But we need to handle multi-line imports
				let insertIdx = lineEnd;
				// Check if the import continues on the next line (multi-line import)
				const afterImport = content.substring(lineEnd);
				const nextLineMatch = afterImport.match(/^\n(\t+[^i\t])/);
				if (nextLineMatch) {
					// Find the closing of this import block
					const closingBrace = content.indexOf("} from", lineEnd);
					if (closingBrace !== -1 && closingBrace < scriptEnd) {
						const closingLine = content.indexOf('\n', closingBrace);
						insertIdx = closingLine;
					}
				}
				content = content.substring(0, insertIdx + 1) + 
				          "\timport { Icon } from '$lib/icons';\n" + 
				          content.substring(insertIdx + 1);
			}
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
