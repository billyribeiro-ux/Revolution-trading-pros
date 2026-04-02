#!/usr/bin/env node
/**
 * Migration Script: Direct @tabler/icons-svelte-runes imports → $lib/icons
 * 
 * Handles files that import directly from @tabler/icons-svelte-runes/icons/xxx
 * instead of from $lib/icons barrel.
 */

import { readFileSync, writeFileSync, readdirSync, statSync } from 'fs';
import { join, relative } from 'path';

const SRC_DIR = join(process.cwd(), 'src');

// Build slug→name mapping from the barrel file's string constants
function buildIconMap() {
	const barrel = readFileSync(join(SRC_DIR, 'lib/icons/index.ts'), 'utf-8');
	const map = new Map(); // slug → export name
	
	// Match: export const IconFoo = 'tabler:some-slug';
	const re = /export const (Icon\w+) = 'tabler:(.+?)'/g;
	let m;
	while ((m = re.exec(barrel)) !== null) {
		map.set(m[2], m[1]); // slug → name
	}
	return map;
}

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

function migrateFile(filePath, iconMap) {
	let content = readFileSync(filePath, 'utf-8');
	
	// Skip barrel file and Icon.svelte
	if (filePath.endsWith('lib/icons/index.ts') || filePath.endsWith('lib/icons/Icon.svelte')) {
		return false;
	}
	
	if (!content.includes("@tabler/icons-svelte-runes")) {
		return false;
	}
	
	let modified = false;
	const iconImports = new Set(); // icon names to import from $lib/icons
	const localNameMap = new Map(); // localName → barrel export name
	
	// Find all direct tabler imports
	// Pattern: import IconFoo from '@tabler/icons-svelte-runes/icons/some-slug';
	const importRegex = /import\s+(\w+)\s+from\s+'@tabler\/icons-svelte-runes\/icons\/(.+?)';?\n?/g;
	
	let match;
	const importsToRemove = [];
	
	while ((match = importRegex.exec(content)) !== null) {
		const localName = match[1];
		const slug = match[2];
		const barrelName = iconMap.get(slug);
		
		if (barrelName) {
			iconImports.add(barrelName);
			localNameMap.set(localName, barrelName);
			importsToRemove.push(match[0]);
			modified = true;
		} else {
			console.warn(`  ⚠ Unknown icon slug: ${slug} (${localName}) in ${relative(SRC_DIR, filePath)}`);
			// Still map it as a string constant
			const constName = localName;
			iconImports.add(constName);
			localNameMap.set(localName, constName);
			importsToRemove.push(match[0]);
			modified = true;
		}
	}
	
	if (!modified) return false;
	
	// Remove old imports
	for (const imp of importsToRemove) {
		content = content.replace(imp, '');
	}
	
	// Add Icon to imports
	iconImports.add('Icon');
	
	// Check if there's already a $lib/icons import
	const existingImportMatch = content.match(/import\s*\{([^}]+)\}\s*from\s*['"]\$lib\/icons['"]/);
	
	if (existingImportMatch) {
		// Merge with existing import
		const existingIcons = existingImportMatch[1].split(',').map(s => s.trim()).filter(Boolean);
		const allIcons = new Set([...existingIcons, ...iconImports]);
		const newImport = `import { ${[...allIcons].join(', ')} } from '$lib/icons'`;
		content = content.replace(existingImportMatch[0], newImport);
	} else {
		// Add new import after the last existing import
		const sortedImports = [...iconImports].sort((a, b) => {
			if (a === 'Icon') return -1;
			if (b === 'Icon') return 1;
			return a.localeCompare(b);
		});
		const newImport = `\timport { ${sortedImports.join(', ')} } from '$lib/icons';\n`;
		
		// Find a good insertion point - after the last import statement
		const lastImportIdx = content.lastIndexOf("import ");
		if (lastImportIdx !== -1) {
			const lineEnd = content.indexOf('\n', lastImportIdx);
			content = content.slice(0, lineEnd + 1) + newImport + content.slice(lineEnd + 1);
		}
	}
	
	// Replace template usage: <IconFoo → <Icon icon={IconFoo}
	// Use localNameMap to handle cases where local name differs from barrel name
	for (const [localName, barrelName] of localNameMap) {
		// Replace opening tags: <IconFoo followed by space, /, or >
		const tagRegex = new RegExp(`<${localName}([\\s/>])`, 'g');
		content = content.replace(tagRegex, `<Icon icon={${barrelName}}$1`);
		
		// Replace closing tags: </IconFoo>
		const closeRegex = new RegExp(`</${localName}>`, 'g');
		content = content.replace(closeRegex, '</Icon>');
		
		// Replace variable references in JS: IconFoo (when used as value, not in import)
		// e.g., const iconMap = { foo: IconFoo } → { foo: IconFoo } (IconFoo is now a string, same usage)
		// This should work automatically since the string constant has the same name
		if (localName !== barrelName) {
			// Rename local variable references to barrel name
			const varRegex = new RegExp(`\\b${localName}\\b`, 'g');
			content = content.replace(varRegex, barrelName);
		}
	}
	
	// Clean up any double blank lines from removed imports
	content = content.replace(/\n{3,}/g, '\n\n');
	
	writeFileSync(filePath, content);
	return true;
}

// Run
const iconMap = buildIconMap();
console.log(`📦 Built icon map: ${iconMap.size} icons`);

const files = findFiles(SRC_DIR);
let count = 0;
const migrated = [];

for (const f of files) {
	if (migrateFile(f, iconMap)) {
		count++;
		migrated.push(relative(SRC_DIR, f));
	}
}

console.log(`\n✅ Migrated ${count} files with direct @tabler imports`);
for (const f of migrated.slice(0, 30)) console.log(`   ${f}`);
if (migrated.length > 30) console.log(`   ... and ${migrated.length - 30} more`);
