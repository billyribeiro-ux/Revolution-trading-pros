<?php

use Illuminate\Support\Facades\Route;
use App\Http\Controllers\Webhooks\StripeWebhookController;
use App\Http\Controllers\Admin\SubscriptionAdminController;
use App\Http\Controllers\Admin\PlanAdminController;
use App\Http\Controllers\Admin\RevenueReportController;

/*
|--------------------------------------------------------------------------
| Subscription Routes
|--------------------------------------------------------------------------
|
| Routes for subscription management, webhooks, and admin dashboard.
|
*/

// Webhooks (no CSRF, no auth)
Route::prefix('webhooks')->group(function () {
    Route::post('/stripe', [StripeWebhookController::class, 'handle'])
        ->name('webhooks.stripe');

    // PayPal webhook would go here
    // Route::post('/paypal', [PayPalWebhookController::class, 'handle'])
    //     ->name('webhooks.paypal');

    // Paddle webhook would go here
    // Route::post('/paddle', [PaddleWebhookController::class, 'handle'])
    //     ->name('webhooks.paddle');
});

// User subscription routes (authenticated)
Route::middleware(['auth:sanctum'])->prefix('subscription')->group(function () {
    // Current subscription
    Route::get('/', 'SubscriptionController@show')
        ->name('subscription.show');

    // Available plans
    Route::get('/plans', 'SubscriptionController@plans')
        ->name('subscription.plans');

    // Subscribe to a plan
    Route::post('/subscribe', 'SubscriptionController@subscribe')
        ->name('subscription.subscribe');

    // Change plan
    Route::put('/change-plan', 'SubscriptionController@changePlan')
        ->name('subscription.change-plan');

    // Cancel subscription
    Route::post('/cancel', 'SubscriptionController@cancel')
        ->name('subscription.cancel');

    // Resume subscription
    Route::post('/resume', 'SubscriptionController@resume')
        ->name('subscription.resume');

    // Usage
    Route::get('/usage', 'SubscriptionController@usage')
        ->name('subscription.usage');

    // Invoices
    Route::get('/invoices', 'SubscriptionController@invoices')
        ->name('subscription.invoices');

    Route::get('/invoices/{invoice}', 'SubscriptionController@invoice')
        ->name('subscription.invoice');

    Route::get('/invoices/{invoice}/download', 'SubscriptionController@downloadInvoice')
        ->name('invoices.download');
});

// Billing routes (authenticated)
Route::middleware(['auth:sanctum'])->prefix('billing')->group(function () {
    // Payment methods
    Route::get('/payment-methods', 'BillingController@paymentMethods')
        ->name('billing.payment-methods');

    Route::post('/payment-methods', 'BillingController@addPaymentMethod')
        ->name('billing.add-payment-method');

    Route::delete('/payment-methods/{id}', 'BillingController@removePaymentMethod')
        ->name('billing.remove-payment-method');

    Route::put('/payment-methods/{id}/default', 'BillingController@setDefaultPaymentMethod')
        ->name('billing.set-default-payment-method');

    // Billing portal
    Route::get('/portal', 'BillingController@portal')
        ->name('billing.portal');

    // Plans page
    Route::get('/plans', 'BillingController@plans')
        ->name('billing.plans');

    // Reactivate
    Route::post('/reactivate', 'BillingController@reactivate')
        ->name('billing.reactivate');
});

// Admin routes
Route::middleware(['auth:sanctum', 'admin'])->prefix('admin')->group(function () {
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
});
