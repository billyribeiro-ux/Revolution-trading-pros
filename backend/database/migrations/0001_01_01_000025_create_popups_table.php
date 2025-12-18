<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('popups', function (Blueprint $table) {
            $table->id();
            // Human-readable name for admin UI
            $table->string('name');
            // Full popup configuration as sent from the frontend (Popup / EnhancedPopup)
            $table->json('config');
            // Basic aggregated metrics (also mirrored inside config.impressions/conversions)
            $table->unsignedBigInteger('impressions')->default(0);
            $table->unsignedBigInteger('conversions')->default(0);
            $table->timestamp('last_impression_at')->nullable();
            $table->timestamp('last_conversion_at')->nullable();
            $table->timestamps();
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('popups');
    }
};
