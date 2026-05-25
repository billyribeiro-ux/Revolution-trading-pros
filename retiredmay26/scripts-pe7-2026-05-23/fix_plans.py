with open('+page.svelte', 'r') as f:
    lines = f.readlines()

# Find line with "let selectedPlan" (around line 10)
insert_idx = None
for i, line in enumerate(lines):
    if "let selectedPlan:" in line and "= $state" in line:
        insert_idx = i + 1
        break

if insert_idx:
    plans_code = '''\n\tinterface Plan {
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
\t\t\tprice: 197,
\t\t\tperiod: '/mo',
\t\t\tperDay: '$6.56/day',
\t\t\tsavingsCopy: '',
\t\t\tcheckoutHref: '/checkout/monthly-room',
\t\t\tfeatured: false,
\t\t\tvariant: 'simple',
\t\t\tfeatures: ['Daily Live Trading', 'Discord Community', 'Watchlists & Alerts', 'Onboarding Course']
\t\t},
\t\t{
\t\t\tid: 'quarterly',
\t\t\tlabel: 'Quarterly',
\t\t\tprice: 497,
\t\t\tperiod: '/qtr',
\t\t\tperDay: '$5.52/day',
\t\t\tsavingsCopy: 'Most Popular — Save $94',
\t\t\tcheckoutHref: '/checkout/quarterly-room',
\t\t\tfeatured: true,
\t\t\tvariant: 'featured',
\t\t\tfeatures: ['Everything in Monthly', 'Options Masterclass Access', 'Small Account Strategy', 'Priority Support']
\t\t},
\t\t{
\t\t\tid: 'annual',
\t\t\tlabel: 'Annual',
\t\t\tprice: 1647,
\t\t\tperiod: '/yr',
\t\t\tperDay: '$4.51/day',
\t\t\tsavingsCopy: 'Best Value — Save $717',
\t\t\tcheckoutHref: '/checkout/annual-room',
\t\t\tfeatured: false,
\t\t\tvariant: 'highlight',
\t\t\tfeatures: ['Everything in Quarterly', '1-on-1 Strategy Session', 'Annual Members-Only Events', 'Direct DM Access']
\t\t}
\t];

\tconst minPrice = Math.min(...plans.map(p => p.price));
\tconst maxPrice = Math.max(...plans.map(p => p.price));
'''
    lines.insert(insert_idx, plans_code)
    
    with open('+page.svelte', 'w') as f:
        f.writelines(lines)
    print('SSOT plans array added')
else:
    print('Could not find insertion point')
