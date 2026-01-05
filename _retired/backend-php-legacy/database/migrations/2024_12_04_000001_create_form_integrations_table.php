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
        Schema::create('form_integrations', function (Blueprint $table) {
            $table->id();
            $table->foreignId('form_id')->constrained('forms')->onDelete('cascade');
            $table->string('integration_type', 50)->index(); // webhook, email_marketing, crm, etc.
            $table->string('provider', 50)->index(); // activecampaign, hubspot, stripe, etc.
            $table->string('name'); // User-defined name
            $table->text('settings'); // Encrypted JSON with API keys, etc.
            $table->json('field_mapping')->nullable(); // Form field to provider mapping
            $table->json('conditional_logic')->nullable(); // When to execute
            $table->boolean('active')->default(true)->index();
            $table->unsignedSmallInteger('priority')->default(10); // Execution order
            $table->timestamp('last_run_at')->nullable();
            $table->text('last_error')->nullable();
            $table->unsignedInteger('success_count')->default(0);
            $table->unsignedInteger('failure_count')->default(0);
            $table->timestamps();
            $table->softDeletes();

            // Indexes for common queries
            $table->index(['form_id', 'active']);
            $table->index(['integration_type', 'provider']);
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('form_integrations');
    }
};
