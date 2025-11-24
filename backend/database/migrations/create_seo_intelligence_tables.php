<?php

declare(strict_types=1);

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

/**
 * SEO Intelligence Engine - Core Tables Migration
 * 
 * Google L8 Enterprise-Grade SEO Intelligence System
 * Implements: NLP, Entity Extraction, Topic Modeling, AI Optimization
 * 
 * @author RevolutionSEO-L8-System
 * @version 1.0.0
 */
return new class extends Migration
{
    /**
     * Run the migrations.
     */
    public function up(): void
    {
        // =====================================================================
        // ENTITY INTELLIGENCE TABLES
        // =====================================================================
        
        // Core entity definitions (people, places, organizations, products, etc.)
        Schema::create('seo_entities', function (Blueprint $table) {
            $table->id();
            $table->string('name', 255)->unique();
            $table->string('type', 50)->index(); // PERSON, ORGANIZATION, LOCATION, PRODUCT, EVENT, WORK_OF_ART, etc.
            $table->decimal('salience_avg', 5, 4)->default(0); // Average importance across all mentions (0-1)
            $table->integer('mention_count')->default(0);
            $table->string('wikipedia_url', 500)->nullable();
            $table->string('knowledge_graph_id', 255)->nullable();
            $table->json('metadata')->nullable(); // description, aliases, related_entities, image_url
            $table->timestamps();
            
            $table->index(['type', 'salience_avg']);
            $table->index('mention_count');
        });

        // Entity mentions in specific content
        Schema::create('seo_entity_mentions', function (Blueprint $table) {
            $table->id();
            $table->foreignId('entity_id')->constrained('seo_entities')->onDelete('cascade');
            $table->string('content_type', 50); // post, page, product
            $table->unsignedBigInteger('content_id');
            $table->decimal('salience', 5, 4); // Importance in this specific content (0-1)
            $table->integer('mention_count')->default(1);
            $table->integer('first_mention_position')->nullable(); // Character position of first mention
            $table->text('context')->nullable(); // Surrounding text for first mention
            $table->string('sentiment', 20)->nullable(); // positive, neutral, negative
            $table->timestamps();
            
            $table->index(['content_type', 'content_id']);
            $table->index(['entity_id', 'salience']);
            $table->unique(['entity_id', 'content_type', 'content_id'], 'uk_entity_content');
        });

        // Entity coverage analysis (comparing content to SERP competitors)
        Schema::create('seo_entity_coverage', function (Blueprint $table) {
            $table->id();
            $table->string('content_type', 50);
            $table->unsignedBigInteger('content_id');
            $table->string('target_keyword', 255)->index();
            $table->json('expected_entities'); // Entities found in top 10 SERP results
            $table->json('found_entities'); // Entities found in our content
            $table->json('missing_entities'); // Entities we should add
            $table->integer('coverage_score')->default(0); // 0-100
            $table->timestamp('analyzed_at');
            $table->timestamps();
            
            $table->index(['content_type', 'content_id']);
            $table->index('coverage_score');
        });

        // =====================================================================
        // TOPIC INTELLIGENCE TABLES
        // =====================================================================
        
        // Topics extracted via NLP (LDA, TF-IDF, BERT)
        Schema::create('seo_topics', function (Blueprint $table) {
            $table->id();
            $table->string('topic_name', 255);
            $table->text('description')->nullable();
            $table->json('keywords'); // Top keywords associated with this topic
            $table->integer('content_count')->default(0); // How many pieces of content cover this topic
            $table->decimal('avg_relevance', 5, 4)->default(0); // Average relevance score
            $table->timestamps();
            
            $table->index('content_count');
        });

        // Topic coverage in specific content
        Schema::create('seo_topic_coverage', function (Blueprint $table) {
            $table->id();
            $table->foreignId('topic_id')->constrained('seo_topics')->onDelete('cascade');
            $table->string('content_type', 50);
            $table->unsignedBigInteger('content_id');
            $table->decimal('relevance_score', 5, 4); // How relevant this topic is to the content (0-1)
            $table->integer('keyword_count')->default(0); // How many topic keywords appear
            $table->json('keywords_found')->nullable(); // Which keywords were found
            $table->timestamps();
            
            $table->index(['content_type', 'content_id']);
            $table->index(['topic_id', 'relevance_score']);
            $table->unique(['topic_id', 'content_type', 'content_id'], 'uk_topic_content');
        });

        // =====================================================================
        // KEYWORD INTELLIGENCE TABLES
        // =====================================================================
        
        // Master keyword database
        Schema::create('seo_keywords', function (Blueprint $table) {
            $table->id();
            $table->string('keyword', 255)->unique();
            $table->integer('search_volume')->default(0);
            $table->integer('difficulty_score')->default(0); // 0-100
            $table->integer('opportunity_score')->default(0); // 0-100 (volume vs difficulty)
            $table->enum('intent', ['informational', 'commercial', 'transactional', 'navigational'])->index();
            $table->decimal('cpc', 8, 2)->nullable(); // Cost per click
            $table->decimal('competition', 5, 4)->nullable(); // 0-1
            $table->enum('trend_direction', ['up', 'down', 'stable'])->default('stable');
            $table->foreignId('parent_topic_id')->nullable()->constrained('seo_topics')->onDelete('set null');
            $table->json('metadata')->nullable(); // related_keywords, questions, serp_features
            $table->timestamp('last_updated')->nullable();
            $table->timestamps();
            
            $table->index('search_volume');
            $table->index('difficulty_score');
            $table->index('opportunity_score');
            $table->index(['intent', 'opportunity_score']);
        });

        // Keyword clusters (semantic groupings)
        Schema::create('seo_keyword_clusters', function (Blueprint $table) {
            $table->id();
            $table->string('cluster_name', 255);
            $table->foreignId('pillar_keyword_id')->constrained('seo_keywords')->onDelete('cascade');
            $table->json('cluster_keywords'); // Array of keyword IDs in this cluster
            $table->integer('total_search_volume')->default(0);
            $table->decimal('avg_difficulty', 5, 2)->default(0);
            $table->text('content_recommendations')->nullable();
            $table->timestamps();
            
            $table->index('total_search_volume');
        });

        // SERP analysis results
        Schema::create('seo_serp_results', function (Blueprint $table) {
            $table->id();
            $table->foreignId('keyword_id')->constrained('seo_keywords')->onDelete('cascade');
            $table->integer('position'); // 1-100
            $table->string('url', 500);
            $table->string('title', 255)->nullable();
            $table->text('description')->nullable();
            $table->string('domain', 255);
            $table->integer('domain_authority')->default(0);
            $table->integer('page_authority')->default(0);
            $table->integer('word_count')->nullable();
            $table->json('entities_found')->nullable(); // Entities extracted from this result
            $table->json('topics_covered')->nullable(); // Topics covered
            $table->json('schema_types')->nullable(); // Schema.org types used
            $table->timestamp('analyzed_at');
            $table->json('serp_features')->nullable(); // featured_snippet, paa, images, videos, etc.
            $table->timestamps();
            
            $table->index(['keyword_id', 'position']);
            $table->index('analyzed_at');
        });

        // SERP competitors analysis
        Schema::create('seo_serp_competitors', function (Blueprint $table) {
            $table->id();
            $table->foreignId('keyword_id')->constrained('seo_keywords')->onDelete('cascade');
            $table->string('domain', 255);
            $table->integer('appearances')->default(1); // How many times in top 10
            $table->decimal('avg_position', 5, 2);
            $table->integer('domain_authority')->default(0);
            $table->json('ranking_urls')->nullable(); // URLs that rank for this keyword
            $table->timestamps();
            
            $table->index(['keyword_id', 'appearances']);
            $table->unique(['keyword_id', 'domain'], 'uk_keyword_competitor');
        });

        // Content gap analysis
        Schema::create('seo_content_gaps', function (Blueprint $table) {
            $table->id();
            $table->string('our_content_type', 50);
            $table->unsignedBigInteger('our_content_id');
            $table->string('target_keyword', 255);
            $table->enum('gap_type', ['entity', 'topic', 'word_count', 'schema', 'heading', 'media', 'link'])->index();
            $table->text('gap_description');
            $table->json('competitor_examples')->nullable(); // Examples from competitors
            $table->enum('priority', ['critical', 'high', 'medium', 'low'])->default('medium')->index();
            $table->integer('estimated_impact')->default(0); // 0-100
            $table->boolean('is_resolved')->default(false);
            $table->timestamp('resolved_at')->nullable();
            $table->timestamps();
            
            $table->index(['our_content_type', 'our_content_id']);
            $table->index(['is_resolved', 'priority']);
        });

        // =====================================================================
        // AI OPTIMIZATION TABLES
        // =====================================================================
        
        // AI-generated suggestions
        Schema::create('seo_ai_suggestions', function (Blueprint $table) {
            $table->id();
            $table->string('content_type', 50);
            $table->unsignedBigInteger('content_id');
            $table->enum('suggestion_type', ['title', 'meta', 'heading', 'paragraph', 'outline', 'schema', 'image_alt', 'internal_link'])->index();
            $table->text('original_text')->nullable();
            $table->text('suggested_text');
            $table->text('reasoning'); // Why this suggestion was made
            $table->integer('impact_score')->default(0); // Estimated SEO impact 0-100
            $table->integer('confidence_score')->default(0); // AI confidence 0-100
            $table->enum('status', ['pending', 'accepted', 'rejected', 'applied'])->default('pending')->index();
            $table->timestamp('applied_at')->nullable();
            $table->json('metadata')->nullable(); // model_used, tokens, cost, alternatives
            $table->timestamps();
            
            $table->index(['content_type', 'content_id', 'status']);
            $table->index(['suggestion_type', 'impact_score']);
        });

        // =====================================================================
        // INTERNAL LINK INTELLIGENCE TABLES
        // =====================================================================
        
        // Internal link suggestions
        Schema::create('seo_internal_link_suggestions', function (Blueprint $table) {
            $table->id();
            $table->string('source_content_type', 50);
            $table->unsignedBigInteger('source_content_id');
            $table->string('target_content_type', 50);
            $table->unsignedBigInteger('target_content_id');
            $table->string('suggested_anchor_text', 255);
            $table->text('context_snippet'); // Where to place the link
            $table->integer('relevance_score')->default(0); // 0-100 (semantic similarity)
            $table->enum('priority', ['high', 'medium', 'low'])->default('medium')->index();
            $table->text('reasoning');
            $table->enum('status', ['pending', 'accepted', 'rejected', 'applied'])->default('pending')->index();
            $table->timestamp('applied_at')->nullable();
            $table->timestamps();
            
            $table->index(['source_content_type', 'source_content_id'], 'seo_link_suggestions_source_idx');
            $table->index(['target_content_type', 'target_content_id'], 'seo_link_suggestions_target_idx');
            $table->index(['status', 'priority']);
        });

        // Link graph cache (for PageRank calculations)
        Schema::create('seo_link_graph', function (Blueprint $table) {
            $table->id();
            $table->string('content_type', 50);
            $table->unsignedBigInteger('content_id');
            $table->json('outbound_links'); // Array of {type, id, anchor_text}
            $table->json('inbound_links'); // Array of {type, id, anchor_text}
            $table->integer('outbound_count')->default(0);
            $table->integer('inbound_count')->default(0);
            $table->decimal('pagerank_score', 10, 8)->default(0); // PageRank score
            $table->boolean('is_orphan')->default(false)->index(); // No inbound links
            $table->boolean('is_hub')->default(false)->index(); // Many outbound links
            $table->timestamp('calculated_at');
            $table->timestamps();
            
            $table->unique(['content_type', 'content_id'], 'uk_content_graph');
            $table->index('pagerank_score');
        });

        // =====================================================================
        // SCHEMA INTELLIGENCE TABLES
        // =====================================================================
        
        // Schema templates
        Schema::create('seo_schema_templates', function (Blueprint $table) {
            $table->id();
            $table->string('schema_type', 100); // BlogPosting, Article, FAQPage, HowTo, Product, etc.
            $table->json('template_json'); // JSON-LD template with placeholders
            $table->json('required_fields'); // Array of required field names
            $table->json('optional_fields'); // Array of optional field names
            $table->json('entity_mappings')->nullable(); // Which entities to include and how
            $table->integer('usage_count')->default(0);
            $table->boolean('is_active')->default(true);
            $table->timestamps();
            
            $table->index('schema_type');
            $table->index('usage_count');
        });

        // Generated schema instances
        Schema::create('seo_schema_instances', function (Blueprint $table) {
            $table->id();
            $table->string('content_type', 50);
            $table->unsignedBigInteger('content_id');
            $table->string('schema_type', 100);
            $table->json('schema_json'); // Generated JSON-LD
            $table->enum('validation_status', ['valid', 'invalid', 'warning'])->default('valid')->index();
            $table->json('validation_errors')->nullable();
            $table->boolean('rich_results_eligible')->default(false)->index();
            $table->timestamp('generated_at');
            $table->timestamps();
            
            $table->index(['content_type', 'content_id']);
            $table->index('schema_type');
        });

        // =====================================================================
        // NLP CACHE TABLE
        // =====================================================================
        
        // Cache for expensive NLP operations
        Schema::create('seo_nlp_cache', function (Blueprint $table) {
            $table->id();
            $table->string('cache_key', 255)->unique(); // Hash of content + operation
            $table->string('operation_type', 50)->index(); // entity_extraction, topic_modeling, sentiment, etc.
            $table->string('content_type', 50);
            $table->unsignedBigInteger('content_id');
            $table->json('result'); // Cached NLP result
            $table->string('model_version', 50)->nullable(); // Which model/version was used
            $table->integer('processing_time_ms')->nullable();
            $table->timestamp('expires_at')->index();
            $table->timestamps();
            
            $table->index(['content_type', 'content_id']);
            $table->index(['operation_type', 'expires_at']);
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('seo_nlp_cache');
        Schema::dropIfExists('seo_schema_instances');
        Schema::dropIfExists('seo_schema_templates');
        Schema::dropIfExists('seo_link_graph');
        Schema::dropIfExists('seo_internal_link_suggestions');
        Schema::dropIfExists('seo_ai_suggestions');
        Schema::dropIfExists('seo_content_gaps');
        Schema::dropIfExists('seo_serp_competitors');
        Schema::dropIfExists('seo_serp_results');
        Schema::dropIfExists('seo_keyword_clusters');
        Schema::dropIfExists('seo_keywords');
        Schema::dropIfExists('seo_topic_coverage');
        Schema::dropIfExists('seo_topics');
        Schema::dropIfExists('seo_entity_coverage');
        Schema::dropIfExists('seo_entity_mentions');
        Schema::dropIfExists('seo_entities');
    }
};
