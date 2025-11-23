<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    /**
     * Run the migrations.
     */
    public function up(): void
    {
        Schema::create('video_analytics', function (Blueprint $table) {
            $table->id();
            $table->foreignId('video_id')->constrained()->onDelete('cascade');
            $table->foreignId('user_id')->nullable()->constrained()->onDelete('set null');
            $table->string('session_id')->index();
            $table->string('event_type'); // view, play, pause, complete, progress, seek, error, etc.
            $table->integer('timestamp_seconds')->nullable(); // video timestamp when event occurred
            $table->integer('watch_time')->default(0); // total watch time in seconds
            $table->decimal('completion_rate', 5, 2)->default(0); // percentage
            $table->integer('interactions')->default(0);
            $table->string('quality')->nullable();
            $table->integer('buffer_events')->default(0);
            $table->json('event_data')->nullable(); // additional event-specific data
            $table->string('ip_address')->nullable();
            $table->string('user_agent')->nullable();
            $table->string('referrer')->nullable();
            $table->timestamps();
            
            $table->index(['video_id', 'event_type']);
            $table->index(['session_id', 'created_at']);
            $table->index('event_type');
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('video_analytics');
    }
};
