<?php

use App\Http\Controllers\WorkflowController;
use Illuminate\Support\Facades\Route;

Route::middleware(['auth:sanctum'])->group(function () {
    // Workflows
    Route::get('/workflows', [WorkflowController::class, 'index']);
    Route::post('/workflows', [WorkflowController::class, 'store']);
    Route::get('/workflows/{workflow}', [WorkflowController::class, 'show']);
    Route::put('/workflows/{workflow}', [WorkflowController::class, 'update']);
    Route::delete('/workflows/{workflow}', [WorkflowController::class, 'destroy']);
    
    // Workflow status
    Route::put('/workflows/{workflow}/status', [WorkflowController::class, 'toggleStatus']);
    
    // Workflow nodes
    Route::get('/workflows/{workflow}/nodes', [WorkflowController::class, 'nodes']);
    Route::post('/workflows/{workflow}/nodes', [WorkflowController::class, 'createNode']);
    Route::put('/workflows/{workflow}/nodes/{node}', [WorkflowController::class, 'updateNode']);
    Route::delete('/workflows/{workflow}/nodes/{node}', [WorkflowController::class, 'deleteNode']);
    
    // Workflow edges
    Route::get('/workflows/{workflow}/edges', [WorkflowController::class, 'edges']);
    Route::post('/workflows/{workflow}/edges', [WorkflowController::class, 'createEdge']);
    Route::delete('/workflows/{workflow}/edges/{edge}', [WorkflowController::class, 'deleteEdge']);
    
    // Workflow execution
    Route::post('/workflows/{workflow}/execute', [WorkflowController::class, 'execute']);
    Route::get('/workflows/{workflow}/runs', [WorkflowController::class, 'runs']);
    
    // Workflow analytics
    Route::get('/workflows/{workflow}/analytics', [WorkflowController::class, 'analytics']);
});
