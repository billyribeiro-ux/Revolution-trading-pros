#!/usr/bin/env python3
import re

file_path = '/Users/billyribeiro/Desktop/my-websites/Revolution-trading-pros/frontend/src/routes/alerts/spx-profit-pulse/+page.svelte'

with open(file_path, 'r') as f:
    content = f.read()

print(f"File size: {len(content)} chars")

# Fix 1: Replace the GSAP forEach block - use exact tabs from file
old_block = '''\t\t\t// Only set initial hidden state for elements NOT yet in viewport
\t\t\tconst elements = document.querySelectorAll('[data-gsap]');
\t\t\telements.forEach((el) => {
\t\t\t\tconst rect = el.getBoundingClientRect();
\t\t\t\tconst isInViewport = rect.top < window.innerHeight * 0.85;
\t\t\t\tif (!isInViewport) {
\t\t\t\t\tgsap.set(el, { opacity: 0, y: 30 });
\t\t\t\t}
\t\t\t});'''

new_block = '''\t\t\t// PE7: Hide ALL elements unconditionally — let ScrollTrigger.batch be the single source of reveal
\t\t\tgsap.set('[data-gsap]', { opacity: 0, y: 30 });'''

if old_block in content:
    content = content.replace(old_block, new_block)
    print("✅ GSAP fixed")
else:
    print("❌ GSAP block not found - checking variations...")
    # Try finding just the start
    if 'forEach((el)' in content:
        print("  Found 'forEach((el)' - pattern mismatch")
    if '// Only set initial' in content:
        print("  Found comment - checking tabs...")
        idx = content.find('// Only set initial')
        print(f"  Context: {repr(content[idx:idx+100])}")

with open(file_path, 'w') as f:
    f.write(content)

print("Done")
