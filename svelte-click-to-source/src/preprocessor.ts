/**
 * Svelte Preprocessor for click-to-source
 * Adds data-source="filepath:line:col" to HTML elements in the markup section only.
 * 
 * Skips:
 * - <script> and <style> blocks (avoids breaking TypeScript generics)
 * - Special elements: <title>, <script>, <style>, <svelte:*>, <slot>
 * - Self-closing tags inside excluded zones
 */

import type { PreprocessorGroup } from 'svelte/compiler';
import MagicString from 'magic-string';

export interface PreprocessorOptions {
  enabled?: boolean;
}

// Elements that must NOT receive data-source attributes
const EXCLUDED_TAGS = new Set([
  'script', 'style', 'title', 'slot', 'head',
  'svelte:head', 'svelte:body', 'svelte:window', 'svelte:document',
  'svelte:component', 'svelte:self', 'svelte:fragment', 'svelte:element',
  'svelte:options', 'svelte:boundary',
]);

/**
 * Find all ranges in the source that are inside <script> or <style> blocks.
 * We must not inject data-source attributes inside these blocks.
 */
function findExcludedRanges(content: string): Array<{ start: number; end: number }> {
  const ranges: Array<{ start: number; end: number }> = [];
  const blockRegex = /<(script|style)(\s[^>]*)?>[\s\S]*?<\/\1>/gi;
  let match;
  while ((match = blockRegex.exec(content)) !== null) {
    ranges.push({ start: match.index, end: match.index + match[0].length });
  }
  return ranges;
}

function isInsideExcludedRange(
  pos: number,
  ranges: Array<{ start: number; end: number }>
): boolean {
  return ranges.some((r) => pos >= r.start && pos < r.end);
}

export function createPreprocessor(options: PreprocessorOptions = {}): PreprocessorGroup {
  const { enabled = process.env.NODE_ENV !== 'production' } = options;

  return {
    name: 'click-to-source-preprocessor',
    
    markup({ content, filename }) {
      if (!enabled || !filename) {
        return { code: content };
      }

      if (filename.includes('node_modules') || filename.includes('.svelte-kit')) {
        return { code: content };
      }

      try {
        const s = new MagicString(content);
        const insertions: Array<{ pos: number; text: string }> = [];
        const excludedRanges = findExcludedRanges(content);

        // Match opening HTML tags (lowercase only = native HTML elements)
        const tagRegex = /<([a-z][a-z0-9-]*)([\s>\/])/gi;
        let match;

        while ((match = tagRegex.exec(content)) !== null) {
          const tagStart = match.index;
          const tagName = match[1].toLowerCase();
          const afterTagName = match.index + match[0].length - 1;

          // Skip if inside <script> or <style> block
          if (isInsideExcludedRange(tagStart, excludedRanges)) {
            continue;
          }

          // Skip excluded tags
          if (EXCLUDED_TAGS.has(tagName)) {
            continue;
          }

          // Calculate line and column
          const beforeTag = content.slice(0, tagStart);
          const lines = beforeTag.split('\n');
          const line = lines.length;
          const column = lines[lines.length - 1].length + 1;
          
          // Create relative path
          const relativePath = filename.replace(process.cwd() + '/', '');
          const attr = ` data-source="${relativePath}:${line}:${column}"`;
          
          insertions.push({ pos: afterTagName, text: attr });
        }

        // Apply in reverse order
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
        console.warn(`[click-to-source] Failed to process ${filename}:`, error);
        return { code: content };
      }
    },
  };
}

export default createPreprocessor;
