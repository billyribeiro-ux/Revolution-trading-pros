<?php

use App\Http\Controllers\DashboardController;
use Illuminate\Support\Facades\Route;

Route::middleware(['auth:sanctum'])->group(function () {
    // Dashboard routes
    Route::get('/dashboards', [DashboardController::class, 'index']);
    Route::get('/dashboards/{id}', [DashboardController::class, 'show']);
    
    // Widget routes
    Route::post('/dashboards/{dashboardId}/widgets', [DashboardController::class, 'addWidget']);
    Route::put('/widgets/{widgetId}/layout', [DashboardController::class, 'updateWidgetLayout']);
    Route::delete('/widgets/{widgetId}', [DashboardController::class, 'removeWidget']);
});
