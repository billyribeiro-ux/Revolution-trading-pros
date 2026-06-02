#!/usr/bin/env python3
"""Conservative comment cleanup: remove high-confidence noise only.
Removes:
  - decorative @version / @author / @requires doc-tag lines
  - `// Old: ...` dead-code reference lines
  - commented-out Svelte rune code blocks (// $effect( ... // }); runs, and
    single-line // $state(/// $derived( commented code)
Preserves everything else: prose/WHY comments, license/purpose headers, and
ALL functional directives (svelte-ignore, eslint-*, @ts-ignore/@ts-expect-error,
prettier-ignore).
"""
import re, sys

PROTECT = ('svelte-ignore', 'eslint-', '@ts-ignore', '@ts-expect-error', 'prettier-ignore')
RUNE_START = re.compile(r'^\s*//\s*\$(effect|state|derived)\b|^\s*//\s*\$:')
DOCTAG = re.compile(r'^\s*\*?\s*@(version|author|requires)\b')
OLD_LINE = re.compile(r'^\s*//\s*Old:')
COMMENT_LINE = re.compile(r'^\s*//')
# pure decorative box-drawing/rule line inside a comment (no words)
BOXART = re.compile(r'^\s*(//+|\*)\s*[═─━—\-_*=]{4,}\s*\*?/?\s*$')

def protected(line): return any(p in line for p in PROTECT)

def clean(path):
    with open(path, encoding='utf-8') as f:
        lines = f.readlines()
    out, i, n, removed = [], 0, len(lines), 0
    while i < n:
        line = lines[i]
        if protected(line):
            out.append(line); i += 1; continue
        # decorative doc tags (inside /** */) — drop the single line
        if DOCTAG.search(line):
            removed += 1; i += 1; continue
        # pure decorative separator line — drop, but NEVER a line that opens or
        # closes a block comment (/* or */), or the unterminated block would
        # swallow the code that follows.
        if BOXART.search(line) and '/*' not in line and '*/' not in line:
            removed += 1; i += 1; continue
        # `// Old:` dead-code reference — drop
        if OLD_LINE.search(line):
            removed += 1; i += 1; continue
        # commented-out rune code: drop the consecutive // run starting here
        if RUNE_START.search(line):
            while i < n and COMMENT_LINE.match(lines[i]) and not protected(lines[i]):
                removed += 1; i += 1
            continue
        out.append(line); i += 1
    if removed:
        # collapse a doc block that became empty (/** then immediately */)
        text = ''.join(out)
        text = re.sub(r'/\*\*\s*\n(\s*\*\s*\n)*\s*\*/\s*\n', '', text)
        with open(path, 'w', encoding='utf-8') as f:
            f.write(text)
    return removed

total_files = total_lines = 0
for p in [l.strip() for l in open(sys.argv[1]) if l.strip()]:
    try:
        r = clean(p)
    except FileNotFoundError:
        continue
    if r:
        total_files += 1; total_lines += r
print(f"cleaned {total_files} files, removed {total_lines} comment lines")
