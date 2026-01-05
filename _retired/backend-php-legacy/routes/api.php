<?php

use App\Http\Controllers\Api\BacklinkController;
use App\Http\Controllers\Api\CartController;
use App\Http\Controllers\Api\CouponController;
use App\Http\Controllers\Api\Error404Controller;
use App\Http\Controllers\Api\FormController;
use App\Http\Controllers\Api\FormSubmissionController;
use App\Http\Controllers\Api\FormPdfController;
use App\Http\Controllers\Api\FormApprovalController;
use App\Http\Controllers\Api\FormInventoryController;
use App\Http\Controllers\Api\HealthCheckController;
use App\Http\Controllers\Api\IndicatorController;
use App\Http\Controllers\Api\MeController;
use App\Http\Controllers\Api\NewsletterController;
use App\Http\Controllers\Api\PopupController;
use App\Http\Controllers\Api\PostController;
use App\Http\Controllers\Api\RankingController;
use App\Http\Controllers\Api\RedirectController;
use App\Http\Controllers\Api\SeoController;
use App\Http\Controllers\Api\SeoSettingsController;
use App\Http\Controllers\Api\TimeController;
use App\Http\Controllers\Api\TimerAnalyticsController;
use App\Http\Controllers\Api\UserSubscriptionController as ApiUserSubscriptionController;
use App\Http\Controllers\Api\VideoController;
use App\Http\Controllers\Api\Admin\PostController as AdminPostController;
use App\Http\Controllers\Api\Admin\MediaController;
use App\Http\Middleware\RateLimitMedia;
use App\Http\Controllers\Admin\CategoryController;
use App\Http\Controllers\Admin\EmailSettingsController;
use App\Http\Controllers\Admin\EmailCampaignController;
use App\Http\Controllers\Admin\EmailTemplateController;
use App\Http\Controllers\Admin\EmailSubscriberController;
use App\Http\Controllers\Admin\EmailMetricsController;
use App\Http\Controllers\Admin\EmailDomainController;
use App\Http\Controllers\Admin\EmailAuditLogController;
use App\Http\Controllers\Admin\EmailWebhookController;
use App\Http\Controllers\Api\EmailTemplateBuilderController;
use App\Http\Controllers\Admin\ProductController;
use App\Http\Controllers\Admin\SettingsController;
use App\Http\Controllers\Admin\SubscriptionPlanController;
use App\Http\Controllers\Admin\TagController;
use App\Http\Controllers\Admin\UserSubscriptionController;
use App\Http\Controllers\Admin\UserController;
use App\Http\Controllers\Admin\MemberController;
use App\Http\Controllers\Admin\NewsletterCategoryController;
use App\Http\Controllers\Api\PastMembersController;
use App\Http\Controllers\Api\SitemapController;
use App\Http\Controllers\Api\RobotsController;
use App\Http\Controllers\Api\ReadingAnalyticsController;
use App\Http\Controllers\Admin\PastMembersDashboardController;
use App\Http\Controllers\Admin\AbandonedCartController;
use Illuminate\Support\Facades\Route;

// ========================================
// HEALTH CHECK ENDPOINTS (Kubernetes Probes)
// ========================================
Route::prefix('health')->group(function () {
    Route::get('/live', [HealthCheckController::class, 'liveness']);
    Route::get('/ready', [HealthCheckController::class, 'readiness']);
    Route::get('/optimization', [HealthCheckController::class, 'optimization']);
});

// ========================================
// INBOUND EMAIL WEBHOOKS (No Auth - Signature Verified)
// ========================================
use App\Http\Controllers\Webhooks\InboundEmailWebhookController;

Route::prefix('webhooks')->group(function () {
    // Postmark inbound email webhook
    Route::post('/postmark/inbound', [InboundEmailWebhookController::class, 'handlePostmark']);

    // AWS SES inbound email webhook (via SNS)
    Route::post('/ses/inbound', [InboundEmailWebhookController::class, 'handleSes']);

    // SendGrid inbound email webhook
    Route::post('/sendgrid/inbound', [InboundEmailWebhookController::class, 'handleSendGrid']);

    // Health check for inbound email webhooks
    Route::get('/inbound/health', [InboundEmailWebhookController::class, 'health']);
});

// ========================================
// SITEMAP ENDPOINTS (SEO - Lightning Stack)
// ========================================
Route::prefix('sitemap')->group(function () {
    Route::get('/', [SitemapController::class, 'index']);
    Route::get('/posts/{page?}', [SitemapController::class, 'posts'])->where('page', '[0-9]+');
    Route::get('/categories', [SitemapController::class, 'categories']);
    Route::get('/tags', [SitemapController::class, 'tags']);
});

// ========================================
// ROBOTS.TXT (SEO - Lightning Stack)
// ========================================
Route::get('/robots.txt', [RobotsController::class, 'index']);

// ========================================
// READING ANALYTICS (ICT11+ Blog Analytics)
// ========================================
// Public tracking endpoint (rate limited in controller)
Route::post('/analytics/reading', [ReadingAnalyticsController::class, 'track']);
Route::post('/analytics/track', function () { return response()->json(['status' => 'ok']); });

// Admin reading analytics (protected)
Route::middleware(['auth:sanctum', 'role:admin|super-admin'])->prefix('admin/reading-analytics')->group(function () {
    Route::get('/', [ReadingAnalyticsController::class, 'getAllAnalytics']);
    Route::get('/{postId}', [ReadingAnalyticsController::class, 'getPostAnalytics'])->where('postId', '[0-9]+');
});

// Public routes
Route::get('/time/now', [TimeController::class, 'now']);
Route::post('/timers/events', [TimerAnalyticsController::class, 'store']);
Route::post('/telemetry/admin-toolbar', function() { return response()->json(['status' => 'ok']); });


// Public popup runtime routes with rate limiting for security
Route::get('/popups/active', [PopupController::class, 'active'])
    ->middleware('throttle:120,1'); // 120 requests/minute - normal page loads
Route::post('/popups/{popup}/impression', [PopupController::class, 'impression'])
    ->middleware('throttle:60,1'); // 60 impressions/minute per IP
Route::post('/popups/{popup}/conversion', [PopupController::class, 'conversion'])
    ->middleware('throttle:30,1'); // 30 conversions/minute per IP
Route::post('/popups/{popup}/form-submit', [PopupController::class, 'formSubmit'])
    ->middleware('throttle:10,1'); // 10 form submissions/minute per IP (stricter for opt-ins)
Route::post('/popups/events', [PopupController::class, 'events'])
    ->middleware('throttle:60,1'); // 60 batch events/minute per IP

// Public consent configuration (for frontend banner)
Route::get('/consent/config', [\App\Http\Controllers\Admin\ConsentSettingsController::class, 'publicSettings']);


// Newsletter
Route::post('/newsletter/subscribe', [NewsletterController::class, 'subscribe']);
Route::get('/newsletter/confirm', [NewsletterController::class, 'confirm']);
Route::get('/newsletter/unsubscribe', [NewsletterController::class, 'unsubscribe']);
Route::get('/newsletter/categories', [NewsletterCategoryController::class, 'forSelect']); // Public: for forms

// Public content
Route::get('/posts', [PostController::class, 'index']);
Route::get('/posts/{slug}', [PostController::class, 'show']);
Route::get('/indicators', [IndicatorController::class, 'index']);
Route::get('/indicators/{slug}', [IndicatorController::class, 'show']);

// Public video routes
Route::get('/videos', [VideoController::class, 'index']);
Route::get('/videos/{id}', [VideoController::class, 'show']);
Route::post('/videos/{id}/track', [VideoController::class, 'trackEvent']);

// Protected routes
Route::middleware(['auth:sanctum'])->group(function () {
    Route::get('/me', [MeController::class, 'show']);
    Route::get('/me/memberships', [MeController::class, 'memberships']);
    Route::get('/me/products', [MeController::class, 'products']);
    Route::get('/me/indicators', [MeController::class, 'indicators']);
    
    // Alias routes for frontend compatibility
    Route::get('/user/memberships', [MeController::class, 'memberships']);
    Route::get('/user/indicators', [MeController::class, 'indicators']);
    
    // User Orders (Self-Service) - ICT 11 Protocol
    Route::get('/my/orders', [MeController::class, 'orders']);
    Route::get('/my/orders/{id}', [MeController::class, 'showOrder']);

    Route::post('/cart/checkout', [CartController::class, 'checkout']);
    Route::post('/cart/calculate-tax', [CartController::class, 'calculateTax']);

    // SEO Routes
    Route::post('/seo/analyze', [SeoController::class, 'analyze']);
    Route::get('/seo/analyze/{contentType}/{contentId}', [SeoController::class, 'getAnalysis']);
    Route::get('/seo/analyze/{contentType}/{contentId}/recommendations', [SeoController::class, 'getRecommendations']);
    Route::post('/seo/auto-fix', [SeoController::class, 'autoFix']);
    Route::post('/seo/competitors/analyze', [SeoController::class, 'analyzeCompetitors']);
    Route::post('/seo/keywords/opportunities', [SeoController::class, 'keywordOpportunities']);
    Route::post('/seo/content/gaps', [SeoController::class, 'contentGaps']);
    Route::post('/seo/technical/audit', [SeoController::class, 'technicalAudit']);
    Route::post('/seo/performance/analyze', [SeoController::class, 'performanceAnalyze']);
    Route::get('/seo/technical/issues', [SeoController::class, 'technicalIssues']);
    Route::post('/seo/sitemap/generate', [SeoController::class, 'generateSitemap']);
    Route::post('/seo/schema/generate', [SeoController::class, 'generateSchema']);
    
    // Rankings
    Route::get('/seo/rankings', [RankingController::class, 'index']);
    Route::get('/seo/rankings/check', [RankingController::class, 'check']);
    Route::post('/seo/rankings/track', [RankingController::class, 'track']);
    Route::post('/seo/rankings/update', [RankingController::class, 'update']);
    Route::get('/seo/rankings/{keyword}/history', [RankingController::class, 'history']);
    Route::delete('/seo/rankings/{id}', [RankingController::class, 'destroy']);
    
    // Backlinks
    Route::get('/seo/backlinks', [BacklinkController::class, 'index']);
    Route::get('/seo/backlinks/new', [BacklinkController::class, 'new']);
    Route::get('/seo/backlinks/toxic', [BacklinkController::class, 'toxic']);
    Route::post('/seo/backlinks/disavow', [BacklinkController::class, 'disavow']);
    
    // Redirects
    Route::get('/redirects', [RedirectController::class, 'index']);
    Route::post('/redirects', [RedirectController::class, 'store']);
    Route::put('/redirects/{id}', [RedirectController::class, 'update']);
    Route::delete('/redirects/{id}', [RedirectController::class, 'destroy']);
    Route::get('/redirects/chains', [RedirectController::class, 'detectChains']);
    Route::post('/redirects/import', [RedirectController::class, 'import']);
    Route::get('/redirects/export', [RedirectController::class, 'export']);
    
    // 404 Errors
    Route::get('/404-errors', [Error404Controller::class, 'index']);
    Route::get('/404-errors/{id}', [Error404Controller::class, 'show']);
    Route::put('/404-errors/{id}/resolve', [Error404Controller::class, 'resolve']);
    Route::post('/404-errors/bulk-delete', [Error404Controller::class, 'bulkDelete']);
    Route::post('/404-errors/find-similar', [Error404Controller::class, 'findSimilar']);
    
    // SEO Settings
    Route::get('/seo-settings', [SeoSettingsController::class, 'index']);
    Route::put('/seo-settings/{key}', [SeoSettingsController::class, 'update']);
    Route::get('/seo-settings/sitemap-configs', [SeoSettingsController::class, 'getSitemapConfigs']);
    Route::put('/seo-settings/sitemap-configs/{id}', [SeoSettingsController::class, 'updateSitemapConfig']);
    Route::get('/seo-settings/local-business', [SeoSettingsController::class, 'getLocalBusiness']);
    Route::post('/seo-settings/local-business', [SeoSettingsController::class, 'updateLocalBusiness']);
    Route::get('/seo/alerts/check', [SeoSettingsController::class, 'checkAlerts']);
    
    // User Subscriptions (Self-Service)
    Route::get('/my/subscriptions', [ApiUserSubscriptionController::class, 'index']);
    Route::post('/my/subscriptions', [ApiUserSubscriptionController::class, 'store']);
    Route::get('/my/subscriptions/metrics', [ApiUserSubscriptionController::class, 'metrics']);
    Route::get('/my/subscriptions/{id}', [ApiUserSubscriptionController::class, 'show']);
    Route::post('/my/subscriptions/{id}/cancel', [ApiUserSubscriptionController::class, 'cancel']);
    Route::post('/my/subscriptions/{id}/pause', [ApiUserSubscriptionController::class, 'pause']);
    Route::post('/my/subscriptions/{id}/resume', [ApiUserSubscriptionController::class, 'resume']);
    Route::post('/my/subscriptions/{id}/reactivate', [ApiUserSubscriptionController::class, 'reactivate']);
    Route::get('/my/subscriptions/{id}/invoices', [ApiUserSubscriptionController::class, 'invoices']);
    Route::get('/my/subscriptions/{id}/payments', [ApiUserSubscriptionController::class, 'payments']);
    Route::post('/my/subscriptions/{id}/payment-method', [ApiUserSubscriptionController::class, 'updatePaymentMethod']);
    Route::post('/my/subscriptions/{id}/retry-payment', [ApiUserSubscriptionController::class, 'retryPayment']);

    // ========================================
    // USER INDICATORS API
    // ========================================
    Route::prefix('user/indicators')->group(function () {
        Route::get('/', [\App\Http\Controllers\Api\UserIndicatorsController::class, 'index']);
        Route::get('/{id}', [\App\Http\Controllers\Api\UserIndicatorsController::class, 'show']);
        Route::get('/{id}/download', [\App\Http\Controllers\Api\UserIndicatorsController::class, 'download']);
        Route::get('/{id}/docs', [\App\Http\Controllers\Api\UserIndicatorsController::class, 'documentation']);
    });
});

// Admin routes (require admin / super-admin role) under /api/admin
Route::middleware(['auth:sanctum', 'role:admin|super-admin'])->prefix('admin')->group(function () {
    // Email Settings
    Route::get('/email/settings', [EmailSettingsController::class, 'index']);
    Route::post('/email/settings', [EmailSettingsController::class, 'update']);
    Route::post('/email/settings/test', [EmailSettingsController::class, 'test']);

    // Email Templates
    Route::get('/email/templates', [EmailTemplateController::class, 'index']);
    Route::post('/email/templates', [EmailTemplateController::class, 'store']);
    Route::get('/email/templates/{id}', [EmailTemplateController::class, 'show']);
    Route::put('/email/templates/{id}', [EmailTemplateController::class, 'update']);
    Route::delete('/email/templates/{id}', [EmailTemplateController::class, 'destroy']);
    Route::post('/email/templates/{id}/preview', [EmailTemplateController::class, 'preview']);
    
    // Email Template Builder
    Route::get('/email/builder/templates', [EmailTemplateBuilderController::class, 'index']);
    Route::post('/email/builder/templates', [EmailTemplateBuilderController::class, 'store']);
    Route::get('/email/builder/templates/{id}', [EmailTemplateBuilderController::class, 'show']);
    Route::put('/email/builder/templates/{id}', [EmailTemplateBuilderController::class, 'update']);
    Route::delete('/email/builder/templates/{id}', [EmailTemplateBuilderController::class, 'destroy']);
    Route::post('/email/builder/templates/{id}/preview', [EmailTemplateBuilderController::class, 'preview']);
    Route::post('/email/builder/templates/{id}/duplicate', [EmailTemplateBuilderController::class, 'duplicate']);
    Route::post('/email/builder/templates/{id}/blocks', [EmailTemplateBuilderController::class, 'addBlock']);
    Route::put('/email/builder/templates/{id}/blocks/{blockId}', [EmailTemplateBuilderController::class, 'updateBlock']);
    Route::delete('/email/builder/templates/{id}/blocks/{blockId}', [EmailTemplateBuilderController::class, 'deleteBlock']);
    Route::post('/email/builder/templates/{id}/blocks/reorder', [EmailTemplateBuilderController::class, 'reorderBlocks']);
    Route::get('/email/builder/variables', [EmailTemplateBuilderController::class, 'getVariables']);
    Route::get('/email/builder/layouts', [EmailTemplateBuilderController::class, 'getLayouts']);

    // Email Campaigns
    Route::get('/email/campaigns', [EmailCampaignController::class, 'index']);
    Route::get('/email/campaigns/stats', [EmailCampaignController::class, 'stats']);
    Route::post('/email/campaigns', [EmailCampaignController::class, 'store']);
    Route::get('/email/campaigns/{id}', [EmailCampaignController::class, 'show']);
    Route::put('/email/campaigns/{id}', [EmailCampaignController::class, 'update']);
    Route::delete('/email/campaigns/{id}', [EmailCampaignController::class, 'destroy']);
    Route::post('/email/campaigns/{id}/duplicate', [EmailCampaignController::class, 'duplicate']);
    Route::post('/email/campaigns/{id}/send', [EmailCampaignController::class, 'send']);
    Route::post('/email/campaigns/{id}/schedule', [EmailCampaignController::class, 'schedule']);
    Route::post('/email/campaigns/{id}/cancel', [EmailCampaignController::class, 'cancel']);
    Route::get('/email/campaigns/{id}/analytics', [EmailCampaignController::class, 'analytics']);
    Route::get('/email/campaigns/{id}/preview', [EmailCampaignController::class, 'preview']);
    Route::post('/email/campaigns/{id}/test', [EmailCampaignController::class, 'sendTest']);

    // Email Subscribers (Audience Management)
    Route::get('/email/subscribers', [EmailSubscriberController::class, 'index']);
    Route::get('/email/subscribers/stats', [EmailSubscriberController::class, 'stats']);
    Route::get('/email/subscribers/tags', [EmailSubscriberController::class, 'tags']);
    Route::get('/email/subscribers/export', [EmailSubscriberController::class, 'export']);
    Route::post('/email/subscribers', [EmailSubscriberController::class, 'store']);
    Route::post('/email/subscribers/import', [EmailSubscriberController::class, 'import']);
    Route::get('/email/subscribers/{id}', [EmailSubscriberController::class, 'show']);
    Route::put('/email/subscribers/{id}', [EmailSubscriberController::class, 'update']);
    Route::delete('/email/subscribers/{id}', [EmailSubscriberController::class, 'destroy']);
    Route::post('/email/subscribers/{id}/tags', [EmailSubscriberController::class, 'addTags']);
    Route::delete('/email/subscribers/{id}/tags', [EmailSubscriberController::class, 'removeTags']);
    Route::post('/email/subscribers/{id}/verify', [EmailSubscriberController::class, 'verify']);
    Route::post('/email/subscribers/{id}/resend-verification', [EmailSubscriberController::class, 'resendVerification']);

    // Email Metrics (Analytics Dashboard)
    Route::get('/email/metrics/dashboard', [EmailMetricsController::class, 'dashboard']);
    Route::get('/email/metrics/overview', [EmailMetricsController::class, 'overview']);
    Route::get('/email/metrics/campaigns', [EmailMetricsController::class, 'campaigns']);
    Route::get('/email/metrics/subscribers', [EmailMetricsController::class, 'subscribers']);
    Route::get('/email/metrics/engagement', [EmailMetricsController::class, 'engagement']);
    Route::get('/email/metrics/deliverability', [EmailMetricsController::class, 'deliverability']);
    Route::get('/email/metrics/realtime', [EmailMetricsController::class, 'realtime']);
    Route::get('/email/metrics/compare', [EmailMetricsController::class, 'compare']);
    Route::get('/email/metrics/export', [EmailMetricsController::class, 'export']);

    // Email Domains
    Route::get('/email/domains', [EmailDomainController::class, 'index']);
    Route::get('/email/domains/stats', [EmailDomainController::class, 'stats']);
    Route::post('/email/domains', [EmailDomainController::class, 'store']);
    Route::get('/email/domains/{id}', [EmailDomainController::class, 'show']);
    Route::delete('/email/domains/{id}', [EmailDomainController::class, 'destroy']);
    Route::post('/email/domains/{id}/verify', [EmailDomainController::class, 'verify']);
    Route::get('/email/domains/{id}/dns', [EmailDomainController::class, 'dnsRecords']);
    Route::post('/email/domains/{id}/check-dns', [EmailDomainController::class, 'checkDns']);
    Route::get('/email/domains/{id}/deliverability', [EmailDomainController::class, 'deliverability']);

    // Email Audit Logs
    Route::get('/email/logs', [EmailAuditLogController::class, 'index']);
    Route::get('/email/logs/stats', [EmailAuditLogController::class, 'stats']);
    Route::get('/email/logs/filters', [EmailAuditLogController::class, 'filterOptions']);
    Route::get('/email/logs/export', [EmailAuditLogController::class, 'export']);
    Route::get('/email/logs/security', [EmailAuditLogController::class, 'securityEvents']);
    Route::get('/email/logs/timeline', [EmailAuditLogController::class, 'resourceTimeline']);
    Route::get('/email/logs/user/{userId}', [EmailAuditLogController::class, 'userActivity']);
    Route::get('/email/logs/{id}', [EmailAuditLogController::class, 'show']);
    Route::post('/email/logs/cleanup', [EmailAuditLogController::class, 'cleanup']);

    // Email Webhooks
    Route::get('/email/webhooks', [EmailWebhookController::class, 'index']);
    Route::get('/email/webhooks/stats', [EmailWebhookController::class, 'stats']);
    Route::get('/email/webhooks/events', [EmailWebhookController::class, 'events']);
    Route::post('/email/webhooks', [EmailWebhookController::class, 'store']);
    Route::get('/email/webhooks/{id}', [EmailWebhookController::class, 'show']);
    Route::put('/email/webhooks/{id}', [EmailWebhookController::class, 'update']);
    Route::delete('/email/webhooks/{id}', [EmailWebhookController::class, 'destroy']);
    Route::post('/email/webhooks/{id}/test', [EmailWebhookController::class, 'test']);
    Route::post('/email/webhooks/{id}/toggle', [EmailWebhookController::class, 'toggle']);
    Route::post('/email/webhooks/{id}/rotate-secret', [EmailWebhookController::class, 'rotateSecret']);
    Route::get('/email/webhooks/{id}/deliveries', [EmailWebhookController::class, 'deliveries']);
    Route::post('/email/webhooks/deliveries/{deliveryId}/retry', [EmailWebhookController::class, 'retryDelivery']);

    // Email Conversations (Inbound Email CRM - ICT11+ Feature)
    Route::get('/email/conversations', [\App\Http\Controllers\Admin\EmailConversationController::class, 'index']);
    Route::get('/email/conversations/stats', [\App\Http\Controllers\Admin\EmailConversationController::class, 'stats']);
    Route::get('/email/conversations/tags', [\App\Http\Controllers\Admin\EmailConversationController::class, 'allTags']);
    Route::get('/email/conversations/{id}', [\App\Http\Controllers\Admin\EmailConversationController::class, 'show']);
    Route::put('/email/conversations/{id}', [\App\Http\Controllers\Admin\EmailConversationController::class, 'update']);
    Route::delete('/email/conversations/{id}', [\App\Http\Controllers\Admin\EmailConversationController::class, 'destroy']);
    Route::post('/email/conversations/{id}/assign', [\App\Http\Controllers\Admin\EmailConversationController::class, 'assign']);
    Route::post('/email/conversations/{id}/resolve', [\App\Http\Controllers\Admin\EmailConversationController::class, 'resolve']);
    Route::post('/email/conversations/{id}/reopen', [\App\Http\Controllers\Admin\EmailConversationController::class, 'reopen']);
    Route::post('/email/conversations/{id}/spam', [\App\Http\Controllers\Admin\EmailConversationController::class, 'markAsSpam']);
    Route::post('/email/conversations/{id}/not-spam', [\App\Http\Controllers\Admin\EmailConversationController::class, 'markAsNotSpam']);
    Route::post('/email/conversations/{id}/archive', [\App\Http\Controllers\Admin\EmailConversationController::class, 'archive']);
    Route::post('/email/conversations/{id}/star', [\App\Http\Controllers\Admin\EmailConversationController::class, 'toggleStar']);
    Route::post('/email/conversations/{id}/read', [\App\Http\Controllers\Admin\EmailConversationController::class, 'markAsRead']);
    Route::post('/email/conversations/{id}/tags', [\App\Http\Controllers\Admin\EmailConversationController::class, 'addTags']);
    Route::delete('/email/conversations/{id}/tags/{tag}', [\App\Http\Controllers\Admin\EmailConversationController::class, 'removeTag']);
    Route::post('/email/conversations/{id}/merge', [\App\Http\Controllers\Admin\EmailConversationController::class, 'merge']);
    Route::post('/email/conversations/bulk-update', [\App\Http\Controllers\Admin\EmailConversationController::class, 'bulkUpdate']);
    Route::get('/email/conversations/{conversationId}/attachments/{attachmentId}/download', [\App\Http\Controllers\Admin\EmailConversationController::class, 'downloadAttachment']);

    // Newsletter Categories (Form-Newsletter Integration)
    Route::get('/newsletter/categories', [NewsletterCategoryController::class, 'index']);
    Route::post('/newsletter/categories', [NewsletterCategoryController::class, 'store']);
    Route::get('/newsletter/categories/{id}', [NewsletterCategoryController::class, 'show']);
    Route::put('/newsletter/categories/{id}', [NewsletterCategoryController::class, 'update']);
    Route::delete('/newsletter/categories/{id}', [NewsletterCategoryController::class, 'destroy']);
    Route::post('/newsletter/categories/reorder', [NewsletterCategoryController::class, 'reorder']);
    Route::post('/newsletter/categories/seed-defaults', [NewsletterCategoryController::class, 'seedDefaults']);
    Route::get('/newsletter/categories/{id}/analytics', [NewsletterCategoryController::class, 'analytics']);
    Route::post('/newsletter/categories/refresh-counts', [NewsletterCategoryController::class, 'refreshCounts']);
    Route::get('/newsletter/form-signups/analytics', [NewsletterCategoryController::class, 'formSignupAnalytics']);

    // Coupons
    Route::get('/coupons', [CouponController::class, 'index']);
    Route::post('/coupons', [CouponController::class, 'store']);
    Route::get('/coupons/{id}', [CouponController::class, 'show']);
    Route::put('/coupons/{id}', [CouponController::class, 'update']);
    Route::delete('/coupons/{id}', [CouponController::class, 'destroy']);
    Route::get('/coupons/user/available', [CouponController::class, 'userCoupons']);

    // Users (Admin staff)
    Route::get('/users', [UserController::class, 'index']);
    Route::get('/users/stats', [UserController::class, 'stats']);
    Route::get('/users/{id}', [UserController::class, 'show']);
    Route::post('/users', [UserController::class, 'store']);
    Route::put('/users/{id}', [UserController::class, 'update']);
    Route::delete('/users/{id}', [UserController::class, 'destroy']);
    Route::post('/users/{id}/ban', [UserController::class, 'ban']);
    Route::post('/users/{id}/unban', [UserController::class, 'unban']);

    // Members Management (All registered users/customers)
    Route::get('/members', [MemberController::class, 'index']);
    Route::get('/members/stats', [MemberController::class, 'stats']);
    Route::get('/members/services', [MemberController::class, 'services']);
    Route::get('/members/churned', [MemberController::class, 'churned']);
    Route::get('/members/export', [MemberController::class, 'export']);
    Route::get('/members/email-templates', [MemberController::class, 'emailTemplates']);
    Route::get('/members/service/{productId}', [MemberController::class, 'byService']);
    Route::get('/members/{id}', [MemberController::class, 'show']);
    Route::post('/members/{id}/send-email', [MemberController::class, 'sendEmail']);
    Route::post('/members/bulk-email', [MemberController::class, 'bulkEmail']);

    // Past Members & Win-Back Campaigns (Legacy)
    Route::get('/past-members', [PastMembersController::class, 'index']);
    Route::get('/past-members/stats', [PastMembersController::class, 'stats']);
    Route::get('/past-members/analytics', [PastMembersController::class, 'analytics']);
    Route::post('/past-members/{userId}/win-back', [PastMembersController::class, 'sendWinBack']);
    Route::post('/past-members/bulk-win-back', [PastMembersController::class, 'sendBulkWinBack']);
    Route::post('/past-members/{userId}/survey', [PastMembersController::class, 'sendSurvey']);
    Route::post('/past-members/bulk-survey', [PastMembersController::class, 'sendBulkSurvey']);

    // Enhanced Past Members Dashboard (v2.0)
    Route::get('/past-members-dashboard/overview', [PastMembersDashboardController::class, 'overview']);
    Route::get('/past-members-dashboard/period/{period}', [PastMembersDashboardController::class, 'byTimePeriod']);
    Route::get('/past-members-dashboard/services', [PastMembersDashboardController::class, 'services']);
    Route::get('/past-members-dashboard/churn-reasons', [PastMembersDashboardController::class, 'churnReasons']);
    Route::get('/past-members-dashboard/campaigns', [PastMembersDashboardController::class, 'campaignHistory']);
    Route::post('/past-members-dashboard/bulk-winback', [PastMembersDashboardController::class, 'sendBulkWinBack']);
    Route::post('/past-members-dashboard/bulk-survey', [PastMembersDashboardController::class, 'sendBulkSurvey']);

    // Abandoned Cart Recovery
    Route::get('/abandoned-carts/dashboard', [AbandonedCartController::class, 'dashboard']);
    Route::get('/abandoned-carts', [AbandonedCartController::class, 'index']);
    Route::get('/abandoned-carts/templates', [AbandonedCartController::class, 'templates']);
    Route::get('/abandoned-carts/{id}', [AbandonedCartController::class, 'show']);
    Route::post('/abandoned-carts/{id}/send-recovery', [AbandonedCartController::class, 'sendRecoveryEmail']);
    Route::post('/abandoned-carts/{id}/mark-recovered', [AbandonedCartController::class, 'markRecovered']);
    Route::post('/abandoned-carts/bulk-recovery', [AbandonedCartController::class, 'sendBulkRecovery']);
    Route::get('/abandoned-carts/track/{code}', [AbandonedCartController::class, 'trackClick']);

    // Settings
    Route::get('/settings', [SettingsController::class, 'index']);
    Route::put('/settings', [SettingsController::class, 'update']);
    Route::get('/settings/{key}', [SettingsController::class, 'show']);
    Route::put('/settings/{key}', [SettingsController::class, 'updateSingle']);

    // ========================================
    // CONSENT MANAGEMENT SYSTEM
    // Complete CMS for cookie consent (Consent Magic Pro equivalent)
    // ========================================
    Route::prefix('consent')->group(function () {
        // Settings
        Route::get('/settings', [\App\Http\Controllers\Admin\ConsentSettingsController::class, 'index']);
        Route::get('/settings/{key}', [\App\Http\Controllers\Admin\ConsentSettingsController::class, 'show']);
        Route::put('/settings/{key}', [\App\Http\Controllers\Admin\ConsentSettingsController::class, 'update']);
        Route::post('/settings/bulk', [\App\Http\Controllers\Admin\ConsentSettingsController::class, 'bulkUpdate']);
        Route::post('/settings/reset', [\App\Http\Controllers\Admin\ConsentSettingsController::class, 'reset']);
        Route::post('/initialize', [\App\Http\Controllers\Admin\ConsentSettingsController::class, 'initialize']);

        // Banner Templates
        Route::get('/templates', [\App\Http\Controllers\Admin\ConsentSettingsController::class, 'templates']);
        Route::get('/templates/active', [\App\Http\Controllers\Admin\ConsentSettingsController::class, 'activeTemplate']);
        Route::post('/templates', [\App\Http\Controllers\Admin\ConsentSettingsController::class, 'createTemplate']);
        Route::get('/templates/{id}', [\App\Http\Controllers\Admin\ConsentSettingsController::class, 'showTemplate']);
        Route::put('/templates/{id}', [\App\Http\Controllers\Admin\ConsentSettingsController::class, 'updateTemplate']);
        Route::delete('/templates/{id}', [\App\Http\Controllers\Admin\ConsentSettingsController::class, 'deleteTemplate']);
        Route::post('/templates/{id}/activate', [\App\Http\Controllers\Admin\ConsentSettingsController::class, 'activateTemplate']);
        Route::post('/templates/{id}/duplicate', [\App\Http\Controllers\Admin\ConsentSettingsController::class, 'duplicateTemplate']);
        Route::get('/templates/{id}/export', [\App\Http\Controllers\Admin\ConsentSettingsController::class, 'exportTemplate']);
        Route::post('/templates/import', [\App\Http\Controllers\Admin\ConsentSettingsController::class, 'importTemplate']);
    });

    // API Connections Management
    Route::prefix('connections')->group(function () {
        Route::get('/', [\App\Http\Controllers\Api\ApiConnectionController::class, 'index']);
        Route::get('/services', [\App\Http\Controllers\Api\ApiConnectionController::class, 'services']);
        Route::get('/summary', [\App\Http\Controllers\Api\ApiConnectionController::class, 'summary']);
        Route::get('/status', [\App\Http\Controllers\Api\ApiConnectionController::class, 'index']); // Alias for frontend compatibility
        Route::get('/health', [\App\Http\Controllers\Api\ApiConnectionController::class, 'health']);
        Route::get('/attention', [\App\Http\Controllers\Api\ApiConnectionController::class, 'needsAttention']);
        Route::get('/category/{category}', [\App\Http\Controllers\Api\ApiConnectionController::class, 'byCategory']);
        Route::get('/{serviceKey}', [\App\Http\Controllers\Api\ApiConnectionController::class, 'show']);
        Route::get('/{serviceKey}/logs', [\App\Http\Controllers\Api\ApiConnectionController::class, 'logs']);
        Route::post('/{serviceKey}/connect', [\App\Http\Controllers\Api\ApiConnectionController::class, 'connect']);
        Route::post('/{serviceKey}/disconnect', [\App\Http\Controllers\Api\ApiConnectionController::class, 'disconnect']);
        Route::post('/{serviceKey}/test', [\App\Http\Controllers\Api\ApiConnectionController::class, 'test']);
        Route::post('/{serviceKey}/verify', [\App\Http\Controllers\Api\ApiConnectionController::class, 'verify']);
        Route::post('/{serviceKey}/refresh', [\App\Http\Controllers\Api\ApiConnectionController::class, 'refreshToken']);
        Route::patch('/{serviceKey}', [\App\Http\Controllers\Api\ApiConnectionController::class, 'update']);
    });

    // Site Health Monitoring
    Route::prefix('site-health')->group(function () {
        Route::get('/', [\App\Http\Controllers\Api\Admin\SiteHealthController::class, 'index']);
        Route::post('/run-tests', [\App\Http\Controllers\Api\Admin\SiteHealthController::class, 'runTests']);
    });

    // Subscription Plans
    Route::get('/subscriptions/plans', [SubscriptionPlanController::class, 'index']);
    Route::post('/subscriptions/plans', [SubscriptionPlanController::class, 'store']);
    Route::get('/subscriptions/plans/stats', [SubscriptionPlanController::class, 'stats']);
    Route::get('/subscriptions/plans/{id}', [SubscriptionPlanController::class, 'show']);
    Route::put('/subscriptions/plans/{id}', [SubscriptionPlanController::class, 'update']);
    Route::delete('/subscriptions/plans/{id}', [SubscriptionPlanController::class, 'destroy']);

    // User Subscriptions
    Route::get('/subscriptions', [UserSubscriptionController::class, 'index']);
    Route::post('/subscriptions', [UserSubscriptionController::class, 'store']);
    Route::get('/subscriptions/{id}', [UserSubscriptionController::class, 'show']);
    Route::put('/subscriptions/{id}', [UserSubscriptionController::class, 'update']);
    Route::delete('/subscriptions/{id}', [UserSubscriptionController::class, 'destroy']);
    Route::post('/subscriptions/{id}/cancel', [UserSubscriptionController::class, 'cancel']);
    Route::post('/subscriptions/{id}/pause', [UserSubscriptionController::class, 'pause']);
    Route::post('/subscriptions/{id}/resume', [UserSubscriptionController::class, 'resume']);
    Route::post('/subscriptions/{id}/renew', [UserSubscriptionController::class, 'renew']);
    Route::get('/users/{userId}/subscriptions', [UserSubscriptionController::class, 'userSubscriptions']);

    // Products (Courses, Indicators, Memberships)
    Route::get('/products', [ProductController::class, 'index']);
    Route::post('/products', [ProductController::class, 'store']);
    Route::get('/products/stats', [ProductController::class, 'stats']);
    Route::get('/products/type/{type}', [ProductController::class, 'byType']);
    Route::get('/products/{id}', [ProductController::class, 'show']);
    Route::put('/products/{id}', [ProductController::class, 'update']);
    Route::delete('/products/{id}', [ProductController::class, 'destroy']);
    Route::post('/products/{id}/assign-user', [ProductController::class, 'assignToUser']);
    Route::post('/products/{id}/remove-user', [ProductController::class, 'removeFromUser']);
    Route::get('/products/{id}/users', [ProductController::class, 'productUsers']);

    // Popups (admin CRUD & analytics)
    Route::get('/popups', [PopupController::class, 'index']);
    Route::post('/popups', [PopupController::class, 'store']);
    Route::put('/popups/{popup}', [PopupController::class, 'update']);
    Route::delete('/popups/{popup}', [PopupController::class, 'destroy']);
    Route::get('/popups/{popup}/analytics', [PopupController::class, 'analytics']);
    Route::get('/popups/{popup}/optin-config', [PopupController::class, 'getOptInConfig']);
    Route::put('/popups/{popup}/optin-config', [PopupController::class, 'updateOptInConfig']);
    
    // Blog Posts (admin CRUD & analytics)
    Route::get('/posts', [AdminPostController::class, 'index']);
    Route::get('/posts/stats', [AdminPostController::class, 'stats']);
    Route::post('/posts', [AdminPostController::class, 'store']);
    Route::get('/posts/{id}', [AdminPostController::class, 'show']);
    Route::put('/posts/{id}', [AdminPostController::class, 'update']);
    Route::delete('/posts/{id}', [AdminPostController::class, 'destroy']);
    Route::post('/posts/bulk-delete', [AdminPostController::class, 'bulkDelete']);
    Route::post('/posts/bulk-update-status', [AdminPostController::class, 'bulkUpdateStatus']);
    Route::get('/posts/{id}/analytics', [AdminPostController::class, 'analytics']);
    Route::post('/posts/export', [AdminPostController::class, 'export']);

    // Blog Categories
    Route::get('/categories', [CategoryController::class, 'index']);
    Route::get('/categories/stats', [CategoryController::class, 'stats']);
    Route::post('/categories', [CategoryController::class, 'store']);
    Route::get('/categories/{id}', [CategoryController::class, 'show']);
    Route::put('/categories/{id}', [CategoryController::class, 'update']);
    Route::delete('/categories/{id}', [CategoryController::class, 'destroy']);
    Route::post('/categories/bulk-delete', [CategoryController::class, 'bulkDelete']);
    Route::post('/categories/bulk-update-visibility', [CategoryController::class, 'bulkUpdateVisibility']);
    Route::post('/categories/reorder', [CategoryController::class, 'reorder']);
    Route::post('/categories/merge', [CategoryController::class, 'merge']);
    Route::post('/categories/export', [CategoryController::class, 'export']);

    // Blog Tags
    Route::get('/tags', [TagController::class, 'index']);
    Route::get('/tags/stats', [TagController::class, 'stats']);
    Route::post('/tags', [TagController::class, 'store']);
    Route::get('/tags/{id}', [TagController::class, 'show']);
    Route::put('/tags/{id}', [TagController::class, 'update']);
    Route::delete('/tags/{id}', [TagController::class, 'destroy']);
    Route::post('/tags/bulk-delete', [TagController::class, 'bulkDelete']);
    Route::post('/tags/bulk-update-visibility', [TagController::class, 'bulkUpdateVisibility']);
    Route::post('/tags/reorder', [TagController::class, 'reorder']);
    Route::post('/tags/merge', [TagController::class, 'merge']);
    Route::post('/tags/export', [TagController::class, 'export']);
    
    // Video Management (Admin)
    Route::post('/videos', [VideoController::class, 'store']);
    Route::put('/videos/{id}', [VideoController::class, 'update']);
    Route::delete('/videos/{id}', [VideoController::class, 'destroy']);
    Route::get('/videos/{id}/analytics', [VideoController::class, 'analytics']);
    Route::get('/videos/{id}/heatmap', [VideoController::class, 'heatmap']);

    // ========================================
    // MEDIA & IMAGE OPTIMIZATION ENGINE
    // Google Enterprise Grade with Rate Limiting
    // ========================================

    // Media Library (Read operations - higher limits)
    Route::get('/media', [MediaController::class, 'index']);
    Route::get('/media/statistics', [MediaController::class, 'statistics']);
    Route::get('/media/collections', [MediaController::class, 'collections']);
    Route::get('/media/presets', [MediaController::class, 'presets']);
    Route::get('/media/{id}', [MediaController::class, 'show']);
    Route::get('/media/{id}/download', [MediaController::class, 'download'])
        ->middleware(RateLimitMedia::class . ':download');

    // Media Upload (Rate limited)
    Route::post('/media/upload', [MediaController::class, 'upload'])
        ->middleware(RateLimitMedia::class . ':upload');
    Route::post('/media/bulk-upload', [MediaController::class, 'bulkUpload'])
        ->middleware(RateLimitMedia::class . ':bulk_upload');
    Route::post('/media/{id}/replace', [MediaController::class, 'replace'])
        ->middleware(RateLimitMedia::class . ':upload');

    // Media Update/Delete (Rate limited)
    Route::put('/media/{id}', [MediaController::class, 'update']);
    Route::delete('/media/{id}', [MediaController::class, 'destroy'])
        ->middleware(RateLimitMedia::class . ':delete');
    Route::post('/media/bulk-delete', [MediaController::class, 'bulkDestroy'])
        ->middleware(RateLimitMedia::class . ':bulk_delete');

    // Image Optimization (Rate limited)
    Route::post('/media/{id}/optimize', [MediaController::class, 'optimize'])
        ->middleware(RateLimitMedia::class . ':optimize');
    Route::post('/media/bulk-optimize', [MediaController::class, 'bulkOptimize'])
        ->middleware(RateLimitMedia::class . ':bulk_optimize');
    Route::post('/media/optimize-all', [MediaController::class, 'optimizeAll'])
        ->middleware(RateLimitMedia::class . ':bulk_optimize');
    Route::post('/media/{id}/regenerate', [MediaController::class, 'regenerate'])
        ->middleware(RateLimitMedia::class . ':optimize');

    // Optimization Queue (No rate limiting - monitoring)
    Route::get('/media/queue/status', [MediaController::class, 'queueStatus']);
    Route::get('/media/jobs/{jobId}', [MediaController::class, 'jobStatus']);
});

// Abandoned Cart Reporting (public - for frontend tracking)
Route::post('/cart/abandoned', [AbandonedCartController::class, 'reportAbandoned']);

// Form preview and submission (public)
Route::get('/forms/preview/{slug}', [FormController::class, 'preview']);
Route::post('/forms/{slug}/submit', [FormSubmissionController::class, 'submit']);

// Forms (protected, admin only)
Route::middleware(['auth:sanctum', 'role:admin|super-admin'])->group(function () {
    Route::get('/forms', [FormController::class, 'index']);
    Route::post('/forms', [FormController::class, 'store']);
    Route::get('/forms/{id}', [FormController::class, 'show']);
    Route::put('/forms/{id}', [FormController::class, 'update']);
    Route::delete('/forms/{id}', [FormController::class, 'destroy']);
    Route::post('/forms/{id}/publish', [FormController::class, 'publish']);
    Route::post('/forms/{id}/unpublish', [FormController::class, 'unpublish']);
    Route::post('/forms/{id}/duplicate', [FormController::class, 'duplicate']);
    Route::get('/forms/field-types', [FormController::class, 'fieldTypes']);
    Route::get('/forms/stats', [FormController::class, 'stats']);

    // Form submissions
    Route::get('/forms/{formId}/submissions', [FormSubmissionController::class, 'index']);
    Route::get('/forms/{formId}/submissions/stats', [FormSubmissionController::class, 'stats']);
    Route::get('/forms/{formId}/submissions/{submissionId}', [FormSubmissionController::class, 'show']);
    Route::put('/forms/{formId}/submissions/{submissionId}/status', [FormSubmissionController::class, 'updateStatus']);
    Route::delete('/forms/{formId}/submissions/{submissionId}', [FormSubmissionController::class, 'destroy']);
    Route::post('/forms/{formId}/submissions/bulk-update-status', [FormSubmissionController::class, 'bulkUpdateStatus']);
    Route::post('/forms/{formId}/submissions/bulk-delete', [FormSubmissionController::class, 'bulkDelete']);
    Route::get('/forms/{formId}/submissions/export', [FormSubmissionController::class, 'export']);

    // Form submission PDFs (FluentForm Pro PDF feature)
    Route::get('/forms/{formId}/submissions/{submissionId}/pdfs', [FormPdfController::class, 'index']);
    Route::post('/forms/{formId}/submissions/{submissionId}/pdf', [FormPdfController::class, 'generate']);
    Route::get('/forms/{formId}/submissions/{submissionId}/pdf/{pdfId}/download', [FormPdfController::class, 'download']);
    Route::get('/forms/{formId}/submissions/{submissionId}/pdf/{pdfId}/preview', [FormPdfController::class, 'preview']);
    Route::delete('/forms/{formId}/submissions/{submissionId}/pdf/{pdfId}', [FormPdfController::class, 'destroy']);
    Route::get('/forms/{formId}/submissions/{submissionId}/pdf/download-direct', [FormPdfController::class, 'downloadDirect']);
    Route::get('/forms/{formId}/submissions/{submissionId}/pdf/stream', [FormPdfController::class, 'streamDirect']);
    Route::post('/forms/{formId}/submissions/{submissionId}/receipt', [FormPdfController::class, 'receipt']);

    // Form PDF templates
    Route::get('/forms/pdf/config', [FormPdfController::class, 'config']); // Configuration options
    Route::get('/forms/{formId}/pdf-templates', [FormPdfController::class, 'templates']);
    Route::post('/forms/{formId}/pdf-templates', [FormPdfController::class, 'storeTemplate']);
    Route::put('/forms/{formId}/pdf-templates/{templateId}', [FormPdfController::class, 'updateTemplate']);
    Route::delete('/forms/{formId}/pdf-templates/{templateId}', [FormPdfController::class, 'destroyTemplate']);
    Route::get('/forms/{formId}/pdf-templates/{templateId}/preview', [FormPdfController::class, 'previewTemplate']);

    // Form Approval Workflow (FluentForms 6.1.8 - December 2025)
    Route::get('/forms/{formId}/approval/pending', [FormApprovalController::class, 'pending']);
    Route::get('/forms/{formId}/approval/status/{status}', [FormApprovalController::class, 'byStatus']);
    Route::get('/forms/{formId}/approval/stats', [FormApprovalController::class, 'stats']);
    Route::match(['get', 'post'], '/forms/{formId}/approval/settings', [FormApprovalController::class, 'settings']);
    Route::post('/submissions/{submissionId}/approve', [FormApprovalController::class, 'approve']);
    Route::post('/submissions/{submissionId}/reject', [FormApprovalController::class, 'reject']);
    Route::post('/submissions/{submissionId}/request-revision', [FormApprovalController::class, 'requestRevision']);
    Route::post('/submissions/{submissionId}/hold', [FormApprovalController::class, 'hold']);
    Route::post('/submissions/{submissionId}/reset-approval', [FormApprovalController::class, 'reset']);
    Route::get('/submissions/{submissionId}/approval-history', [FormApprovalController::class, 'history']);
    Route::post('/submissions/bulk-approve', [FormApprovalController::class, 'bulkApprove']);
    Route::post('/submissions/bulk-reject', [FormApprovalController::class, 'bulkReject']);

    // Form Inventory Management (FluentForms 6.1.8 - December 2025)
    Route::get('/forms/fields/{fieldId}/inventory', [FormInventoryController::class, 'fieldInventory']);
    Route::get('/forms/fields/{fieldId}/inventory/{productId}', [FormInventoryController::class, 'getStock']);
    Route::put('/forms/fields/{fieldId}/inventory/{productId}', [FormInventoryController::class, 'updateStock']);
    Route::post('/forms/inventory/reserve', [FormInventoryController::class, 'reserve']);
    Route::post('/forms/inventory/release', [FormInventoryController::class, 'release']);
    Route::post('/forms/inventory/check-availability', [FormInventoryController::class, 'checkAvailability']);
    Route::post('/forms/inventory/validate-cart', [FormInventoryController::class, 'validateCart']);
    Route::get('/forms/{formId}/inventory/low-stock', [FormInventoryController::class, 'lowStock']);
    Route::get('/forms/{formId}/inventory/report', [FormInventoryController::class, 'report']);
    Route::post('/forms/inventory/bulk-update', [FormInventoryController::class, 'bulkUpdate']);
});

// Download routes
Route::middleware(['auth:sanctum', 'verified'])->group(function () {
    Route::get('/download/indicator/{slug}/{downloadId}', [IndicatorController::class, 'download']);
});


// ========================================
// ENTERPRISE POST SERVICE - Related Posts
// ========================================
use App\Services\Post\FeedService;

// Related posts endpoint (using existing PostController)
Route::get('/posts/{slug}/related-scores', [PostController::class, 'relatedWithScores']);

// RSS/Atom Feed routes
Route::get('/feed/rss', function (FeedService $feedService) {
    return response($feedService->generateRss())
        ->header('Content-Type', 'application/rss+xml');
});

Route::get('/feed/atom', function (FeedService $feedService) {
    return response($feedService->generateAtom())
        ->header('Content-Type', 'application/atom+xml');
});

// ========================================
// ENTERPRISE ANALYTICS ENGINE
// ========================================
use App\Http\Controllers\Api\AnalyticsController;
use App\Http\Controllers\Api\PaymentController;

// ========================================
// STRIPE PAYMENT GATEWAY
// ========================================

// Public payment config (no auth required)
Route::get('/payments/config', [PaymentController::class, 'config']);

// Stripe webhook (no auth - uses signature verification)
Route::post('/payments/webhook', [PaymentController::class, 'webhook']);

// Protected payment routes
Route::middleware(['auth:sanctum'])->prefix('payments')->group(function () {
    Route::post('/create-intent', [PaymentController::class, 'createPaymentIntent']);
    Route::post('/create-checkout', [PaymentController::class, 'createCheckoutSession']);
    Route::post('/confirm', [PaymentController::class, 'confirmPayment']);
    Route::get('/order/{order:order_number}/status', [PaymentController::class, 'orderStatus']);
});

// Admin-only refund endpoint
Route::middleware(['auth:sanctum', 'role:admin|super-admin'])->group(function () {
    Route::post('/payments/refund', [PaymentController::class, 'refund']);
});

// Public event tracking (client-side)
Route::prefix('analytics')->group(function () {
    Route::post('/track', [AnalyticsController::class, 'trackEvent']);
    Route::post('/pageview', [AnalyticsController::class, 'trackPageView']);
    Route::post('/batch', [AnalyticsController::class, 'trackBatch']);
});

// Protected analytics API (admin)
Route::middleware(['auth:sanctum'])->prefix('admin/analytics')->group(function () {
    // Dashboard & KPIs
    Route::get('/dashboard', [AnalyticsController::class, 'getDashboard']);
    Route::get('/kpis', [AnalyticsController::class, 'getKpiDefinitions']);
    Route::get('/kpis/{kpiKey}', [AnalyticsController::class, 'getKpi']);

    // Funnel Analytics
    Route::get('/funnels', [AnalyticsController::class, 'getFunnels']);
    Route::get('/funnels/{funnelKey}', [AnalyticsController::class, 'getFunnelAnalysis']);
    Route::get('/funnels/{funnelKey}/dropoff', [AnalyticsController::class, 'getFunnelDropOff']);
    Route::get('/funnels/{funnelKey}/segment', [AnalyticsController::class, 'getFunnelBySegment']);

    // Cohort Analytics
    Route::get('/cohorts', [AnalyticsController::class, 'getCohorts']);
    Route::get('/cohorts/{cohortKey}/matrix', [AnalyticsController::class, 'getCohortMatrix']);
    Route::get('/cohorts/{cohortKey}/curve', [AnalyticsController::class, 'getCohortCurve']);
    Route::get('/cohorts/{cohortKey}/ltv', [AnalyticsController::class, 'getCohortLTV']);

    // Attribution Analytics
    Route::get('/attribution/channels', [AnalyticsController::class, 'getChannelAttribution']);
    Route::get('/attribution/campaigns', [AnalyticsController::class, 'getCampaignAttribution']);
    Route::get('/attribution/paths', [AnalyticsController::class, 'getConversionPaths']);
    Route::get('/attribution/compare', [AnalyticsController::class, 'compareAttributionModels']);

    // Forecasting
    Route::get('/forecast/{kpiKey}', [AnalyticsController::class, 'getForecast']);
    Route::get('/forecast/{kpiKey}/accuracy', [AnalyticsController::class, 'getForecastAccuracy']);

    // Segments
    Route::get('/segments', [AnalyticsController::class, 'getSegments']);
    Route::get('/segments/{segmentKey}', [AnalyticsController::class, 'getSegment']);

    // Event Explorer
    Route::get('/events/timeseries', [AnalyticsController::class, 'getEventTimeSeries']);
    Route::get('/events/breakdown', [AnalyticsController::class, 'getEventBreakdown']);
    Route::get('/realtime', [AnalyticsController::class, 'getRealTimeMetrics']);
});

// ========================================
// REVOLUTION BEHAVIOR L8 SYSTEM
// ========================================
use App\Http\Controllers\Admin\BehaviorController;

// Public behavior tracking (client-side)
Route::post('/behavior/events', [BehaviorController::class, 'ingestEvents']);

// Protected behavior analytics API (admin)
Route::middleware(['auth:sanctum', 'role:admin|super-admin'])->prefix('admin/behavior')->group(function () {
    Route::get('/dashboard', [BehaviorController::class, 'getDashboard']);
    Route::get('/sessions/{sessionId}', [BehaviorController::class, 'getSession']);
    Route::get('/friction-points', [BehaviorController::class, 'getFrictionPoints']);
    Route::get('/intent-signals', [BehaviorController::class, 'getIntentSignals']);
    Route::patch('/friction-points/{id}/resolve', [BehaviorController::class, 'resolveFrictionPoint']);
});

// ========================================
// REVOLUTION EXPERIMENTS & FEATURE FLAGS
// ========================================
use App\Http\Controllers\Api\ExperimentController;

// Public experiment config (client-side)
Route::prefix('experiments')->group(function () {
    Route::get('/config', [ExperimentController::class, 'getConfig']);
    Route::post('/exposure', [ExperimentController::class, 'trackExposure']);
    Route::post('/conversion', [ExperimentController::class, 'trackConversion']);
});

// Protected experiments API (admin)
Route::middleware(['auth:sanctum', 'role:admin|super-admin'])->prefix('admin/experiments')->group(function () {
    Route::get('/', [ExperimentController::class, 'index']);
    Route::post('/', [ExperimentController::class, 'store']);
    Route::get('/{id}', [ExperimentController::class, 'show']);
    Route::put('/{id}', [ExperimentController::class, 'update']);
    Route::delete('/{id}', [ExperimentController::class, 'destroy']);
});

// Protected feature flags API (admin)
Route::middleware(['auth:sanctum', 'role:admin|super-admin'])->prefix('admin/feature-flags')->group(function () {
    Route::get('/', [ExperimentController::class, 'listFlags']);
    Route::post('/', [ExperimentController::class, 'storeFlag']);
    Route::put('/{id}', [ExperimentController::class, 'updateFlag']);
    Route::delete('/{id}', [ExperimentController::class, 'destroyFlag']);
});

// ========================================
// REVOLUTION CRM L8 SYSTEM
// ========================================
use App\Http\Controllers\Admin\ContactController;
use App\Http\Controllers\Admin\DealController;
use App\Http\Controllers\Admin\PipelineController;

// Protected CRM API (admin)
Route::middleware(['auth:sanctum', 'role:admin|super-admin'])->prefix('admin/crm')->group(function () {
    // Contacts
    Route::get('/contacts', [ContactController::class, 'index']);
    Route::post('/contacts', [ContactController::class, 'store']);
    Route::get('/contacts/{id}', [ContactController::class, 'show']);
    Route::put('/contacts/{id}', [ContactController::class, 'update']);
    Route::delete('/contacts/{id}', [ContactController::class, 'destroy']);
    Route::get('/contacts/{id}/timeline', [ContactController::class, 'timeline']);
    Route::post('/contacts/{id}/recalculate-score', [ContactController::class, 'recalculateScore']);
    
    // Deals
    Route::get('/deals', [DealController::class, 'index']);
    Route::post('/deals', [DealController::class, 'store']);
    Route::get('/deals/{id}', [DealController::class, 'show']);
    Route::put('/deals/{id}', [DealController::class, 'update']);
    Route::patch('/deals/{id}/stage', [DealController::class, 'updateStage']);
    Route::post('/deals/{id}/win', [DealController::class, 'win']);
    Route::post('/deals/{id}/lose', [DealController::class, 'lose']);
    Route::get('/deals/forecast', [DealController::class, 'forecast']);
    
    // Pipelines
    Route::get('/pipelines', [PipelineController::class, 'index']);
    Route::post('/pipelines', [PipelineController::class, 'store']);
    Route::get('/pipelines/{id}', [PipelineController::class, 'show']);
    Route::put('/pipelines/{id}', [PipelineController::class, 'update']);
    Route::delete('/pipelines/{id}', [PipelineController::class, 'destroy']);
    Route::post('/pipelines/{id}/stages', [PipelineController::class, 'addStage']);

    // ========================================
    // FLUENTCRM PRO - EMAIL SEQUENCES (Drip Campaigns)
    // ========================================
    Route::prefix('sequences')->group(function () {
        Route::get('/', [\App\Http\Controllers\Admin\SequenceController::class, 'index']);
        Route::post('/', [\App\Http\Controllers\Admin\SequenceController::class, 'store']);
        Route::get('/{id}', [\App\Http\Controllers\Admin\SequenceController::class, 'show']);
        Route::put('/{id}', [\App\Http\Controllers\Admin\SequenceController::class, 'update']);
        Route::delete('/{id}', [\App\Http\Controllers\Admin\SequenceController::class, 'destroy']);
        Route::post('/{id}/duplicate', [\App\Http\Controllers\Admin\SequenceController::class, 'duplicate']);

        // Sequence Emails
        Route::post('/{id}/emails', [\App\Http\Controllers\Admin\SequenceController::class, 'storeEmail']);
        Route::get('/{id}/emails/{emailId}', [\App\Http\Controllers\Admin\SequenceController::class, 'showEmail']);
        Route::put('/{id}/emails/{emailId}', [\App\Http\Controllers\Admin\SequenceController::class, 'updateEmail']);
        Route::delete('/{id}/emails/{emailId}', [\App\Http\Controllers\Admin\SequenceController::class, 'destroyEmail']);
        Route::post('/{id}/emails/duplicate', [\App\Http\Controllers\Admin\SequenceController::class, 'duplicateEmail']);

        // Sequence Subscribers
        Route::get('/{id}/subscribers', [\App\Http\Controllers\Admin\SequenceController::class, 'getSubscribers']);
        Route::post('/{id}/subscribe', [\App\Http\Controllers\Admin\SequenceController::class, 'subscribe']);
        Route::post('/{id}/unsubscribe', [\App\Http\Controllers\Admin\SequenceController::class, 'unsubscribe']);
        Route::post('/{id}/reapply', [\App\Http\Controllers\Admin\SequenceController::class, 'reapply']);
    });

    // ========================================
    // FLUENTCRM PRO - RECURRING CAMPAIGNS (Scheduled Newsletters)
    // ========================================
    Route::prefix('recurring-campaigns')->group(function () {
        Route::get('/', [\App\Http\Controllers\Admin\RecurringCampaignController::class, 'index']);
        Route::post('/', [\App\Http\Controllers\Admin\RecurringCampaignController::class, 'store']);
        Route::get('/{id}', [\App\Http\Controllers\Admin\RecurringCampaignController::class, 'show']);
        Route::put('/{id}', [\App\Http\Controllers\Admin\RecurringCampaignController::class, 'update']);
        Route::delete('/{id}', [\App\Http\Controllers\Admin\RecurringCampaignController::class, 'destroy']);
        Route::patch('/{id}/status', [\App\Http\Controllers\Admin\RecurringCampaignController::class, 'changeStatus']);
        Route::post('/{id}/duplicate', [\App\Http\Controllers\Admin\RecurringCampaignController::class, 'duplicate']);
        Route::put('/{id}/settings', [\App\Http\Controllers\Admin\RecurringCampaignController::class, 'updateSettings']);
        Route::put('/{id}/labels', [\App\Http\Controllers\Admin\RecurringCampaignController::class, 'updateLabels']);
        Route::put('/{id}/email', [\App\Http\Controllers\Admin\RecurringCampaignController::class, 'updateCampaignEmail']);

        // Recurring Campaign Emails (sent instances)
        Route::get('/{id}/emails', [\App\Http\Controllers\Admin\RecurringCampaignController::class, 'getEmails']);
        Route::get('/{id}/emails/{emailId}', [\App\Http\Controllers\Admin\RecurringCampaignController::class, 'getEmail']);

        // Bulk operations
        Route::post('/bulk-delete', [\App\Http\Controllers\Admin\RecurringCampaignController::class, 'deleteBulk']);
        Route::post('/bulk-action', [\App\Http\Controllers\Admin\RecurringCampaignController::class, 'handleBulkAction']);
    });

    // ========================================
    // FLUENTCRM PRO - SMART LINKS (Action Links)
    // ========================================
    Route::prefix('smart-links')->group(function () {
        Route::get('/', [\App\Http\Controllers\Admin\SmartLinkController::class, 'index']);
        Route::post('/', [\App\Http\Controllers\Admin\SmartLinkController::class, 'store']);
        Route::get('/{id}', [\App\Http\Controllers\Admin\SmartLinkController::class, 'show']);
        Route::put('/{id}', [\App\Http\Controllers\Admin\SmartLinkController::class, 'update']);
        Route::delete('/{id}', [\App\Http\Controllers\Admin\SmartLinkController::class, 'destroy']);
        Route::post('/activate', [\App\Http\Controllers\Admin\SmartLinkController::class, 'activate']);
        Route::get('/{id}/clicks', [\App\Http\Controllers\Admin\SmartLinkController::class, 'getClicks']);
        Route::get('/{id}/analytics', [\App\Http\Controllers\Admin\SmartLinkController::class, 'getAnalytics']);
    });

    // ========================================
    // FLUENTCRM PRO - AUTOMATION FUNNELS
    // ========================================
    Route::prefix('automations')->group(function () {
        Route::get('/trigger-types', [\App\Http\Controllers\Admin\AutomationFunnelController::class, 'getTriggerTypes']);
        Route::get('/action-types', [\App\Http\Controllers\Admin\AutomationFunnelController::class, 'getActionTypes']);

        Route::get('/', [\App\Http\Controllers\Admin\AutomationFunnelController::class, 'index']);
        Route::post('/', [\App\Http\Controllers\Admin\AutomationFunnelController::class, 'store']);
        Route::get('/{id}', [\App\Http\Controllers\Admin\AutomationFunnelController::class, 'show']);
        Route::put('/{id}', [\App\Http\Controllers\Admin\AutomationFunnelController::class, 'update']);
        Route::delete('/{id}', [\App\Http\Controllers\Admin\AutomationFunnelController::class, 'destroy']);
        Route::post('/{id}/duplicate', [\App\Http\Controllers\Admin\AutomationFunnelController::class, 'duplicate']);
        Route::post('/{id}/activate', [\App\Http\Controllers\Admin\AutomationFunnelController::class, 'activate']);
        Route::post('/{id}/pause', [\App\Http\Controllers\Admin\AutomationFunnelController::class, 'pause']);

        // Funnel Actions
        Route::post('/{id}/actions', [\App\Http\Controllers\Admin\AutomationFunnelController::class, 'storeAction']);
        Route::put('/{id}/actions/{actionId}', [\App\Http\Controllers\Admin\AutomationFunnelController::class, 'updateAction']);
        Route::delete('/{id}/actions/{actionId}', [\App\Http\Controllers\Admin\AutomationFunnelController::class, 'destroyAction']);
        Route::post('/{id}/actions/reorder', [\App\Http\Controllers\Admin\AutomationFunnelController::class, 'reorderActions']);

        // Funnel Subscribers
        Route::get('/{id}/subscribers', [\App\Http\Controllers\Admin\AutomationFunnelController::class, 'getSubscribers']);
        Route::post('/{id}/subscribers', [\App\Http\Controllers\Admin\AutomationFunnelController::class, 'enrollContact']);
        Route::delete('/{id}/subscribers/{subscriberId}', [\App\Http\Controllers\Admin\AutomationFunnelController::class, 'removeSubscriber']);
        Route::post('/{id}/subscribers/{subscriberId}/retry', [\App\Http\Controllers\Admin\AutomationFunnelController::class, 'retrySubscriber']);
    });

    // ========================================
    // FLUENTCRM PRO - CONTACT LISTS
    // ========================================
    Route::prefix('lists')->group(function () {
        Route::get('/', [\App\Http\Controllers\Admin\ContactListController::class, 'index']);
        Route::post('/', [\App\Http\Controllers\Admin\ContactListController::class, 'store']);
        Route::get('/{id}', [\App\Http\Controllers\Admin\ContactListController::class, 'show']);
        Route::put('/{id}', [\App\Http\Controllers\Admin\ContactListController::class, 'update']);
        Route::delete('/{id}', [\App\Http\Controllers\Admin\ContactListController::class, 'destroy']);
        Route::get('/{id}/contacts', [\App\Http\Controllers\Admin\ContactListController::class, 'getContacts']);
        Route::post('/{id}/contacts', [\App\Http\Controllers\Admin\ContactListController::class, 'addContacts']);
        Route::delete('/{id}/contacts', [\App\Http\Controllers\Admin\ContactListController::class, 'removeContacts']);
    });

    // ========================================
    // FLUENTCRM PRO - CONTACT TAGS
    // ========================================
    Route::prefix('contact-tags')->group(function () {
        Route::get('/', [\App\Http\Controllers\Admin\ContactTagController::class, 'index']);
        Route::post('/', [\App\Http\Controllers\Admin\ContactTagController::class, 'store']);
        Route::get('/{id}', [\App\Http\Controllers\Admin\ContactTagController::class, 'show']);
        Route::put('/{id}', [\App\Http\Controllers\Admin\ContactTagController::class, 'update']);
        Route::delete('/{id}', [\App\Http\Controllers\Admin\ContactTagController::class, 'destroy']);
        Route::get('/{id}/contacts', [\App\Http\Controllers\Admin\ContactTagController::class, 'getContacts']);
        Route::post('/{id}/apply', [\App\Http\Controllers\Admin\ContactTagController::class, 'applyToContacts']);
        Route::post('/{id}/remove', [\App\Http\Controllers\Admin\ContactTagController::class, 'removeFromContacts']);
        Route::post('/bulk-apply', [\App\Http\Controllers\Admin\ContactTagController::class, 'bulkApply']);
    });

    // ========================================
    // FLUENTCRM PRO - CRM COMPANIES (B2B)
    // ========================================
    Route::prefix('companies')->group(function () {
        Route::get('/industries', [\App\Http\Controllers\Admin\CrmCompanyController::class, 'getIndustries']);
        Route::get('/sizes', [\App\Http\Controllers\Admin\CrmCompanyController::class, 'getSizes']);

        Route::get('/', [\App\Http\Controllers\Admin\CrmCompanyController::class, 'index']);
        Route::post('/', [\App\Http\Controllers\Admin\CrmCompanyController::class, 'store']);
        Route::get('/{id}', [\App\Http\Controllers\Admin\CrmCompanyController::class, 'show']);
        Route::put('/{id}', [\App\Http\Controllers\Admin\CrmCompanyController::class, 'update']);
        Route::delete('/{id}', [\App\Http\Controllers\Admin\CrmCompanyController::class, 'destroy']);
        Route::get('/{id}/contacts', [\App\Http\Controllers\Admin\CrmCompanyController::class, 'getContacts']);
        Route::get('/{id}/deals', [\App\Http\Controllers\Admin\CrmCompanyController::class, 'getDeals']);
    });
});

// ========================================
// SANITY-EQUIVALENT CONTENT LAKE API
// ========================================
use App\Http\Controllers\Api\ContentLakeController;

// Public preview endpoint
Route::get('/preview', [ContentLakeController::class, 'getByPreviewToken']);

// Protected Content Lake API
Route::middleware(['auth:sanctum'])->prefix('content-lake')->group(function () {
    // GROQ Query API
    Route::post('/query', [ContentLakeController::class, 'query']);

    // Portable Text API
    Route::post('/portable-text/render', [ContentLakeController::class, 'renderPortableText']);
    Route::post('/portable-text/validate', [ContentLakeController::class, 'validatePortableText']);

    // Document Perspective API
    Route::get('/documents/{documentId}', [ContentLakeController::class, 'getDocument']);
    Route::patch('/documents/{documentId}/draft', [ContentLakeController::class, 'updateDraft']);
    Route::post('/documents/{documentId}/publish', [ContentLakeController::class, 'publishDocument']);
    Route::post('/documents/{documentId}/unpublish', [ContentLakeController::class, 'unpublishDocument']);
    Route::get('/documents/{documentId}/diff', [ContentLakeController::class, 'getDocumentDiff']);
    Route::post('/documents/{documentId}/preview-token', [ContentLakeController::class, 'createPreviewToken']);

    // Document History API
    Route::get('/documents/{documentId}/history', [ContentLakeController::class, 'getHistory']);
    Route::get('/documents/{documentId}/history/{revisionNumber}', [ContentLakeController::class, 'getRevision']);
    Route::post('/documents/{documentId}/history/{revisionNumber}/restore', [ContentLakeController::class, 'restoreRevision']);
    Route::post('/documents/{documentId}/history/compare', [ContentLakeController::class, 'compareRevisions']);
});

// Admin-only Content Lake API
Route::middleware(['auth:sanctum', 'role:admin|super-admin'])->prefix('admin/content-lake')->group(function () {
    // Webhooks API
    Route::get('/webhooks', [ContentLakeController::class, 'listWebhooks']);
    Route::post('/webhooks', [ContentLakeController::class, 'createWebhook']);
    Route::put('/webhooks/{webhookId}', [ContentLakeController::class, 'updateWebhook']);
    Route::delete('/webhooks/{webhookId}', [ContentLakeController::class, 'deleteWebhook']);
    Route::post('/webhooks/{webhookId}/test', [ContentLakeController::class, 'testWebhook']);
    Route::get('/webhooks/{webhookId}/deliveries', [ContentLakeController::class, 'getWebhookDeliveries']);

    // Image Hotspot/Crop API
    Route::post('/media/{mediaId}/hotspot', [ContentLakeController::class, 'setHotspot']);
    Route::post('/media/{mediaId}/crop', [ContentLakeController::class, 'setCrop']);
    Route::get('/media/{mediaId}/url', [ContentLakeController::class, 'getImageUrl']);
    Route::post('/media/{mediaId}/placeholders', [ContentLakeController::class, 'generatePlaceholders']);

    // Schema API
    Route::get('/schemas', [ContentLakeController::class, 'listSchemas']);
    Route::get('/schemas/{name}', [ContentLakeController::class, 'getSchema']);
    Route::post('/schemas', [ContentLakeController::class, 'registerSchema']);
    Route::post('/schemas/{name}/validate', [ContentLakeController::class, 'validateAgainstSchema']);
    Route::get('/schemas/{name}/typescript', [ContentLakeController::class, 'generateTypeScript']);

    // Release Bundles API
    Route::get('/releases', [ContentLakeController::class, 'listReleases']);
    Route::get('/releases/{bundleId}', [ContentLakeController::class, 'getRelease']);
    Route::post('/releases', [ContentLakeController::class, 'createRelease']);
    Route::post('/releases/{bundleId}/documents', [ContentLakeController::class, 'addDocumentToRelease']);
    Route::delete('/releases/{bundleId}/documents/{documentId}', [ContentLakeController::class, 'removeDocumentFromRelease']);
    Route::post('/releases/{bundleId}/schedule', [ContentLakeController::class, 'scheduleRelease']);
    Route::post('/releases/{bundleId}/publish', [ContentLakeController::class, 'publishRelease']);
    Route::post('/releases/{bundleId}/cancel', [ContentLakeController::class, 'cancelRelease']);
    Route::delete('/releases/{bundleId}', [ContentLakeController::class, 'deleteRelease']);
});

// ========================================
// BING SEO & INDEXNOW - COMPETITIVE ADVANTAGE
// ========================================
use App\Http\Controllers\Api\BingSeoController;
use App\Http\Controllers\Api\PerformanceController;

// Public IndexNow key file (required for verification)
Route::get('/{key}.txt', [BingSeoController::class, 'indexNowKeyFile'])
    ->where('key', '[a-f0-9\-]{36}');

// Protected Bing SEO API (admin)
Route::middleware(['auth:sanctum', 'role:admin|super-admin'])->prefix('admin/bing-seo')->group(function () {
    // Configuration & Status
    Route::get('/config', [BingSeoController::class, 'getConfiguration']);
    Route::get('/checklist', [BingSeoController::class, 'getChecklist']);

    // IndexNow URL Submission (instant indexing)
    Route::post('/indexnow/submit', [BingSeoController::class, 'submitUrl']);
    Route::post('/indexnow/batch', [BingSeoController::class, 'batchSubmitUrls']);

    // Bing Webmaster API
    Route::post('/webmaster/submit', [BingSeoController::class, 'submitToWebmaster']);
    Route::post('/sitemap/submit', [BingSeoController::class, 'submitSitemap']);

    // Analytics
    Route::get('/stats', [BingSeoController::class, 'getSubmissionStats']);
    Route::get('/performance', [BingSeoController::class, 'getSearchPerformance']);
    Route::get('/crawl-stats', [BingSeoController::class, 'getCrawlStats']);

    // Meta Tags & Schema
    Route::get('/meta-tags', [BingSeoController::class, 'getMetaTags']);
    Route::post('/schema', [BingSeoController::class, 'generateSchema']);
});

// ========================================
// SHARP IMAGE SERVICE (HIGH-PERFORMANCE)
// ========================================
use App\Http\Controllers\Api\SharpMediaController;

// Public Sharp service health check
Route::get('/sharp/health', [SharpMediaController::class, 'health']);

// Protected Sharp Media API (admin)
Route::middleware(['auth:sanctum', 'role:admin|super-admin'])->prefix('admin/sharp')->group(function () {
    // Upload & Processing
    Route::post('/upload', [SharpMediaController::class, 'upload']);
    Route::post('/bulk-upload', [SharpMediaController::class, 'bulkUpload']);
    Route::post('/process-url', [SharpMediaController::class, 'processUrl']);

    // Optimization
    Route::post('/optimize/{id}', [SharpMediaController::class, 'optimize']);

    // Media Management
    Route::get('/media', [SharpMediaController::class, 'index']);
    Route::get('/media/{id}', [SharpMediaController::class, 'show']);
    Route::delete('/media/{id}', [SharpMediaController::class, 'destroy']);

    // Statistics
    Route::get('/statistics', [SharpMediaController::class, 'statistics']);
});

// ========================================
// PERFORMANCE OPTIMIZATION ENGINE
// ========================================

// Protected Performance API (admin)
Route::middleware(['auth:sanctum', 'role:admin|super-admin'])->prefix('admin/performance')->group(function () {
    // Dashboard & Metrics
    Route::get('/dashboard', [PerformanceController::class, 'dashboard']);
    Route::get('/server', [PerformanceController::class, 'serverMetrics']);
    Route::get('/database', [PerformanceController::class, 'databaseMetrics']);
    Route::get('/cache', [PerformanceController::class, 'cacheMetrics']);

    // Optimization
    Route::get('/recommendations', [PerformanceController::class, 'recommendations']);
    Route::post('/warm-caches', [PerformanceController::class, 'warmCaches']);
    Route::get('/core-web-vitals', [PerformanceController::class, 'coreWebVitals']);

    // Query Analysis
    Route::post('/queries/analyze', [PerformanceController::class, 'analyzeQueries']);
    Route::get('/queries/results', [PerformanceController::class, 'queryResults']);
});

// ========================================
// TRADING ROOMS & ALERT SERVICES API
// ========================================
use App\Http\Controllers\Api\TradingRoomController;
use App\Http\Controllers\Api\TradingRoomSSOController;

// Public trading room routes (for member dashboards)
Route::middleware(['auth:sanctum'])->prefix('trading-rooms')->group(function () {
    // List rooms user has access to
    Route::get('/', [TradingRoomController::class, 'index']);
    Route::get('/{slug}', [TradingRoomController::class, 'show']);

    // Daily Videos (by room)
    Route::get('/{slug}/videos', [TradingRoomController::class, 'listVideos']);

    // Traders
    Route::get('/traders', [TradingRoomController::class, 'listTraders']);

    // ========================================
    // TRADING ROOM SSO (JWT Authentication)
    // ========================================
    // Get all accessible rooms with SSO info
    Route::get('/sso/accessible', [TradingRoomSSOController::class, 'getAccessibleRooms']);

    // Generate JWT token for room access
    Route::post('/{slug}/sso', [TradingRoomSSOController::class, 'generateToken']);

    // Direct SSO redirect (browser)
    Route::get('/{slug}/sso/redirect', [TradingRoomSSOController::class, 'redirect']);
});

// Public: Verify JWT token (for trading room platform to call back)
Route::post('/trading-rooms/sso/verify', [TradingRoomSSOController::class, 'verifyToken']);

// Admin trading room routes (full CRUD)
Route::middleware(['auth:sanctum', 'role:admin|super-admin'])->prefix('admin/trading-rooms')->group(function () {
    // Trading Rooms CRUD
    Route::get('/', [TradingRoomController::class, 'index']);
    Route::post('/', [TradingRoomController::class, 'store']);
    Route::get('/{id}', [TradingRoomController::class, 'show'])->where('id', '[0-9]+');
    Route::put('/{id}', [TradingRoomController::class, 'update']);
    Route::delete('/{id}', [TradingRoomController::class, 'destroy']);

    // Traders CRUD
    Route::get('/traders', [TradingRoomController::class, 'listTraders']);
    Route::post('/traders', [TradingRoomController::class, 'storeTrader']);
    Route::put('/traders/{id}', [TradingRoomController::class, 'updateTrader']);
    Route::delete('/traders/{id}', [TradingRoomController::class, 'destroyTrader']);

    // Daily Videos CRUD
    Route::get('/videos/{roomSlug}', [TradingRoomController::class, 'listVideos']);
    Route::post('/videos', [TradingRoomController::class, 'storeVideo']);
    Route::post('/videos/bulk', [TradingRoomController::class, 'bulkStoreVideos']);
    Route::put('/videos/{id}', [TradingRoomController::class, 'updateVideo']);
    Route::delete('/videos/{id}', [TradingRoomController::class, 'destroyVideo']);
});
