<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('scheduled_reports', function (Blueprint $table) {
            $table->id();
            $table->foreignId('dashboard_id')->constrained()->onDelete('cascade');
            $table->foreignId('user_id')->constrained()->onDelete('cascade');
            $table->string('name');
            $table->enum('frequency', ['daily', 'weekly', 'monthly']);
            $table->json('recipients'); // Array of email addresses
            $table->string('format')->default('json'); // json, pdf, csv
            $table->boolean('include_data')->default(true);
            $table->json('filters')->nullable();
            $table->timestamp('next_run_at');
            $table->timestamp('last_run_at')->nullable();
            $table->boolean('is_active')->default(true);
            $table->timestamps();

            $table->index(['user_id', 'is_active']);
            $table->index('next_run_at');
        });

        Schema::create('report_executions', function (Blueprint $table) {
            $table->id();
            $table->foreignId('scheduled_report_id')->constrained('scheduled_reports')->onDelete('cascade');
            $table->enum('status', ['success', 'failed']);
            $table->string('filename')->nullable();
            $table->text('error_message')->nullable();
            $table->timestamp('executed_at');

            $table->index(['scheduled_report_id', 'executed_at']);
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('report_executions');
        Schema::dropIfExists('scheduled_reports');
    }
};
