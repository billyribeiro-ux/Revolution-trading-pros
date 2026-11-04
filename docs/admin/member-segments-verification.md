# Member segment verification

Core rule evaluation, authoring and member targeting are locally verified; the wider member module remains incomplete.

- Fresh disposable database `admin_move_verify_segments_r50`: guarded proof passed109.99s,88migration versions/latest096. Includes all prior email/SMTP/Connections regressions.
- Actual local orders/refunds/currencies, tags, memberships and products drive rule results. Eight existing condition fields use bound values; numeric comparisons preserve Decimal precision. Static memberships use `user_member_segments`; legacy rule-bearing rows remain dynamic.
- CRUD validates data, records audits atomically, protects structural system segments and keeps superadmin-only deletion. Ordinary admins are explicitly rejected for deletion.
- Member listing now applies actual segment/tag/search/date/product/plan/spending filters and pagination. Spending tiers match existing UI values and use net completed USD order value; other currencies are not silently added to USD.
- Browser3 passed10.0s using only disposable local accounts/tags: segment creation/editing/reloads, matching-member navigation/reload showing exactly two accounts, and deletion/reload. Console/network checks and fixture cleanup passed.
- Member configuration proxies preserve DELETE assignment bodies and failure statuses, enforce admin versus superadmin parity, reject traversal and bound actual request bytes. Three proxy tests passed. Four client-contract and two asynchronous audience ordering/recovery regressions passed. Svelte0/0 and scopedlint0.

Earlier r48 omitted a required fixture product type. R49 used an ordinary admin for a system-delete test that requires superadmin. Both failed runs are excluded from passing evidence; r50 corrected fixtures without weakening authorization. Browser1 exposed missing tag action proxies; browser2 had an incorrectly escaped URL assertion. Browser3 is the passing run.

Open: detail drawer analytics and export, tag/filter backend validation and recorded usage, campaign actions and broader member workflows. The running API uses only the disposable local database; no live providers or customer data were used.

Analytics/export implementation evidence (browser verification pending): fresh `admin_move_verify_segments_r51` passed110.66s,89migration versions/latest097. Current matching members, net USD spend, account age, purchasing-member fraction, accepted-message/open-signal counts and purchased products come from stored facts. Missing averages/rates and missing comparable30day baselines are NULL. Daily captured counts are scoped to the unchanged rule version, with an hourly worker and compare-and-save barrier. Changed rules cannot inherit prior counts; no historic audience is inferred. Controlled historical count fixtures verify only arithmetic/version isolation. The worker, storage errors, CSV matching/formula escaping/exact decimal text and authorization have integration evidence. Browser download/error assertions are written but have not run against097 yet.

097 browser verification completed: analytics browser1 passed2tests13.0s, including actual saved/current data, unavailable means unmeasured, real downloaded CSV containing only the two matching disposable accounts, persistence after reload,503/malformed200/retry, and console/network assertions. Current full frontend suite97files2887passed/32existing skips; fullESLint0, Svelte0/0, rootcheck0warnings2110files and explicit8281build87Cloudflare rules passed. Remaining tag/filter/campaign/member workflows still block module/source completion.
