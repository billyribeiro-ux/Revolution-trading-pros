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
        Schema::create('system_health_metrics', function (Blueprint $table) {
            $table->id();
            $table->string('metric_type'); // api_response_time, database_query_time, cache_hit_rate, etc.
            $table->string('service_name'); // email, crm, analytics, subscriptions, etc.
            $table->decimal('value', 10, 2);
            $table->string('unit'); // ms, percentage, count, etc.
            $table->enum('status', ['healthy', 'warning', 'critical'])->default('healthy');
            $table->json('metadata')->nullable();
            $table->timestamp('recorded_at');

            $table->index(['service_name', 'recorded_at']);
            $table->index(['metric_type', 'recorded_at']);
            $table->index('status');
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('system_health_metrics');
    }
};
