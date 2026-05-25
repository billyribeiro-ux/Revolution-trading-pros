#!/usr/bin/env python3
import re

file_path = '/Users/billyribeiro/Desktop/my-websites/Revolution-trading-pros/frontend/src/routes/alerts/spx-profit-pulse/+page.svelte'

with open(file_path, 'r') as f:
    content = f.read()

# Fix 1: GSAP forEach
old_gsap = '''\t\t\t// Only set initial hidden state for elements NOT yet in viewport
\t\t\tconst elements = document.querySelectorAll('[data-gsap]');
\t\t\telements.forEach((el) => {
\t\t\t\tconst rect = el.getBoundingClientRect();
\t\t\t\tconst isInViewport = rect.top < window.innerHeight * 0.85;
\t\t\t\tif (!isInViewport) {
\t\t\t\t\tgsap.set(el, { opacity: 0, y: 30 });
\t\t\t\t}
\t\t\t});'''

new_gsap = '''\t\t\t// PE7: Hide ALL elements unconditionally — let ScrollTrigger.batch be the single source of reveal
\t\t\tgsap.set('[data-gsap]', { opacity: 0, y: 30 });'''

if old_gsap in content:
    content = content.replace(old_gsap, new_gsap)
    print('✅ GSAP fix applied')
else:
    print('❌ GSAP pattern not found')

# Fix 2: Move ScrollTrigger.refresh outside context
old_refresh = '''\t\t\t\t\tonce: true
\t\t\t});

\t\t\tScrollTrigger.refresh();
\t\t});'''

new_refresh = '''\t\t\t\t\tonce: true
\t\t\t});
\t\t});

\t\t// PE7: Refresh ScrollTrigger after layout settles (fonts, images loaded)
\t\tconst refreshTrigger = document.fonts?.ready ?? Promise.resolve();
\t\trefreshTrigger.then(() => {
\t\t\trequestAnimationFrame(() => ScrollTrigger.refresh());
\t\t});'''

if old_refresh in content:
    content = content.replace(old_refresh, new_refresh)
    print('✅ ScrollTrigger.refresh moved')
else:
    print('❌ ScrollTrigger.refresh pattern not found')

with open(file_path, 'w') as f:
    f.write(content)

print('Done!')
