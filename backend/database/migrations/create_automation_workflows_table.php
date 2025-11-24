<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::create('workflows', function (Blueprint $table) {
            $table->id();
            $table->foreignId('user_id')->constrained()->onDelete('cascade');
            $table->string('name');
            $table->text('description')->nullable();
            $table->enum('status', ['active', 'paused', 'archived'])->default('active');
            $table->json('trigger_config'); // Trigger configuration
            $table->integer('version')->default(1);
            $table->integer('execution_count')->default(0);
            $table->integer('success_count')->default(0);
            $table->integer('failure_count')->default(0);
            $table->timestamp('last_executed_at')->nullable();
            $table->integer('avg_execution_time_ms')->nullable();
            $table->timestamps();
            $table->softDeletes();

            $table->index(['user_id', 'status']);
            $table->index('status');
        });

        Schema::create('workflow_nodes', function (Blueprint $table) {
            $table->id();
            $table->foreignId('workflow_id')->constrained()->onDelete('cascade');
            $table->enum('node_type', [
                'trigger', 'condition', 'action', 'delay', 
                'branch', 'parallel', 'merge', 'end'
            ]);
            $table->json('config'); // Node-specific configuration
            $table->integer('position_x')->default(0);
            $table->integer('position_y')->default(0);
            $table->foreignId('parent_node_id')->nullable()->constrained('workflow_nodes')->onDelete('cascade');
            $table->integer('order')->default(0);
            $table->timestamps();

            $table->index(['workflow_id', 'node_type']);
            $table->index('parent_node_id');
        });

        Schema::create('workflow_edges', function (Blueprint $table) {
            $table->id();
            $table->foreignId('workflow_id')->constrained()->onDelete('cascade');
            $table->foreignId('from_node_id')->constrained('workflow_nodes')->onDelete('cascade');
            $table->foreignId('to_node_id')->constrained('workflow_nodes')->onDelete('cascade');
            $table->enum('condition_type', ['always', 'if_true', 'if_false', 'parallel'])->default('always');
            $table->string('label')->nullable();
            $table->timestamps();

            $table->index(['workflow_id', 'from_node_id']);
            $table->index('to_node_id');
        });

        Schema::create('workflow_runs', function (Blueprint $table) {
            $table->id();
            $table->foreignId('workflow_id')->constrained()->onDelete('cascade');
            $table->string('trigger_event_id')->nullable();
            $table->enum('status', ['running', 'completed', 'failed', 'paused', 'cancelled'])->default('running');
            $table->timestamp('started_at');
            $table->timestamp('completed_at')->nullable();
            $table->integer('duration_ms')->nullable();
            $table->json('context'); // Execution context with all variables
            $table->text('error_message')->nullable();
            $table->foreignId('triggered_by_user_id')->nullable()->constrained('users')->onDelete('set null');

            $table->index(['workflow_id', 'status']);
            $table->index(['status', 'started_at']);
            $table->index('trigger_event_id');
        });

        Schema::create('workflow_run_logs', function (Blueprint $table) {
            $table->id();
            $table->foreignId('workflow_run_id')->constrained()->onDelete('cascade');
            $table->foreignId('node_id')->constrained('workflow_nodes')->onDelete('cascade');
            $table->enum('status', ['pending', 'running', 'completed', 'failed', 'skipped'])->default('pending');
            $table->timestamp('executed_at');
            $table->json('input_data')->nullable();
            $table->json('output_data')->nullable();
            $table->text('error')->nullable();
            $table->integer('duration_ms')->nullable();

            $table->index(['workflow_run_id', 'node_id']);
            $table->index(['status', 'executed_at']);
        });

        Schema::create('workflow_triggers', function (Blueprint $table) {
            $table->id();
            $table->foreignId('workflow_id')->constrained()->onDelete('cascade');
            $table->string('trigger_type'); // contact.created, form.submitted, etc.
            $table->json('config'); // Trigger-specific configuration
            $table->boolean('is_active')->default(true);
            $table->timestamp('last_triggered_at')->nullable();
            $table->integer('trigger_count')->default(0);
            $table->timestamps();

            $table->index(['workflow_id', 'trigger_type']);
            $table->index(['trigger_type', 'is_active']);
        });

        Schema::create('workflow_schedules', function (Blueprint $table) {
            $table->id();
            $table->foreignId('workflow_id')->constrained()->onDelete('cascade');
            $table->enum('schedule_type', ['cron', 'interval', 'once']);
            $table->json('schedule_config'); // Cron expression, interval seconds, or specific datetime
            $table->timestamp('next_run_at');
            $table->timestamp('last_run_at')->nullable();
            $table->boolean('is_active')->default(true);
            $table->timestamps();

            $table->index(['workflow_id', 'is_active']);
            $table->index(['next_run_at', 'is_active']);
        });

        Schema::create('workflow_versions', function (Blueprint $table) {
            $table->id();
            $table->foreignId('workflow_id')->constrained()->onDelete('cascade');
            $table->integer('version_number');
            $table->json('definition'); // Complete workflow definition snapshot
            $table->foreignId('created_by')->constrained('users')->onDelete('cascade');
            $table->timestamp('created_at');
            $table->string('change_description')->nullable();

            $table->unique(['workflow_id', 'version_number']);
            $table->index(['workflow_id', 'created_at']);
        });
    }

    public function down(): void
    {
        Schema::dropIfExists('workflow_versions');
        Schema::dropIfExists('workflow_schedules');
        Schema::dropIfExists('workflow_triggers');
        Schema::dropIfExists('workflow_run_logs');
        Schema::dropIfExists('workflow_runs');
        Schema::dropIfExists('workflow_edges');
        Schema::dropIfExists('workflow_nodes');
        Schema::dropIfExists('workflows');
    }
};
