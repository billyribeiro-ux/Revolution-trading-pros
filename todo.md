# Tailwind-to-Scoped-CSS Migration TODO

Last updated: 2026-06-06
Branch: `codex/tailwind-to-page-css-migration`

## Source of Truth

Re-run this scan after each completed file:

```sh
rg -l "@apply|@reference|class:[A-Za-z0-9_-]+|class=\"[^\"]*\{[^\"]*\}|style=\"[^\"]*\{[^\"]*\}" frontend/src --glob '*.svelte'
```

Current evidence:

- `@apply` / `@reference`: 0 Svelte files remaining.
- Broad dynamic class/style migration scan: 496 Svelte files remaining.
- Active method: finish one larger file completely, validate it, update this TODO, update `changelog.md`, commit, push, then move to the next file.

## Completed Slices

- [x] `c536dda64` Clean site health background classes.
- [x] `0032efc91` Modernize admin settings class composition.
- [x] `65fff3755` Modernize board detail dynamic styling.
- [x] `92a1e19fa` Modernize abandoned cart dynamic styling.
- [x] `cefa85b36` Modernize automation detail styling.
- [x] `6e018e80e` Modernize contact detail class handling.
- [x] `eb42c7876` Modernize datasource page dynamic styling.
- [x] `1b489ef0c` Modernize CRM deals dynamic styling.
- [x] `26ec8d617` Modernize admin dashboard class composition.
- [x] `d47db2644` Modernize admin blog status classes.
- [x] `4858ad4cd` Modernize asset manager dynamic styling.
- [x] `2bf007c46` Modernize block editor class composition.
- [x] `0fbca9d03` Modernize SPX alert page dynamic styling.
- [x] `bce717ccf` Modernize navbar dynamic styling.
- [x] `36a2dc8bc` Modernize day trading room dynamic styling.
- [x] `df8b28c9f` Modernize VideoEmbed dynamic styling.
- [x] `2393b464d` Modernize small accounts page styling.
- [x] `c4d42d017` Modernize swing trading page styling.
- [x] `90505bcec` Modernize block settings panel styling.
- [x] `e0108c4f8` Modernize member form modal styling.
- [x] `9d7c1d783` Modernize admin media page styling.
- [x] `44f033fb1` Modernize about page icon class styling.
- [x] `c1c018176` Modernize explosive swings page styling.
- [x] `83eb8e388` Modernize past members page styling.
- [x] `38674e355` Modernize global component library styling.
- [x] `da34af91e` Modernize memberships page styling.
- [x] `2aaf57377` Modernize admin connections page styling.
- [x] `4b254bb8f` Modernize CRM templates toast styling.
- [x] `6a3a8fba3` Modernize CRM lead detail styling.
- [x] `9b267c75a` Modernize course detail drawer styling.
- [x] `744b59b8f` Modernize admin categories page styling.
- [x] `fe9559197` Modernize high octane scanner badges.
- [x] `52ea78418` Modernize hero binding syntax.
- [x] `326f0c98d` Modernize popup creation preview styles.
- [x] `356d12e48` Modernize popup modal styling bindings.
- [x] `dc57e424d` Modernize weekly publish modal bindings.
- [x] `768820da1` Modernize gallery block bindings.
- [x] `25082f7ce` Modernize mission page pillar classes.
- [x] `0c354db3e` Modernize video upload modal reactivity.
- [x] `325ab08c7` Modernize SEO analyzer bindings.
- [x] `0847a8eb1` Modernize dashboard sidebar bindings.
- [x] `41199e6ca` Modernize scheduling panel bindings.
- [x] `80df09e4c` Modernize CRM leads page bindings.
- [x] `0842c9246` Modernize member detail drawer bindings.
- [x] `8b4426860` Modernize courses page bindings.
- [x] `5b206ad7b` Modernize consent settings bindings.
- [x] `8bd9e9f6b` Modernize weekly hero bindings.
- [x] `2468a2977` Modernize admin members page bindings.
- [x] `2514262c4` Modernize email campaigns page bindings.
- [x] `0783ba210` Modernize automation edit page bindings.
- [x] `b7f44f88b` Modernize churned members page bindings.
- [x] `b36701519` Modernize image block bindings.
- [x] `94a7b4fee` Modernize day trading course page bindings.
- [x] `7ac82f449` Modernize revision history bindings.
- [x] `09b4acb94` Modernize subscription drawer bindings.
- [x] `f53890721` Modernize ETF resource page bindings.
- [x] `e2d09c9d7` Modernize dashboard page bindings.
- [x] `9ce0b653c` Modernize login form attachments.
- [x] `dfbc97d2f` Modernize blog create page bindings.
- [x] `2065a5c3d` Modernize CRM sequences page bindings.
- [x] `b765a465a` Modernize AI assistant bindings.
- [x] `ffaaef32e` Modernize admin toolbar bindings.
- [x] `f31b10b30` Modernize form field renderer bindings.
- [x] `1144f654b` Modernize countdown timer bindings.

## Current Priority Queue

- [x] `frontend/src/lib/components/dashboard/DashboardSidebar.svelte`
- [x] `frontend/src/lib/components/blog/BlockEditor/SchedulingPanel.svelte`
- [x] `frontend/src/routes/admin/crm/leads/+page.svelte`
- [x] `frontend/src/lib/components/admin/MemberDetailDrawer.svelte`
- [x] `frontend/src/routes/courses/+page.svelte`
- [x] `frontend/src/routes/admin/consent/settings/+page.svelte`
- [x] `frontend/src/routes/dashboard/explosive-swings/components/WeeklyHero.svelte`
- [x] `frontend/src/routes/admin/members/+page.svelte`
- [x] `frontend/src/routes/admin/email/campaigns/+page.svelte`
- [x] `frontend/src/routes/admin/crm/automations/[id]/edit/+page.svelte`
- [x] `frontend/src/routes/admin/members/churned/+page.svelte`
- [x] `frontend/src/lib/components/cms/blocks/media/ImageBlock.svelte`
- [x] `frontend/src/routes/courses/day-trading-masterclass/+page.svelte`
- [x] `frontend/src/lib/components/blog/BlockEditor/RevisionHistory.svelte`
- [x] `frontend/src/lib/components/admin/SubscriptionDetailDrawer.svelte`
- [x] `frontend/src/routes/resources/etf-stocks-list/+page.svelte`
- [x] `frontend/src/routes/dashboard/+page.svelte`
- [x] `frontend/src/lib/components/auth/LoginForm.svelte`
- [x] `frontend/src/routes/admin/blog/create/+page.svelte`
- [x] `frontend/src/routes/admin/crm/sequences/+page.svelte`
- [x] `frontend/src/lib/components/blog/BlockEditor/AIAssistant.svelte`
- [x] `frontend/src/lib/components/AdminToolbar.svelte`
- [x] `frontend/src/lib/components/forms/FormFieldRenderer.svelte`
- [x] `frontend/src/lib/components/CountdownTimer.svelte`
- [ ] `frontend/src/routes/admin/media/analytics/+page.svelte`
- [ ] `frontend/src/routes/admin/analytics/+page.svelte`
- [ ] `frontend/src/routes/admin/trading-rooms/[slug]/components/WeeklyVideoUploader.svelte`
- [ ] `frontend/src/lib/components/blog/BlockEditor/ImageUploader.svelte`
- [ ] `frontend/src/lib/components/blog/BlockEditor/PresetPicker.svelte`
- [ ] `frontend/src/routes/mentorship/+page.svelte`
- [ ] `frontend/src/routes/admin/watchlist/+page.svelte`
- [ ] `frontend/src/routes/checkout/thank-you/+page.svelte`
- [ ] `frontend/src/routes/resources/stock-indexes-list/+page.svelte`
- [ ] `frontend/src/routes/register/+page.svelte`
- [ ] `frontend/src/lib/components/admin/CourseFormModal.svelte`
- [ ] `frontend/src/routes/admin/crm/webhooks/[id]/edit/+page.svelte`
- [ ] `frontend/src/routes/admin/courses/+page.svelte`
- [ ] `frontend/src/routes/dashboard/indicators/[id]/+page.svelte`
- [ ] `frontend/src/routes/admin/crm/automations/new/+page.svelte`
- [ ] `frontend/src/routes/dashboard/day-trading-room/start-here/+page.svelte`
- [ ] `frontend/src/routes/dashboard/swing-trading-room/start-here/+page.svelte`
- [ ] `frontend/src/routes/courses/swing-trading-pro/+page.svelte`
- [ ] `frontend/src/routes/admin/crm/+page.svelte`
- [ ] `frontend/src/lib/consent/components/ConsentBannerDesigner.svelte`
- [ ] `frontend/src/lib/components/media/ImageCropModal.svelte`
- [ ] `frontend/src/lib/components/admin/SegmentDetailDrawer.svelte`
- [ ] `frontend/src/lib/components/blog/BlockEditor/VirtualBlockList.svelte`
- [ ] `frontend/src/routes/admin/seo/bing/+page.svelte`
- [ ] `frontend/src/routes/admin/members/analytics/+page.svelte`
- [ ] `frontend/src/lib/components/admin/SubscriptionFormModal.svelte`
- [ ] `frontend/src/routes/admin/seo/+page.svelte`
- [ ] `frontend/src/lib/consent/templates/TemplateEditor.svelte`
- [ ] `frontend/src/routes/dashboard/small-account-mentorship/start-here/+page.svelte`
- [ ] `frontend/src/routes/dashboard/day-trading-room/learning-center/+page.svelte`
- [ ] `frontend/src/routes/admin/members/subscriptions/+page.svelte`
- [ ] `frontend/src/routes/admin/trading-rooms/[slug]/components/TradeEntryManager.svelte`
- [ ] `frontend/src/routes/dashboard/small-account-mentorship/trader-store/[slug]/+page.svelte`
- [ ] `frontend/src/routes/dashboard/day-trading-room/trader-store/[slug]/+page.svelte`
- [ ] `frontend/src/routes/admin/crm/webhooks/+page.svelte`

## Per-File Completion Checklist

- [ ] Read the full file before editing.
- [ ] Convert only evidence-backed patterns in that file.
- [ ] Prefer Svelte 5 class arrays/objects over legacy `class:`.
- [ ] Prefer `style:` directives for dynamic style values and custom properties.
- [ ] Use `$derived`/`$derived.by` for computed state instead of assignment effects.
- [ ] Use Svelte reactive built-ins where MCP/autofixer identifies real mutable built-in state.
- [ ] Run Svelte MCP `svelte_autofixer` until zero issues and zero suggestions.
- [ ] Run targeted file scan until no flagged patterns remain in the completed file.
- [ ] Run `pnpm --dir frontend check`.
- [ ] Run `pnpm --dir frontend format:check`.
- [ ] Run `git diff --check`.
- [ ] Run `pnpm --dir frontend lint`.
- [ ] Run `pnpm --dir frontend build`.
- [ ] Update `todo.md`.
- [ ] Update `changelog.md` and keep `CHANGELOG.md` synchronized.
- [ ] Commit and push.

## Final Removal Work

- [ ] Continue the broad Svelte file sweep until the dynamic class/style scan is zero or every remaining hit is documented as a deliberate false positive.
- [ ] Re-run global zero-reference checks for Tailwind infrastructure and utility APIs.
- [ ] Remove Tailwind Vite plugin and packages only after zero-reference proof.
- [ ] Run final full gates and representative browser smoke checks.
