#!/usr/bin/env node
/**
 * Fix all remaining patterns where a $derived variable holding an icon string
 * is used as a component: <TypeIcon size={20} /> → <Icon icon={typeIconStr} size={20} />
 * 
 * Patterns to find and fix:
 * 1. let/const XxxIcon = $derived(expr) → let/const xxxIconStr = $derived(expr)
 *    <XxxIcon .../> → <Icon icon={xxxIconStr} .../>
 * 2. {@const XxxIcon = expr} <XxxIcon .../> → {@const xxxIconStr = expr} <Icon icon={xxxIconStr} .../>
 * 3. Also fix remaining <IconComponent .../> and similar
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
		} else if (entry.endsWith('.svelte')) {
			files.push(fullPath);
		}
	}
	return files;
}

// Mapping of variable names to replacement names
const ICON_VAR_PATTERNS = [
	'TypeIcon', 'FileIcon', 'VariantIcon', 'ErrorIcon', 'IconComponent',
	'StatusIcon', 'SeverityIcon', 'ActionIcon', 'CategoryIcon',
	'SvelteComponent', 'SvelteComponent_1',
];

function fixFile(filePath) {
	let content = readFileSync(filePath, 'utf-8');
	const original = content;

	if (filePath.endsWith('lib/icons/index.ts') || filePath.endsWith('lib/icons/Icon.svelte') ||
	    filePath.endsWith('DynamicIcon.svelte') || filePath.endsWith('RtpIcon.svelte')) {
		return false;
	}

	// Find all variable names that are assigned from icon expressions and used as components
	// Look for: let/const VarName = $derived(iconExpr) or let/const VarName = $derived.by(() => ...)
	const scriptEnd = content.indexOf('</script>');
	if (scriptEnd === -1) return false;
	
	const scriptContent = content.substring(0, scriptEnd);
	const templateContent = content.substring(scriptEnd);
	
	// Find all variables in script that likely hold icon values
	const iconVarRegex = /(?:let|const)\s+(\w+)\s*=\s*\$derived(?:\.by)?\(/g;
	let match;
	const iconVars = new Set();
	
	while ((match = iconVarRegex.exec(scriptContent)) !== null) {
		const varName = match[1];
		// Check if this variable is used as a component in template
		if (templateContent.includes(`<${varName} `) || templateContent.includes(`<${varName}\n`)) {
			iconVars.add(varName);
		}
	}
	
	// Also check for known patterns
	for (const pattern of ICON_VAR_PATTERNS) {
		if (templateContent.includes(`<${pattern} `) || templateContent.includes(`<${pattern}\n`)) {
			iconVars.add(pattern);
		}
	}
	
	// Also find {@const VarName = expr} patterns in template
	const constRegex = /\{@const\s+(\w+)\s*=/g;
	while ((match = constRegex.exec(templateContent)) !== null) {
		const varName = match[1];
		if (templateContent.includes(`<${varName} `) || templateContent.includes(`<${varName}\n`)) {
			iconVars.add(varName);
		}
	}
	
	if (iconVars.size === 0) return false;
	
	for (const varName of iconVars) {
		// Create a safe replacement name
		const safeName = varName.charAt(0).toLowerCase() + varName.slice(1) + 'Str';
		
		// Replace in script: let/const VarName = → let/const safeNameStr =
		const scriptDeclRegex = new RegExp(`((?:let|const)\\s+)${varName}(\\s*=)`, 'g');
		content = content.replace(scriptDeclRegex, `$1${safeName}$2`);
		
		// Replace {@const VarName = → {@const safeNameStr =
		const constDeclRegex = new RegExp(`\\{@const\\s+${varName}(\\s*=)`, 'g');
		content = content.replace(constDeclRegex, `{@const ${safeName}$1`);
		
		// Replace <VarName props /> → <Icon icon={safeNameStr} props />
		const selfCloseRegex = new RegExp(`<${varName}(\\s[^>]*?)\\/>`, 'g');
		content = content.replace(selfCloseRegex, `<Icon icon={${safeName}}$1/>`);
		
		// Replace <VarName props>...</VarName> → <Icon icon={safeNameStr} props>...</Icon>
		const openRegex = new RegExp(`<${varName}(\\s[^>]*)>`, 'g');
		content = content.replace(openRegex, `<Icon icon={${safeName}}$1>`);
		const closeRegex = new RegExp(`</${varName}>`, 'g');
		content = content.replace(closeRegex, '</Icon>');
		
		// Also replace any remaining references to the variable in expressions
		// like: icon={VarName} → icon={safeNameStr}
		const exprRegex = new RegExp(`\\{${varName}\\}`, 'g');
		// Only replace if it's clearly an icon context - skip if it's a common word
		if (varName !== 'Icon') {
			content = content.replace(exprRegex, `{${safeName}}`);
		}
	}
	
	// Ensure Icon is imported if we added <Icon> usage
	if (content !== original) {
		const newScriptEnd = content.indexOf('</script>');
		const newScript = content.substring(0, newScriptEnd);
		const newTemplate = content.substring(newScriptEnd);
		
		if (newTemplate.includes('<Icon ') && 
		    !newScript.includes("{ Icon,") && !newScript.includes("{ Icon }") && 
		    !newScript.includes("{Icon,") && !newScript.includes("{Icon }") &&
		    !newScript.includes(", Icon,") && !newScript.includes(", Icon }")) {
			
			const importMatch = newScript.match(/import\s*\{([^}]+)\}\s*from\s*['"]\$lib\/icons['"]/);
			if (importMatch) {
				const names = importMatch[1].split(',').map(s => s.trim()).filter(Boolean);
				if (!names.includes('Icon')) {
					names.unshift('Icon');
					const newImport = `import { ${names.join(', ')} } from '$lib/icons'`;
					content = content.replace(importMatch[0], newImport);
				}
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
