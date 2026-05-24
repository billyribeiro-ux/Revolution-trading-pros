#!/usr/bin/env python3
"""Complete fix for SPX Profit Pulse page"""

file_path = '/Users/billyribeiro/Desktop/my-websites/Revolution-trading-pros/frontend/src/routes/alerts/spx-profit-pulse/+page.svelte'

with open(file_path, 'r') as f:
    content = f.read()

print(f"Original file size: {len(content)} characters")

# Fix 1: GSAP - Replace forEach with unconditional gsap.set
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
    print("✅ Fix 1: GSAP forEach removed")
else:
    print("❌ Fix 1: GSAP pattern not found")

# Fix 2: Move ScrollTrigger.refresh outside context with fonts.ready
old_refresh = '''\t\t\t\tonce: true
\t\t\t});

\t\t\tScrollTrigger.refresh();
\t\t});'''

new_refresh = '''\t\t\t\tonce: true
\t\t\t});
\t\t});

\t\t// PE7: Refresh ScrollTrigger after layout settles (fonts, images loaded)
\t\tconst refreshTrigger = document.fonts?.ready ?? Promise.resolve();
\t\trefreshTrigger.then(() => {
\t\t\trequestAnimationFrame(() => ScrollTrigger.refresh());
\t\t});'''

if old_refresh in content:
    content = content.replace(old_refresh, new_refresh)
    print("✅ Fix 2: ScrollTrigger.refresh moved")
else:
    print("❌ Fix 2: ScrollTrigger.refresh pattern not found")

# Fix 3: Add SSOT Plan interface and plans array after selectedPlan
old_plan_state = '''\t// --- Pricing State ---
\tlet selectedPlan: 'monthly' | 'quarterly' | 'annual' = $state('quarterly');'''

new_plan_state = '''\t// --- Pricing State (Svelte 5 Runes) ---
\tlet selectedPlan: 'monthly' | 'quarterly' | 'annual' = $state('quarterly');

\t// SSOT: Single source of truth for all pricing data
\tinterface Plan {
\t\tid: 'monthly' | 'quarterly' | 'annual';
\t\tlabel: string;
\t\tprice: number;
\t\tperiod: string;
\t\tperDay: string;
\t\tsavingsCopy: string;
\t\tcheckoutHref: string;
\t\tfeatured: boolean;
\t\tvariant: 'simple' | 'featured' | 'highlight';
\t\tfeatures: string[];
\t}

\tconst plans: Plan[] = [
\t\t{
\t\t\tid: 'monthly',
\t\t\tlabel: 'Monthly',
\t\t\tprice: 97,
\t\t\tperiod: '/mo',
\t\t\tperDay: '$3.23/day',
\t\t\tsavingsCopy: '',
\t\t\tcheckoutHref: '/checkout/monthly-spx',
\t\t\tfeatured: false,
\t\t\tvariant: 'simple',
\t\t\tfeatures: ['Daily SPX Alerts', 'SMS + Discord', 'Entry/Exit/Updates', 'Risk Parameters']
\t\t},
\t\t{
\t\t\tid: 'quarterly',
\t\t\tlabel: 'Quarterly',
\t\t\tprice: 247,
\t\t\tperiod: '/qtr',
\t\t\tperDay: '$2.74/day',
\t\t\tsavingsCopy: 'Most Popular — Save $44',
\t\t\tcheckoutHref: '/checkout/quarterly-spx',
\t\t\tfeatured: true,
\t\t\tvariant: 'featured',
\t\t\tfeatures: ['Everything in Monthly', 'Weekly Video Breakdowns', 'Market Context Reports', 'Priority Support']
\t\t},
\t\t{
\t\t\tid: 'annual',
\t\t\tlabel: 'Annual',
\t\t\tprice: 777,
\t\t\tperiod: '/yr',
\t\t\tperDay: '$2.13/day',
\t\t\tsavingsCopy: 'Best Value — Save $387',
\t\t\tcheckoutHref: '/checkout/annual-spx',
\t\t\tfeatured: false,
\t\t\tvariant: 'highlight',
\t\t\tfeatures: ['Everything in Quarterly', '1-on-1 Strategy Session', 'Annual Members Events', 'Direct DM Access']
\t\t}
\t];

\tconst minPrice = Math.min(...plans.map(p => p.price));
\tconst maxPrice = Math.max(...plans.map(p => p.price));'''

if old_plan_state in content:
    content = content.replace(old_plan_state, new_plan_state)
    print("✅ Fix 3: SSOT Plans added")
else:
    print("❌ Fix 3: Plan state pattern not found")

# Fix 4: Fix background colors
bg_replacements = [
    ('bg-white', 'bg-rtp-surface'),
    ('bg-slate-100', 'bg-rtp-bg'),
    ('bg-slate-50', 'bg-rtp-bg'),
]

for old, new in bg_replacements:
    count = content.count(old)
    if count > 0:
        content = content.replace(old, new)
        print(f"✅ Fix 4: Replaced {count} x '{old}' -> '{new}'")

# Fix 5: Fix text colors  
text_replacements = [
    ('text-slate-900', 'text-white'),
    ('text-slate-600', 'text-rtp-muted'),
    ('text-slate-700', 'text-white/80'),
]

for old, new in text_replacements:
    count = content.count(old)
    if count > 0:
        content = content.replace(old, new)
        print(f"✅ Fix 5: Replaced {count} x '{old}' -> '{new}'")

# Fix 6: Add SVG diagram after caption
old_caption = '''\t\t\t\t<div class="text-center mt-4 text-xs text-slate-400">
\t\t\t\t\tWe buy when momentum overcomes Theta decay.
\t\t\t\t</div>'''

new_caption = '''\t\t\t\t<!-- Momentum vs Theta Decay Diagram -->
\t\t\t\t<svg class="w-full h-48" viewBox="0 0 400 200" aria-hidden="true">
\t\t\t\t\t<defs>
\t\t\t\t\t\t<linearGradient id="momentumGrad" x1="0%" y1="0%" x2="0%" y2="100%">
\t\t\t\t\t\t\t<stop offset="0%" style="stop-color:#3b82f6;stop-opacity:0.3" />
\t\t\t\t\t\t\t<stop offset="100%" style="stop-color:#3b82f6;stop-opacity:0" />
\t\t\t\t\t\t</linearGradient>
\t\t\t\t\t</defs>
\t\t\t\t\t<!-- Grid lines -->
\t\t\t\t\t<line x1="40" y1="20" x2="40" y2="160" stroke="#334155" stroke-width="1" />
\t\t\t\t\t<line x1="40" y1="160" x2="380" y2="160" stroke="#334155" stroke-width="1" />
\t\t\t\t\t<!-- Theta Decay curve (downward) -->
\t\t\t\t\t<path d="M40,40 Q120,60 200,100 T360,140" fill="none" stroke="#64748b" stroke-width="2" stroke-dasharray="5,5" />
\t\t\t\t\t<!-- Momentum curve (upward breakout) -->
\t\t\t\t\t<path d="M40,140 Q150,130 220,80 T360,30" fill="url(#momentumGrad)" stroke="#3b82f6" stroke-width="3" />
\t\t\t\t\t<!-- Entry point marker -->
\t\t\t\t\t<circle cx="220" cy="80" r="6" fill="#10b981" stroke="#fff" stroke-width="2" />
\t\t\t\t\t<!-- Labels -->
\t\t\t\t\t<text x="45" y="15" fill="#94a3b8" font-size="10">Price</text>
\t\t\t\t\t<text x="340" y="175" fill="#94a3b8" font-size="10">Time</text>
\t\t\t\t\t<text x="230" y="70" fill="#10b981" font-size="10" font-weight="bold">Entry</text>
\t\t\t\t</svg>
\t\t\t\t<div class="text-center mt-4 text-xs text-slate-400">
\t\t\t\t\tWe buy when momentum overcomes Theta decay.
\t\t\t\t</div>'''

if old_caption in content:
    content = content.replace(old_caption, new_caption)
    print("✅ Fix 6: SVG diagram added")
else:
    print("❌ Fix 6: Caption pattern not found")

# Write back
with open(file_path, 'w') as f:
    f.write(content)

print(f"\n✅ All fixes applied! New file size: {len(content)} characters")
