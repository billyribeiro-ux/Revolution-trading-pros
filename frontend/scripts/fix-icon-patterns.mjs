#!/usr/bin/env node
/**
 * Fix remaining icon migration patterns:
 * 1. svelte:component this={IconXxx} → <Icon icon={IconXxx} />
 * 2. {@const Comp = iconExpr} <Comp /> → <Icon icon={iconExpr} />
 * 3. icon: IconXxx in data structures (these are now strings, which is fine)
 * 4. Type annotations using icon components → string
 * 5. Duplicate Icon identifier (import conflict)
 * 6. Remove empty import lines left by tabler migration
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
	
	// Skip barrel and wrapper files
	if (filePath.endsWith('lib/icons/index.ts') || 
	    filePath.endsWith('lib/icons/Icon.svelte') ||
	    filePath.endsWith('icons/RtpIcon.svelte') ||
	    filePath.endsWith('DynamicIcon.svelte')) {
		return false;
	}

	// 1. Fix duplicate Icon identifier - if file imports Icon from @iconify/svelte AND from $lib/icons
	// Remove the @iconify/svelte import since $lib/icons already re-exports Icon
	content = content.replace(/\timport\s+Icon\s+from\s+'@iconify\/svelte';\n/g, '');
	content = content.replace(/\timport\s+IconifyIcon\s+from\s+'@iconify\/svelte';\n/g, '');

	// 2. Fix <svelte:component this={IconXxx} ... /> patterns
	// Pattern: <svelte:component this={someIconVar} size={24} />
	content = content.replace(
		/<svelte:component\s+this=\{(\w+)\}\s*((?:[^>]*?))\s*\/>/g,
		(match, varName, props) => {
			// Only convert if it looks like an icon variable
			if (varName.startsWith('Icon') || varName.startsWith('icon') || 
			    varName === 'StatusIcon' || varName === 'TypeIcon' || 
			    varName === 'Comp' || varName === 'Component') {
				return `<Icon icon={${varName}} ${props.trim()} />`;
			}
			return match;
		}
	);

	// 3. Fix {@const IconComp = expr} then <IconComp .../> patterns
	// Replace <SomeIconVar ... /> where SomeIconVar is used as a dynamic component
	// This is tricky - we need to find variables that hold icon values and are used as components
	
	// Pattern: {@const SomeVar = iconExpr}
	// Then: <SomeVar size={24} />
	const constIconPattern = /\{@const\s+(\w+)\s*=\s*([^}]+)\}/g;
	let constMatch;
	const iconVarNames = new Set();
	while ((constMatch = constIconPattern.exec(content)) !== null) {
		const varName = constMatch[1];
		const expr = constMatch[2].trim();
		// If the expression references icon-like values
		if (expr.includes('Icon') || expr.includes('icon')) {
			iconVarNames.add(varName);
		}
	}
	
	// Replace usages of these variables as components
	for (const varName of iconVarNames) {
		// Self-closing: <VarName ... />
		const selfCloseRegex = new RegExp(`<${varName}(\\s[^>]*)\\/>`, 'g');
		content = content.replace(selfCloseRegex, `<Icon icon={${varName}}$1/>`);
		
		// Opening + closing: <VarName ...>...</VarName>
		const openCloseRegex = new RegExp(`<${varName}(\\s[^>]*)>([\\s\\S]*?)<\\/${varName}>`, 'g');
		content = content.replace(openCloseRegex, `<Icon icon={${varName}}$1>$2</Icon>`);
	}

	// 4. Fix type annotations: typeof IconXxx → string
	content = content.replace(/typeof\s+Icon[A-Z]\w+/g, 'string');
	
	// 5. Fix IconComponent type references used for icon component types
	// type XxxIcon = typeof IconSomething → type XxxIcon = string
	// Already handled by #4

	// 6. Clean up empty comment-only blocks left by stripped imports
	// Lines that are just whitespace or empty after import removal
	content = content.replace(/\n\t\/\/ [A-Z][a-z]+ & [A-Z]\w+\n\t{10,}\n/g, '\n');
	content = content.replace(/\n\t\/\/ [A-Z][a-z]+ [A-Z]\w+\n\t{10,}\n/g, '\n');
	content = content.replace(/\n\t\/\/ [A-Z]\w+\n\t{5,}\n/g, '\n');
	
	// 7. Fix patterns where icons are stored in arrays/objects and rendered dynamically
	// { icon: IconXxx, label: 'Foo' } → icon is now a string, rendering needs <Icon icon={item.icon} />
	// Look for patterns like: <item.icon ... /> or <entry.icon ... />
	content = content.replace(/<(\w+\.icon)\s/g, (match, expr) => {
		return `<Icon icon={${expr}} `;
	});
	
	// 8. Fix patterns: const SomeIcon = IconXxx (in script blocks, these are already strings, fine)
	// But if used as <SomeIcon />, needs to become <Icon icon={SomeIcon} />
	// Find all local variables assigned from Icon* constants
	const scriptMatch = content.match(/<script[^>]*>([\s\S]*?)<\/script>/);
	if (scriptMatch) {
		const scriptContent = scriptMatch[1];
		const localIconVars = new Set();
		
		// Match: const/let SomeVar = IconXxx or condition ? IconA : IconB
		const assignRegex = /(?:const|let)\s+(\w+)\s*=\s*(?:.*?Icon[A-Z])/g;
		let assignMatch;
		while ((assignMatch = assignRegex.exec(scriptContent)) !== null) {
			const varName = assignMatch[1];
			if (!varName.startsWith('Icon')) {
				localIconVars.add(varName);
			}
		}
		
		// Also find $derived variables that reference icons
		const derivedRegex = /(?:const|let)\s+(\w+)\s*=\s*\$derived\(.*?Icon[A-Z]/g;
		while ((assignMatch = derivedRegex.exec(scriptContent)) !== null) {
			const varName = assignMatch[1];
			if (!varName.startsWith('Icon')) {
				localIconVars.add(varName);
			}
		}
		
		// Replace template usages of these variables
		for (const varName of localIconVars) {
			// Skip if it's already inside an <Icon> tag
			const selfCloseRegex = new RegExp(`<${varName}(\\s[^>]*)\\/>`, 'g');
			content = content.replace(selfCloseRegex, (match, props) => {
				// Don't double-wrap
				if (match.includes('icon={')) return match;
				return `<Icon icon={${varName}}${props}/>`;
			});
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
for (const f of fixed.slice(0, 40)) console.log(`  ${f}`);
if (fixed.length > 40) console.log(`  ... and ${fixed.length - 40} more`);
