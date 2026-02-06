/**
 * Svelte Preprocessor for click-to-source
 * 
 * Adds data-source="filepath:line:col" to every element in dev mode.
 * This allows the runtime to know exactly where each element is defined.
 */

import type { PreprocessorGroup } from 'svelte/compiler';
import { parse } from 'svelte/compiler';
import MagicString from 'magic-string';

export interface PreprocessorOptions {
  /** Enable/disable the preprocessor. Default: NODE_ENV !== 'production' */
  enabled?: boolean;
}

interface TemplateNode {
  type: string;
  name?: string;
  start: number;
  end: number;
  attributes?: Array<{
    type: string;
    name: string;
    start: number;
    end: number;
  }>;
  children?: TemplateNode[];
}

/**
 * Creates a Svelte preprocessor that injects source location data attributes
 */
export function createPreprocessor(options: PreprocessorOptions = {}): PreprocessorGroup {
  const { enabled = process.env.NODE_ENV !== 'production' } = options;

  return {
    name: 'click-to-source',
    
    markup({ content, filename }) {
      if (!enabled || !filename) {
        return { code: content };
      }

      // Skip node_modules and generated files
      if (filename.includes('node_modules') || filename.includes('.svelte-kit')) {
        return { code: content };
      }

      try {
        const s = new MagicString(content);
        const ast = parse(content, { filename });

        // Track insertions to avoid conflicts
        const insertions: Array<{ pos: number; text: string }> = [];

        // Walk the template AST
        if (ast.html) {
          walkTemplate(ast.html, filename, insertions, content);
        }

        // Apply insertions in reverse order (so positions stay valid)
        insertions
          .sort((a, b) => b.pos - a.pos)
          .forEach(({ pos, text }) => {
            s.appendLeft(pos, text);
          });

        return {
          code: s.toString(),
          map: s.generateMap({ source: filename, hires: true }),
        };
      } catch (error) {
        // If parsing fails, return original (don't break the build)
        console.warn(`[click-to-source] Failed to process ${filename}:`, error);
        return { code: content };
      }
    },
  };
}

/**
 * Recursively walks the template AST and collects insertion points
 */
function walkTemplate(
  node: TemplateNode,
  filename: string,
  insertions: Array<{ pos: number; text: string }>,
  source: string
): void {
  // Only process actual elements (not components, blocks, etc.)
  if (node.type === 'Element' && node.name) {
    // Calculate line and column from start position
    const { line, column } = getLineAndColumn(source, node.start);
    
    // Find where to insert the attribute (after the tag name)
    const insertPos = findAttributeInsertPosition(node, source);
    
    if (insertPos > 0) {
      // Make path relative and clean
      const relativePath = filename.replace(process.cwd() + '/', '');
      const attr = ` data-source="${relativePath}:${line}:${column}"`;
      
      insertions.push({ pos: insertPos, text: attr });
    }
  }

  // Recurse into children
  if (node.children) {
    for (const child of node.children) {
      walkTemplate(child as TemplateNode, filename, insertions, source);
    }
  }
}

/**
 * Finds the position right after the tag name where we can insert attributes
 */
function findAttributeInsertPosition(node: TemplateNode, source: string): number {
  // Start after '<tagname'
  const tagStart = node.start;
  const tagContent = source.slice(tagStart, tagStart + 100); // Look at first 100 chars
  
  // Match '<tagname' and find where to insert
  const match = tagContent.match(/^<([a-zA-Z][a-zA-Z0-9_-]*)/);
  if (match) {
    return tagStart + match[0].length;
  }
  
  return -1;
}

/**
 * Converts a character offset to line and column numbers
 */
function getLineAndColumn(source: string, offset: number): { line: number; column: number } {
  const lines = source.slice(0, offset).split('\n');
  return {
    line: lines.length,
    column: lines[lines.length - 1].length + 1,
  };
}

export default createPreprocessor;
