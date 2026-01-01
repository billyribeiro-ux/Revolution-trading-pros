<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;
use Illuminate\Support\Facades\DB;

/**
 * Trading Rooms & Alert Services System
 *
 * Creates tables for:
 * - Trading rooms (Day Trading, Swing Trading, Small Account Mentorship)
 * - Alert services (SPX Profit Pulse, Explosive Swing, + future services)
 * - Room-specific traders
 * - Room-specific daily videos
 * - Learning center content per room
 *
 * @version 1.0.0 - December 2025
 */
return new class extends Migration
{
    public function up(): void
    {
        // ═══════════════════════════════════════════════════════════════════════════
        // TRADING ROOMS & ALERT SERVICES
        // ═══════════════════════════════════════════════════════════════════════════

        Schema::create('trading_rooms', function (Blueprint $table) {
            $table->id();
            $table->string('name');                              // "Day Trading Room"
            $table->string('slug')->unique();                    // "day-trading-room"
            $table->enum('type', ['trading_room', 'alert_service'])->default('trading_room');
            $table->text('description')->nullable();
            $table->string('short_description')->nullable();     // For cards
            $table->string('icon')->nullable();                  // st-icon class
            $table->string('color')->nullable();                 // Brand color
            $table->string('image_url')->nullable();             // Hero image
            $table->string('logo_url')->nullable();              // Room logo
            $table->boolean('is_active')->default(true);
            $table->boolean('is_featured')->default(false);
            $table->integer('sort_order')->default(0);
            $table->json('features')->nullable();                // Array of features
            $table->json('schedule')->nullable();                // Trading hours/schedule
            $table->json('metadata')->nullable();                // Extra settings
            $table->timestamps();
            $table->softDeletes();

            $table->index(['type', 'is_active']);
            $table->index('sort_order');
        });

        // ═══════════════════════════════════════════════════════════════════════════
        // ROOM TRADERS
        // ═══════════════════════════════════════════════════════════════════════════

        Schema::create('room_traders', function (Blueprint $table) {
            $table->id();
            $table->string('name');
            $table->string('slug')->unique();
            $table->string('title')->nullable();                 // "Head Trader", "Senior Analyst"
            $table->text('bio')->nullable();
            $table->string('photo_url')->nullable();
            $table->string('email')->nullable();
            $table->json('social_links')->nullable();            // Twitter, YouTube, etc.
            $table->json('specialties')->nullable();             // Array of trading specialties
            $table->boolean('is_active')->default(true);
            $table->integer('sort_order')->default(0);
            $table->timestamps();
            $table->softDeletes();

            $table->index('is_active');
        });

        // Pivot table for many-to-many (trader can be in multiple rooms)
        Schema::create('trading_room_trader', function (Blueprint $table) {
            $table->id();
            $table->foreignId('trading_room_id')->constrained()->onDelete('cascade');
            $table->foreignId('room_trader_id')->constrained()->onDelete('cascade');
            $table->boolean('is_primary')->default(false);       // Primary trader for room
            $table->timestamps();

            $table->unique(['trading_room_id', 'room_trader_id']);
        });

        // ═══════════════════════════════════════════════════════════════════════════
        // ROOM DAILY VIDEOS
        // ═══════════════════════════════════════════════════════════════════════════

        Schema::create('room_daily_videos', function (Blueprint $table) {
            $table->id();
            $table->foreignId('trading_room_id')->constrained()->onDelete('cascade');
            $table->foreignId('trader_id')->nullable()->constrained('room_traders')->onDelete('set null');
            $table->string('title');
            $table->text('description')->nullable();
            $table->string('video_url');                         // Vimeo, YouTube, Bunny.net, or direct
            $table->enum('video_platform', ['vimeo', 'youtube', 'bunny', 'wistia', 'direct'])->default('vimeo');
            $table->string('video_id')->nullable();              // Platform-specific video ID
            $table->string('thumbnail_url')->nullable();
            $table->integer('duration')->nullable();             // In seconds
            $table->date('video_date');                          // The date of the video content
            $table->boolean('is_featured')->default(false);
            $table->boolean('is_published')->default(true);
            $table->integer('views_count')->default(0);
            $table->json('tags')->nullable();                    // Searchable tags
            $table->json('metadata')->nullable();                // Extra data
            $table->timestamps();
            $table->softDeletes();

            $table->index(['trading_room_id', 'is_published', 'video_date']);
            $table->index('trader_id');
            $table->index('video_date');
            // Fulltext only supported on MySQL/MariaDB
            if (DB::connection()->getDriverName() !== 'sqlite') {
                $table->fullText(['title', 'description']);
            }
        });

        // ═══════════════════════════════════════════════════════════════════════════
        // ROOM LEARNING CENTER CONTENT
        // ═══════════════════════════════════════════════════════════════════════════

        Schema::create('room_learning_content', function (Blueprint $table) {
            $table->id();
            $table->foreignId('trading_room_id')->constrained()->onDelete('cascade');
            $table->foreignId('trader_id')->nullable()->constrained('room_traders')->onDelete('set null');
            $table->string('title');
            $table->text('description')->nullable();
            $table->enum('content_type', ['video', 'course', 'pdf', 'article', 'webinar'])->default('video');
            $table->string('content_url')->nullable();           // Video URL or file URL
            $table->string('thumbnail_url')->nullable();
            $table->integer('duration')->nullable();             // In minutes
            $table->string('difficulty_level')->nullable();      // Beginner, Intermediate, Advanced
            $table->string('category')->nullable();              // Strategy, Tools, Mindset, etc.
            $table->integer('sort_order')->default(0);
            $table->boolean('is_published')->default(true);
            $table->integer('views_count')->default(0);
            $table->json('metadata')->nullable();
            $table->timestamps();
            $table->softDeletes();

            $table->index(['trading_room_id', 'content_type', 'is_published']);
            $table->index('category');
        });

        // ═══════════════════════════════════════════════════════════════════════════
        // ROOM VIDEO ARCHIVES (Trading Room Recordings)
        // ═══════════════════════════════════════════════════════════════════════════

        Schema::create('room_archives', function (Blueprint $table) {
            $table->id();
            $table->foreignId('trading_room_id')->constrained()->onDelete('cascade');
            $table->string('title');
            $table->text('description')->nullable();
            $table->string('video_url');
            $table->string('thumbnail_url')->nullable();
            $table->integer('duration')->nullable();             // In seconds
            $table->date('recording_date');
            $table->enum('session_type', ['morning', 'afternoon', 'special', 'review'])->default('morning');
            $table->boolean('is_published')->default(true);
            $table->integer('views_count')->default(0);
            $table->json('timestamps')->nullable();              // Key moments in video
            $table->json('metadata')->nullable();
            $table->timestamps();
            $table->softDeletes();

            $table->index(['trading_room_id', 'is_published', 'recording_date']);
        });

        // ═══════════════════════════════════════════════════════════════════════════
        // SEED INITIAL DATA
        // ═══════════════════════════════════════════════════════════════════════════

        // Insert trading rooms
        DB::table('trading_rooms')->insert([
            [
                'name' => 'Day Trading Room',
                'slug' => 'day-trading-room',
                'type' => 'trading_room',
                'description' => 'Live day trading room with real-time market analysis and trade alerts.',
                'short_description' => 'Live day trading with expert traders',
                'icon' => 'st-icon-day-trading',
                'color' => '#0984ae',
                'is_active' => true,
                'is_featured' => true,
                'sort_order' => 1,
                'features' => json_encode(['Live Trading', 'Real-time Alerts', 'Daily Analysis', 'Q&A Sessions']),
                'schedule' => json_encode(['market_open' => '9:00 AM ET', 'market_close' => '4:00 PM ET']),
                'created_at' => now(),
                'updated_at' => now(),
            ],
            [
                'name' => 'Swing Trading Room',
                'slug' => 'swing-trading-room',
                'type' => 'trading_room',
                'description' => 'Swing trading strategies for multi-day to multi-week positions.',
                'short_description' => 'Multi-day swing trading strategies',
                'icon' => 'st-icon-swing-trading',
                'color' => '#10b981',
                'is_active' => true,
                'is_featured' => true,
                'sort_order' => 2,
                'features' => json_encode(['Swing Setups', 'Weekly Analysis', 'Position Management', 'Trade Reviews']),
                'schedule' => json_encode(['sessions' => 'Daily market analysis + weekly planning']),
                'created_at' => now(),
                'updated_at' => now(),
            ],
            [
                'name' => 'Small Account Mentorship',
                'slug' => 'small-account-mentorship',
                'type' => 'trading_room',
                'description' => 'Specialized mentorship for growing small trading accounts with disciplined strategies.',
                'short_description' => 'Grow your small account the right way',
                'icon' => 'st-icon-mentorship',
                'color' => '#f59e0b',
                'is_active' => true,
                'is_featured' => true,
                'sort_order' => 3,
                'features' => json_encode(['Account Building', 'Risk Management', 'Position Sizing', '1-on-1 Guidance']),
                'schedule' => json_encode(['sessions' => 'Live sessions + on-demand content']),
                'created_at' => now(),
                'updated_at' => now(),
            ],
        ]);

        // Insert alert services
        DB::table('trading_rooms')->insert([
            [
                'name' => 'SPX Profit Pulse',
                'slug' => 'spx-profit-pulse',
                'type' => 'alert_service',
                'description' => 'Real-time SPX trading alerts with entry, target, and stop levels.',
                'short_description' => 'SPX options trading alerts',
                'icon' => 'st-icon-alerts',
                'color' => '#ef4444',
                'is_active' => true,
                'is_featured' => true,
                'sort_order' => 10,
                'features' => json_encode(['Real-time Alerts', 'Entry/Exit Levels', 'Risk Management', 'SMS & Email']),
                'created_at' => now(),
                'updated_at' => now(),
            ],
            [
                'name' => 'Explosive Swing',
                'slug' => 'explosive-swing',
                'type' => 'alert_service',
                'description' => 'High-probability swing trade alerts for explosive moves.',
                'short_description' => 'Explosive swing trade alerts',
                'icon' => 'st-icon-explosive',
                'color' => '#8b5cf6',
                'is_active' => true,
                'is_featured' => true,
                'sort_order' => 11,
                'features' => json_encode(['Swing Alerts', 'Technical Analysis', 'Weekly Watchlist', 'Trade Tracking']),
                'created_at' => now(),
                'updated_at' => now(),
            ],
        ]);
    }

    public function down(): void
    {
        Schema::dropIfExists('room_archives');
        Schema::dropIfExists('room_learning_content');
        Schema::dropIfExists('room_daily_videos');
        Schema::dropIfExists('trading_room_trader');
        Schema::dropIfExists('room_traders');
        Schema::dropIfExists('trading_rooms');
    }
};
