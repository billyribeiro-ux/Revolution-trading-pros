import re

with open('+page.svelte', 'r') as f:
    content = f.read()

# Fix 1: Replace the forEach viewport check with unconditional gsap.set
old_pattern = r'''const elements = document\.querySelectorAll\('\[data-gsap\]'\);
			elements\.forEach\(\(el\) => \{
				const rect = el\.getBoundingClientRect\(\);
				const isInViewport = rect\.top < window\.innerHeight \* 0\.85;
				if \(!isInViewport\) \{
					gsap\.set\(el, \{ opacity: 0, y: 30 \}\);
				\}
			\}\);'''

new_code = '''gsap.set('[data-gsap]', { opacity: 0, y: 30 });'''

content = re.sub(old_pattern, new_code, content)

# Fix 2: Move ScrollTrigger.refresh outside context
old_refresh = '''\t\t\tScrollTrigger.refresh();
\t\t});
\t})();

\t\treturn () => ctx?.revert();'''

new_refresh = '''\t\t});
\t\t\n\t\t// PE7: Refresh ScrollTrigger after layout settles
\t\tconst refreshTrigger = document.fonts?.ready ?? Promise.resolve();
\t\trefreshTrigger.then(() => {
\t\t\trequestAnimationFrame(() => ScrollTrigger.refresh());
\t\t});
\t})();

\t\treturn () => ctx?.revert();'''

content = content.replace(old_refresh, new_refresh)

with open('+page.svelte', 'w') as f:
    f.write(content)

print('GSAP fixes applied')
