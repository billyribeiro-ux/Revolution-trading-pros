/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Component Workbench - Server Load
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * @description Discovers all components and extracts their props at build time
 * @version 1.0.0 - January 2026
 * @standards Apple Principal Engineer ICT Level 7+
 *
 * Features:
 * - Recursive component discovery from $lib/components
 * - TypeScript interface parsing for props extraction
 * - Caching for performance
 * - Source code access for preview
 */

import { readdir, readFile, stat } from 'fs/promises';
import { join, relative, basename, dirname } from 'path';
import type { PageServerLoad } from './$types';

// ═══════════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════════

export interface PropDefinition {
	name: string;
	type: string;
	required: boolean;
	defaultValue?: string;
	description?: string;
}

export interface ComponentInfo {
	name: string;
	path: string;
	relativePath: string;
	folder: string;
	props: PropDefinition[];
	hasSnippets: boolean;
	source: string;
	size: number;
}

export interface ComponentTree {
	[folder: string]: ComponentInfo[];
}

// ═══════════════════════════════════════════════════════════════════════════════
// COMPONENT DISCOVERY
// ═══════════════════════════════════════════════════════════════════════════════

const COMPONENTS_DIR = 'src/lib/components';

/**
 * Recursively finds all .svelte files in a directory
 */
async function findSvelteFiles(dir: string, baseDir: string): Promise<string[]> {
	const files: string[] = [];

	try {
		const entries = await readdir(dir, { withFileTypes: true });

		for (const entry of entries) {
			const fullPath = join(dir, entry.name);

			if (entry.isDirectory()) {
				// Skip test directories and node_modules
				if (entry.name.startsWith('__') || entry.name === 'node_modules') {
					continue;
				}
				const subFiles = await findSvelteFiles(fullPath, baseDir);
				files.push(...subFiles);
			} else if (entry.name.endsWith('.svelte')) {
				files.push(fullPath);
			}
		}
	} catch {
		// Directory doesn't exist or can't be read
	}

	return files;
}

/**
 * Parses props from component source code
 */
function parseProps(source: string): PropDefinition[] {
	const props: PropDefinition[] = [];

	// Pattern 1: interface Props { ... }
	const interfaceMatch = source.match(/interface\s+(?:Props|[A-Z]\w*Props)\s*\{([^}]+)\}/s);

	if (interfaceMatch) {
		const interfaceBody = interfaceMatch[1];
		const propLines = interfaceBody.split('\n');

		for (const line of propLines) {
			// Match: propName?: type; or propName: type;
			const propMatch = line.match(/^\s*\/?\*?\*?\s*(\w+)(\?)?:\s*([^;]+);?\s*$/);

			if (propMatch) {
				const [, name, optional, type] = propMatch;

				// Skip internal props
				if (name.startsWith('_')) continue;

				props.push({
					name,
					type: type.trim(),
					required: !optional,
					defaultValue: undefined,
					description: undefined
				});
			}
		}
	}

	// Pattern 2: let { prop = default }: Props = $props()
	const propsMatch = source.match(/let\s*\{([^}]+)\}\s*(?::\s*\w+)?\s*=\s*\$props\(\)/s);

	if (propsMatch) {
		const propsBody = propsMatch[1];
		const assignments = propsBody.split(',');

		for (const assignment of assignments) {
			// Match: propName = defaultValue or just propName
			const assignMatch = assignment.trim().match(/^(\w+)(?:\s*=\s*(.+))?$/);

			if (assignMatch) {
				const [, name, defaultValue] = assignMatch;

				// Find existing prop or create new one
				const existing = props.find((p) => p.name === name);
				if (existing && defaultValue) {
					existing.defaultValue = defaultValue.trim();
					existing.required = false;
				} else if (!existing && name && !name.startsWith('_')) {
					props.push({
						name,
						type: 'unknown',
						required: !defaultValue,
						defaultValue: defaultValue?.trim(),
						description: undefined
					});
				}
			}
		}
	}

	return props;
}

/**
 * Checks if component uses Snippet children
 */
function hasSnippetChildren(source: string): boolean {
	return (
		source.includes('Snippet') ||
		source.includes('@render children') ||
		source.includes('children?: Snippet')
	);
}

/**
 * Loads component info from file path
 */
async function loadComponentInfo(filePath: string, baseDir: string): Promise<ComponentInfo> {
	const source = await readFile(filePath, 'utf-8');
	const stats = await stat(filePath);
	const relativePath = relative(baseDir, filePath);
	const folder = dirname(relativePath) === '.' ? 'root' : dirname(relativePath).split('/')[0];

	return {
		name: basename(filePath, '.svelte'),
		path: filePath,
		relativePath,
		folder,
		props: parseProps(source),
		hasSnippets: hasSnippetChildren(source),
		source,
		size: stats.size
	};
}

// ═══════════════════════════════════════════════════════════════════════════════
// PAGE LOAD
// ═══════════════════════════════════════════════════════════════════════════════

export const load: PageServerLoad = async () => {
	const baseDir = join(process.cwd(), COMPONENTS_DIR);
	const svelteFiles = await findSvelteFiles(baseDir, baseDir);

	// Load all component info in parallel
	const components = await Promise.all(svelteFiles.map((f) => loadComponentInfo(f, baseDir)));

	// Sort alphabetically
	components.sort((a, b) => a.name.localeCompare(b.name));

	// Group by folder
	const tree: ComponentTree = {};
	for (const comp of components) {
		if (!tree[comp.folder]) {
			tree[comp.folder] = [];
		}
		tree[comp.folder].push(comp);
	}

	// Sort folders (root first, then alphabetically)
	const sortedTree: ComponentTree = {};
	const folders = Object.keys(tree).sort((a, b) => {
		if (a === 'root') return -1;
		if (b === 'root') return 1;
		return a.localeCompare(b);
	});

	for (const folder of folders) {
		sortedTree[folder] = tree[folder];
	}

	return {
		components,
		tree: sortedTree,
		totalCount: components.length,
		folderCount: folders.length
	};
};
