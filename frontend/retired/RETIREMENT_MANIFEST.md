# RETIREMENT MANIFEST

## Generated: 2026-01-26
## Forensic Investigation by: ICT Level 7+ Principal Engineer

---

## EXECUTIVE SUMMARY

| Category | Count | Action |
|----------|-------|--------|
| Orphaned Components (src/lib/components) | 110+ | Retire |
| Orphaned Route Components | 5 | Retire |
| Unused Utilities | 7 | Retire |
| Unused Stores | 2 | Retire |
| Dead Routes (No Links) | 54+ | Review/Retire |
| Unused CSS Files | 10 | Retire |
| Deprecated Syntax Files | 2 | Migrate |
| Storybook Files | 9 | Keep (Dev Only) |
| Implementation Reference Files | 93 | Keep (Design Reference) |

---

## 1. ORPHANED COMPONENTS (src/lib/components)

Files in `src/lib/components` with **ZERO imports** anywhere in the codebase.

### 1.1 Forms Pro Components (RETIRE ALL)
| File | Evidence | Retired Date |
|------|----------|--------------|
| src/lib/components/forms/pro/ChainedSelectField.svelte | 0 imports | |
| src/lib/components/forms/pro/TaxonomyField.svelte | 0 imports | |
| src/lib/components/forms/pro/NPSField.svelte | 0 imports | |
| src/lib/components/forms/pro/FormStyler.svelte | 0 imports | |
| src/lib/components/forms/pro/AddressField.svelte | 0 imports | |
| src/lib/components/forms/pro/PayPalPayment.svelte | 0 imports | |
| src/lib/components/forms/pro/EnhancedCheckbox.svelte | 0 imports | |
| src/lib/components/forms/pro/PostTitleField.svelte | 0 imports | |
| src/lib/components/forms/pro/RepeaterField.svelte | 0 imports | |
| src/lib/components/forms/pro/PostContentField.svelte | 0 imports | |
| src/lib/components/forms/pro/MolliePayment.svelte | 0 imports | |
| src/lib/components/forms/pro/FormStepField.svelte | 0 imports | |
| src/lib/components/forms/pro/RazorPayPayment.svelte | 0 imports | |
| src/lib/components/forms/pro/PaymentField.svelte | 0 imports | |
| src/lib/components/forms/pro/FeaturedImageField.svelte | 0 imports | |
| src/lib/components/forms/pro/PaymentMethodSelector.svelte | 0 imports | |
| src/lib/components/forms/pro/PostExcerptField.svelte | 0 imports | |
| src/lib/components/forms/pro/SquarePayment.svelte | 0 imports | |
| src/lib/components/forms/pro/InventoryField.svelte | 0 imports | |
| src/lib/components/forms/pro/ToggleField.svelte | 0 imports | |
| src/lib/components/forms/pro/FileUploadField.svelte | 0 imports | |
| src/lib/components/forms/pro/GeolocationAddress.svelte | 0 imports | |
| src/lib/components/forms/pro/SaveProgressButton.svelte | 0 imports | |
| src/lib/components/forms/pro/ColorPickerField.svelte | 0 imports | |
| src/lib/components/forms/pro/PdfDownload.svelte | 0 imports | |
| src/lib/components/forms/pro/PaymentSummary.svelte | 0 imports | |
| src/lib/components/forms/pro/DynamicField.svelte | 0 imports | |
| src/lib/components/forms/pro/CouponField.svelte | 0 imports | |
| src/lib/components/forms/pro/GDPRField.svelte | 0 imports | |
| src/lib/components/forms/pro/AccordionTabField.svelte | 0 imports | |
| src/lib/components/forms/pro/CalculatorField.svelte | 0 imports | |
| src/lib/components/forms/pro/RangeSliderField.svelte | 0 imports | |
| src/lib/components/forms/pro/StripePayment.svelte | 0 imports | |
| src/lib/components/forms/pro/SignatureField.svelte | 0 imports | |
| src/lib/components/forms/pro/AuthorizeNetPayment.svelte | 0 imports | |
| src/lib/components/forms/pro/AdminApprovalStatus.svelte | 0 imports | |
| src/lib/components/forms/pro/PhoneIntlField.svelte | 0 imports | |
| src/lib/components/forms/pro/PaddlePayment.svelte | 0 imports | |
| src/lib/components/forms/pro/FormReport.svelte | 0 imports | |
| src/lib/components/forms/pro/PaystackPayment.svelte | 0 imports | |

### 1.2 Forms Base Components (RETIRE ALL)
| File | Evidence | Retired Date |
|------|----------|--------------|
| src/lib/components/forms/MultiStepFormRenderer.svelte | 0 imports | |
| src/lib/components/forms/RepeaterField.svelte | 0 imports | |
| src/lib/components/forms/RelatedForms.svelte | 0 imports | |
| src/lib/components/forms/FormCollaborators.svelte | 0 imports | |
| src/lib/components/forms/ConditionalLogicBuilder.svelte | 0 imports | |
| src/lib/components/forms/FormImportExport.svelte | 0 imports | |
| src/lib/components/forms/FormAIAssistant.svelte | 0 imports | |
| src/lib/components/forms/QuizField.svelte | 0 imports | |
| src/lib/components/forms/FormEmbedGenerator.svelte | 0 imports | |
| src/lib/components/forms/FormEmbed.svelte | 0 imports | |
| src/lib/components/forms/FormAnalyticsDashboard.svelte | 0 imports | |
| src/lib/components/forms/fields/RepeaterField.svelte | 0 imports | |
| src/lib/components/forms/fields/CalculatedField.svelte | 0 imports | |
| src/lib/components/forms/fields/AddressAutocomplete.svelte | 0 imports | |

### 1.3 Admin Components (RETIRE ALL)
| File | Evidence | Retired Date |
|------|----------|--------------|
| src/lib/components/admin/AdminCard.svelte | 0 imports | |
| src/lib/components/admin/ServiceConnectionStatus.stories.svelte | 0 imports | |
| src/lib/components/admin/ConnectionGate.stories.svelte | 0 imports | |
| src/lib/components/admin/BulkEditModal.svelte | 0 imports | |
| src/lib/components/admin/VideoChaptersEditor.svelte | 0 imports | |
| src/lib/components/admin/ScheduledPublishing.svelte | 0 imports | |
| src/lib/components/admin/IndicatorBuilder.svelte | 0 imports | |
| src/lib/components/admin/CourseBuilder.svelte | 0 imports | |
| src/lib/components/admin/BulkUploadQueue.svelte | 0 imports | |
| src/lib/components/admin/VideoAnalyticsDashboard.svelte | 0 imports | |
| src/lib/components/admin/MediaUploadHub.svelte | 0 imports | |

### 1.4 Analytics Components (RETIRE ALL)
| File | Evidence | Retired Date |
|------|----------|--------------|
| src/lib/components/analytics/AIInsightsPanel.svelte | 0 imports | |
| src/lib/components/analytics/EventExplorer.svelte | 0 imports | |
| src/lib/components/analytics/SegmentList.svelte | 0 imports | |
| src/lib/components/analytics/RetentionCurve.svelte | 0 imports | |
| src/lib/components/analytics/BehaviorHeatmap.svelte | 0 imports | |
| src/lib/components/analytics/UserJourneyMap.svelte | 0 imports | |
| src/lib/components/analytics/RevenueBreakdown.svelte | 0 imports | |

### 1.5 Media Components (RETIRE ALL)
| File | Evidence | Retired Date |
|------|----------|--------------|
| src/lib/components/media/MediaAnalytics.svelte | 0 imports | |
| src/lib/components/media/OptimizationStats.svelte | 0 imports | |
| src/lib/components/media/MediaGrid.svelte | 0 imports | |
| src/lib/components/media/MediaUpload.svelte | 0 imports | |
| src/lib/components/media/MediaPreview.svelte | 0 imports | |
| src/lib/components/media/ImageComparison.svelte | 0 imports | |
| src/lib/components/media/ImageComparisonSlider.svelte | 0 imports | |
| src/lib/components/media/DuplicateDetector.svelte | 0 imports | |

### 1.6 Blog Components (RETIRE ALL)
| File | Evidence | Retired Date |
|------|----------|--------------|
| src/lib/components/blog/RichTextEditor.svelte | 0 imports | |
| src/lib/components/blog/BlockEditor/MediaLibrary.svelte | 0 imports | |
| src/lib/components/blog/BlockEditor/BlockToolbar.svelte | 0 imports | |
| src/lib/components/blog/BlockEditor/CollaborationPanel.svelte | 0 imports | |
| src/lib/components/blog/CodeBlock.svelte | 0 imports | |

### 1.7 Workflow Components (RETIRE ALL)
| File | Evidence | Retired Date |
|------|----------|--------------|
| src/lib/components/workflow/NodeProperties.svelte | 0 imports | |
| src/lib/components/workflow/NodePalette.svelte | 0 imports | |
| src/lib/components/workflow/WorkflowToolbar.svelte | 0 imports | |
| src/lib/components/workflow/AIInsights.svelte | 0 imports | |

### 1.8 Dashboard Components (RETIRE ALL)
| File | Evidence | Retired Date |
|------|----------|--------------|
| src/lib/components/dashboard/PerformanceChart.svelte | 0 imports | |
| src/lib/components/dashboard/DashboardContentSidebar.svelte | 0 imports | |
| src/lib/components/dashboard/stats/StatsBar.svelte | 0 imports | |

### 1.9 Other Orphaned Components (RETIRE ALL)
| File | Evidence | Retired Date |
|------|----------|--------------|
| src/lib/components/LazySection.svelte | 0 imports | |
| src/lib/components/BatchOperations.svelte | 0 imports | |
| src/lib/components/patterns/PageHeader.svelte | 0 imports | |
| src/lib/components/patterns/DataTable.svelte | 0 imports | |
| src/lib/components/patterns/ErrorBoundary.svelte | 0 imports | |
| src/lib/components/auth/LottieSuccess.svelte | 0 imports | |
| src/lib/components/layout/AppSidebar.svelte | 0 imports | |
| src/lib/components/DynamicIcon.svelte | 0 imports | |
| src/lib/components/PopupDisplay.svelte | 0 imports | |
| src/lib/components/charts/EnterpriseChart.svelte | 0 imports | |
| src/lib/components/ApiErrorDisplay.svelte | 0 imports | |
| src/lib/components/ssr/ClientOnly.svelte | 0 imports | |
| src/lib/components/courses/LessonNav.svelte | 0 imports | |
| src/lib/components/courses/CoursePlayer.svelte | 0 imports | |
| src/lib/components/courses/CourseDownloads.svelte | 0 imports | |
| src/lib/components/cart/AddToCartButton.svelte | 0 imports | |
| src/lib/components/DashboardWidgetManager.svelte | 0 imports | |
| src/lib/components/PopupModal.svelte | 0 imports | |
| src/lib/components/ErrorBoundary.svelte | 0 imports | |

---

## 2. ORPHANED ROUTE COMPONENTS

Components inside route `components/` folders not imported by their parent route.

| File | Evidence | Retired Date |
|------|----------|--------------|
| src/routes/admin/trading-rooms/[slug]/components/WeeklyVideoUploader.svelte | Not imported in route | |
| src/routes/admin/trading-rooms/[slug]/components/TradeEntryManager.svelte | Not imported in route | |
| src/routes/dashboard/explosive-swings/components/ErrorBanner.svelte | Not imported in route | |
| src/routes/dashboard/explosive-swings/components/VideoGrid.svelte | Not imported in route | |
| src/routes/dashboard/explosive-swings/components/AlertsFeed.svelte | Not imported in route | |

---

## 3. UNUSED UTILITIES

TypeScript modules in `src/lib/utils` with zero imports.

| File | Evidence | Retired Date |
|------|----------|--------------|
| src/lib/utils/webVitals.ts | 0 imports | |
| src/lib/utils/cart-helpers.ts | 0 imports | |
| src/lib/utils/logger.ts | 0 imports | |
| src/lib/utils/gravatar.ts | 0 imports | |
| src/lib/utils/lazyLoad.ts | 0 imports | |
| src/lib/utils/ssr.ts | 0 imports | |
| src/lib/utils/analytics-helpers.ts | 0 imports | |

---

## 4. UNUSED STORES

Store files in `src/lib/stores` with zero imports.

| File | Evidence | Retired Date |
|------|----------|--------------|
| src/lib/stores/auditLog.svelte.ts | 0 imports | |
| src/lib/stores/theme.svelte.ts | 0 imports | |

---

## 5. DEAD ROUTES (No Links)

Routes with **ZERO** `href` links or `goto()` calls pointing to them.

### 5.1 Test/Debug Routes (RETIRE)
| Route | File | Evidence |
|-------|------|----------|
| /behavior | src/routes/behavior/+page.svelte | No links |
| /test-backend | src/routes/test-backend/+page.svelte | No links |

### 5.2 Admin CRM Routes (REVIEW - May be linked dynamically)
| Route | File | Evidence |
|-------|------|----------|
| /admin/crm/settings | +page.svelte | No links |
| /admin/crm/lists | +page.svelte | No links |
| /admin/crm/leads/[id] | +page.svelte | No links |
| /admin/crm/contacts/[id] | +page.svelte | No links |
| /admin/crm/managers | +page.svelte | No links |
| /admin/crm/deals/[id] | +page.svelte | No links |
| /admin/crm/smart-links | +page.svelte | No links |
| /admin/crm/tags/[id] | +page.svelte | No links |
| /admin/crm/import-export | +page.svelte | No links |
| /admin/crm/logs | +page.svelte | No links |
| /admin/crm/sequences | +page.svelte | No links |
| /admin/crm/templates | +page.svelte | No links |
| /admin/crm/automations/[id] | +page.svelte | No links |
| /admin/crm/automations/[id]/edit | +page.svelte | No links |
| /admin/crm/campaigns | +page.svelte | No links |
| /admin/crm/webhooks/[id]/edit | +page.svelte | No links |
| /admin/crm/abandoned-carts | +page.svelte | No links |
| /admin/crm/recurring-campaigns | +page.svelte | No links |
| /admin/crm/companies | +page.svelte | No links |
| /admin/crm/segments | +page.svelte | No links |

### 5.3 Admin Other Routes (REVIEW)
| Route | File | Evidence |
|-------|------|----------|
| /admin/behavior | +page.svelte | No links |
| /admin/products/[id]/edit | +page.svelte | No links |
| /admin/forms/entries | +page.svelte | No links |
| /admin/forms/[id]/edit | +page.svelte | No links |
| /admin/forms/[id]/submissions | +page.svelte | No links |
| /admin/forms/[id]/analytics | +page.svelte | No links |
| /admin/trading-rooms/[slug] | +page.svelte | No links |
| /admin/indicators/[id] | +page.svelte | No links |
| /admin/page-builder | +page.svelte | No links |
| /admin/blog/edit/[id] | +page.svelte | No links |
| /admin/subscriptions/plans | +page.svelte | No links |
| /admin/subscriptions/invoice-settings | +page.svelte | No links |
| /admin/dashboard | +page.svelte | No links |
| /admin/coupons/edit/[id] | +page.svelte | No links |
| /admin/courses/[id] | +page.svelte | No links |
| /admin/courses/[id]/lessons/[lessonId] | +page.svelte | No links |
| /admin/courses/create | +page.svelte | No links |
| /admin/consent | +page.svelte | No links |
| /admin/cms/locales | +page.svelte | No links |
| /admin/cms/versions | +page.svelte | No links |
| /admin/cms/webhooks | +page.svelte | No links |
| /admin/cms/scheduled | +page.svelte | No links |

### 5.4 Public Routes (REVIEW)
| Route | File | Evidence |
|-------|------|----------|
| /embed/form/[slug] | +page.svelte | May be iframe embedded |
| /classes/[slug] | +page.svelte | Dynamic route |
| /chatroom-archive/[room_slug]/[date_slug] | +page.svelte | Dynamic route |
| /indicators/rsi | +page.svelte | No links |
| /indicators/macd | +page.svelte | No links |
| /indicators/[id] | +page.svelte | Dynamic route |
| /crm/contacts/[id] | +page.svelte | No links |
| /crm/deals/[id] | +page.svelte | No links |

---

## 6. UNUSED CSS FILES

| File | Evidence | Retired Date |
|------|----------|--------------|
| src/lib/styles/design-tokens.css | 0 imports | |
| src/lib/styles/retired/simpler-icons.css | Already in retired folder | |
| src/lib/styles/retired/performance.css | Already in retired folder | |
| src/lib/styles/tokens/typography.css | 0 imports | |
| src/lib/styles/tokens/spacing.css | 0 imports | |
| src/lib/styles/tokens/index.css | 0 imports | |
| src/lib/styles/tokens/shadows.css | 0 imports | |
| src/lib/styles/tokens/transitions.css | 0 imports | |
| src/lib/styles/base/reset.css | 0 imports | |
| src/lib/styles/base/global.css | 0 imports | |
| src/lib/styles/base/index.css | 0 imports | |

---

## 7. DEPRECATED SYNTAX FILES (MIGRATE)

Files using Svelte 4 syntax that need migration.

| File | Issue | Action |
|------|-------|--------|
| src/lib/components/media/ImageCropModal.svelte | Uses `on:` event handlers | Migrate to native events |
| src/routes/admin/crm/templates/+page.svelte | Uses `on:` event handlers | Migrate to native events |
| src/routes/+layout.svelte | Uses `<slot>` | Expected for root layout |

---

## 8. STORYBOOK FILES (KEEP - Dev Only)

These are for Storybook development and should **NOT** be retired.

| File | Purpose |
|------|---------|
| src/stories/Button.svelte | Storybook component |
| src/stories/Button.stories.svelte | Storybook story |
| src/stories/Header.svelte | Storybook component |
| src/stories/Header.stories.svelte | Storybook story |
| src/stories/Page.svelte | Storybook component |
| src/stories/Page.stories.svelte | Storybook story |
| src/stories/*.css | Storybook styles |
| src/stories/assets/* | Storybook assets |

---

## 9. IMPLEMENTATION FOLDER (KEEP - Design Reference)

The `Implementation/` folder contains **93 design reference files** (screenshots, mockups).
These are NOT code files and should be kept for design reference.

---

## 10. BACKEND FINDINGS

### 10.1 Potentially Unused Rust Modules
| File | Evidence | Action |
|------|----------|--------|
| src/bin/bootstrap_dev.rs | Not a standard module declaration | KEEP - Binary target |

### 10.2 Backend Summary
- **99 Rust files** total
- Backend is well-organized with proper module declarations
- No orphaned modules detected (all properly declared in mod.rs)

---

## RETIREMENT PROCESS

### Step 1: Verify
```bash
# Double-check a file is truly unused before retiring
grep -r "ComponentName" src --include="*.svelte" --include="*.ts"
```

### Step 2: Move
```bash
# Use git mv to preserve history
git mv src/lib/components/unused/File.svelte retired/components/File.svelte
```

### Step 3: Test
```bash
npm run build
npm run check
```

### Step 4: Commit
```bash
git add retired/
git commit -m "chore: retire unused ComponentName - zero imports found"
```

---

## RESTORATION PROCESS

If a retired file is needed again:

1. Check this manifest for original location
2. `git mv retired/<category>/<file> <original-location>`
3. Remove entry from this manifest
4. Document why it was restored

---

## SAFETY RULES

1. **NEVER auto-retire** - Always manually verify before moving
2. **NEVER delete** - Move to retired/, never `rm`
3. **ALWAYS test** - Run build after each retirement batch
4. **ALWAYS document** - Every retirement needs manifest entry
5. **PRESERVE git history** - Use `git mv` when possible
