#!/usr/bin/env node
/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Component Preview Script - IDE Integration
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * @description Opens a component in the workbench from the command line
 * @version 1.0.0 - February 2026
 * @standards Apple Principal Engineer ICT Level 7+
 *
 * Usage:
 *   node scripts/preview-component.js <file-path>
 *   node scripts/preview-component.js src/lib/components/dashboard/VideoCard.svelte
 */

import { readFileSync, existsSync } from 'fs';
import { relative, resolve, dirname } from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// ═══════════════════════════════════════════════════════════════════════════════
// CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════════

const WORKBENCH_URL = 'http://localhost:5174/workbench';
const COMPONENTS_BASE = 'src/lib/components';

// ═══════════════════════════════════════════════════════════════════════════════
// MAIN FUNCTION
// ═══════════════════════════════════════════════════════════════════════════════

function main() {
	const args = process.argv.slice(2);

	if (args.length === 0) {
		console.error('❌ Error: No file path provided');
		console.log('Usage: node scripts/preview-component.js <file-path>');
		process.exit(1);
	}

	const filePath = args[0];
	const absolutePath = resolve(process.cwd(), filePath);

	// Validate file exists
	if (!existsSync(absolutePath)) {
		console.error(`❌ Error: File not found: ${filePath}`);
		process.exit(1);
	}

	// Validate it's a Svelte file
	if (!filePath.endsWith('.svelte')) {
		console.error('❌ Error: File must be a .svelte component');
		process.exit(1);
	}

	// Get relative path from components directory
	const projectRoot = resolve(__dirname, '..');
	const componentsDir = resolve(projectRoot, COMPONENTS_BASE);
	
	// Check if file is inside components directory
	if (!absolutePath.startsWith(componentsDir)) {
		console.error(`❌ Error: Component must be inside ${COMPONENTS_BASE}/`);
		console.log(`   File: ${absolutePath}`);
		console.log(`   Expected: ${componentsDir}`);
		process.exit(1);
	}

	const relativePath = relative(componentsDir, absolutePath);
	const workbenchUrl = `${WORKBENCH_URL}?component=${encodeURIComponent(relativePath)}`;

	// Output the URL for IDE to open
	console.log('🎨 Opening component in workbench...');
	console.log(`📦 Component: ${relativePath}`);
	console.log(`🔗 URL: ${workbenchUrl}`);
	console.log('');
	console.log('WORKBENCH_URL=' + workbenchUrl);
}

// ═══════════════════════════════════════════════════════════════════════════════
// EXECUTE
// ═══════════════════════════════════════════════════════════════════════════════

main();
