#!/usr/bin/env node
/**
 * Fix remaining icon migration issues:
 * 1. Change icon: ComponentType → icon: string in interfaces/types
 * 2. Remove unused Component/ComponentType imports
 * 3. Fix {@const IconComponent = ...} followed by <svelte:component this={IconComponent}> 
 * 4. Fix Identifier 'Icon' already declared (duplicate imports)
 * 5. Fix broken import lines with trailing fragments
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

	// 1. Fix icon?: ComponentType → icon?: string
	content = content.replace(/icon\??\s*:\s*ComponentType\b/g, (match) => {
		return match.replace('ComponentType', 'string');
	});
	
	// 2. Fix icon: Component (non-type-import contexts)  
	content = content.replace(/icon\??\s*:\s*Component\b(?!\s*Library)/g, (match) => {
		return match.replace(/Component\b/, 'string');
	});

	// 3. Remove unused ComponentType import if no longer used
	if (!content.includes('ComponentType') && content.includes("import type { ComponentType }")) {
		content = content.replace(/\timport type \{ ComponentType \} from 'svelte';\n/g, '');
	}
	// Remove ComponentType from multi-import if unused
	if (!content.includes('ComponentType') || content.match(/icon.*string/)) {
		content = content.replace(/, ComponentType/g, '');
		content = content.replace(/ComponentType, /g, '');
	}

	// 4. Fix <svelte:component this={...}> patterns remaining
	content = content.replace(/<svelte:component\s+this=\{([^}]+)\}([^>]*?)\/>/g, (match, expr, props) => {
		// Only convert if it looks like an icon variable
		if (expr.includes('Icon') || expr.includes('icon') || expr === 'SvelteComponent' || expr.includes('SvelteComponent')) {
			return `<Icon icon={${expr}}${props}/>`;
		}
		return match;
	});
	
	// Also handle non-self-closing
	content = content.replace(/<svelte:component\s+this=\{([^}]+)\}([^>]*)>([\s\S]*?)<\/svelte:component>/g, (match, expr, props, children) => {
		if (expr.includes('Icon') || expr.includes('icon') || expr.includes('SvelteComponent')) {
			if (children.trim()) {
				return `<Icon icon={${expr}}${props}>${children}</Icon>`;
			}
			return `<Icon icon={${expr}}${props} />`;
		}
		return match;
	});

	// 5. Fix duplicate $lib/icons imports (two separate import lines from $lib/icons)
	const iconImports = [];
	const iconImportRegex = /\timport\s*\{([^}]+)\}\s*from\s*['"]\$lib\/icons['"];?\n/g;
	let m;
	while ((m = iconImportRegex.exec(content)) !== null) {
		iconImports.push({ full: m[0], names: m[1] });
	}
	
	if (iconImports.length > 1) {
		// Merge all into one
		const allNames = new Set();
		for (const imp of iconImports) {
			for (const name of imp.names.split(',').map(s => s.trim()).filter(Boolean)) {
				allNames.add(name);
			}
		}
		// Remove all existing
		for (const imp of iconImports) {
			content = content.replace(imp.full, '');
		}
		// Sort: Icon first
		const sorted = [...allNames].sort((a, b) => {
			if (a === 'Icon') return -1;
			if (b === 'Icon') return 1;
			return a.localeCompare(b);
		});
		// Find insertion point
		const scriptEnd = content.indexOf('</script>');
		const scriptContent = content.substring(0, scriptEnd);
		const lines = scriptContent.split('\n');
		let lastImportLine = -1;
		let inMultiLine = false;
		for (let i = 0; i < lines.length; i++) {
			if (lines[i].trim().startsWith('import ')) {
				if (lines[i].includes(' from ')) {
					lastImportLine = i;
					inMultiLine = false;
				} else {
					inMultiLine = true;
				}
			} else if (inMultiLine && lines[i].includes('} from ')) {
				lastImportLine = i;
				inMultiLine = false;
			}
		}
		if (lastImportLine >= 0) {
			lines.splice(lastImportLine + 1, 0, `\timport { ${sorted.join(', ')} } from '$lib/icons';`);
			content = lines.join('\n') + content.substring(scriptEnd);
		}
	}

	// 6. Fix broken import fragments like: import { Icon, ..., import { Icon } from '$lib/icons';
	content = content.replace(/import\s*\{[^}]*,\s*import\s*\{[^}]*\}\s*from\s*'\$lib\/icons';/g, (match) => {
		// Extract all icon names from the mess
		const names = new Set();
		const nameRegex = /\b(Icon\w*)\b/g;
		let nm;
		while ((nm = nameRegex.exec(match)) !== null) {
			names.add(nm[1]);
		}
		const sorted = [...names].sort((a, b) => {
			if (a === 'Icon') return -1;
			if (b === 'Icon') return 1;
			return a.localeCompare(b);
		});
		return `import { ${sorted.join(', ')} } from '$lib/icons';`;
	});

	// 7. Clean up lines that are just whitespace after other fixes
	content = content.replace(/\n\n\n+/g, '\n\n');

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
