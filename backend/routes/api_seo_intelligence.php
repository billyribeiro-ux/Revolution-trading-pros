<?php

use App\Http\Controllers\Api\SeoIntelligenceController;
use Illuminate\Support\Facades\Route;

/**
 * SEO Intelligence API Routes
 * 
 * Google L8 Enterprise-Grade SEO Intelligence API
 * 
 * @author RevolutionSEO-L8-System
 * @version 1.0.0
 */

Route::prefix('seo')->middleware(['auth:sanctum'])->group(function () {
    
    // =====================================================================
    // NLP & ENTITY INTELLIGENCE
    // =====================================================================
    
    // Analyze content with NLP
    Route::post('/analyze', [SeoIntelligenceController::class, 'analyzeContent']);
    
    // Get entity coverage
    Route::get('/entities/coverage/{contentType}/{contentId}', [SeoIntelligenceController::class, 'getEntityCoverage']);
    
    // =====================================================================
    // AI CONTENT OPTIMIZATION
    // =====================================================================
    
    // Generate title suggestions
    Route::post('/ai/titles', [SeoIntelligenceController::class, 'generateTitles']);
    
    // Generate meta description suggestions
    Route::post('/ai/meta', [SeoIntelligenceController::class, 'generateMeta']);
    
    // Generate content outline
    Route::post('/ai/outline', [SeoIntelligenceController::class, 'generateOutline']);
    
    // Get AI suggestions for content
    Route::get('/ai/suggestions/{contentType}/{contentId}', [SeoIntelligenceController::class, 'getSuggestions']);
    
    // Accept AI suggestion
    Route::post('/ai/suggestions/{suggestionId}/accept', [SeoIntelligenceController::class, 'acceptSuggestion']);
    
    // =====================================================================
    // KEYWORD INTELLIGENCE
    // =====================================================================
    
    // Research keywords
    Route::post('/keywords/research', [SeoIntelligenceController::class, 'researchKeywords']);
    
    // Analyze SERP
    Route::post('/keywords/serp', [SeoIntelligenceController::class, 'analyzeSERP']);
    
    // Get keyword clusters
    Route::post('/keywords/clusters', [SeoIntelligenceController::class, 'getKeywordClusters']);
    
    // =====================================================================
    // INTERNAL LINK INTELLIGENCE
    // =====================================================================
    
    // Get link suggestions for content
    Route::get('/links/suggestions/{contentType}/{contentId}', [SeoIntelligenceController::class, 'getLinkSuggestions']);
    
    // Get full link graph
    Route::get('/links/graph', [SeoIntelligenceController::class, 'getLinkGraph']);
    
    // Get orphan pages
    Route::get('/links/orphans', [SeoIntelligenceController::class, 'getOrphans']);
    
    // =====================================================================
    // SCHEMA INTELLIGENCE
    // =====================================================================
    
    // Generate schema for content
    Route::post('/schema/generate', [SeoIntelligenceController::class, 'generateSchema']);
    
    // =====================================================================
    // CACHE MANAGEMENT
    // =====================================================================
    
    // Get cache statistics
    Route::get('/cache/stats', [SeoIntelligenceController::class, 'getCacheStats']);
    
    // Warm cache
    Route::post('/cache/warm', [SeoIntelligenceController::class, 'warmCache']);
    
    // Monitor cache health
    Route::get('/cache/health', [SeoIntelligenceController::class, 'monitorCache']);
});
