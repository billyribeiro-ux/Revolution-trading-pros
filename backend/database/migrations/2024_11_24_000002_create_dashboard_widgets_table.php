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
        Schema::create('dashboard_widgets', function (Blueprint $table) {
            $table->uuid('id')->primary();
            $table->foreignUuid('dashboard_id')->constrained()->onDelete('cascade');
            $table->string('widget_type'); // crm_kpi, revenue_mrr, email_performance, etc.
            $table->string('title');
            $table->json('config')->nullable(); // Widget-specific configuration
            $table->json('data_sources')->nullable(); // Data source configurations
            $table->integer('position_x')->default(0);
            $table->integer('position_y')->default(0);
            $table->integer('width')->default(4);
            $table->integer('height')->default(4);
            $table->integer('min_width')->default(2);
            $table->integer('min_height')->default(2);
            $table->boolean('is_visible')->default(true);
            $table->boolean('is_locked')->default(false);
            $table->integer('refresh_interval')->default(300); // seconds
            $table->timestamps();

            $table->index(['dashboard_id', 'widget_type']);
            $table->index('is_visible');
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('dashboard_widgets');
    }
};
