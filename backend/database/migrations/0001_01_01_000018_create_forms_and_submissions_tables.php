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
        // Forms table
        Schema::create('forms', function (Blueprint $table) {
            $table->id();
            $table->string('title');
            $table->string('slug')->unique();
            $table->text('description')->nullable();
            $table->json('settings')->nullable();
            $table->json('styles')->nullable();
            $table->enum('status', ['draft', 'published', 'archived'])->default('draft');
            $table->foreignId('created_by')->nullable()->constrained('users')->onDelete('set null');
            $table->integer('submission_count')->default(0);
            $table->timestamp('published_at')->nullable();
            $table->timestamps();
            $table->softDeletes();
        });

        // Form fields table
        Schema::create('form_fields', function (Blueprint $table) {
            $table->id();
            $table->foreignId('form_id')->constrained()->onDelete('cascade');
            $table->string('field_type');
            $table->string('label');
            $table->string('name');
            $table->string('placeholder')->nullable();
            $table->text('help_text')->nullable();
            $table->string('default_value')->nullable();
            $table->json('options')->nullable();
            $table->json('validation')->nullable();
            $table->json('conditional_logic')->nullable();
            $table->json('attributes')->nullable();
            $table->boolean('required')->default(false);
            $table->integer('order')->default(0);
            $table->integer('width')->default(100);
            $table->timestamps();
        });

        // Form submissions table
        Schema::create('form_submissions', function (Blueprint $table) {
            $table->id();
            $table->foreignId('form_id')->constrained()->onDelete('cascade');
            $table->foreignId('user_id')->nullable()->constrained()->onDelete('set null');
            $table->string('submission_id')->unique();
            $table->enum('status', ['unread', 'read', 'starred', 'archived', 'spam'])->default('unread');
            $table->string('ip_address')->nullable();
            $table->text('user_agent')->nullable();
            $table->string('referrer')->nullable();
            $table->json('metadata')->nullable();
            $table->timestamps();
            $table->softDeletes();
        });

        // Form submission data table
        Schema::create('form_submission_data', function (Blueprint $table) {
            $table->id();
            $table->foreignId('submission_id')->constrained('form_submissions')->onDelete('cascade');
            $table->foreignId('field_id')->constrained('form_fields')->onDelete('cascade');
            $table->string('field_name');
            $table->text('value');
            $table->timestamps();
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('form_submission_data');
        Schema::dropIfExists('form_submissions');
        Schema::dropIfExists('form_fields');
        Schema::dropIfExists('forms');
    }
};
