<?php

use Illuminate\Support\Facades\Route;
use App\Http\Controllers\Webhooks\StripeWebhookController;
use App\Http\Controllers\Admin\SubscriptionAdminController;
use App\Http\Controllers\Admin\PlanAdminController;
use App\Http\Controllers\Admin\RevenueReportController;
use App\Http\Controllers\Admin\InvoiceSettingsController;

/*
|--------------------------------------------------------------------------
| Subscription Routes
|--------------------------------------------------------------------------
|
| Routes for subscription management, webhooks, and admin dashboard.
|
*/

// Webhooks (no CSRF, no auth)
Route::prefix('webhooks')->withoutMiddleware(['auth:sanctum'])->group(function () {
    Route::post('/stripe', [StripeWebhookController::class, 'handle'])
        ->name('webhooks.stripe');
});

// Admin routes
Route::middleware(['auth:sanctum'])->prefix('admin')->group(function () {
    // Dashboard
    Route::get('/subscriptions/dashboard', [SubscriptionAdminController::class, 'dashboard'])
        ->name('admin.subscriptions.dashboard');

    // Subscriptions CRUD
    Route::get('/subscriptions', [SubscriptionAdminController::class, 'index'])
        ->name('admin.subscriptions.index');

    Route::get('/subscriptions/{subscription}', [SubscriptionAdminController::class, 'show'])
        ->name('admin.subscriptions.show');

    Route::put('/subscriptions/{subscription}', [SubscriptionAdminController::class, 'update'])
        ->name('admin.subscriptions.update');

    Route::post('/subscriptions/{subscription}/cancel', [SubscriptionAdminController::class, 'cancel'])
        ->name('admin.subscriptions.cancel');

    Route::post('/subscriptions/{subscription}/reactivate', [SubscriptionAdminController::class, 'reactivate'])
        ->name('admin.subscriptions.reactivate');

    Route::post('/subscriptions/{subscription}/extend-trial', [SubscriptionAdminController::class, 'extendTrial'])
        ->name('admin.subscriptions.extend-trial');

    // Plans CRUD
    Route::get('/plans', [PlanAdminController::class, 'index'])
        ->name('admin.plans.index');

    Route::post('/plans', [PlanAdminController::class, 'store'])
        ->name('admin.plans.store');

    Route::get('/plans/{plan}', [PlanAdminController::class, 'show'])
        ->name('admin.plans.show');

    Route::put('/plans/{plan}', [PlanAdminController::class, 'update'])
        ->name('admin.plans.update');

    Route::delete('/plans/{plan}', [PlanAdminController::class, 'destroy'])
        ->name('admin.plans.destroy');

    Route::post('/plans/{plan}/duplicate', [PlanAdminController::class, 'duplicate'])
        ->name('admin.plans.duplicate');

    Route::put('/plans/{plan}/features', [PlanAdminController::class, 'updateFeatures'])
        ->name('admin.plans.features');

    Route::put('/plans/{plan}/limits', [PlanAdminController::class, 'updateLimits'])
        ->name('admin.plans.limits');

    Route::post('/plans/reorder', [PlanAdminController::class, 'reorder'])
        ->name('admin.plans.reorder');

    // Revenue reports
    Route::get('/reports/revenue', [RevenueReportController::class, 'index'])
        ->name('admin.reports.revenue');

    Route::get('/reports/revenue/export', [RevenueReportController::class, 'export'])
        ->name('admin.reports.revenue.export');

    // Invoice Settings (Visual Customization)
    Route::prefix('invoice-settings')->group(function () {
        Route::get('/', [InvoiceSettingsController::class, 'index'])
            ->name('admin.invoice-settings.index');

        Route::put('/', [InvoiceSettingsController::class, 'update'])
            ->name('admin.invoice-settings.update');

        Route::post('/logo', [InvoiceSettingsController::class, 'uploadLogo'])
            ->name('admin.invoice-settings.upload-logo');

        Route::delete('/logo', [InvoiceSettingsController::class, 'removeLogo'])
            ->name('admin.invoice-settings.remove-logo');

        Route::get('/preview', [InvoiceSettingsController::class, 'preview'])
            ->name('admin.invoice-settings.preview');

        Route::get('/preview-html', [InvoiceSettingsController::class, 'previewHtml'])
            ->name('admin.invoice-settings.preview-html');

        Route::post('/reset', [InvoiceSettingsController::class, 'resetToDefaults'])
            ->name('admin.invoice-settings.reset');

        Route::get('/export', [InvoiceSettingsController::class, 'export'])
            ->name('admin.invoice-settings.export');

        Route::post('/import', [InvoiceSettingsController::class, 'import'])
            ->name('admin.invoice-settings.import');
    });
});
