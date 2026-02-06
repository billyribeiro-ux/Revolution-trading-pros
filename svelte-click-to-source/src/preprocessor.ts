/**
 * Svelte Preprocessor for click-to-source
 * Adds data-source="filepath:line:col" to every element in dev mode.
 */

import type { PreprocessorGroup } from 'svelte/compiler';
import MagicString from 'magic-string';

export interface PreprocessorOptions {
  enabled?: boolean;
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

        // Match opening tags: <div, <span, <button, etc. (lowercase = HTML elements)
        const tagRegex = /<([a-z][a-z0-9]*)([\s>])/gi;
        let match;

        while ((match = tagRegex.exec(content)) !== null) {
          const tagStart = match.index;
          const afterTagName = match.index + match[0].length - 1;
          
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
