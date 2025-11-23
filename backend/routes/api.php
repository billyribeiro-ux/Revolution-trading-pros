<?php

use App\Http\Controllers\Api\AuthController;
use App\Http\Controllers\Api\BacklinkController;
use App\Http\Controllers\Api\CartController;
use App\Http\Controllers\Api\CouponController;
use App\Http\Controllers\Api\Error404Controller;
use App\Http\Controllers\Api\FormController;
use App\Http\Controllers\Api\FormSubmissionController;
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
use App\Http\Controllers\Admin\CategoryController;
use App\Http\Controllers\Admin\EmailSettingsController;
use App\Http\Controllers\Admin\EmailTemplateController;
use App\Http\Controllers\Admin\ProductController;
use App\Http\Controllers\Admin\SettingsController;
use App\Http\Controllers\Admin\SubscriptionPlanController;
use App\Http\Controllers\Admin\TagController;
use App\Http\Controllers\Admin\UserSubscriptionController;
use App\Http\Controllers\Admin\UserController;
use Illuminate\Support\Facades\Route;

// Public routes
Route::get('/time/now', [TimeController::class, 'now']);
Route::post('/timers/events', [TimerAnalyticsController::class, 'store']);
Route::post('/telemetry/admin-toolbar', function() { return response()->json(['status' => 'ok']); });
Route::post('/register', [AuthController::class, 'register']);
Route::post('/login', [AuthController::class, 'login']);
Route::post('/login/mfa', [AuthController::class, 'loginWithMFA']);
Route::post('/login/biometric', [AuthController::class, 'loginWithBiometric']);
Route::post('/auth/refresh', [AuthController::class, 'refreshToken']);
Route::post('/forgot-password', [AuthController::class, 'forgotPassword']);
Route::post('/reset-password', [AuthController::class, 'resetPassword']);

// Public popup runtime routes
Route::get('/popups/active', [PopupController::class, 'active']);
Route::post('/popups/{popup}/impression', [PopupController::class, 'impression']);
Route::post('/popups/{popup}/conversion', [PopupController::class, 'conversion']);
Route::post('/popups/events', [PopupController::class, 'events']);

// Email verification routes
Route::get('/email/verify/{id}/{hash}', [AuthController::class, 'verify'])
    ->middleware(['signed'])->name('verification.verify');
Route::post('/email/verification-notification', [AuthController::class, 'sendVerification'])
    ->middleware(['auth:sanctum'])->name('verification.send');

// Newsletter
Route::post('/newsletter/subscribe', [NewsletterController::class, 'subscribe']);
Route::get('/newsletter/confirm', [NewsletterController::class, 'confirm']);
Route::get('/newsletter/unsubscribe', [NewsletterController::class, 'unsubscribe']);

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
    Route::post('/logout', [AuthController::class, 'logout']);
    Route::get('/me', [MeController::class, 'show']);
    Route::get('/me/memberships', [MeController::class, 'memberships']);
    Route::get('/me/products', [MeController::class, 'products']);
    Route::get('/me/indicators', [MeController::class, 'indicators']);
    Route::get('/me/security-events', [AuthController::class, 'listSecurityEvents']);
    Route::get('/auth/check', [AuthController::class, 'check']);
    Route::post('/security/events', [AuthController::class, 'securityEvent']);
    Route::put('/me/password', [AuthController::class, 'changePassword']);
    Route::post('/me/mfa/enable', [AuthController::class, 'enableMFA']);
    Route::post('/me/mfa/verify', [AuthController::class, 'verifyMFA']);
    Route::post('/me/mfa/disable', [AuthController::class, 'disableMFA']);

    Route::post('/change-password', [AuthController::class, 'changePassword']);

    Route::post('/cart/checkout', [CartController::class, 'checkout']);
    
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

    // Coupons
    Route::get('/coupons', [CouponController::class, 'index']);
    Route::post('/coupons', [CouponController::class, 'store']);
    Route::get('/coupons/{id}', [CouponController::class, 'show']);
    Route::put('/coupons/{id}', [CouponController::class, 'update']);
    Route::delete('/coupons/{id}', [CouponController::class, 'destroy']);

    // Users
    Route::get('/users', [UserController::class, 'index']);
    Route::get('/users/stats', [UserController::class, 'stats']);
    Route::get('/users/{id}', [UserController::class, 'show']);
    Route::post('/users', [UserController::class, 'store']);
    Route::put('/users/{id}', [UserController::class, 'update']);
    Route::delete('/users/{id}', [UserController::class, 'destroy']);
    Route::post('/users/{id}/ban', [UserController::class, 'ban']);
    Route::post('/users/{id}/unban', [UserController::class, 'unban']);

    // Settings
    Route::get('/settings', [SettingsController::class, 'index']);
    Route::put('/settings', [SettingsController::class, 'update']);
    Route::get('/settings/{key}', [SettingsController::class, 'show']);
    Route::put('/settings/{key}', [SettingsController::class, 'updateSingle']);

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
});

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
