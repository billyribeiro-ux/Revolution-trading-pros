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
        Schema::create('competitor_domains', function (Blueprint $table) {
            $table->id();
            $table->string('domain')->unique();
            $table->string('name')->nullable();
            $table->integer('estimated_authority')->nullable();
            $table->integer('estimated_traffic')->nullable();
            $table->json('tracked_keywords')->nullable();
            $table->json('metadata')->nullable();
            $table->boolean('is_active')->default(true);
            $table->timestamps();

            $table->index('is_active');
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('competitor_domains');
    }
};
