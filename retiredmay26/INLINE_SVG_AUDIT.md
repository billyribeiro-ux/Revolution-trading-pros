# Inline Icon SVG Audit — Heroicons → Tabler Migration Candidates

**Generated:** 2026-04-28
**Branch:** payments-fix-2026-04
**Scope:** All inline `<svg viewBox="0 0 20 20">` and `<svg viewBox="0 0 24 24">` icon-shaped SVGs

These are handcrafted Heroicons-style path SVGs that could be replaced with `<Icon name="IconX" />` calls using the existing `@tabler/icons-svelte-runes` system at [src/lib/components/Icon.svelte](frontend/src/lib/components/Icon.svelte).

---

## Summary

- **Files affected:** 217
- **Total inline icon SVGs:** 850
- **Tabler component to replace with:** [`<Icon name="..." size="md" />`](frontend/src/lib/components/Icon.svelte)

---

## Files by impact (descending)

Files routed to areas you can browse in-app first; click any URL to jump to the file.

### Members dashboard — `routes/dashboard/`

Files you actually log in to use. **Highest priority.**

| Count | File | URL when running |
|---|---|---|
| 14 | [routes/dashboard/explosive-swings/components/WeeklyHero.svelte](frontend/src/routes/dashboard/explosive-swings/components/WeeklyHero.svelte) | /dashboard/explosive-swings |
| 11 | [routes/dashboard/explosive-swings/components/VideoUploadModal.svelte](frontend/src/routes/dashboard/explosive-swings/components/VideoUploadModal.svelte) | /dashboard/explosive-swings |
| 10 | [routes/dashboard/explosive-swings/components/PublishWeeklyModal.svelte](frontend/src/routes/dashboard/explosive-swings/components/PublishWeeklyModal.svelte) | /dashboard/explosive-swings |
| 9 | [routes/dashboard/explosive-swings/analytics/components/PerformanceOverview.svelte](frontend/src/routes/dashboard/explosive-swings/analytics/components/PerformanceOverview.svelte) | /dashboard/explosive-swings/analytics |
| 8 | [routes/dashboard/high-octane-scanner/+page.svelte](frontend/src/routes/dashboard/high-octane-scanner/+page.svelte) | /dashboard/high-octane-scanner |
| 8 | [routes/dashboard/explosive-swings/search/components/SearchFilters.svelte](frontend/src/routes/dashboard/explosive-swings/search/components/SearchFilters.svelte) | /dashboard/explosive-swings/search |
| 7 | [routes/dashboard/explosive-swings/components/ResourceLinks.svelte](frontend/src/routes/dashboard/explosive-swings/components/ResourceLinks.svelte) | /dashboard/explosive-swings |
| 6 | [routes/dashboard/explosive-swings/components/ExportMenu.svelte](frontend/src/routes/dashboard/explosive-swings/components/ExportMenu.svelte) | /dashboard/explosive-swings |
| 6 | [routes/dashboard/explosive-swings/components/AlertCard.svelte](frontend/src/routes/dashboard/explosive-swings/components/AlertCard.svelte) | /dashboard/explosive-swings |
| 5 | [routes/dashboard/explosive-swings/components/TradeEntryModal.svelte](frontend/src/routes/dashboard/explosive-swings/components/TradeEntryModal.svelte) | /dashboard/explosive-swings |
| 5 | [routes/dashboard/explosive-swings/components/EmptyState.svelte](frontend/src/routes/dashboard/explosive-swings/components/EmptyState.svelte) | /dashboard/explosive-swings |
| 5 | [routes/dashboard/explosive-swings/components/ClosePositionModal.svelte](frontend/src/routes/dashboard/explosive-swings/components/ClosePositionModal.svelte) | /dashboard/explosive-swings |
| 5 | [routes/dashboard/explosive-swings/components/ActivePositionCard.svelte](frontend/src/routes/dashboard/explosive-swings/components/ActivePositionCard.svelte) | /dashboard/explosive-swings |
| 5 | [routes/dashboard/explosive-swings/archive/+page.svelte](frontend/src/routes/dashboard/explosive-swings/archive/+page.svelte) | /dashboard/explosive-swings/archive |
| 5 | [routes/dashboard/explosive-swings/analytics/components/StreakIndicator.svelte](frontend/src/routes/dashboard/explosive-swings/analytics/components/StreakIndicator.svelte) | /dashboard/explosive-swings/analytics |
| 4 | [routes/dashboard/explosive-swings/search/components/SearchInput.svelte](frontend/src/routes/dashboard/explosive-swings/search/components/SearchInput.svelte) | /dashboard/explosive-swings/search |
| 4 | [routes/dashboard/explosive-swings/components/PerformanceSummary.svelte](frontend/src/routes/dashboard/explosive-swings/components/PerformanceSummary.svelte) | /dashboard/explosive-swings |
| 4 | [routes/dashboard/classes/+page.svelte](frontend/src/routes/dashboard/classes/+page.svelte) | /dashboard/classes |
| 3 | [routes/dashboard/explosive-swings/trades/+page.svelte](frontend/src/routes/dashboard/explosive-swings/trades/+page.svelte) | /dashboard/explosive-swings/trades |
| 3 | [routes/dashboard/explosive-swings/search/components/SearchResults.svelte](frontend/src/routes/dashboard/explosive-swings/search/components/SearchResults.svelte) | /dashboard/explosive-swings/search |
| 3 | [routes/dashboard/explosive-swings/invalidated/+page.svelte](frontend/src/routes/dashboard/explosive-swings/invalidated/+page.svelte) | /dashboard/explosive-swings/invalidated |
| 3 | [routes/dashboard/explosive-swings/components/VideoGrid.svelte](frontend/src/routes/dashboard/explosive-swings/components/VideoGrid.svelte) | /dashboard/explosive-swings |
| 3 | [routes/dashboard/explosive-swings/components/AddTradeModal.svelte](frontend/src/routes/dashboard/explosive-swings/components/AddTradeModal.svelte) | /dashboard/explosive-swings |
| 2 | [routes/dashboard/small-account-mentorship/favorites/+page.svelte](frontend/src/routes/dashboard/small-account-mentorship/favorites/+page.svelte) | /dashboard/small-account-mentorship/favorites |
| 2 | [routes/dashboard/explosive-swings/video/[slug]/+page.svelte](frontend/src/routes/dashboard/explosive-swings/video/[slug]/+page.svelte) | /dashboard/explosive-swings/video/[slug] |
| 2 | [routes/dashboard/explosive-swings/trades/components/TradesTable.svelte](frontend/src/routes/dashboard/explosive-swings/trades/components/TradesTable.svelte) | /dashboard/explosive-swings/trades |
| 2 | [routes/dashboard/explosive-swings/trades/components/ErrorState.svelte](frontend/src/routes/dashboard/explosive-swings/trades/components/ErrorState.svelte) | /dashboard/explosive-swings/trades |
| 2 | [routes/dashboard/explosive-swings/search/+page.svelte](frontend/src/routes/dashboard/explosive-swings/search/+page.svelte) | /dashboard/explosive-swings/search |
| 2 | [routes/dashboard/explosive-swings/favorites/+page.svelte](frontend/src/routes/dashboard/explosive-swings/favorites/+page.svelte) | /dashboard/explosive-swings/favorites |
| 2 | [routes/dashboard/explosive-swings/components/WeeklyVideoCard.svelte](frontend/src/routes/dashboard/explosive-swings/components/WeeklyVideoCard.svelte) | /dashboard/explosive-swings |
| 2 | [routes/dashboard/explosive-swings/components/VideoModal.svelte](frontend/src/routes/dashboard/explosive-swings/components/VideoModal.svelte) | /dashboard/explosive-swings |
| 2 | [routes/dashboard/explosive-swings/components/VideoCard.svelte](frontend/src/routes/dashboard/explosive-swings/components/VideoCard.svelte) | /dashboard/explosive-swings |
| 2 | [routes/dashboard/explosive-swings/components/UpdatePositionModal.svelte](frontend/src/routes/dashboard/explosive-swings/components/UpdatePositionModal.svelte) | /dashboard/explosive-swings |
| 2 | [routes/dashboard/explosive-swings/components/TickerPill.svelte](frontend/src/routes/dashboard/explosive-swings/components/TickerPill.svelte) | /dashboard/explosive-swings |
| 2 | [routes/dashboard/explosive-swings/components/LatestUpdatesCard.svelte](frontend/src/routes/dashboard/explosive-swings/components/LatestUpdatesCard.svelte) | /dashboard/explosive-swings |
| 2 | [routes/dashboard/explosive-swings/components/ErrorBoundary.svelte](frontend/src/routes/dashboard/explosive-swings/components/ErrorBoundary.svelte) | /dashboard/explosive-swings |
| 2 | [routes/dashboard/explosive-swings/components/ErrorBanner.svelte](frontend/src/routes/dashboard/explosive-swings/components/ErrorBanner.svelte) | /dashboard/explosive-swings |
| 2 | [routes/dashboard/explosive-swings/components/ApiErrorDisplay.svelte](frontend/src/routes/dashboard/explosive-swings/components/ApiErrorDisplay.svelte) | /dashboard/explosive-swings |
| 2 | [routes/dashboard/explosive-swings/components/AlertsFeed.svelte](frontend/src/routes/dashboard/explosive-swings/components/AlertsFeed.svelte) | /dashboard/explosive-swings |
| 2 | [routes/dashboard/explosive-swings/analytics/+page.svelte](frontend/src/routes/dashboard/explosive-swings/analytics/+page.svelte) | /dashboard/explosive-swings/analytics |
| 2 | [routes/dashboard/explosive-swings/+page.svelte](frontend/src/routes/dashboard/explosive-swings/+page.svelte) | /dashboard/explosive-swings |
| 1 | [routes/dashboard/explosive-swings/watchlist/+page.svelte](frontend/src/routes/dashboard/explosive-swings/watchlist/+page.svelte) | /dashboard/explosive-swings/watchlist |
| 1 | [routes/dashboard/explosive-swings/video-library/+page.svelte](frontend/src/routes/dashboard/explosive-swings/video-library/+page.svelte) | /dashboard/explosive-swings/video-library |
| 1 | [routes/dashboard/explosive-swings/trades/components/EmptyState.svelte](frontend/src/routes/dashboard/explosive-swings/trades/components/EmptyState.svelte) | /dashboard/explosive-swings/trades |
| 1 | [routes/dashboard/explosive-swings/search/components/TradeResultCard.svelte](frontend/src/routes/dashboard/explosive-swings/search/components/TradeResultCard.svelte) | /dashboard/explosive-swings/search |
| 1 | [routes/dashboard/explosive-swings/search/components/TradePlanResultCard.svelte](frontend/src/routes/dashboard/explosive-swings/search/components/TradePlanResultCard.svelte) | /dashboard/explosive-swings/search |
| 1 | [routes/dashboard/explosive-swings/search/components/SearchEmptyState.svelte](frontend/src/routes/dashboard/explosive-swings/search/components/SearchEmptyState.svelte) | /dashboard/explosive-swings/search |
| 1 | [routes/dashboard/explosive-swings/search/components/AlertResultCard.svelte](frontend/src/routes/dashboard/explosive-swings/search/components/AlertResultCard.svelte) | /dashboard/explosive-swings/search |
| 1 | [routes/dashboard/explosive-swings/notifications/AlertNotificationManager.svelte](frontend/src/routes/dashboard/explosive-swings/notifications/AlertNotificationManager.svelte) | /dashboard/explosive-swings/notifications/AlertNotificationManager.svelte |
| 1 | [routes/dashboard/explosive-swings/components/Sidebar.svelte](frontend/src/routes/dashboard/explosive-swings/components/Sidebar.svelte) | /dashboard/explosive-swings |
| 1 | [routes/dashboard/explosive-swings/components/NewAlertToast.svelte](frontend/src/routes/dashboard/explosive-swings/components/NewAlertToast.svelte) | /dashboard/explosive-swings |
| 1 | [routes/dashboard/explosive-swings/components/InvalidatePositionModal.svelte](frontend/src/routes/dashboard/explosive-swings/components/InvalidatePositionModal.svelte) | /dashboard/explosive-swings |
| 1 | [routes/dashboard/explosive-swings/alerts/+page.svelte](frontend/src/routes/dashboard/explosive-swings/alerts/+page.svelte) | /dashboard/explosive-swings/alerts |
| 1 | [routes/dashboard/[room_slug]/daily-videos/+page.svelte](frontend/src/routes/dashboard/[room_slug]/daily-videos/+page.svelte) | /dashboard/[room_slug]/daily-videos |

### Marketing & public pages — `routes/` (non-dashboard, non-admin)

| Count | File | URL when running |
|---|---|---|
| 30 | [routes/resources/+page.svelte](frontend/src/routes/resources/+page.svelte) | /resources |
| 29 | [routes/alerts/spx-profit-pulse/+page.svelte](frontend/src/routes/alerts/spx-profit-pulse/+page.svelte) | /alerts/spx-profit-pulse |
| 18 | [routes/our-mission/+page.svelte](frontend/src/routes/our-mission/+page.svelte) | /our-mission |
| 14 | [routes/live-trading-rooms/swing-trading/+page.svelte](frontend/src/routes/live-trading-rooms/swing-trading/+page.svelte) | /live-trading-rooms/swing-trading |
| 14 | [routes/alerts/explosive-swings/+page.svelte](frontend/src/routes/alerts/explosive-swings/+page.svelte) | /alerts/explosive-swings |
| 12 | [routes/alerts/[slug]/checkout/+page.svelte](frontend/src/routes/alerts/[slug]/checkout/+page.svelte) | /alerts/[slug]/checkout |
| 11 | [routes/live-trading-rooms/small-accounts/+page.svelte](frontend/src/routes/live-trading-rooms/small-accounts/+page.svelte) | /live-trading-rooms/small-accounts |
| 9 | [routes/my/subscriptions/+page.svelte](frontend/src/routes/my/subscriptions/+page.svelte) | /my/subscriptions |
| 9 | [routes/mentorship/+page.svelte](frontend/src/routes/mentorship/+page.svelte) | /mentorship |
| 9 | [routes/live-trading-rooms/+page.svelte](frontend/src/routes/live-trading-rooms/+page.svelte) | /live-trading-rooms |
| 9 | [routes/account/sessions/+page.svelte](frontend/src/routes/account/sessions/+page.svelte) | /account/sessions |
| 8 | [routes/account/+page.svelte](frontend/src/routes/account/+page.svelte) | /account |
| 7 | [routes/live-trading-rooms/day-trading/+page.svelte](frontend/src/routes/live-trading-rooms/day-trading/+page.svelte) | /live-trading-rooms/day-trading |
| 7 | [routes/blog/+error.svelte](frontend/src/routes/blog/+error.svelte) | /blog/+error.svelte |
| 6 | [routes/my/indicators/[slug]/+page.svelte](frontend/src/routes/my/indicators/[slug]/+page.svelte) | /my/indicators/[slug] |
| 6 | [routes/checkout/thank-you/+page.svelte](frontend/src/routes/checkout/thank-you/+page.svelte) | /checkout/thank-you |
| 6 | [routes/blog/+page.svelte](frontend/src/routes/blog/+page.svelte) | /blog |
| 4 | [routes/my/indicators/+page.svelte](frontend/src/routes/my/indicators/+page.svelte) | /my/indicators |
| 4 | [routes/my/courses/+page.svelte](frontend/src/routes/my/courses/+page.svelte) | /my/courses |
| 3 | [routes/cookie-policy/+page.svelte](frontend/src/routes/cookie-policy/+page.svelte) | /cookie-policy |
| 3 | [routes/blog/[slug]/+page.svelte](frontend/src/routes/blog/[slug]/+page.svelte) | /blog/[slug] |
| 2 | [routes/workflows/+page.svelte](frontend/src/routes/workflows/+page.svelte) | /workflows |
| 2 | [routes/auth/callback/+page.svelte](frontend/src/routes/auth/callback/+page.svelte) | /auth/callback |
| 2 | [routes/alerts/+page.svelte](frontend/src/routes/alerts/+page.svelte) | /alerts |
| 1 | [routes/signup/+page.svelte](frontend/src/routes/signup/+page.svelte) | /signup |
| 1 | [routes/resources/stock-indexes-list/+page.svelte](frontend/src/routes/resources/stock-indexes-list/+page.svelte) | /resources/stock-indexes-list |
| 1 | [routes/reset-password/+page.svelte](frontend/src/routes/reset-password/+page.svelte) | /reset-password |
| 1 | [routes/forgot-password/+page.svelte](frontend/src/routes/forgot-password/+page.svelte) | /forgot-password |
| 1 | [routes/classes/+page.svelte](frontend/src/routes/classes/+page.svelte) | /classes |
| 1 | [routes/checkout/+page.svelte](frontend/src/routes/checkout/+page.svelte) | /checkout |

### Admin panel — `routes/admin/`

| Count | File | URL when running |
|---|---|---|
| 2 | [routes/admin/analytics/goals/+page.svelte](frontend/src/routes/admin/analytics/goals/+page.svelte) | /admin/analytics/goals |
| 1 | [routes/admin/subscriptions/invoice-settings/+page.svelte](frontend/src/routes/admin/subscriptions/invoice-settings/+page.svelte) | /admin/subscriptions/invoice-settings |
| 1 | [routes/admin/analytics/recordings/+page.svelte](frontend/src/routes/admin/analytics/recordings/+page.svelte) | /admin/analytics/recordings |

### Reusable components — `lib/components/`

These appear on multiple pages — fixing them has the biggest reach.

| Count | File |
|---|---|
| 32 | [lib/components/blog/BlockEditor/AssetManager.svelte](frontend/src/lib/components/blog/BlockEditor/AssetManager.svelte) |
| 12 | [lib/components/forms/pro/PdfDownload.svelte](frontend/src/lib/components/forms/pro/PdfDownload.svelte) |
| 12 | [lib/components/forms/pro/DoubleOptIn.svelte](frontend/src/lib/components/forms/pro/DoubleOptIn.svelte) |
| 11 | [lib/components/resources/ResourceViewer.svelte](frontend/src/lib/components/resources/ResourceViewer.svelte) |
| 11 | [lib/components/media/MediaGrid.svelte](frontend/src/lib/components/media/MediaGrid.svelte) |
| 11 | [lib/components/forms/pro/AdminApprovalStatus.svelte](frontend/src/lib/components/forms/pro/AdminApprovalStatus.svelte) |
| 9 | [lib/components/blog/BlockEditor/ImageUploader.svelte](frontend/src/lib/components/blog/BlockEditor/ImageUploader.svelte) |
| 8 | [lib/components/forms/pro/SaveProgressButton.svelte](frontend/src/lib/components/forms/pro/SaveProgressButton.svelte) |
| 8 | [lib/components/admin/ServiceConnectionStatus.svelte](frontend/src/lib/components/admin/ServiceConnectionStatus.svelte) |
| 7 | [lib/components/resources/ResourceCard.svelte](frontend/src/lib/components/resources/ResourceCard.svelte) |
| 7 | [lib/components/resources/ResourceAnalytics.svelte](frontend/src/lib/components/resources/ResourceAnalytics.svelte) |
| 7 | [lib/components/media/DuplicateDetector.svelte](frontend/src/lib/components/media/DuplicateDetector.svelte) |
| 7 | [lib/components/forms/pro/FormReport.svelte](frontend/src/lib/components/forms/pro/FormReport.svelte) |
| 7 | [lib/components/forms/RepeaterField.svelte](frontend/src/lib/components/forms/RepeaterField.svelte) |
| 7 | [lib/components/dashboard/alerts/AlertCard.svelte](frontend/src/lib/components/dashboard/alerts/AlertCard.svelte) |
| 7 | [lib/components/blog/SocialShare.svelte](frontend/src/lib/components/blog/SocialShare.svelte) |
| 7 | [lib/components/admin/RoomSelector.svelte](frontend/src/lib/components/admin/RoomSelector.svelte) |
| 6 | [lib/components/media/UploadProgress.svelte](frontend/src/lib/components/media/UploadProgress.svelte) |
| 6 | [lib/components/media/OptimizationStats.svelte](frontend/src/lib/components/media/OptimizationStats.svelte) |
| 6 | [lib/components/media/MediaAnalytics.svelte](frontend/src/lib/components/media/MediaAnalytics.svelte) |
| 6 | [lib/components/media/ImageComparison.svelte](frontend/src/lib/components/media/ImageComparison.svelte) |
| 6 | [lib/components/forms/pro/PaddlePayment.svelte](frontend/src/lib/components/forms/pro/PaddlePayment.svelte) |
| 6 | [lib/components/admin/VideoUploader.svelte](frontend/src/lib/components/admin/VideoUploader.svelte) |
| 6 | [lib/components/admin/BunnyVideoUploader.svelte](frontend/src/lib/components/admin/BunnyVideoUploader.svelte) |
| 5 | [lib/consent/templates/BannerRenderer.svelte](frontend/src/lib/consent/templates/BannerRenderer.svelte) |
| 5 | [lib/components/media/ImageCropModal.svelte](frontend/src/lib/components/media/ImageCropModal.svelte) |
| 5 | [lib/components/forms/pro/PaymentMethodSelector.svelte](frontend/src/lib/components/forms/pro/PaymentMethodSelector.svelte) |
| 5 | [lib/components/forms/MultiStepFormRenderer.svelte](frontend/src/lib/components/forms/MultiStepFormRenderer.svelte) |
| 5 | [lib/components/courses/VideoUploader.svelte](frontend/src/lib/components/courses/VideoUploader.svelte) |
| 5 | [lib/components/courses/CourseReviews.svelte](frontend/src/lib/components/courses/CourseReviews.svelte) |
| 5 | [lib/components/cms/blocks/layout/GroupBlock.svelte](frontend/src/lib/components/cms/blocks/layout/GroupBlock.svelte) |
| 5 | [lib/components/ClassVideos.svelte](frontend/src/lib/components/ClassVideos.svelte) |
| 4 | [lib/consent/templates/TemplatePreviewCard.svelte](frontend/src/lib/consent/templates/TemplatePreviewCard.svelte) |
| 4 | [lib/components/workflow/WorkflowAnalytics.svelte](frontend/src/lib/components/workflow/WorkflowAnalytics.svelte) |
| 4 | [lib/components/forms/pro/SignatureField.svelte](frontend/src/lib/components/forms/pro/SignatureField.svelte) |
| 4 | [lib/components/forms/pro/RepeaterField.svelte](frontend/src/lib/components/forms/pro/RepeaterField.svelte) |
| 4 | [lib/components/forms/pro/GeolocationAddress.svelte](frontend/src/lib/components/forms/pro/GeolocationAddress.svelte) |
| 4 | [lib/components/forms/pro/FormStepField.svelte](frontend/src/lib/components/forms/pro/FormStepField.svelte) |
| 4 | [lib/components/courses/QuizPlayer.svelte](frontend/src/lib/components/courses/QuizPlayer.svelte) |
| 4 | [lib/components/cms/blocks/media/VideoBlock.svelte](frontend/src/lib/components/cms/blocks/media/VideoBlock.svelte) |
| 4 | [lib/components/blog/TableOfContents.svelte](frontend/src/lib/components/blog/TableOfContents.svelte) |
| 4 | [lib/components/blog/BlockEditor/PerformanceOverlay.svelte](frontend/src/lib/components/blog/BlockEditor/PerformanceOverlay.svelte) |
| 3 | [lib/components/video/VideoTranscript.svelte](frontend/src/lib/components/video/VideoTranscript.svelte) |
| 3 | [lib/components/ui/DatePicker.svelte](frontend/src/lib/components/ui/DatePicker.svelte) |
| 3 | [lib/components/resources/StockListViewer.svelte](frontend/src/lib/components/resources/StockListViewer.svelte) |
| 3 | [lib/components/resources/ResourceGrid.svelte](frontend/src/lib/components/resources/ResourceGrid.svelte) |
| 3 | [lib/components/resources/FavoriteButton.svelte](frontend/src/lib/components/resources/FavoriteButton.svelte) |
| 3 | [lib/components/forms/pro/StripePayment.svelte](frontend/src/lib/components/forms/pro/StripePayment.svelte) |
| 3 | [lib/components/forms/pro/SquarePayment.svelte](frontend/src/lib/components/forms/pro/SquarePayment.svelte) |
| 3 | [lib/components/forms/pro/PaymentSummary.svelte](frontend/src/lib/components/forms/pro/PaymentSummary.svelte) |
| 3 | [lib/components/forms/pro/InventoryField.svelte](frontend/src/lib/components/forms/pro/InventoryField.svelte) |
| 3 | [lib/components/forms/pro/GDPRField.svelte](frontend/src/lib/components/forms/pro/GDPRField.svelte) |
| 3 | [lib/components/forms/pro/EnhancedCheckbox.svelte](frontend/src/lib/components/forms/pro/EnhancedCheckbox.svelte) |
| 3 | [lib/components/forms/pro/CouponField.svelte](frontend/src/lib/components/forms/pro/CouponField.svelte) |
| 3 | [lib/components/forms/pro/AuthorizeNetPayment.svelte](frontend/src/lib/components/forms/pro/AuthorizeNetPayment.svelte) |
| 3 | [lib/components/forms/QuizField.svelte](frontend/src/lib/components/forms/QuizField.svelte) |
| 2 | [lib/components/workflow/WorkflowCanvas.svelte](frontend/src/lib/components/workflow/WorkflowCanvas.svelte) |
| 2 | [lib/components/ui/MobileResponsiveTable.svelte](frontend/src/lib/components/ui/MobileResponsiveTable.svelte) |
| 2 | [lib/components/ui/FileDropZone.svelte](frontend/src/lib/components/ui/FileDropZone.svelte) |
| 2 | [lib/components/resources/RecentlyAccessed.svelte](frontend/src/lib/components/resources/RecentlyAccessed.svelte) |
| 2 | [lib/components/media/MediaUpload.svelte](frontend/src/lib/components/media/MediaUpload.svelte) |
| 2 | [lib/components/media/MediaPreview.svelte](frontend/src/lib/components/media/MediaPreview.svelte) |
| 2 | [lib/components/media/ImageComparisonSlider.svelte](frontend/src/lib/components/media/ImageComparisonSlider.svelte) |
| 2 | [lib/components/media/DropZone.svelte](frontend/src/lib/components/media/DropZone.svelte) |
| 2 | [lib/components/forms/pro/RazorPayPayment.svelte](frontend/src/lib/components/forms/pro/RazorPayPayment.svelte) |
| 2 | [lib/components/forms/pro/PaystackPayment.svelte](frontend/src/lib/components/forms/pro/PaystackPayment.svelte) |
| 2 | [lib/components/forms/pro/PayPalPayment.svelte](frontend/src/lib/components/forms/pro/PayPalPayment.svelte) |
| 2 | [lib/components/forms/pro/MolliePayment.svelte](frontend/src/lib/components/forms/pro/MolliePayment.svelte) |
| 2 | [lib/components/forms/pro/FileUploadField.svelte](frontend/src/lib/components/forms/pro/FileUploadField.svelte) |
| 2 | [lib/components/forms/pro/FeaturedImageField.svelte](frontend/src/lib/components/forms/pro/FeaturedImageField.svelte) |
| 2 | [lib/components/dashboard/video/VideoCard.svelte](frontend/src/lib/components/dashboard/video/VideoCard.svelte) |
| 2 | [lib/components/dashboard/pagination/Pagination.svelte](frontend/src/lib/components/dashboard/pagination/Pagination.svelte) |
| 2 | [lib/components/dashboard/WatchHistory.svelte](frontend/src/lib/components/dashboard/WatchHistory.svelte) |
| 2 | [lib/components/dashboard/TradingRoomArchive.svelte](frontend/src/lib/components/dashboard/TradingRoomArchive.svelte) |
| 2 | [lib/components/consent/ConsentBanner.svelte](frontend/src/lib/components/consent/ConsentBanner.svelte) |
| 2 | [lib/components/cms/blocks/content/ListBlock.svelte](frontend/src/lib/components/cms/blocks/content/ListBlock.svelte) |
| 2 | [lib/components/blog/FloatingTocWidget.svelte](frontend/src/lib/components/blog/FloatingTocWidget.svelte) |
| 2 | [lib/components/auth/SocialLoginButtons.svelte](frontend/src/lib/components/auth/SocialLoginButtons.svelte) |
| 2 | [lib/components/ClassDownloads.svelte](frontend/src/lib/components/ClassDownloads.svelte) |
| 1 | [lib/icons/IconX.svelte](frontend/src/lib/icons/IconX.svelte) |
| 1 | [lib/icons/IconVolumeOff.svelte](frontend/src/lib/icons/IconVolumeOff.svelte) |
| 1 | [lib/icons/IconVolume.svelte](frontend/src/lib/icons/IconVolume.svelte) |
| 1 | [lib/icons/IconVideo.svelte](frontend/src/lib/icons/IconVideo.svelte) |
| 1 | [lib/icons/IconStar.svelte](frontend/src/lib/icons/IconStar.svelte) |
| 1 | [lib/icons/IconSquare.svelte](frontend/src/lib/icons/IconSquare.svelte) |
| 1 | [lib/icons/IconSparkles.svelte](frontend/src/lib/icons/IconSparkles.svelte) |
| 1 | [lib/icons/IconPlus.svelte](frontend/src/lib/icons/IconPlus.svelte) |
| 1 | [lib/icons/IconPlayerPlay.svelte](frontend/src/lib/icons/IconPlayerPlay.svelte) |
| 1 | [lib/icons/IconPlayerPause.svelte](frontend/src/lib/icons/IconPlayerPause.svelte) |
| 1 | [lib/icons/IconPhoto.svelte](frontend/src/lib/icons/IconPhoto.svelte) |
| 1 | [lib/icons/IconMinus.svelte](frontend/src/lib/icons/IconMinus.svelte) |
| 1 | [lib/icons/IconMaximize.svelte](frontend/src/lib/icons/IconMaximize.svelte) |
| 1 | [lib/icons/IconList.svelte](frontend/src/lib/icons/IconList.svelte) |
| 1 | [lib/icons/IconLink.svelte](frontend/src/lib/icons/IconLink.svelte) |
| 1 | [lib/icons/IconLayoutGrid.svelte](frontend/src/lib/icons/IconLayoutGrid.svelte) |
| 1 | [lib/icons/IconCopy.svelte](frontend/src/lib/icons/IconCopy.svelte) |
| 1 | [lib/icons/IconColumns.svelte](frontend/src/lib/icons/IconColumns.svelte) |
| 1 | [lib/icons/IconCode.svelte](frontend/src/lib/icons/IconCode.svelte) |
| 1 | [lib/icons/IconCircle.svelte](frontend/src/lib/icons/IconCircle.svelte) |
| 1 | [lib/icons/IconChevronRight.svelte](frontend/src/lib/icons/IconChevronRight.svelte) |
| 1 | [lib/icons/IconChevronLeft.svelte](frontend/src/lib/icons/IconChevronLeft.svelte) |
| 1 | [lib/icons/IconChevronDown.svelte](frontend/src/lib/icons/IconChevronDown.svelte) |
| 1 | [lib/icons/IconCheck.svelte](frontend/src/lib/icons/IconCheck.svelte) |
| 1 | [lib/consent/components/ContentPlaceholder.svelte](frontend/src/lib/consent/components/ContentPlaceholder.svelte) |
| 1 | [lib/consent/components/ConsentSettingsButton.svelte](frontend/src/lib/consent/components/ConsentSettingsButton.svelte) |
| 1 | [lib/consent/components/ConsentPreferencesModal.svelte](frontend/src/lib/consent/components/ConsentPreferencesModal.svelte) |
| 1 | [lib/components/workflow/WorkflowNode.svelte](frontend/src/lib/components/workflow/WorkflowNode.svelte) |
| 1 | [lib/components/video/VideoCardOptimized.svelte](frontend/src/lib/components/video/VideoCardOptimized.svelte) |
| 1 | [lib/components/video/RelatedVideos.svelte](frontend/src/lib/components/video/RelatedVideos.svelte) |
| 1 | [lib/components/video/BunnyVideoPlayer.svelte](frontend/src/lib/components/video/BunnyVideoPlayer.svelte) |
| 1 | [lib/components/ui/ThumbnailSelector.svelte](frontend/src/lib/components/ui/ThumbnailSelector.svelte) |
| 1 | [lib/components/ui/ExportButton.svelte](frontend/src/lib/components/ui/ExportButton.svelte) |
| 1 | [lib/components/ui/Button.svelte](frontend/src/lib/components/ui/Button.svelte) |
| 1 | [lib/components/ui/BlurHashImage.svelte](frontend/src/lib/components/ui/BlurHashImage.svelte) |
| 1 | [lib/components/sections/TestimonialsSection.svelte](frontend/src/lib/components/sections/TestimonialsSection.svelte) |
| 1 | [lib/components/sections/SocialMediaSection.svelte](frontend/src/lib/components/sections/SocialMediaSection.svelte) |
| 1 | [lib/components/media/OptimizedImage.svelte](frontend/src/lib/components/media/OptimizedImage.svelte) |
| 1 | [lib/components/media/MediaSkeleton.svelte](frontend/src/lib/components/media/MediaSkeleton.svelte) |
| 1 | [lib/components/forms/pro/TaxonomyField.svelte](frontend/src/lib/components/forms/pro/TaxonomyField.svelte) |
| 1 | [lib/components/forms/pro/PhoneIntlField.svelte](frontend/src/lib/components/forms/pro/PhoneIntlField.svelte) |
| 1 | [lib/components/forms/pro/FormStyler.svelte](frontend/src/lib/components/forms/pro/FormStyler.svelte) |
| 1 | [lib/components/forms/pro/DynamicField.svelte](frontend/src/lib/components/forms/pro/DynamicField.svelte) |
| 1 | [lib/components/forms/pro/CalculatorField.svelte](frontend/src/lib/components/forms/pro/CalculatorField.svelte) |
| 1 | [lib/components/forms/pro/AccordionTabField.svelte](frontend/src/lib/components/forms/pro/AccordionTabField.svelte) |
| 1 | [lib/components/dashboard/TradeAlertModal.svelte](frontend/src/lib/components/dashboard/TradeAlertModal.svelte) |
| 1 | [lib/components/dashboard/FavoriteButton.svelte](frontend/src/lib/components/dashboard/FavoriteButton.svelte) |
| 1 | [lib/components/courses/CourseCard.svelte](frontend/src/lib/components/courses/CourseCard.svelte) |
| 1 | [lib/components/checkout/PricingSelector.svelte](frontend/src/lib/components/checkout/PricingSelector.svelte) |
| 1 | [lib/components/blog/BlockEditor/SchedulingPanel.svelte](frontend/src/lib/components/blog/BlockEditor/SchedulingPanel.svelte) |
| 1 | [lib/components/auth/TestimonialCarousel.svelte](frontend/src/lib/components/auth/TestimonialCarousel.svelte) |

---

## Top 30 most common path signatures (mechanical migration candidates)

Run this list against a Tabler icon picker to confirm each match before bulk-replacing.

**Note:** Most paths here are Lucide icons (`lucide-svelte` style), not Heroicons. The Tabler equivalents below are best-fit visual matches.

| Count | Path prefix (first 50 chars) | Best Tabler match |
|---|---|---|
| 24 | `M8 5v14l11-7z`... | `IconPlayerPlayFilled` |
| 20 | `M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4`... | `IconDownload` (download tray) |
| 17 | `M22 11.08V12a10 10 0 1 1-5.93-9.14`... | `IconCircleCheck` |
| 15 | `M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 `... | `IconFile` |
| 13 | `M7 11V7a5 5 0 0 1 10 0v4`... | `IconLock` |
| 12 | `M18 6L6 18M6 6l12 12`... | `IconX` |
| 8 | `M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z`... | `IconShield` |
| 7 | `M19 9l-7 7-7-7`... | `IconChevronDown` |
| 6 | `M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 `... | `IconBook` |
| 5 | `m15 18-6-6 6-6`... | `IconChevronLeft` |
| 5 | `M5 12h14M12 5l7 7-7 7`... | `IconArrowRight` |
| 5 | `M3 3v18h18`... | `IconChartBar` (axes) |
| 4 | `M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 `... | `IconCopy` |
| 4 | `M22 12h-4l-3 9L9 3l-3 9H2`... | `IconActivity` (heartbeat) |
| 4 | `M12 8v4m0 4h.01`... | `IconAlertCircle` (info dot) |
| 4 | `M12 8v4M12 16h.01`... | `IconAlertCircle` (info dot) |
| 3 | `m6 9 6 6 6-6`... | `IconChevronDown` |
| 3 | `m6 6 12 12`... | `IconX` |
| 3 | `m21 21-4.35-4.35`... | `IconSearch` (loupe handle) |
| 3 | `m21 21-4.3-4.3`... | `IconSearch` (loupe handle) |
| 3 | `M9 18l6-6-6-6`... | `IconChevronRight` |
| 3 | `M9 18V5l12-2v13`... | `IconMusic` |
| 3 | `M9 12l2 2 4-4`... | `IconCheck` |
| 3 | `M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1`... | `IconCopy` |
| 3 | `M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-`... | `IconMail` |
| 3 | `M4 19.5A2.5 2.5 0 0 1 6.5 17H20`... | `IconBook` |
| 3 | `M3.51 15a9 9 0 1 0 2.13-9.36L1 10`... | `IconRefresh` |
| 3 | `M3 9h18M9 21V9`... | `IconLayoutDashboard` (grid) |
| 3 | `M21 21l-4.35-4.35`... | `IconSearch` (loupe handle) |
| 3 | `M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4M17 8l-5-5-`... | `IconUpload` |

---

## How to interpret

**To check what each file looks like in-browser:**
1. Spin up the local dev server (`cd frontend && pnpm dev`)
2. Open the URL in the right-most column
3. The icons listed are everywhere a `<svg viewBox="0 0 24 24">` or `<svg viewBox="0 0 20 20">` appears

**Top 10 user-facing pages to eyeball first** (highest icon density on non-admin surfaces):

- **/resources** — 30 icons in [routes/resources/+page.svelte](frontend/src/routes/resources/+page.svelte)
- **/alerts/spx-profit-pulse** — 29 icons in [routes/alerts/spx-profit-pulse/+page.svelte](frontend/src/routes/alerts/spx-profit-pulse/+page.svelte)
- **/our-mission** — 18 icons in [routes/our-mission/+page.svelte](frontend/src/routes/our-mission/+page.svelte)
- **/live-trading-rooms/swing-trading** — 14 icons in [routes/live-trading-rooms/swing-trading/+page.svelte](frontend/src/routes/live-trading-rooms/swing-trading/+page.svelte)
- **/dashboard/explosive-swings** — 14 icons in [routes/dashboard/explosive-swings/components/WeeklyHero.svelte](frontend/src/routes/dashboard/explosive-swings/components/WeeklyHero.svelte)
- **/alerts/explosive-swings** — 14 icons in [routes/alerts/explosive-swings/+page.svelte](frontend/src/routes/alerts/explosive-swings/+page.svelte)
- **/alerts/[slug]/checkout** — 12 icons in [routes/alerts/[slug]/checkout/+page.svelte](frontend/src/routes/alerts/[slug]/checkout/+page.svelte)
- **/live-trading-rooms/small-accounts** — 11 icons in [routes/live-trading-rooms/small-accounts/+page.svelte](frontend/src/routes/live-trading-rooms/small-accounts/+page.svelte)
- **/dashboard/explosive-swings** — 11 icons in [routes/dashboard/explosive-swings/components/VideoUploadModal.svelte](frontend/src/routes/dashboard/explosive-swings/components/VideoUploadModal.svelte)
- **/dashboard/explosive-swings** — 10 icons in [routes/dashboard/explosive-swings/components/PublishWeeklyModal.svelte](frontend/src/routes/dashboard/explosive-swings/components/PublishWeeklyModal.svelte)

---

## Migration approach (when you decide)

**A) Mechanical pass (fast):** Build a path → Tabler-name lookup table for the top ~30 signatures (see table above), bulk find-and-replace via grep+sed. Catches roughly 60-70% of icons with high confidence. Remaining icons stay inline and get flagged with a `<!-- TODO Tabler -->` marker.

**B) Per-file pass (slow, complete):** Agent reads each file, identifies each icon by visual inspection of its path data, picks the closest Tabler name from `@tabler/icons-svelte-runes`. Hours of work even with parallel agents.

**C) Members-dashboard-only first:** Only migrate `routes/dashboard/` files. Smallest surface area, fastest payoff for actual end-users.

**Recommendation:** **A on whole repo + C as the priority audit-and-cleanup target**. Mechanical bulk replace catches the easy wins everywhere; then a focused pass over `routes/dashboard/` catches anything the mechanical pass missed in the area that matters most to logged-in members.
